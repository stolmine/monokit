# Audio Device Selection & SC Lifecycle Management

## Overview

Implementation plan for self-contained monokit binary that:
1. Spawns and manages sclang as a child process
2. Supports `AUDIO.OUT` command for runtime device selection
3. Handles device changes via sclang restart with environment variables

**Goal:** User runs `monokit` - everything else is invisible to them.

---

## Architecture Change

### Current (Manual)
```
Terminal 1: sclang sc/monokit_server.scd
Terminal 2: monokit
```

### New (Self-Contained)
```
Terminal: monokit
         └── spawns sclang internally
         └── passes config via env vars
         └── restarts sclang on device change
```

---

## Key Findings

### sclang Binary Locations (macOS)
```
/Applications/SuperCollider.app/Contents/MacOS/sclang  (standard)
/opt/homebrew/bin/sclang                               (Homebrew ARM)
/usr/local/bin/sclang                                  (Homebrew Intel)
```

### sclang Behavior
- `sclang script.scd` runs script then **hangs** (enters REPL) - this is desired for server
- Environment variables accessible via `"VAR".getenv` in SC
- Server boot takes 2-15 seconds typically

### Ready Detection
- SC can send OSC message when ready: `NetAddr("127.0.0.1", 57121).sendMsg('/monokit/ready')`
- Rust waits for this before proceeding

---

## Phase 1: SC Process Manager

**File:** `src/sc_process.rs` (NEW)

```rust
use std::process::{Command, Child, Stdio};
use std::path::PathBuf;
use std::env;
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::mpsc;

pub struct ScProcess {
    child: Option<Child>,
    sclang_path: PathBuf,
    script_path: PathBuf,
}

impl ScProcess {
    pub fn new() -> Result<Self, String> {
        let sclang_path = find_sclang()?;
        let script_path = find_script()?;

        Ok(Self {
            child: None,
            sclang_path,
            script_path,
        })
    }

    /// Start sclang with optional audio device
    pub fn start(&mut self, audio_out_device: Option<&str>) -> Result<(), String> {
        // Kill existing if running
        self.stop();

        let mut cmd = Command::new(&self.sclang_path);
        cmd.arg(&self.script_path);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Pass device via environment variable
        if let Some(device) = audio_out_device {
            cmd.env("MONOKIT_AUDIO_OUT", device);
        }

        let child = cmd.spawn()
            .map_err(|e| format!("Failed to spawn sclang: {}", e))?;

        self.child = Some(child);
        Ok(())
    }

    /// Stop sclang process
    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    /// Restart with new device
    pub fn restart_with_device(&mut self, device: &str) -> Result<(), String> {
        self.start(Some(device))
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }
}

impl Drop for ScProcess {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Find sclang binary on macOS
fn find_sclang() -> Result<PathBuf, String> {
    let candidates = [
        "/Applications/SuperCollider.app/Contents/MacOS/sclang",
        "/opt/homebrew/bin/sclang",
        "/usr/local/bin/sclang",
    ];

    for path in candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    // Try PATH
    if let Ok(output) = Command::new("which").arg("sclang").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    Err("sclang not found. Install SuperCollider.".to_string())
}

/// Find monokit_server.scd
fn find_script() -> Result<PathBuf, String> {
    // Check relative to executable
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let script = dir.join("sc/monokit_server.scd");
            if script.exists() {
                return Ok(script);
            }
            // Also check parent (for cargo run)
            if let Some(parent) = dir.parent() {
                let script = parent.join("sc/monokit_server.scd");
                if script.exists() {
                    return Ok(script);
                }
            }
        }
    }

    // Check current directory
    let cwd = PathBuf::from("sc/monokit_server.scd");
    if cwd.exists() {
        return Ok(cwd);
    }

    // Check home directory
    if let Some(home) = dirs::home_dir() {
        let script = home.join(".monokit/monokit_server.scd");
        if script.exists() {
            return Ok(script);
        }
    }

    Err("monokit_server.scd not found".to_string())
}
```

---

## Phase 2: SC Script Modifications

**File:** `sc/monokit_server.scd`

Add at the beginning (after the opening paren):

```supercollider
(
// Read audio device from environment variable
var outDev = "MONOKIT_AUDIO_OUT".getenv;

if(outDev.notNil && (outDev.size > 0), {
    s.options.outDevice = outDev;
    ("MONOKIT: Using output device: " ++ outDev).postln;
}, {
    "MONOKIT: Using default output device".postln;
});

// Disable input by default (prevents sample rate mismatch issues)
s.options.numInputBusChannels = 0;

s.waitForBoot {
    // ... existing SynthDef code ...
```

Add after synths are created (before the closing of waitForBoot):

```supercollider
    // Signal ready to Rust
    ~meterAddr = NetAddr("127.0.0.1", 57121);
    ~meterAddr.sendMsg('/monokit/ready');
    "MONOKIT: Server ready, sent /monokit/ready".postln;
```

Add OSC responder for device query (near other OSCdefs):

```supercollider
// Audio device query
OSCdef(\monokit_audio_out_list, { |msg|
    var devices = ServerOptions.outDevices;
    var current = s.options.outDevice ? "default";
    ~meterAddr.sendMsg('/monokit/audio/out/list', devices.size, current, *devices);
}, '/monokit/audio/out/list');
```

---

## Phase 3: Ready Detection in Meter Thread

**File:** `src/meter.rs`

Add to message parsing:

```rust
} else if msg.addr == "/monokit/ready" {
    let _ = event_tx.send(MetroEvent::ScReady);
} else if msg.addr == "/monokit/audio/out/list" {
    if let Some((current, devices)) = parse_audio_device_list(&msg.args) {
        let _ = event_tx.send(MetroEvent::AudioOutDeviceList { current, devices });
    }
}
```

**File:** `src/types.rs`

Add MetroEvent variants:

```rust
ScReady,
AudioOutDeviceList {
    current: String,
    devices: Vec<String>,
},
```

---

## Phase 4: Main.rs Integration

**File:** `src/main.rs`

Add SC process management:

```rust
mod sc_process;
use sc_process::ScProcess;

fn main() -> Result<()> {
    // ... arg parsing ...

    // Load config to get saved audio device
    let config = config::load_config().unwrap_or_default();
    let audio_device = config.display.audio_out_device.clone();

    // Start SuperCollider
    let mut sc_process = match ScProcess::new() {
        Ok(sc) => sc,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            eprintln!("Please install SuperCollider from https://supercollider.github.io");
            std::process::exit(1);
        }
    };

    if let Err(e) = sc_process.start(audio_device.as_deref()) {
        eprintln!("ERROR: Failed to start SuperCollider: {}", e);
        std::process::exit(1);
    }

    // Create channels
    let (metro_tx, metro_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();

    // Spawn meter thread (receives SC messages including /monokit/ready)
    let meter_event_tx = event_tx.clone();
    thread::spawn(move || meter::meter_thread(meter_event_tx));

    // Wait for SC ready (with timeout)
    let sc_ready = wait_for_sc_ready(&event_rx, Duration::from_secs(20));
    if !sc_ready {
        eprintln!("ERROR: SuperCollider failed to start within 20 seconds");
        std::process::exit(1);
    }

    // Now spawn metro thread (SC is ready)
    // ... rest of existing startup ...
}

fn wait_for_sc_ready(event_rx: &mpsc::Receiver<MetroEvent>, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        match event_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(MetroEvent::ScReady) => return true,
            Ok(_) => continue,  // Other events, keep waiting
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => return false,
        }
    }
    false
}
```

---

## Phase 5: AUDIO.OUT Command

**File:** `src/commands/system/audio.rs` (NEW)

```rust
use crate::types::MetroCommand;
use anyhow::Result;
use std::sync::mpsc::Sender;

pub fn handle_audio_out<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        // Query mode
        metro_tx.send(MetroCommand::QueryAudioOutDevices)?;
        output("QUERYING OUTPUT DEVICES...".to_string());
    } else {
        // Set mode - restart SC with new device
        let device_name = parts[1..].join(" ");
        metro_tx.send(MetroCommand::SetAudioOutDevice(device_name.clone()))?;
        output(format!("SETTING OUTPUT: {}", device_name.to_uppercase()));
        output("RESTARTING AUDIO ENGINE...".to_string());
    }
    Ok(())
}
```

**File:** `src/types.rs`

Add MetroCommand variants:

```rust
QueryAudioOutDevices,
SetAudioOutDevice(String),
```

---

## Phase 6: SC Restart Logic

The `SetAudioOutDevice` command needs to trigger SC restart. This requires the SC process to be accessible from where commands are processed.

**Option A: Pass ScProcess via Arc<Mutex<>>**

```rust
// In main.rs
let sc_process = Arc::new(Mutex::new(sc_process));

// Pass to App
app.sc_process = Some(sc_process.clone());

// In command handler or event processing
MetroEvent::RestartScWithDevice(device) => {
    if let Some(ref sc) = app.sc_process {
        let mut sc = sc.lock().unwrap();
        sc.restart_with_device(&device)?;
    }
}
```

**Option B: Send restart command via channel to main loop**

Add `MetroEvent::RestartSc(String)` and handle in main event loop where ScProcess is owned.

---

## Phase 7: Config Persistence

**File:** `src/config.rs`

Add to DisplayConfig:

```rust
#[serde(default)]
pub audio_out_device: Option<String>,
```

Save on successful device change, load on startup to pass to SC.

---

## Phase 8: Help System

**File:** `src/help_content.rs`

```rust
"",
"# AUDIO OUTPUT",
"  AUDIO.OUT         LIST OUTPUT DEVICES",
"  AUDIO.OUT <NAME>  SET OUTPUT DEVICE",
"  AUDIO             ALIAS FOR AUDIO.OUT",
"",
"  DEVICE CHANGE RESTARTS AUDIO",
"  SAVED TO CONFIG FOR NEXT LAUNCH",
"",
```

---

## Implementation Order

1. **Phase 1:** Create `src/sc_process.rs` - sclang spawn/kill/restart
2. **Phase 2:** Modify `sc/monokit_server.scd` - env var reading, ready signal
3. **Phase 3:** Modify `src/meter.rs` - parse /monokit/ready
4. **Phase 4:** Modify `src/main.rs` - spawn SC, wait for ready
5. **Phase 5:** Create `src/commands/system/audio.rs` - AUDIO.OUT handler
6. **Phase 6:** Wire up SC restart on device change
7. **Phase 7:** Add config persistence
8. **Phase 8:** Update help

---

## File Summary

| File | Action |
|------|--------|
| `src/sc_process.rs` | **Create** - SC lifecycle management |
| `sc/monokit_server.scd` | Modify - env var, ready signal, device query |
| `src/meter.rs` | Modify - parse /monokit/ready, device list |
| `src/types.rs` | Modify - Add ScReady, AudioOutDeviceList events |
| `src/main.rs` | Modify - spawn SC, wait for ready |
| `src/commands/system/audio.rs` | **Create** - AUDIO.OUT handler |
| `src/commands/system/mod.rs` | Modify - add audio module |
| `src/commands/mod.rs` | Modify - dispatch AUDIO.OUT |
| `src/config.rs` | Modify - audio_out_device field |
| `src/help_content.rs` | Modify - document AUDIO.OUT |
| `src/app/mod.rs` | Modify - hold ScProcess reference |

---

## Testing Checklist

- [ ] `monokit` starts SC automatically
- [ ] Startup waits for SC ready (with progress indicator)
- [ ] Error shown if sclang not found
- [ ] `AUDIO.OUT` lists devices
- [ ] `AUDIO.OUT <device>` restarts SC with new device
- [ ] Device persists to config.toml
- [ ] Next launch uses saved device
- [ ] Ctrl+C cleanly kills SC child process
- [ ] Device names with spaces work

---

## Future: Bundled Distribution

For a fully self-contained release:

1. **Embed .scd as string constant:**
   ```rust
   const MONOKIT_SERVER_SCD: &str = include_str!("../sc/monokit_server.scd");
   ```
   Write to temp file on startup, or ~/.monokit/monokit_server.scd

2. **Bundle scsynth** (harder):
   - Would need to ship scsynth binary + plugins
   - Legal/licensing considerations
   - Platform-specific builds

3. **Homebrew formula:**
   - Depend on `supercollider` cask
   - Install monokit binary + .scd file
