#!/bin/bash
set -e

# Monokit Release Script
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.2.0
#
# This script:
# 1. Validates the version and environment
# 2. Updates Cargo.toml version
# 3. Builds and bundles for macOS (arm64)
# 4. Creates signed tarball with SHA256
# 5. Creates git tag
# 6. Optionally pushes tag to trigger GitHub release

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 0.2.0"
    exit 1
fi

# Validate version format (semver)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "ERROR: Version must be in semver format (e.g., 0.2.0)"
    exit 1
fi

echo "=== Monokit Release v${VERSION} ==="
echo ""

# Check for uncommitted changes
if ! git diff --quiet HEAD; then
    echo "ERROR: You have uncommitted changes. Please commit or stash them first."
    git status --short
    exit 1
fi

# Check we're on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "WARNING: Not on main branch (currently on: $BRANCH)"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if tag already exists
if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
    echo "ERROR: Tag v${VERSION} already exists"
    exit 1
fi

# Update version in Cargo.toml
echo "Updating Cargo.toml version to ${VERSION}..."
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# Verify the change
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
if [ "$CARGO_VERSION" != "$VERSION" ]; then
    echo "ERROR: Failed to update Cargo.toml version"
    exit 1
fi
echo "  Cargo.toml version: ${CARGO_VERSION}"

# Run tests
echo ""
echo "Running tests..."
cargo test --quiet

# Build the bundle
echo ""
echo "Building release bundle..."
CODESIGN=1 ./scripts/bundle.sh "${VERSION}"

# Create tarball
ARCH="aarch64-apple-darwin"
NAME="monokit-${VERSION}-${ARCH}"
DIST_DIR="dist/bundle"

echo ""
echo "Creating release tarball..."
cd "${DIST_DIR}"
tar -czvf "${NAME}.tar.gz" "${NAME}"
shasum -a 256 "${NAME}.tar.gz" > "${NAME}.tar.gz.sha256"

SHA256=$(cat "${NAME}.tar.gz.sha256" | awk '{print $1}')
SIZE=$(ls -lh "${NAME}.tar.gz" | awk '{print $5}')

cd - > /dev/null

echo ""
echo "=== Release Artifacts ==="
echo "  Tarball: ${DIST_DIR}/${NAME}.tar.gz"
echo "  SHA256:  ${SHA256}"
echo "  Size:    ${SIZE}"

# Commit version bump
echo ""
echo "Committing version bump..."
git add Cargo.toml
git commit -m "chore: Bump version to ${VERSION}"

# Create tag
echo "Creating tag v${VERSION}..."
git tag -a "v${VERSION}" -m "Release v${VERSION}

## What's New
- scsynth-direct bundled mode (no SuperCollider installation required)
- Recording support via DiskOut UGen
- ~16MB self-contained bundle

## Installation
\`\`\`bash
tar -xzf ${NAME}.tar.gz
cd ${NAME}
./monokit
\`\`\`

## SHA256
${SHA256}
"

echo ""
echo "=== Release Ready ==="
echo ""
echo "To complete the release:"
echo ""
echo "  1. Push the commit and tag:"
echo "     git push origin main"
echo "     git push origin v${VERSION}"
echo ""
echo "  2. Upload artifacts to GitHub Release:"
echo "     - ${DIST_DIR}/${NAME}.tar.gz"
echo "     - ${DIST_DIR}/${NAME}.tar.gz.sha256"
echo ""
echo "  Or push now with: git push origin main && git push origin v${VERSION}"
echo ""

read -p "Push to origin now? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin main
    git push origin "v${VERSION}"
    echo ""
    echo "Pushed! GitHub Actions will create the release."
    echo "Check: https://github.com/stolmine/monokit/releases"
fi
