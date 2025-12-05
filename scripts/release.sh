#!/bin/bash
set -e

# Monokit Release Script
# Usage: ./scripts/release.sh 0.1.0

VERSION=$1
ARCH="aarch64-apple-darwin"

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 0.1.0"
    exit 1
fi

NAME="monokit-${VERSION}-${ARCH}"
DIST_DIR="dist"

echo "=== Building monokit v${VERSION} ==="

# Run tests first
echo "Running tests..."
cargo test --quiet

# Build release binary
echo "Building release binary..."
cargo build --release

# Create dist directory
echo "Creating distribution package..."
rm -rf "${DIST_DIR}/${NAME}"
mkdir -p "${DIST_DIR}/${NAME}"

# Copy binary and SC files
cp target/release/monokit "${DIST_DIR}/${NAME}/"
cp -r sc "${DIST_DIR}/${NAME}/"

# Create tarball
echo "Creating tarball..."
cd "${DIST_DIR}"
tar -czvf "${NAME}.tar.gz" "${NAME}"

# Generate checksum
echo "Generating checksum..."
shasum -a 256 "${NAME}.tar.gz" > "${NAME}.tar.gz.sha256"
cd ..

echo ""
echo "=== Build complete ==="
echo "Tarball: ${DIST_DIR}/${NAME}.tar.gz"
echo "Checksum: ${DIST_DIR}/${NAME}.tar.gz.sha256"
echo ""
echo "Next steps:"
echo "  1. git tag -a v${VERSION} -m \"v${VERSION}\""
echo "  2. git push origin v${VERSION}"
echo "  3. Create GitHub release and upload:"
echo "     - ${DIST_DIR}/${NAME}.tar.gz"
echo "     - ${DIST_DIR}/${NAME}.tar.gz.sha256"
