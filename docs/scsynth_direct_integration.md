# Direct scsynth Integration Plan

## 1. Overview

**Goal:** Eliminate sclang dependency by spawning scsynth directly and communicating via OSC.

**Why:**
- Reduce bundle size from ~200 MB (full SuperCollider) to ~13 MB (scsynth + plugins)
- Faster startup (no sclang interpretation layer)
- Simplified installation (single self-contained bundle)
- Follows Sonic Pi's proven architecture

**Current State:** Monokit spawns sclang, which boots scsynth and loads SynthDefs from `monokit_server.scd`

**Target State:** Monokit spawns scsynth directly, loads pre-compiled `.scsyndef` files, communicates exclusively via OSC

---

## 2. Current Architecture

**Rust Side (`src/sc_process.rs`):**
- Spawns sclang with `monokit_server.scd` as argument
- Sends graceful shutdown via stdin: `Server.quitAll; 0.exit;`
- Listens for `/monokit/ready` OSC message from sclang
- Finds sclang in standard locations or via `which`

**SuperCollider Side (`sc/monokit_server.scd`):**
- Boots scsynth via `s.waitForBoot`
- Compiles 3 SynthDefs: `\monokit`, `\monokit_spectrum`, `\monokit_scope`
- Sets up OSC responders for parameter control
- Forwards meter/spectrum/scope data to Rust on port 57121
- Handles recording via `s.record()` / `s.stopRecording`
- Queries available audio devices via `ServerOptions.outDevices`

**Communication Flow:**
```
Rust (port 57120) → sclang → scsynth
scsynth → sclang (SendPeakRMS/SendReply) → Rust (port 57121)
```

---

## 3. Target Architecture

**Direct Spawn:**
```
Rust (port 57120) → scsynth → Rust (port 57121)
```

**Boot Sequence:**
1. Rust spawns scsynth with command-line args
2. Rust sends `/notify 1` to register as OSC client
3. Rust sends `/d_load` to load pre-compiled `.scsyndef` files
4. Rust sends `/s_new` to instantiate synths
5. Rust sends `/monokit/ready` to itself (or just proceeds)

**SynthDef Loading:**
- SynthDefs compiled offline to `.scsyndef` binary format
- Stored in `sc/synthdefs/` directory
- Loaded via `/d_load` OSC message at boot

---

## 4. Phase 1: SynthDef Pre-compilation

**Objective:** Compile SynthDefs to binary `.scsyndef` files at build time

### Implementation

**Create compilation script (`sc/compile_synthdefs.sh`):**
```bash
#!/bin/bash
cd "$(dirname "$0")"
sclang -D compile_synthdefs.scd
```

**Create compilation scd (`sc/compile_synthdefs.scd`):**
```supercollider
(
// Copy SynthDef definitions from monokit_server.scd
// Replace .add with .writeDefFile("synthdefs/")

SynthDef(\monokit, { /* ... */ }).writeDefFile("synthdefs/");
SynthDef(\monokit_spectrum, { /* ... */ }).writeDefFile("synthdefs/");
SynthDef(\monokit_scope, { /* ... */ }).writeDefFile("synthdefs/");

"SynthDefs compiled to synthdefs/".postln;
0.exit;
)
```

**Integrate with build system:**
- Add to `build.rs` (Rust build script) or Makefile
- Run compilation before cargo build
- Check for sclang availability (graceful degradation if missing)

**Output location:**
```
sc/synthdefs/monokit.scsyndef
sc/synthdefs/monokit_spectrum.scsyndef
sc/synthdefs/monokit_scope.scsyndef
```

**Testing:**
- Verify `.scsyndef` files created with correct size (>1KB each)
- Confirm files load successfully in scsynth

---

## 5. Phase 2: Direct scsynth Spawning

**Objective:** Modify `src/sc_process.rs` to spawn scsynth instead of sclang

### Command-Line Args

```bash
scsynth -u 57110 -a 2 -i 0 -o 2 -z 128 -Z 0 -D 0 -R 0 -l 1
```

**Key args:**
- `-u 57110` - UDP port for OSC communication
- `-a 2` - Maximum audio API channels (2=stereo)
- `-i 0` - Input channels (0=no input, avoids sample rate mismatch)
- `-o 2` - Output channels (2=stereo)
- `-z 128` - Audio block size (128 samples default)
- `-Z 0` - Hardware buffer size (0=use default)
- `-D <index>` - Audio device index (platform-specific)
- `-R 0` - Disable loading all default SynthDefs
- `-l 1` - Maximum number of logins (1=single client)

**Optional args:**
- `-U <path>` - Plugin directory path (for bundled plugins)
- `-H <path>` - Hardware device name (macOS CoreAudio)

### OSC Boot Sequence

**1. Register as OSC client:**
```rust
// Send to scsynth on port 57110
osc_client.send("/notify", vec![1])?;
```

**2. Load SynthDefs:**
```rust
// Absolute path to synthdefs directory
let synthdefs_dir = find_synthdefs_dir()?;
osc_client.send("/d_loadDir", vec![synthdefs_dir.to_str().unwrap()])?;
```

**3. Wait for `/done /d_loadDir` response**
- scsynth sends confirmation when SynthDefs loaded
- Timeout after 5 seconds

**4. Instantiate synths:**
```rust
// /s_new <defName> <nodeID> <addAction> <targetID> [<paramName> <value>...]
osc_client.send("/s_new", vec!["monokit", 1000, 0, 0])?;
osc_client.send("/s_new", vec!["monokit_spectrum", 1001, 1, 0])?; // addToTail
osc_client.send("/s_new", vec!["monokit_scope", 1002, 1, 0])?;
```

**5. Ready to accept parameter messages**

### Error Handling

- **scsynth not found:** Check standard locations, return clear error
- **Boot timeout:** Wait 10 seconds for `/done` messages, fail gracefully
- **SynthDef load failure:** Check for `/fail` OSC response, report missing files
- **Audio device failure:** Detect error output, suggest valid devices

### Timeout Values
- scsynth spawn: 2 seconds to start process
- `/notify` response: 3 seconds
- `/d_loadDir` response: 5 seconds
- Total boot timeout: 10 seconds

---

## 6. Phase 3: OSC Message Routing

**Challenge:** SendPeakRMS and SendReply send to sclang's notify client, not arbitrary ports

### Current Flow (via sclang)

```
scsynth SendPeakRMS → sclang OSCdef → NetAddr.sendMsg → Rust (57121)
scsynth SendReply → sclang OSCdef → NetAddr.sendMsg → Rust (57121)
```

### Solution: Direct OSC Reply

**Problem:** SendPeakRMS/SendReply use `/reply` address to registered client

**Fix:** Register Rust as notify client, handle `/reply` messages directly

**Implementation:**

1. **Rust binds to port 57121 and sends `/notify` from that port**
   - scsynth will reply to the port that sent `/notify`
   - Ensure Rust OSC client listens on 57121

2. **Parse `/reply` messages in Rust:**
   ```rust
   // /reply <cmdName> <nodeID> <replyID> [<values>...]
   match cmd_name {
       "/monokit/meter" => handle_meter(msg),
       "/monokit/spectrum" => handle_spectrum(msg),
       "/monokit/scope" => handle_scope(msg),
       _ => {} // ignore
   }
   ```

3. **Update SynthDefs to use SendReply instead of SendPeakRMS:**
   - SendPeakRMS is a convenience wrapper around SendReply
   - Use explicit SendReply with custom reply addresses
   - Example: `SendReply.kr(Impulse.kr(20), '/monokit/meter', [peak, rms])`

**Alternative: Dedicated Reply Port**
- scsynth `-R <port>` flag can set reply port (if supported)
- Would simplify message routing
- Check scsynth version compatibility

---

## 7. Phase 4: Recording

**Current Approach:** `s.record()` / `s.stopRecording` (server-side recording via sclang)

### Solution: DiskOut UGen

**Replace:** Server.record with DiskOut UGen in SynthDef

**Implementation:**

1. **Add DiskOut to monokit SynthDef:**
   ```supercollider
   arg diskOutBus = 100, diskOutActive = 0;

   Out.kr(diskOutBus, [sigL, sigR] * diskOutActive);
   ```

2. **Recording control via OSC:**
   - `/b_alloc` - Allocate buffer for recording
   - `/b_write` - Start writing buffer to disk
   - `/b_free` - Free buffer when done

3. **Rust-side recording logic:**
   ```rust
   // Start recording
   osc.send("/b_alloc", vec![bufnum, frames, channels, path])?;
   osc.send("/s_set", vec![1000, "diskOutActive", 1])?;

   // Stop recording
   osc.send("/s_set", vec![1000, "diskOutActive", 0])?;
   osc.send("/b_close", vec![bufnum])?;
   osc.send("/b_free", vec![bufnum])?;
   ```

**Pros:**
- No sclang dependency
- Full control over format/path

**Cons:**
- More complex OSC sequencing
- Buffer management required
- No automatic file naming (must generate in Rust)

**File Format:**
- WAV int24 (current format)
- Generate timestamped filenames in Rust: `monokit_YYYYMMDD_HHMMSS.wav`

---

## 8. Phase 5: Audio Device Handling

**Current:** Environment variable `MONOKIT_AUDIO_OUT` sets device name

**Challenge:** scsynth uses device *index*, not name

### Device Enumeration

**macOS CoreAudio:**
- Use `coreaudio-rs` crate to list devices
- Match device name to index
- Pass index to scsynth via `-D <index>`

**Linux ALSA/JACK:**
- Query via platform APIs
- Map device names to indices

**Implementation:**

1. **Create `src/audio_devices.rs` module:**
   ```rust
   pub fn list_audio_devices() -> Vec<AudioDevice> { /* ... */ }
   pub fn find_device_index(name: &str) -> Option<usize> { /* ... */ }
   ```

2. **Update `sc_process.rs`:**
   ```rust
   let device_index = if let Some(name) = audio_out_device {
       audio_devices::find_device_index(name)
           .ok_or("Device not found")?
   } else {
       0  // Default device
   };

   cmd.arg("-D").arg(device_index.to_string());
   ```

3. **Device switching:**
   - Requires restarting scsynth (no hot-swap)
   - Current restart logic in `restart_with_device()` still applies

**Fallback:**
- If device enumeration fails, use index 0 (default)
- Log warning to user

---

## 9. Phase 6: Bundling

**Goal:** Self-contained distribution with scsynth + plugins

### Files to Bundle

**From SuperCollider.app (macOS):**
```
SuperCollider.app/Contents/Resources/scsynth
SuperCollider.app/Contents/Resources/plugins/ (directory)
```

**Monokit files:**
```
monokit                           # Rust binary
sc/synthdefs/monokit.scsyndef
sc/synthdefs/monokit_spectrum.scsyndef
sc/synthdefs/monokit_scope.scsyndef
```

**Total bundle structure:**
```
monokit-0.2.0-aarch64-apple-darwin/
  monokit
  scsynth
  plugins/
    SVF.scx
    FreeVerb2.scx
    (other required .scx files)
  synthdefs/
    monokit.scsyndef
    monokit_spectrum.scsyndef
    monokit_scope.scsynth
```

### Size Estimates

- scsynth binary: ~5 MB
- Required plugins: ~2-3 MB (SVF, FreeVerb2, core UGens)
- SynthDefs: ~10 KB
- Monokit binary: ~5 MB

**Total:** ~13 MB (vs ~200 MB for full SuperCollider)

### Code Signing (macOS)

**Challenge:** scsynth binary is already signed by SuperCollider

**Options:**

1. **Resign with ad-hoc signature:**
   ```bash
   codesign --force --deep --sign - scsynth
   ```

2. **Use developer certificate:**
   ```bash
   codesign --force --deep --sign "Developer ID Application: ..." scsynth
   ```

3. **Notarization:**
   - Required for macOS Gatekeeper
   - Apple Developer account needed
   - Use `xcrun notarytool` for automated submission

**Recommendation:** Start with ad-hoc signing for testing, move to proper signing for public release

### Plugin Discovery

**Update scsynth spawn args:**
```bash
scsynth -u 57110 -U /path/to/bundled/plugins ...
```

**Path resolution:**
```rust
let plugins_dir = exe_dir.join("plugins");
if plugins_dir.exists() {
    cmd.arg("-U").arg(plugins_dir);
}
```

### Homebrew Formula Changes

**Current formula (simplified):**
```ruby
depends_on "supercollider"  # Optional dependency
```

**New formula (self-contained):**
```ruby
# No SuperCollider dependency!

def install
  bin.install "monokit"
  pkgshare.install "scsynth"
  pkgshare.install "plugins"
  pkgshare.install "synthdefs"
end
```

**Caveats:**
```ruby
def caveats
  <<~EOS
    Monokit includes a bundled scsynth.
    No SuperCollider installation required.
  EOS
end
```

---

## 10. Feature Flag Implementation

**Goal:** Enable/disable scsynth-only mode, fallback to sclang

### Cargo Feature Flag

**Cargo.toml:**
```toml
[features]
default = ["scsynth-direct"]
scsynth-direct = []
sclang-compat = []
```

### Conditional Compilation

**src/sc_process.rs:**
```rust
impl ScProcess {
    pub fn start(&mut self, audio_out_device: Option<&str>) -> Result<(), String> {
        #[cfg(feature = "scsynth-direct")]
        return self.start_scsynth_direct(audio_out_device);

        #[cfg(not(feature = "scsynth-direct"))]
        return self.start_sclang(audio_out_device);
    }

    #[cfg(feature = "scsynth-direct")]
    fn start_scsynth_direct(&mut self, device: Option<&str>) -> Result<(), String> {
        // New scsynth-only boot sequence
    }

    #[cfg(not(feature = "scsynth-direct"))]
    fn start_sclang(&mut self, device: Option<&str>) -> Result<(), String> {
        // Current sclang boot sequence
    }
}
```

### Runtime Environment Variable

**Alternative approach:**
```rust
let use_scsynth = env::var("MONOKIT_USE_SCSYNTH").is_ok();

if use_scsynth {
    self.start_scsynth_direct(device)?;
} else {
    self.start_sclang(device)?;
}
```

**Recommendation:** Use cargo feature flag for clean separation, env var for testing

---

## 11. Testing Plan

### Phase 1 Testing
- [x] `.scsyndef` files compile successfully
- [x] Files have reasonable size (>1KB)
- [x] SynthDef definitions match original

### Phase 2 Testing
- [ ] scsynth spawns without error
- [ ] `/notify` response received within 3 seconds
- [ ] `/d_loadDir` loads all 3 SynthDefs
- [ ] `/s_new` instantiates synths without error
- [ ] scsynth stays running (no immediate crash)

### Phase 3 Testing
- [ ] Meter data arrives on port 57121
- [ ] Spectrum data arrives with correct format
- [ ] Scope waveform data arrives
- [ ] CPU percentage data arrives
- [ ] Message parsing handles all formats correctly

### Phase 4 Testing
- [ ] Recording starts without error
- [ ] WAV file created with correct format (int24)
- [ ] Recording stops cleanly
- [ ] No buffer overruns or dropouts
- [ ] Timestamped filenames generated correctly

### Phase 5 Testing
- [ ] Audio device enumeration returns valid list
- [ ] Device name → index mapping works
- [ ] scsynth uses correct audio device
- [ ] Device switching (restart) works correctly
- [ ] Default device fallback works

### Phase 6 Testing
- [ ] Bundle extracts to correct locations
- [ ] scsynth finds bundled plugins
- [ ] SynthDefs load from bundled location
- [ ] Code signing doesn't break execution
- [ ] Homebrew formula installs correctly
- [ ] Bundle size is ~13 MB

### Integration Tests

**Test all phases together:**
- [ ] Boot scsynth-only monokit
- [ ] Send trigger command → hear sound
- [ ] Adjust parameters → sound changes
- [ ] Record audio → WAV file created
- [ ] Switch audio device → scsynth restarts
- [ ] Meters/spectrum/scope display correctly
- [ ] Graceful shutdown → no orphaned processes

### Platform Testing

**macOS:**
- [ ] Apple Silicon (M1/M2/M3)
- [ ] Intel (x86_64)
- [ ] macOS 12+ (Monterey and later)

**Linux (future):**
- [ ] ALSA backend
- [ ] JACK backend
- [ ] PipeWire backend

---

## 12. Risks and Mitigations

### Risk: SendPeakRMS/SendReply routing fails

**Impact:** No meter/spectrum/scope data

**Mitigation:**
- Test `/reply` message handling thoroughly
- Add verbose logging for OSC message flow
- Consider polling alternatives (control bus queries)

### Risk: DiskOut recording is unreliable

**Impact:** Recording feature breaks

**Mitigation:**
- Test buffer allocation carefully
- Add retry logic for buffer commands
- Consider external recording tools (sox, ffmpeg) as fallback

### Risk: Audio device enumeration is platform-specific

**Impact:** Device switching breaks on some platforms

**Mitigation:**
- Start with macOS only (current platform)
- Add Linux support incrementally
- Use default device (index 0) as safe fallback

### Risk: Bundle size exceeds expectations

**Impact:** Download/install takes longer

**Mitigation:**
- Strip debug symbols from binaries
- Include only essential plugins
- Compress tarball with xz (better than gzip)

### Risk: Code signing issues on macOS

**Impact:** Users hit Gatekeeper warnings

**Mitigation:**
- Document how to bypass Gatekeeper (Ctrl+Click, "Open")
- Invest in proper Apple Developer account for notarization
- Provide unsigned alternative for power users

### Risk: UGen graph complexity hits limits

**Impact:** SynthDef fails to load or crashes scsynth

**Mitigation:**
- Monitor UGen count in SynthDefs (currently ~100 UGens)
- Profile scsynth CPU usage (currently ~5-10%)
- Consider splitting into multiple synths if needed

### Risk: Plugin dependencies missing

**Impact:** SynthDef fails with "UGen not found" error

**Mitigation:**
- Audit all UGens used (SVF, FreeVerb2, etc.)
- Bundle complete plugin set for those UGens
- Test on clean system without SuperCollider installed

---

## Implementation Timeline

**Phase 1 (SynthDef Compilation):** 2-3 days
- Create compilation scripts
- Integrate with build system
- Verify output files

**Phase 2 (Direct Spawning):** 1 week
- Refactor `sc_process.rs`
- Implement OSC boot sequence
- Error handling and timeouts

**Phase 3 (OSC Routing):** 1 week
- Modify SynthDefs for direct replies
- Update Rust OSC message parsing
- Test all data paths (meter/spectrum/scope)

**Phase 4 (Recording):** 1 week
- Implement DiskOut approach
- Buffer management
- File naming and format

**Phase 5 (Device Handling):** 3-4 days
- Audio device enumeration
- Index mapping
- Test device switching

**Phase 6 (Bundling):** 1 week
- Extract scsynth and plugins
- Create bundle structure
- Code signing
- Homebrew formula updates

**Total Estimate:** 5-6 weeks

---

## Success Criteria

- [ ] Monokit boots without sclang installed
- [ ] All audio features work (synth, FX, recording)
- [ ] All visualization features work (meters, spectrum, scope)
- [ ] Audio device selection works
- [ ] Bundle size is ≤15 MB
- [ ] Startup time improves by ≥30%
- [ ] No regression in audio quality or stability
- [ ] Homebrew installation works end-to-end
- [ ] All 558 existing tests still pass

---

## References

- **Sonic Pi architecture:** github.com/sonic-pi-net/sonic-pi
- **scsynth command-line args:** SuperCollider documentation
- **OSC protocol:** SuperCollider Server Command Reference
- **coreaudio-rs crate:** docs.rs/coreaudio
- **macOS code signing:** developer.apple.com/documentation/security/notarizing_macos_software_before_distribution
