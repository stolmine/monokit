# Cross-Platform Audio Device Selection

## Overview

Design notes for future cross-platform audio support in monokit. Initial implementation targets macOS only.

---

## Platform Audio Landscape

| Platform | Backend | Device Listing | Notes |
|----------|---------|----------------|-------|
| **macOS** | CoreAudio | `ServerOptions.outDevices` | Works well, clear device names |
| **Linux** | JACK | JACK manages routing | No "device" selection - use JACK connections |
| **Linux** | PulseAudio | `ServerOptions.outDevices` | Works but names can be cryptic |
| **Linux** | ALSA | `ServerOptions.outDevices` | Hardware devices, complex names |
| **Linux** | PipeWire | Via JACK or Pulse API | Depends on compatibility layer |
| **Windows** | ASIO | `ServerOptions.outDevices` | Pro audio, requires drivers |
| **Windows** | WASAPI | `ServerOptions.outDevices` | Built-in, higher latency |

---

## Proposed Abstraction

```
┌─────────────────────────────────────────────────────────┐
│                    Rust (monokit)                       │
├─────────────────────────────────────────────────────────┤
│  AudioConfig                                            │
│  ├── backend: AudioBackend (CoreAudio/JACK/Pulse/ASIO) │
│  ├── output_device: Option<String>                      │
│  ├── sample_rate: u32                                   │
│  └── buffer_size: u32                                   │
├─────────────────────────────────────────────────────────┤
│  Environment Variables to SC                            │
│  ├── MONOKIT_AUDIO_BACKEND                              │
│  ├── MONOKIT_AUDIO_OUT                                  │
│  ├── MONOKIT_SAMPLE_RATE                                │
│  └── MONOKIT_BUFFER_SIZE                                │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              SC Script (monokit_server.scd)             │
├─────────────────────────────────────────────────────────┤
│  Platform detection + backend selection                 │
│  ├── macOS: Always CoreAudio                            │
│  ├── Linux: Check MONOKIT_AUDIO_BACKEND env             │
│  │   └── Default: Try JACK → PulseAudio → ALSA          │
│  └── Windows: Check MONOKIT_AUDIO_BACKEND env           │
│      └── Default: ASIO if available, else WASAPI        │
└─────────────────────────────────────────────────────────┘
```

---

## SC Script Cross-Platform Logic (Future)

```supercollider
(
var platform = thisProcess.platform.name;
var backend = "MONOKIT_AUDIO_BACKEND".getenv;
var outDev = "MONOKIT_AUDIO_OUT".getenv;
var sampleRate = "MONOKIT_SAMPLE_RATE".getenv;
var bufferSize = "MONOKIT_BUFFER_SIZE".getenv;

// Platform-specific defaults
case
{ platform == \osx } {
    // macOS: CoreAudio is the only option
    if(outDev.notNil && (outDev.size > 0), {
        s.options.outDevice = outDev;
    });
}
{ platform == \linux } {
    // Linux: Multiple backends possible
    case
    { backend == "jack" } {
        s.options.device = "JackRouter";
        // JACK handles routing externally
    }
    { backend == "pulse" } {
        s.options.device = "pulse";
        if(outDev.notNil, { s.options.outDevice = outDev });
    }
    { backend == "alsa" } {
        if(outDev.notNil, { s.options.outDevice = outDev });
    }
    { true } {
        // Auto-detect: try JACK first
        if(Server.default.options.device.isNil, {
            "MONOKIT: Auto-detecting audio backend...".postln;
        });
    };
}
{ platform == \windows } {
    // Windows: ASIO preferred
    case
    { backend == "asio" } {
        s.options.device = "ASIO";
        if(outDev.notNil, { s.options.outDevice = outDev });
    }
    { backend == "wasapi" } {
        // WASAPI is default if no ASIO
        if(outDev.notNil, { s.options.outDevice = outDev });
    }
    { true } {
        // Auto: try ASIO, fall back to default
        if(ServerOptions.outDevices.any({ |d| d.containsi("ASIO") }), {
            s.options.device = "ASIO";
        });
        if(outDev.notNil, { s.options.outDevice = outDev });
    };
};

// Sample rate and buffer size (all platforms)
if(sampleRate.notNil, { s.options.sampleRate = sampleRate.asInteger });
if(bufferSize.notNil, { s.options.hardwareBufferSize = bufferSize.asInteger });
```

---

## Rust Types (Future)

```rust
// src/audio_config.rs

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioBackend {
    // macOS
    CoreAudio,
    // Linux
    Jack,
    PulseAudio,
    Alsa,
    PipeWire,
    // Windows
    Asio,
    Wasapi,
    // Auto-detect
    Auto,
}

impl Default for AudioBackend {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        return AudioBackend::CoreAudio;

        #[cfg(target_os = "linux")]
        return AudioBackend::Auto;  // Try JACK → Pulse → ALSA

        #[cfg(target_os = "windows")]
        return AudioBackend::Auto;  // Try ASIO → WASAPI
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioConfig {
    pub backend: AudioBackend,
    pub output_device: Option<String>,
    pub sample_rate: Option<u32>,
    pub buffer_size: Option<u32>,
}

impl AudioConfig {
    /// Convert to environment variables for SC
    pub fn to_env_vars(&self) -> Vec<(String, String)> {
        let mut vars = vec![];

        let backend_str = match self.backend {
            AudioBackend::CoreAudio => "coreaudio",
            AudioBackend::Jack => "jack",
            AudioBackend::PulseAudio => "pulse",
            AudioBackend::Alsa => "alsa",
            AudioBackend::PipeWire => "pipewire",
            AudioBackend::Asio => "asio",
            AudioBackend::Wasapi => "wasapi",
            AudioBackend::Auto => "auto",
        };
        vars.push(("MONOKIT_AUDIO_BACKEND".into(), backend_str.into()));

        if let Some(ref dev) = self.output_device {
            vars.push(("MONOKIT_AUDIO_OUT".into(), dev.clone()));
        }
        if let Some(rate) = self.sample_rate {
            vars.push(("MONOKIT_SAMPLE_RATE".into(), rate.to_string()));
        }
        if let Some(size) = self.buffer_size {
            vars.push(("MONOKIT_BUFFER_SIZE".into(), size.to_string()));
        }

        vars
    }
}
```

---

## Commands (Future Full Set)

```
AUDIO.OUT              # List output devices (platform-aware)
AUDIO.OUT <device>     # Set output device
AUDIO.BACKEND          # Show current backend
AUDIO.BACKEND <name>   # Set backend (Linux/Windows only)
AUDIO.RATE             # Show sample rate
AUDIO.RATE <hz>        # Set sample rate (e.g., 44100, 48000)
AUDIO.BUF              # Show buffer size
AUDIO.BUF <samples>    # Set buffer size (e.g., 256, 512, 1024)
```

---

## Implementation Phases

### Phase A: macOS Only (Current Target)
- CoreAudio backend only
- `AUDIO.OUT` command for device selection
- Device passed via `MONOKIT_AUDIO_OUT` env var
- Config persistence

### Phase B: Linux Support
- Add backend selection (JACK/Pulse/ALSA)
- JACK special handling (external routing)
- Auto-detection logic

### Phase C: Windows Support
- Add ASIO/WASAPI backends
- ASIO driver detection

### Phase D: Advanced Settings
- Sample rate selection
- Buffer size selection
- Latency reporting

---

## Platform-Specific Notes

### macOS
- CoreAudio is the only backend
- `ServerOptions.outDevices` returns clean device names
- Device names are stable across reboots

### Linux with JACK
- JACK is a routing layer, not a device selector
- SC connects to JACK, user routes in JACK patchbay (qjackctl, Carla, etc.)
- `s.options.device = "JackRouter"` or auto-detected
- No device list meaningful - JACK handles routing

### Linux with PulseAudio
- Device names can be cryptic UUIDs
- May need to parse `pactl list sinks` for friendly names
- Consider showing both name and description

### Linux with PipeWire
- Modern replacement for JACK + Pulse
- Can present as JACK or PulseAudio to SC
- Routing via `wpctl` or GUI tools

### Windows with ASIO
- Requires ASIO drivers (RME, Focusrite, ASIO4ALL, etc.)
- Lowest latency option
- Device names from driver

### Windows with WASAPI
- Built-in Windows audio
- Higher latency but no driver needed
- Good fallback

---

## References

- [SC ServerOptions](https://doc.sccode.org/Classes/ServerOptions.html)
- [SC Platform detection](https://doc.sccode.org/Classes/Platform.html)
- [JACK Audio Connection Kit](https://jackaudio.org/)
- [PipeWire](https://pipewire.org/)
