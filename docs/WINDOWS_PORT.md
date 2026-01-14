# Windows Port Plan for Monokit

## Overview

Port monokit to Windows, building on existing macOS and Linux support. The codebase already has partial Windows awareness (plugin path separators, user extensions path) that we can extend.

**Testing Strategy:** Build and test locally via WSL before setting up CI pipeline.

---

## Phase 0: Windows Dependencies

### 0.1 SuperCollider 3.14+

**Download:** https://supercollider.github.io/downloads.html

Install to default location: `C:\Program Files\SuperCollider\`

This provides:
- `scsynth.exe` - audio synthesis server
- `sclang.exe` - language interpreter (for synthdef compilation)
- Core plugins in `plugins/` directory

### 0.2 sc3-plugins

**Download:** https://github.com/supercollider/sc3-plugins/releases

Get: `sc3-plugins-3.13.0-Windows-64bit-VS.zip`

**Install location:** `%LOCALAPPDATA%\SuperCollider\Extensions\`
(typically `C:\Users\<user>\AppData\Local\SuperCollider\Extensions\`)

```powershell
# Extract and install
$ExtPath = "$env:LOCALAPPDATA\SuperCollider\Extensions"
New-Item -ItemType Directory -Force -Path $ExtPath
Expand-Archive sc3-plugins-3.13.0-Windows-64bit-VS.zip -DestinationPath $ExtPath
```

Provides: BMoog, DFM1, SVF filters and other UGens used by monokit.

### 0.3 mi-UGens (Mutable Instruments)

**Download:** https://github.com/v7b1/mi-UGens/releases

Get: `mi-UGens-Windows.zip`

**Install location:** Same as sc3-plugins (`%LOCALAPPDATA%\SuperCollider\Extensions\`)

```powershell
Expand-Archive mi-UGens-Windows.zip -DestinationPath "$env:LOCALAPPDATA\SuperCollider\Extensions"
```

Provides: MiPlaits, MiRings, MiClouds UGens required for Plaits voice, sampler resonator, and granular effects.

### 0.4 ASIO4ALL (Optional, Recommended)

**Download:** https://www.asio4all.org/

Low-latency audio driver. Without this, scsynth uses WASAPI which has higher latency.

---

## Phase 1: Cross-Platform Foundation

### 1.1 Config/Data Path Handling

**Files to modify:**

| File | Lines | Current Issue | Fix |
|------|-------|---------------|-----|
| `src/config.rs` | 272-276 | Uses `HOME` env var, hardcodes `.config/monokit` | Use `dirs::config_dir()` |
| `src/scene.rs` | 44-51 | Duplicates fallback logic with `HOME` | Use `crate::config::monokit_config_dir()` consistently |
| `src/preset/mod.rs` | 33-40 | Same pattern as scene.rs | Use `crate::config::monokit_config_dir()` consistently |
| `src/commands/synth/sampler/utils.rs` | 38 | Hardcodes `.config/monokit/samples` | Use config helper |

**Implementation:**
```rust
// src/config.rs - Replace monokit_config_dir()
pub fn monokit_config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("monokit"))
        .context("Could not determine config directory")
}
```

**Platform paths after fix:**
- macOS: `~/Library/Application Support/monokit/`
- Windows: `%APPDATA%\monokit\`
- Linux: `~/.config/monokit/`

### 1.2 Temporary File Paths

**Files to modify:**

| File | Lines | Current Path | Fix |
|------|-------|--------------|-----|
| `src/metro.rs` | 15 | `/tmp/monokit_osc.log` | `temp_dir().join("monokit_osc.log")` |
| `src/scsynth_direct.rs` | 184 | `/tmp/scsynth.log` | `temp_dir().join("scsynth.log")` |
| `src/app/script_exec/mod.rs` | 16 | `/tmp/monokit_debug.txt` | `temp_dir().join("monokit_debug.txt")` |
| `src/eval/patterns.rs` | 113 | `/tmp/monokit_debug.txt` | `temp_dir().join("monokit_debug.txt")` |
| `src/commands/logging.rs` | 9 | `/tmp/monokit_commands.log` | `temp_dir().join("monokit_commands.log")` |

### 1.3 Binary Discovery - Add Windows Paths

**File: `src/scsynth_direct.rs`**

`find_scsynth()` (lines 907-945) - Add Windows candidates:
```rust
#[cfg(target_os = "windows")]
let system_candidates = [
    r"C:\Program Files\SuperCollider\scsynth.exe",
    r"C:\Program Files (x86)\SuperCollider\scsynth.exe",
];
```

`find_synthdefs_dir()` (lines 947-996) - Add Windows paths for bundled/installed synthdefs.

`find_plugins_dir()` (lines 998-1026) - Add Windows plugin paths:
```rust
#[cfg(target_os = "windows")]
let system_candidates = [
    r"C:\Program Files\SuperCollider\plugins",
    r"C:\ProgramData\SuperCollider\Extensions",
];
```

**File: `src/sc_process.rs`**

`find_sclang()` (lines 193-217) - Add Windows candidates:
```rust
#[cfg(target_os = "windows")]
let candidates = [
    r"C:\Program Files\SuperCollider\sclang.exe",
];
```

### 1.4 PATH Lookup Command

**Files:** `src/scsynth_direct.rs:935-941`, `src/sc_process.rs:207-214`

Replace `which` with platform-specific lookup:
```rust
#[cfg(target_os = "windows")]
let output = Command::new("where").arg("scsynth").output();

#[cfg(not(target_os = "windows"))]
let output = Command::new("which").arg("scsynth").output();
```

### 1.5 Process Termination

**Files:** `src/scsynth_direct.rs:749`, `src/sc_process.rs:112`

Replace `pkill` with platform-specific termination:
```rust
#[cfg(target_os = "windows")]
let _ = Command::new("taskkill")
    .args(["/F", "/IM", "scsynth.exe"])
    .output();

#[cfg(not(target_os = "windows"))]
let _ = Command::new("pkill")
    .args(["-f", "scsynth"])
    .output();
```

---

## Phase 2: Windows Audio Setup

### 2.1 ASIO4ALL (Primary)

scsynth on Windows supports ASIO via `-H` flag. ASIO4ALL is a free universal ASIO driver.

**File: `src/scsynth_direct.rs` lines 154-171**

Add Windows audio handling block:
```rust
#[cfg(target_os = "windows")]
{
    // ASIO4ALL will be the default driver if installed
    // scsynth auto-selects ASIO when available
    // No explicit -H flag needed initially

    if let Some(device) = audio_out_device {
        // If user specifies device, pass it
        cmd.arg("-H").arg(device);
    }
}
```

### 2.2 WASAPI Fallback

If ASIO4ALL is not installed, scsynth falls back to Windows default audio (PortAudio/WASAPI).

No code changes needed for fallback - scsynth handles this automatically.

### 2.3 Device Enumeration (Deferred)

Device enumeration via `cpal` crate is deferred until basic audio works on all platforms. For now:
- Windows: Skip enumeration, use scsynth defaults
- Existing: macOS CoreAudio enumeration, Linux PipeWire routing

**File: `src/audio_devices.rs` lines 148-151**

Update non-macOS stub to be more specific:
```rust
#[cfg(target_os = "linux")]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Err("Linux uses JACK/PipeWire routing instead of device selection".to_string())
}

#[cfg(target_os = "windows")]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Err("Windows audio device enumeration not yet implemented. Using default device.".to_string())
}
```

---

## Phase 3: Local Testing

### 3.1 Build from WSL

```bash
# Install Windows Rust target
rustup target add x86_64-pc-windows-msvc

# Cross-compile (requires linker setup)
cargo build --release --target x86_64-pc-windows-msvc --features scsynth-direct
```

Alternative: Build natively on Windows side:
```powershell
# From Windows, navigate to repo
cd C:\Users\bramm\repos\monokit
cargo build --release --features scsynth-direct
```

### 3.2 Test Prerequisites

Complete Phase 0 dependency installation:
1. SuperCollider 3.14+ installed to `C:\Program Files\SuperCollider\`
2. sc3-plugins extracted to `%LOCALAPPDATA%\SuperCollider\Extensions\`
3. mi-UGens extracted to `%LOCALAPPDATA%\SuperCollider\Extensions\`
4. ASIO4ALL installed (optional, for low latency)

### 3.3 Test Checklist

- [ ] Binary launches without errors
- [ ] scsynth process starts (check Task Manager)
- [ ] Audio output works (play a trigger)
- [ ] Config saves to `%APPDATA%\monokit\`
- [ ] Scenes save/load correctly
- [ ] Presets save/load correctly
- [ ] Sample loading works
- [ ] TUI renders correctly in Windows Terminal

---

## Files Summary

| File | Changes |
|------|---------|
| `src/config.rs` | Replace `monokit_config_dir()` implementation |
| `src/scene.rs` | Remove duplicated fallback logic |
| `src/preset/mod.rs` | Remove duplicated fallback logic |
| `src/commands/synth/sampler/utils.rs` | Use config helper for sample path |
| `src/metro.rs` | Use `temp_dir()` for log path |
| `src/scsynth_direct.rs` | Add Windows paths, temp_dir, taskkill, audio handling |
| `src/sc_process.rs` | Add Windows paths, where command, taskkill |
| `src/commands/logging.rs` | Use `temp_dir()` for log path |
| `src/app/script_exec/mod.rs` | Use `temp_dir()` for debug path |
| `src/eval/patterns.rs` | Use `temp_dir()` for debug path |
| `src/audio_devices.rs` | Add Windows-specific stub message |

---

## Current Status

**Completed:**
- [x] Phase 0: Dependencies downloaded and extracted
  - SuperCollider 3.14.1 → `C:\SuperCollider\SuperCollider\`
  - sc3-plugins → `%LOCALAPPDATA%\SuperCollider\Extensions\SC3plugins\`
  - mi-UGens → `%LOCALAPPDATA%\SuperCollider\Extensions\mi-UGens\`
- [x] Phase 1: Cross-platform foundation
  - Config paths use `dirs::config_dir()`
  - Temp paths use `std::env::temp_dir()`
  - Binary discovery includes Windows paths
  - Process termination uses `taskkill` on Windows
- [x] Phase 2: Windows audio setup
  - Added Windows audio handling (ASIO/WASAPI)
  - Updated audio device error messages

**Pending:**
- [x] Install Rust on Windows (https://rustup.rs/)
- [x] Install Visual Studio Build Tools (for MSVC linker)
- [ ] Build: `cargo build --release --features scsynth-direct`
- [ ] Test on Windows

---

## Next Agent Breadcrumb

**Status as of last session:** All code changes complete. Build tools installed. Ready to build.

**To continue:**
1. Open new terminal (to pick up PATH changes), then build:
   ```powershell
   cd C:\Users\bramm\repos\monokit
   cargo build --release --features scsynth-direct
   ```
2. If build succeeds, run `target\release\monokit.exe`
3. Test: Execute `TR` command to verify audio works

**Dependencies installed at:**
- SuperCollider: `C:\SuperCollider\SuperCollider\`
- Extensions: `C:\Users\bramm\AppData\Local\SuperCollider\Extensions\`

---

## Verification

1. **Build:** `cargo build --release --features scsynth-direct` on Windows
2. **Run:** Launch monokit.exe from Windows Terminal
3. **Audio test:** Execute `TR` command, verify sound output
4. **Config test:** Run `THEME gruvbox`, restart, verify theme persists
5. **Scene test:** `SAVE test`, restart, `LOAD test`, verify state restored
