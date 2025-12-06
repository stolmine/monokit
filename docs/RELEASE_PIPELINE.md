# Release Pipeline

Automated release process for monokit.

## Quick Release

```bash
./scripts/release.sh 0.2.1
git push origin main && git push origin v0.2.1
```

## What Happens

1. **Local (`release.sh`)**
   - Validates version format and clean git state
   - Updates Cargo.toml version
   - Runs tests
   - Builds signed bundle
   - Creates tarball + SHA256
   - Commits version bump
   - Creates annotated git tag

2. **GitHub Actions (on tag push)**
   - Installs SuperCollider + sc3-plugins on macOS-14 (Apple Silicon)
   - Compiles SynthDefs via sclang
   - Runs tests
   - Builds release bundle with code signing
   - Creates tarball + SHA256
   - Creates GitHub Release with artifacts

3. **Homebrew Update (automatic)**
   - Downloads SHA256 from build artifacts
   - Updates `homebrew-monokit` formula with new version + hash
   - Commits and pushes to tap repository

## Prerequisites

### GitHub Secrets

Add to `stolmine/monokit` repository settings:

| Secret | Purpose |
|--------|---------|
| `HOMEBREW_TAP_TOKEN` | PAT with write access to `stolmine/homebrew-monokit` |

### Creating the Token

1. Go to https://github.com/settings/tokens?type=beta
2. Generate new token:
   - Name: `monokit-homebrew-tap`
   - Repository access: `stolmine/homebrew-monokit` only
   - Permissions: Contents (Read and write)
3. Add as secret in monokit repo settings

## Bundle Contents

The release bundle (~16MB) includes:

```
monokit-X.Y.Z-aarch64-apple-darwin/
├── monokit           # Main binary
├── Resources/
│   ├── scsynth       # Audio engine
│   ├── plugins/      # UGen plugins (core + sc3-plugins)
│   └── synthdefs/    # Compiled SynthDefs
└── Frameworks/       # Required dylibs (libsndfile, libfftw3f, etc.)
```

## Distribution Channels

### GitHub Releases

Direct download of self-contained bundle:

```bash
tar -xzf monokit-X.Y.Z-aarch64-apple-darwin.tar.gz
cd monokit-X.Y.Z-aarch64-apple-darwin
./monokit
```

### Homebrew

One-line install:

```bash
brew tap stolmine/monokit
brew install monokit
```

Installs to:
- Binary: `/opt/homebrew/bin/monokit` (symlink)
- Bundle: `/opt/homebrew/Cellar/monokit/X.Y.Z/libexec/`

## Manual Release (if needed)

If GitHub Actions fails, build locally:

```bash
# Build bundle
CODESIGN=1 ./scripts/bundle.sh 0.2.1

# Create tarball
cd dist/bundle
tar -czvf monokit-0.2.1-aarch64-apple-darwin.tar.gz monokit-0.2.1-aarch64-apple-darwin
shasum -a 256 *.tar.gz > monokit-0.2.1-aarch64-apple-darwin.tar.gz.sha256

# Upload to GitHub Release manually
# Then update homebrew-monokit formula with SHA256
```

## Troubleshooting

### Permission denied on release.sh

```bash
xattr -d com.apple.provenance ./scripts/release.sh
# or
bash ./scripts/release.sh 0.2.1
```

### Tag already exists

```bash
git tag -d v0.2.1
git push origin :refs/tags/v0.2.1
```

### Homebrew update fails

Check `HOMEBREW_TAP_TOKEN` secret is valid and has write access to the tap repo.

### sc3-plugins not found in bundle

Ensure `~/Library/Application Support/SuperCollider/Extensions/SC3plugins/` exists locally, or verify GitHub Actions downloaded them correctly.

## Third-Party Components

The bundle includes components from:

- **SuperCollider** (GPL-2.0) - scsynth audio engine
- **sc3-plugins** (GPL-2.0) - BlackrainUGens (SVF filter)

See https://github.com/supercollider/sc3-plugins for sc3-plugins source.
