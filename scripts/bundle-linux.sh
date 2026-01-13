#!/bin/bash
set -e

# Monokit Linux Bundle Script
# Creates a self-contained bundle with scsynth and required plugins
# Usage: ./scripts/bundle-linux.sh [version]

VERSION=${1:-dev}
ARCH="x86_64-unknown-linux-gnu"
NAME="monokit-${VERSION}-${ARCH}"

SC_PLUGINS="/usr/lib/SuperCollider/plugins"
MI_UGENS="${HOME}/.local/share/SuperCollider/Extensions/mi-UGens"

DIST_DIR="dist/bundle"
BUNDLE_DIR="${DIST_DIR}/${NAME}"

echo "=== Creating monokit Linux bundle v${VERSION} ==="

# Check for scsynth
SCSYNTH_PATH=$(which scsynth 2>/dev/null || echo "")
if [ -z "${SCSYNTH_PATH}" ]; then
    echo "ERROR: scsynth not found. Install SuperCollider first."
    exit 1
fi
echo "Found scsynth at: ${SCSYNTH_PATH}"

if [ ! -d "${SC_PLUGINS}" ]; then
    echo "ERROR: Plugins directory not found at ${SC_PLUGINS}"
    exit 1
fi

echo "Building monokit with scsynth-direct feature..."
cargo build --release --features scsynth-direct

echo "Compiling SynthDefs..."
SCLANG_PATH=$(which sclang 2>/dev/null || echo "")
if [ -z "${SCLANG_PATH}" ]; then
    echo "ERROR: sclang not found"
    exit 1
fi

# Kill any lingering sclang/scsynth processes
pkill -9 sclang 2>/dev/null || true
pkill -9 scsynth 2>/dev/null || true
sleep 2

# Compile SynthDefs
REPO_ROOT="$(pwd)"
echo "  Running sclang..."
if ! (cd "$(dirname ${SCLANG_PATH})" && \
      timeout 60 sclang -D "${REPO_ROOT}/build_scripts/compile_synthdefs.scd" 2>&1); then
    EXIT_CODE=$?
    echo "ERROR: SynthDef compilation failed with exit code $EXIT_CODE"
    exit 1
fi

# Verify synthdefs were created
if [ ! -d "sc/synthdefs" ] || [ -z "$(ls -A sc/synthdefs/*.scsyndef 2>/dev/null)" ]; then
    echo "ERROR: Compiled SynthDefs not found"
    exit 1
fi
echo "  SynthDefs compiled successfully"

echo "Creating bundle directory structure..."
rm -rf "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}"
mkdir -p "${BUNDLE_DIR}/lib"
mkdir -p "${BUNDLE_DIR}/plugins"
mkdir -p "${BUNDLE_DIR}/synthdefs"

echo "Copying monokit binary..."
cp target/release/monokit "${BUNDLE_DIR}/"

echo "Copying scsynth..."
cp "${SCSYNTH_PATH}" "${BUNDLE_DIR}/"

echo "Copying SynthDefs..."
cp sc/synthdefs/*.scsyndef "${BUNDLE_DIR}/synthdefs/"

echo "Copying required plugins..."

# Core UGen plugins required by monokit (Linux uses .so extension)
CORE_PLUGINS=(
    "BinaryOpUGens.so"
    "UnaryOpUGens.so"
    "IOUGens.so"
    "DelayUGens.so"
    "FilterUGens.so"
    "DynNoiseUGens.so"
    "NoiseUGens.so"
    "PanUGens.so"
    "TriggerUGens.so"
    "OscUGens.so"
    "BufIOUGens.so"
    "GrainUGens.so"
    "FFT_UGens.so"
    "PV_ThirdParty.so"
    "DemandUGens.so"
    "PhysicalModelingUGens.so"
    "MulAddUGens.so"
    "DiskIO_UGens.so"
    "ReverbUGens.so"
    "LFUGens.so"
)

echo "  Core plugins:"
for plugin in "${CORE_PLUGINS[@]}"; do
    if [ -f "${SC_PLUGINS}/${plugin}" ]; then
        cp "${SC_PLUGINS}/${plugin}" "${BUNDLE_DIR}/plugins/"
        echo "    - ${plugin}"
    else
        echo "    WARNING: ${plugin} not found"
    fi
done

# sc3-plugins (BlackrainUGens for SVF/BMoog, TJUGens for DFM1)
echo "  sc3-plugins:"
if [ -f "${SC_PLUGINS}/BlackrainUGens.so" ]; then
    cp "${SC_PLUGINS}/BlackrainUGens.so" "${BUNDLE_DIR}/plugins/"
    echo "    - BlackrainUGens.so (SVF, BMoog filters)"
else
    echo "    WARNING: BlackrainUGens.so not found"
fi

if [ -f "${SC_PLUGINS}/TJUGens.so" ]; then
    cp "${SC_PLUGINS}/TJUGens.so" "${BUNDLE_DIR}/plugins/"
    echo "    - TJUGens.so (DFM1 diode filter)"
else
    echo "    WARNING: TJUGens.so not found"
fi

# mi-UGens (MiPlaits, MiClouds, MiRings)
echo "  mi-UGens:"
if [ -f "${MI_UGENS}/MiPlaits.so" ]; then
    cp "${MI_UGENS}/MiPlaits.so" "${BUNDLE_DIR}/plugins/"
    echo "    - MiPlaits.so (Plaits voice)"
else
    echo "    WARNING: MiPlaits.so not found"
fi

if [ -f "${MI_UGENS}/MiClouds.so" ]; then
    cp "${MI_UGENS}/MiClouds.so" "${BUNDLE_DIR}/plugins/"
    echo "    - MiClouds.so (Granular effect)"
else
    echo "    WARNING: MiClouds.so not found"
fi

if [ -f "${MI_UGENS}/MiRings.so" ]; then
    cp "${MI_UGENS}/MiRings.so" "${BUNDLE_DIR}/plugins/"
    echo "    - MiRings.so (Resonator)"
else
    echo "    WARNING: MiRings.so not found"
fi

# Copy required shared libraries that scsynth depends on
echo "Copying shared libraries..."
# Get scsynth dependencies and copy non-system ones
for lib in $(ldd "${SCSYNTH_PATH}" | grep "=> /" | awk '{print $3}'); do
    # Skip system libraries (libc, libm, libpthread, etc.)
    libname=$(basename "$lib")
    case "$libname" in
        libc.so*|libm.so*|libpthread.so*|libdl.so*|librt.so*|ld-linux*.so*|libgcc_s.so*|libstdc++.so*)
            continue
            ;;
        libscsynth*|libsndfile*|libFLAC*|libogg*|libvorbis*|libopus*|libmpg123*|libmp3lame*|libfftw3f*)
            cp "$lib" "${BUNDLE_DIR}/lib/" 2>/dev/null && echo "    - $libname" || true
            ;;
    esac
done

# Also get dependencies of libscsynth itself
if [ -f "${BUNDLE_DIR}/lib/libscsynth.so.1" ]; then
    for lib in $(ldd "${BUNDLE_DIR}/lib/libscsynth.so.1" | grep "=> /" | awk '{print $3}'); do
        libname=$(basename "$lib")
        case "$libname" in
            libc.so*|libm.so*|libpthread.so*|libdl.so*|librt.so*|ld-linux*.so*|libgcc_s.so*|libstdc++.so*)
                continue
                ;;
            *)
                if [ ! -f "${BUNDLE_DIR}/lib/$libname" ]; then
                    cp "$lib" "${BUNDLE_DIR}/lib/" 2>/dev/null && echo "    - $libname (from libscsynth)" || true
                fi
                ;;
        esac
    done
fi

echo ""
echo "Bundle structure:"
ls -lh "${BUNDLE_DIR}/"
echo ""
echo "  Plugin count: $(ls ${BUNDLE_DIR}/plugins/*.so 2>/dev/null | wc -l)"
echo "  SynthDef count: $(ls ${BUNDLE_DIR}/synthdefs/*.scsyndef 2>/dev/null | wc -l)"
echo "  Library count: $(ls ${BUNDLE_DIR}/lib/*.so* 2>/dev/null | wc -l)"

BUNDLE_SIZE=$(du -sh "${BUNDLE_DIR}" | awk '{print $1}')
echo "  Bundle size: ${BUNDLE_SIZE}"

# Create wrapper script that sets library path
cat > "${BUNDLE_DIR}/monokit-run.sh" << 'WRAPPER'
#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export LD_LIBRARY_PATH="${SCRIPT_DIR}/lib:${LD_LIBRARY_PATH}"
exec "${SCRIPT_DIR}/monokit" "$@"
WRAPPER
chmod +x "${BUNDLE_DIR}/monokit-run.sh"

echo ""
echo "=== Bundle complete ==="
echo "Location: ${BUNDLE_DIR}"
echo ""
echo "To test the bundle:"
echo "  cd ${BUNDLE_DIR}"
echo "  ./monokit-run.sh"
echo ""
echo "To create tarball:"
echo "  cd ${DIST_DIR}"
echo "  tar -czvf ${NAME}.tar.gz ${NAME}"
echo "  sha256sum ${NAME}.tar.gz > ${NAME}.tar.gz.sha256"
