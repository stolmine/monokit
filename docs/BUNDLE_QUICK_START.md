# Bundle Quick Start Guide

## Status: IN USE

This bundle approach is the current distribution method for monokit v0.1.1+. The Homebrew formula downloads pre-built bundles created via this process.

## Creating a Bundle

### Prerequisites

1. SuperCollider installed at `/Applications/SuperCollider.app`
2. sc3-plugins installed (for SVF and FreeVerb2)
3. Monokit SynthDefs compiled (`sc/synthdefs/*.scsyndef` files exist)

### Create Bundle

```bash
# Development bundle
./scripts/bundle.sh dev

# Version-specific bundle
./scripts/bundle.sh 0.2.0

# With code signing
CODESIGN=1 ./scripts/bundle.sh 0.2.0
```

### Output

Bundle created at: `dist/bundle/monokit-{version}-{arch}/`

## Testing the Bundle

### Test on Development Machine

```bash
cd dist/bundle/monokit-dev-aarch64-apple-darwin
./monokit
```

The monokit binary should find bundled scsynth and plugins automatically.

### Verify Bundle Contents

```bash
cd dist/bundle/monokit-dev-aarch64-apple-darwin
ls -la

# Should see:
# - monokit (binary)
# - scsynth (binary)
# - plugins/ (directory with .scx files)
# - synthdefs/ (directory with .scsyndef files)

# Check plugins
ls -la plugins/ | wc -l
# Should have 18+ core plugins + SVF.scx + FreeVerb2.scx

# Check synthdefs
ls -la synthdefs/
# Should have: monokit.scsyndef, monokit_spectrum.scsyndef, monokit_scope.scsyndef
```

### Test Audio

```bash
./monokit

# In monokit:
# Press spacebar to trigger
# Should hear sound
# Meters should show activity
```

## Creating Distributable Tarball

```bash
cd dist/bundle
tar -czvf monokit-0.2.0-aarch64-apple-darwin.tar.gz monokit-0.2.0-aarch64-apple-darwin
shasum -a 256 monokit-0.2.0-aarch64-apple-darwin.tar.gz > monokit-0.2.0-aarch64-apple-darwin.tar.gz.sha256
```

## Installing sc3-plugins

If bundle script warns about missing SVF.scx or FreeVerb2.scx:

### Option 1: Download from GitHub

```bash
# Visit: https://github.com/supercollider/sc3-plugins/releases
# Download latest release for your platform
# Extract to: ~/Library/Application Support/SuperCollider/Extensions/
```

### Option 2: Homebrew (if available)

```bash
brew install sc3-plugins
```

### Option 3: Manual Compilation

```bash
git clone --recursive https://github.com/supercollider/sc3-plugins.git
cd sc3-plugins
mkdir build && cd build
cmake -DSC_PATH=/Applications/SuperCollider.app/Contents/Resources/include/plugin_interface ..
cmake --build . --config Release
cmake --install .
```

### Verify Installation

```bash
find ~/Library/Application\ Support/SuperCollider/Extensions/ -name "SVF.scx"
find ~/Library/Application\ Support/SuperCollider/Extensions/ -name "FreeVerb2.scx"
```

## Troubleshooting

### Bundle script can't find SuperCollider

**Error:** `ERROR: SuperCollider.app not found at /Applications/SuperCollider.app`

**Solution:**
```bash
# Install SuperCollider
# Download from: https://supercollider.github.io/

# Or install via Homebrew
brew install supercollider
```

### Bundle script can't find SynthDefs

**Error:** `ERROR: SynthDefs not found. Run sc/compile_synthdefs.sh first.`

**Solution:**
```bash
cd sc
./compile_synthdefs.sh
cd ..
./scripts/bundle.sh dev
```

### Missing SVF.scx or FreeVerb2.scx

**Warning:** `WARNING: SVF.scx not found!`

**Solution:**
Install sc3-plugins (see above), then re-run bundle script.

### Bundle doesn't work on target machine

**Symptom:** monokit can't find scsynth or plugins

**Debug:**
```bash
# Check if bundled files exist
ls -la monokit scsynth plugins/ synthdefs/

# Run monokit with verbose output
./monokit
# Watch for "[monokit]" debug messages about paths
```

**Common causes:**
- Bundle directory structure incorrect
- Missing executable permissions
- macOS Gatekeeper blocking unsigned binaries

### macOS Gatekeeper blocks scsynth

**Error:** "scsynth cannot be opened because the developer cannot be verified"

**Solution:**
```bash
# Ad-hoc sign the bundle
codesign --force --deep --sign - scsynth
codesign --force --deep --sign - monokit

# Or bypass Gatekeeper
xattr -d com.apple.quarantine scsynth
xattr -d com.apple.quarantine monokit
```

## Bundle Size

Expected sizes:
- scsynth: ~1.6 MB
- monokit: ~5 MB
- plugins/: ~5 MB
- synthdefs/: ~35 KB
- **Total: ~12 MB**

If bundle is significantly larger, check:
```bash
du -sh monokit-*/
du -sh monokit-*/plugins/
```

## Distribution Checklist

Before distributing a bundle:

- [ ] Bundle script ran without errors
- [ ] SVF.scx and FreeVerb2.scx included
- [ ] Tested on clean system (no SuperCollider installed)
- [ ] Audio works (trigger sounds, meters show activity)
- [ ] All three SynthDefs present
- [ ] Code signed (if required)
- [ ] Tarball created with SHA256
- [ ] GitHub release created
- [ ] README updated with installation instructions

## Next Steps

The bundle creation process is integrated into the release workflow:

1. Push git tag to trigger `.github/workflows/release.yml`
2. Workflow builds bundle and creates tarball
3. Workflow creates GitHub release with tarball
4. Workflow auto-updates Homebrew formula with new URL and SHA256
5. Users get new version via `brew upgrade monokit`

Manual bundle creation is still useful for testing and development.

## Related Documentation

- `PLUGIN_REQUIREMENTS.md` - Detailed plugin information
- `HOMEBREW_BUNDLE_FORMULA.md` - Homebrew formula updates
- `PHASE6_IMPLEMENTATION_SUMMARY.md` - Implementation details
- `scsynth_direct_integration.md` - Overall integration plan
