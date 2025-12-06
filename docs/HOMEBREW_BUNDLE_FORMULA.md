# Homebrew Formula Changes for Bundled Distribution

## Status: IMPLEMENTED

The Homebrew formula has been updated to use pre-built bundles with automated release updates.

## Overview

This document outlines the changes made to the Homebrew formula for distributing monokit as a self-contained bundle with scsynth and plugins.

## Old Formula (Source Build, Pre-v0.1.1)

Previously, monokit required users to have Rust, SuperCollider, and sc3-plugins installed:

```ruby
class Monokit < Formula
  desc "Monokit - FM synthesis drum machine"
  homepage "https://github.com/user/monokit"
  url "https://github.com/user/monokit/archive/v0.2.0.tar.gz"
  sha256 "..."

  depends_on "rust" => :build
  depends_on "supercollider"  # External dependency

  def install
    system "cargo", "install", *std_cargo_args
    pkgshare.install "sc"
  end

  def caveats
    <<~EOS
      Monokit requires SuperCollider to be installed.
      If not already installed:
        brew install supercollider

      For sc3-plugins (optional but recommended):
        Download from https://github.com/supercollider/sc3-plugins
    EOS
  end
end
```

## Current Formula (Bundled Distribution, v0.1.1+)

The formula now downloads pre-built bundles with everything included:

```ruby
class Monokit < Formula
  desc "Teletype-style scripting for a SuperCollider complex oscillator voice"
  homepage "https://github.com/stolmine/monokit"
  version "0.1.1"
  license "GPL-2.0"

  on_macos do
    on_arm do
      url "https://github.com/stolmine/monokit/releases/download/v0.1.1/monokit-0.1.1-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    # Install everything to libexec to keep bundle structure intact
    libexec.install "monokit", "Resources", "Frameworks"
    # Symlink binary to bin
    bin.install_symlink libexec/"monokit"
  end

  def caveats
    <<~EOS
      Self-contained bundle - no SuperCollider installation required.

      User config stored at:
        ~/.config/monokit/
    EOS
  end

  test do
    assert_match "monokit", shell_output("#{bin}/monokit --version 2>&1", 1)
  end
end
```

## Key Differences

### Dependencies

**Before:**
- Requires `supercollider` package
- User must install separately
- Version compatibility issues possible

**After:**
- No external dependencies
- Everything bundled
- Guaranteed version compatibility

### Install Structure

**Before:**
```
/opt/homebrew/bin/monokit           # Binary
/opt/homebrew/share/monokit/sc/     # SynthDef sources
```

**After:**
```
/opt/homebrew/bin/monokit                       # Symlink to binary
/opt/homebrew/libexec/monokit/monokit           # Actual binary
/opt/homebrew/libexec/monokit/Resources/        # Bundled scsynth + plugins + synthdefs
/opt/homebrew/libexec/monokit/Frameworks/       # Bundled dylibs
```

### Build Process

**Before:**
- Build from source with `cargo build`
- SynthDefs compiled at runtime by sclang

**After:**
- Download pre-built bundle (binary release)
- SynthDefs pre-compiled to .scsyndef
- No build step required

### File Size

**Before:**
- monokit binary: ~5 MB
- SuperCollider (separate): ~200 MB

**After:**
- Complete bundle: ~12 MB
- Self-contained

## Path Resolution

The monokit binary (in `src/scsynth_direct.rs`) finds bundled resources using symlink resolution:

1. **Resolve symlink**: `/opt/homebrew/bin/monokit` → `/opt/homebrew/libexec/monokit/monokit`
2. **Check for Resources/ in same directory as actual binary**
3. **Fallback to system SuperCollider if not found**

The `get_exe_dir()` function in `scsynth_direct.rs` uses `fs::canonicalize()` to resolve the symlink and find the actual executable location. This ensures Homebrew's symlink structure works correctly.

When monokit is at `/opt/homebrew/libexec/monokit/monokit`:
- Bundled scsynth: `/opt/homebrew/libexec/monokit/Resources/scsynth`
- Bundled plugins: `/opt/homebrew/libexec/monokit/Resources/plugins/`
- Bundled synthdefs: `/opt/homebrew/libexec/monokit/Resources/synthdefs/`
- Bundled dylibs: `/opt/homebrew/libexec/monokit/Frameworks/`

## License Implications

### Before
- monokit: (project license)
- SuperCollider: User installs separately (GPL v3)

### After
- monokit bundle: Inherits GPL v3 from bundled SuperCollider
- Must distribute source or provide access
- Must include GPL notices

**Formula should include:**
```ruby
license "GPL-3.0"  # Due to bundled SuperCollider components
```

## Automated Release Updates

The formula is automatically updated after each GitHub release via the `update-homebrew` job in `.github/workflows/release.yml`:

1. Release workflow creates and uploads bundle tarball
2. Workflow calculates SHA256 of tarball
3. Workflow commits updated formula to `homebrew-monokit` repository
4. Formula URL and SHA256 are updated automatically

This ensures the formula always points to the latest release without manual intervention.

## Testing the Formula

Before submitting to Homebrew:

1. Build bundle:
   ```bash
   ./scripts/bundle.sh 0.2.0
   ```

2. Create tarball:
   ```bash
   cd dist/bundle
   tar -czvf monokit-0.2.0-aarch64-apple-darwin.tar.gz monokit-0.2.0-aarch64-apple-darwin
   ```

3. Test formula locally:
   ```bash
   brew install --build-from-source ./Formula/monokit.rb
   ```

4. Verify monokit finds bundled resources:
   ```bash
   monokit  # Should start without errors
   ```

5. Uninstall and test clean install:
   ```bash
   brew uninstall monokit
   brew install monokit
   ```

## Release Checklist

- [x] Build bundle with `scripts/bundle.sh`
- [x] Verify SVF.scx and FreeVerb2.scx are included
- [x] Test bundle works on clean system (no SuperCollider installed)
- [x] Generate tarball and SHA256
- [x] Create GitHub release with tarball (automated via `.github/workflows/release.yml`)
- [x] Update Homebrew formula with new URL and SHA256 (automated)
- [x] Formula uses libexec structure with symlink
- [x] Test formula installation
- [ ] Submit PR to homebrew-core (future consideration)

## Code Signing Considerations (macOS)

Homebrew bottles are typically unsigned or ad-hoc signed. For the bundled scsynth:

```bash
codesign --force --deep --sign - scsynth
```

This creates an ad-hoc signature that satisfies macOS Gatekeeper for local use.

For distribution via Homebrew, this is sufficient. For standalone .dmg or .app distribution, proper Apple Developer signing and notarization would be required.

## Related Files

- `/Users/why/repos/monokit/scripts/bundle.sh` - Bundle creation script
- `/Users/why/repos/monokit/src/scsynth_direct.rs` - Path resolution logic
- `/Users/why/repos/monokit/docs/PLUGIN_REQUIREMENTS.md` - Plugin documentation
