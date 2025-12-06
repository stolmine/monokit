#!/bin/bash
set -e

# Monokit Bundle Script
# Creates a self-contained bundle with scsynth and required plugins
# Usage: ./scripts/bundle.sh [version]

VERSION=${1:-dev}
ARCH="aarch64-apple-darwin"
NAME="monokit-${VERSION}-${ARCH}"

SC_APP="/Applications/SuperCollider.app"
SC_RESOURCES="${SC_APP}/Contents/Resources"
SC_PLUGINS="${SC_RESOURCES}/plugins"

DIST_DIR="dist/bundle"
BUNDLE_DIR="${DIST_DIR}/${NAME}"

echo "=== Creating monokit bundle v${VERSION} ==="

if [ ! -d "${SC_APP}" ]; then
    echo "ERROR: SuperCollider.app not found at ${SC_APP}"
    echo "Please install SuperCollider first."
    exit 1
fi

if [ ! -f "${SC_RESOURCES}/scsynth" ]; then
    echo "ERROR: scsynth not found at ${SC_RESOURCES}/scsynth"
    exit 1
fi

if [ ! -d "${SC_PLUGINS}" ]; then
    echo "ERROR: Plugins directory not found at ${SC_PLUGINS}"
    exit 1
fi

echo "Building monokit with scsynth-direct feature..."
cargo build --release --features scsynth-direct

echo "Creating bundle directory structure..."
rm -rf "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}/Resources"
mkdir -p "${BUNDLE_DIR}/Resources/plugins"
mkdir -p "${BUNDLE_DIR}/Resources/synthdefs"
mkdir -p "${BUNDLE_DIR}/Frameworks"

echo "Copying monokit binary..."
cp target/release/monokit "${BUNDLE_DIR}/"

echo "Copying scsynth..."
cp "${SC_RESOURCES}/scsynth" "${BUNDLE_DIR}/Resources/"

echo "Copying required frameworks..."
# Frameworks go at bundle root level (scsynth looks for ../Frameworks relative to Resources/)

# scsynth requires libsndfile and libfftw3f (SC 3.14+ has different naming)
SC_FRAMEWORKS="${SC_APP}/Contents/Frameworks"

# Copy all .dylib files from SC Frameworks (scsynth needs these)
for fw in "${SC_FRAMEWORKS}"/*.dylib; do
    if [ -f "${fw}" ]; then
        cp "${fw}" "${BUNDLE_DIR}/Frameworks/"
        echo "    - $(basename ${fw})"
    fi
done

echo "Copying SynthDefs..."
if [ ! -d "sc/synthdefs" ]; then
    echo "ERROR: SynthDefs not found. Run sc/compile_synthdefs.sh first."
    exit 1
fi
cp sc/synthdefs/*.scsyndef "${BUNDLE_DIR}/Resources/synthdefs/"

echo "Copying required plugins..."

# Core UGen plugins required by monokit
CORE_PLUGINS=(
    "BinaryOpUGens.scx"
    "UnaryOpUGens.scx"
    "LID_UGens.scx"
    "IOUGens.scx"
    "DelayUGens.scx"
    "FilterUGens.scx"
    "DynNoiseUGens.scx"
    "NoiseUGens.scx"
    "PanUGens.scx"
    "TriggerUGens.scx"
    "OscUGens.scx"
    "BufIOUGens.scx"
    "GrainUGens.scx"
    "FFT_UGens.scx"
    "PV_ThirdParty.scx"
    "DemandUGens.scx"
    "PhysicalModelingUGens.scx"
    "MulAddUGens.scx"
    "DiskIO_UGens.scx"
)

echo "  Core plugins:"
for plugin in "${CORE_PLUGINS[@]}"; do
    if [ -f "${SC_PLUGINS}/${plugin}" ]; then
        cp "${SC_PLUGINS}/${plugin}" "${BUNDLE_DIR}/Resources/plugins/"
        echo "    - ${plugin}"
    else
        echo "    WARNING: ${plugin} not found (may not be needed)"
    fi
done

# ReverbUGens.scx contains FreeVerb2 (core SuperCollider)
if [ -f "${SC_PLUGINS}/ReverbUGens.scx" ]; then
    cp "${SC_PLUGINS}/ReverbUGens.scx" "${BUNDLE_DIR}/Resources/plugins/"
    echo "    - ReverbUGens.scx (FreeVerb2)"
else
    echo "    WARNING: ReverbUGens.scx not found!"
fi

# LFUGens.scx contains LFTri, LFSaw, LFPulse, etc.
if [ -f "${SC_PLUGINS}/LFUGens.scx" ]; then
    cp "${SC_PLUGINS}/LFUGens.scx" "${BUNDLE_DIR}/Resources/plugins/"
    echo "    - LFUGens.scx (LFO oscillators)"
else
    echo "    WARNING: LFUGens.scx not found!"
fi

# sc3-plugins required by monokit (SVF is in BlackrainUGens)
echo "  Searching for sc3-plugins (BlackrainUGens contains SVF)..."

# Check common sc3-plugins locations
SC3_PLUGIN_LOCATIONS=(
    "${HOME}/Library/Application Support/SuperCollider/Extensions/SC3plugins/BlackrainUGens"
    "${SC_PLUGINS}"
    "/usr/local/lib/SuperCollider/plugins"
    "/opt/homebrew/lib/SuperCollider/plugins"
)

SVF_FOUND=false

for location in "${SC3_PLUGIN_LOCATIONS[@]}"; do
    if [ -f "${location}/BlackrainUGens.scx" ] && [ "${SVF_FOUND}" = false ]; then
        cp "${location}/BlackrainUGens.scx" "${BUNDLE_DIR}/Resources/plugins/"
        echo "    - BlackrainUGens.scx (contains SVF filter) from ${location}"
        SVF_FOUND=true
    fi
done

if [ "${SVF_FOUND}" = false ]; then
    echo ""
    echo "WARNING: BlackrainUGens.scx not found!"
    echo "The monokit SynthDef uses SVF (State Variable Filter) from sc3-plugins."
    echo "SVF is part of BlackrainUGens - install sc3-plugins:"
    echo "  https://github.com/supercollider/sc3-plugins/releases"
    echo "Then copy SC3plugins folder to:"
    echo "  ~/Library/Application Support/SuperCollider/Extensions/"
    echo ""
fi

echo "Bundle structure:"
ls -lh "${BUNDLE_DIR}/"
echo ""
echo "Resources:"
ls -lh "${BUNDLE_DIR}/Resources/"
echo ""
ls -lh "${BUNDLE_DIR}/Resources/plugins/" | wc -l | xargs echo "  Plugin count:"
ls -lh "${BUNDLE_DIR}/Resources/synthdefs/" | wc -l | xargs echo "  SynthDef count:"
ls -lh "${BUNDLE_DIR}/Frameworks/" | wc -l | xargs echo "  Framework count:"

BUNDLE_SIZE=$(du -sh "${BUNDLE_DIR}" | awk '{print $1}')
echo "  Bundle size: ${BUNDLE_SIZE}"

# Always sign and clear quarantine for local testing
echo ""
echo "Code signing bundle..."
codesign --force --deep --sign - "${BUNDLE_DIR}/Resources/scsynth"
codesign --force --deep --sign - "${BUNDLE_DIR}/monokit"
for fw in "${BUNDLE_DIR}"/Frameworks/*.dylib; do
    codesign --force --sign - "$fw" 2>/dev/null || true
done
xattr -cr "${BUNDLE_DIR}"
echo "  Signed with ad-hoc signature"

echo ""
echo "=== Bundle complete ==="
echo "Location: ${BUNDLE_DIR}"
echo ""
echo "To test the bundle:"
echo "  cd ${BUNDLE_DIR}"
echo "  ./monokit"
echo ""
echo "To create tarball:"
echo "  cd ${DIST_DIR}"
echo "  tar -czvf ${NAME}.tar.gz ${NAME}"
echo "  shasum -a 256 ${NAME}.tar.gz > ${NAME}.tar.gz.sha256"
