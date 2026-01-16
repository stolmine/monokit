# Cross-Platform Port Guide

Guide for porting monokit to Windows and Linux.

## Overview

**Current State:** macOS (Apple Silicon) only
**Target:** Windows 10/11, Linux (Ubuntu/Debian), macOS (Intel + ARM)

**Architecture:** Rust CLI + SuperCollider sound engine (scsynth)

The port requires changes in four areas:
1. Rust code (path handling, process management, audio enumeration)
2. Build scripts (bundling scsynth + plugins)
3. CI/CD (GitHub Actions for each platform)
4. Distribution (platform-specific package managers)

---

## Platform-Specific Code Areas

### 1. Path Handling

| File | Issue | Fix |
|------|-------|-----|
| `src/config.rs:272-276` | Hardcoded `~/.config/monokit` | Use `dirs::config_dir()` |
| `src/scene.rs:44-51` | Hardcoded `.config/monokit/scenes` | Use `dirs::config_dir()` |
| `src/preset/mod.rs:34-40` | Hardcoded `.config/monokit/presets` | Use `dirs::config_dir()` |
| `src/commands/synth/sampler/utils.rs:18-38` | Manual tilde expansion, hardcoded sample path | Use `dirs::config_dir()` |

**Platform paths:**
```
macOS:   ~/Library/Application Support/monokit/
Windows: %APPDATA%\monokit\
Linux:   ~/.config/monokit/
```

### 2. Temporary Files

| File | Current | Fix |
|------|---------|-----|
| `src/scsynth_direct.rs:143` | `/tmp/scsynth.log` | `std::env::temp_dir()` |
| `src/metro.rs:15` | `/tmp/monokit_osc.log` | `std::env::temp_dir()` |
| `src/eval/patterns.rs:113` | `/tmp/monokit_debug.txt` | `std::env::temp_dir()` |
| `src/app/script_exec/mod.rs:16` | `/tmp/monokit_debug.txt` | `std::env::temp_dir()` |
| `src/commands/logging.rs:9` | `/tmp/monokit_commands.log` | `std::env::temp_dir()` |

### 3. Binary Discovery

**scsynth paths (`src/scsynth_direct.rs:787-825`):**
```
macOS:   /Applications/SuperCollider.app/Contents/Resources/scsynth
         /opt/homebrew/bin/scsynth
Windows: C:\Program Files\SuperCollider\scsynth.exe
Linux:   /usr/bin/scsynth
         /usr/local/bin/scsynth
```

**sclang paths (`src/sc_process.rs:193-217`):**
```
macOS:   /Applications/SuperCollider.app/Contents/MacOS/sclang
         /opt/homebrew/bin/sclang
Windows: C:\Program Files\SuperCollider\sclang.exe
Linux:   /usr/bin/sclang
         /usr/local/bin/sclang
```

**Plugin paths:**
```
macOS:   ~/Library/Application Support/SuperCollider/Extensions/
Windows: %LOCALAPPDATA%\SuperCollider\Extensions\
Linux:   ~/.local/share/SuperCollider/Extensions/
```

### 4. Process Termination

| Current | Windows | Linux |
|---------|---------|-------|
| `pkill -f scsynth` | `taskkill /F /IM scsynth.exe` | `pkill -f scsynth` |

**Cross-platform approach:** Use `Child::kill()` on stored process handle instead of shell commands.

### 5. Audio Device Enumeration

**Current:** `src/audio_devices.rs` uses CoreAudio (macOS only)

**Options:**

| Approach | Pros | Cons |
|----------|------|------|
| **cpal crate** | Single codebase, well-maintained | Adds ~15 deps |
| **WASAPI native** | No new deps for Windows | Maintain 2 codebases |
| **Skip selection** | Zero code | Degraded UX |

**Recommendation:** Use cpal for enumeration across all platforms. Delete CoreAudio code.

```rust
// Cargo.toml
cpal = { version = "0.15", default-features = false }

// audio_devices.rs (unified)
use cpal::traits::{DeviceTrait, HostTrait};

pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    let host = cpal::default_host();
    let devices: Vec<_> = host.output_devices()
        .map_err(|e| e.to_string())?
        .filter_map(|d| d.name().ok().map(|name| AudioDevice { name }))
        .collect();
    Ok(devices)
}
```

**Note:** Device switching still requires scsynth restart (scsynth limitation, not enumeration).

---

## CI/CD Configuration

### SuperCollider Installation

| Platform | Method | Version |
|----------|--------|---------|
| macOS | `brew install --cask supercollider` | Latest |
| Windows | `choco install supercollider -y` | 3.12.1 |
| Linux | `apt-get install supercollider` | Varies |

### Plugin Installation

**sc3-plugins:**
```
macOS:   sc3-plugins-3.13.0-macOS.zip
Windows: sc3-plugins-3.13.0-Windows-64bit-VS.zip
Linux:   sc3-plugins-3.13.0-Linux.zip (or build from source)
```
Source: https://github.com/supercollider/sc3-plugins/releases

**mi-UGens (for Plaits/Rings):**
```
macOS:   mi-UGens-macOS.zip
Windows: mi-UGens-Windows.zip
Linux:   Build from source
```
Source: https://github.com/v7b1/mi-UGens/releases

### GitHub Actions Workflow

```yaml
# .github/workflows/release.yml

jobs:
  build-macos:
    runs-on: macos-14
    # ... existing macOS job ...

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install SuperCollider
        run: choco install supercollider -y

      - name: Install sc3-plugins
        shell: pwsh
        run: |
          $ProgressPreference = 'SilentlyContinue'
          Invoke-WebRequest -Uri "https://github.com/supercollider/sc3-plugins/releases/download/Version-3.13.0/sc3-plugins-3.13.0-Windows-64bit-VS.zip" -OutFile sc3-plugins.zip
          Expand-Archive sc3-plugins.zip -DestinationPath sc3-plugins
          $extPath = "$env:LOCALAPPDATA\SuperCollider\Extensions"
          New-Item -ItemType Directory -Force -Path $extPath
          Copy-Item -Recurse sc3-plugins\* $extPath

      - name: Install mi-UGens
        shell: pwsh
        run: |
          $ProgressPreference = 'SilentlyContinue'
          Invoke-WebRequest -Uri "https://github.com/v7b1/mi-UGens/releases/download/v0.0.8/mi-UGens-Windows.zip" -OutFile mi-UGens.zip
          Expand-Archive mi-UGens.zip -DestinationPath mi-UGens
          Copy-Item -Recurse mi-UGens\* "$env:LOCALAPPDATA\SuperCollider\Extensions\"

      - name: Compile SynthDefs
        shell: pwsh
        run: |
          & "C:\Program Files\SuperCollider\sclang.exe" build_scripts\compile_synthdefs.scd

      - name: Build
        run: cargo build --release --features scsynth-direct

      - name: Bundle
        shell: pwsh
        run: .\scripts\bundle.ps1 $env:GITHUB_REF_NAME.TrimStart('v')

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: windows-x64-bundle
          path: dist\bundle\*.zip

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install SuperCollider
        run: |
          sudo apt-get update
          sudo apt-get install -y supercollider libsndfile1-dev

      - name: Install sc3-plugins
        run: |
          # May need to build from source or use PPA
          sudo apt-get install -y sc3-plugins || true

      - name: Build
        run: cargo build --release --features scsynth-direct

      - name: Bundle
        run: ./scripts/bundle-linux.sh ${GITHUB_REF#refs/tags/v}

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: linux-x64-bundle
          path: dist/bundle/*.tar.gz
```

---

## Bundle Scripts

### Windows: `scripts/bundle.ps1`

```powershell
param([string]$Version = "dev")

$ErrorActionPreference = "Stop"
$Arch = "x86_64-pc-windows-msvc"
$Name = "monokit-$Version-$Arch"
$BundleDir = "dist\bundle\$Name"

# Create bundle directory
New-Item -ItemType Directory -Force -Path $BundleDir

# Copy monokit binary
Copy-Item "target\release\monokit.exe" $BundleDir

# Copy scsynth and dependencies
$SCPath = "C:\Program Files\SuperCollider"
Copy-Item "$SCPath\scsynth.exe" $BundleDir
Copy-Item "$SCPath\*.dll" $BundleDir

# Copy plugins
$PluginDir = "$BundleDir\plugins"
New-Item -ItemType Directory -Force -Path $PluginDir
Copy-Item -Recurse "$SCPath\plugins\*" $PluginDir

# Copy SC3plugins from extensions
$ExtPath = "$env:LOCALAPPDATA\SuperCollider\Extensions"
if (Test-Path "$ExtPath\SC3plugins") {
    Copy-Item -Recurse "$ExtPath\SC3plugins" $PluginDir
}

# Copy mi-UGens
if (Test-Path "$ExtPath\mi-UGens") {
    Copy-Item -Recurse "$ExtPath\mi-UGens" $PluginDir
}

# Copy synthdefs
Copy-Item -Recurse "sc\synthdefs" $BundleDir

# Create zip
Compress-Archive -Path $BundleDir -DestinationPath "dist\bundle\$Name.zip"

Write-Host "Bundle created: dist\bundle\$Name.zip"
```

### Linux: `scripts/bundle-linux.sh`

```bash
#!/bin/bash
set -e

VERSION="${1:-dev}"
ARCH="x86_64-unknown-linux-gnu"
NAME="monokit-${VERSION}-${ARCH}"
BUNDLE_DIR="dist/bundle/${NAME}"

mkdir -p "${BUNDLE_DIR}"

# Copy monokit binary
cp target/release/monokit "${BUNDLE_DIR}/"

# Copy scsynth
cp /usr/bin/scsynth "${BUNDLE_DIR}/" || cp /usr/local/bin/scsynth "${BUNDLE_DIR}/"

# Copy plugins
PLUGIN_DIR="${BUNDLE_DIR}/plugins"
mkdir -p "${PLUGIN_DIR}"
cp -r /usr/lib/SuperCollider/plugins/* "${PLUGIN_DIR}/" 2>/dev/null || true
cp -r /usr/share/SuperCollider/Extensions/* "${PLUGIN_DIR}/" 2>/dev/null || true

# Copy synthdefs
cp -r sc/synthdefs "${BUNDLE_DIR}/"

# Create tarball
cd dist/bundle
tar -czvf "${NAME}.tar.gz" "${NAME}"
sha256sum "${NAME}.tar.gz" > "${NAME}.tar.gz.sha256"

echo "Bundle created: dist/bundle/${NAME}.tar.gz"
```

---

## Distribution

| Platform | Primary | Alternative |
|----------|---------|-------------|
| macOS | Homebrew tap | Portable tarball |
| Windows | Portable zip | Scoop bucket |
| Linux | Portable tarball | AppImage, .deb |

### Scoop Manifest (Windows)

```json
{
    "version": "0.5.1",
    "description": "Text-based scripting language for monophonic drum synthesis",
    "homepage": "https://github.com/stolmine/monokit",
    "license": "MIT",
    "architecture": {
        "64bit": {
            "url": "https://github.com/stolmine/monokit/releases/download/v0.5.1/monokit-0.5.1-x86_64-pc-windows-msvc.zip",
            "hash": "<sha256>"
        }
    },
    "bin": "monokit.exe"
}
```

---

## Implementation Phases

### Phase 1: Cross-Platform Foundation
- [ ] Abstract config paths using `dirs::config_dir()`
- [ ] Abstract temp paths using `std::env::temp_dir()`
- [ ] Add Windows/Linux paths to binary discovery
- [ ] Replace `pkill` with process handle termination
- [ ] Add `#[cfg(windows)]` and `#[cfg(unix)]` guards

### Phase 2: Audio Device Enumeration ✅ DONE (v0.6.2)
- [x] Add cpal dependency
- [x] Rewrite `audio_devices.rs` using cpal
- [x] Remove CoreAudio dependency (macOS-only)
- [ ] Test enumeration on all platforms

### Phase 3: Build System
- [ ] Create `scripts/bundle.ps1` for Windows
- [ ] Create `scripts/bundle-linux.sh` for Linux
- [ ] Add Windows job to GitHub Actions
- [ ] Add Linux job to GitHub Actions
- [ ] Verify artifact uploads

### Phase 4: Distribution
- [ ] Test portable bundles on each platform
- [ ] Create Scoop manifest for Windows
- [ ] Document installation for each platform
- [ ] Update README with platform support

---

## Manual Testing Checklist

### Windows

```powershell
# 1. Install SuperCollider
choco install supercollider -y

# 2. Verify installation
Get-ChildItem "C:\Program Files\SuperCollider"

# 3. Test sclang
& "C:\Program Files\SuperCollider\sclang.exe" -v

# 4. Test scsynth
& "C:\Program Files\SuperCollider\scsynth.exe" -v

# 5. Build monokit (after Phase 1 code changes)
cargo build --release

# 6. Run monokit
.\target\release\monokit.exe
```

### Linux

```bash
# 1. Install SuperCollider
sudo apt-get update
sudo apt-get install supercollider

# 2. Verify installation
which scsynth sclang

# 3. Test sclang
sclang -v

# 4. Build monokit
cargo build --release

# 5. Run monokit
./target/release/monokit
```

---

## Known Issues & Considerations

1. **mi-UGens on Linux** - No pre-built binaries, must build from source
2. **sc3-plugins on Linux** - May need PPA or source build for latest version
3. **Windows Terminal** - Verify TUI rendering in cmd.exe, PowerShell, Windows Terminal
4. **ASIO vs WASAPI** - scsynth on Windows may prefer ASIO for low latency
5. **Code signing** - Windows may require signing to avoid SmartScreen warnings

---

## Resources

- [SuperCollider Downloads](https://supercollider.github.io/downloads.html)
- [SuperCollider on Chocolatey](https://community.chocolatey.org/packages/SuperCollider)
- [sc3-plugins Releases](https://github.com/supercollider/sc3-plugins/releases)
- [mi-UGens](https://github.com/v7b1/mi-UGens)
- [cpal crate](https://crates.io/crates/cpal)
- [dirs crate](https://crates.io/crates/dirs)
