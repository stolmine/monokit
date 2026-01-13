#!/bin/bash
set -e

# AppImage Build Script for Monokit
# Usage: ./build-appimage.sh [version]

VERSION=${1:-0.5.2}
ARCH="x86_64"
APP_NAME="Monokit"
APP_ID="com.stolmine.Monokit"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
BUILD_DIR="${REPO_ROOT}/dist/appimage"
APPDIR="${BUILD_DIR}/${APP_NAME}.AppDir"

echo "=== Building ${APP_NAME} AppImage v${VERSION} ==="

# Check for appimagetool
if ! command -v appimagetool &> /dev/null; then
    echo "Downloading appimagetool..."
    curl -L -o /tmp/appimagetool https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
    chmod +x /tmp/appimagetool
    APPIMAGETOOL="/tmp/appimagetool"
else
    APPIMAGETOOL="appimagetool"
fi

# Build the release bundle first if it doesn't exist
BUNDLE_DIR="${REPO_ROOT}/dist/bundle/monokit-${VERSION}-x86_64-unknown-linux-gnu"
if [ ! -d "${BUNDLE_DIR}" ]; then
    echo "Building release bundle first..."
    cd "${REPO_ROOT}"
    ./scripts/bundle-linux.sh "${VERSION}"
fi

# Create AppDir structure
echo "Creating AppDir structure..."
rm -rf "${APPDIR}"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/lib/monokit"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/256x256/apps"
mkdir -p "${APPDIR}/usr/share/metainfo"

# Copy binary and resources
echo "Copying files..."
cp "${BUNDLE_DIR}/monokit" "${APPDIR}/usr/lib/monokit/"
cp "${BUNDLE_DIR}/scsynth" "${APPDIR}/usr/lib/monokit/"
cp -r "${BUNDLE_DIR}/plugins" "${APPDIR}/usr/lib/monokit/"
cp -r "${BUNDLE_DIR}/synthdefs" "${APPDIR}/usr/lib/monokit/"
cp -r "${BUNDLE_DIR}/lib" "${APPDIR}/usr/lib/monokit/"

# Create AppRun script
cat > "${APPDIR}/AppRun" << 'APPRUN'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export LD_LIBRARY_PATH="${HERE}/usr/lib/monokit/lib:${LD_LIBRARY_PATH}"
export MONOKIT_APPIMAGE=1
exec "${HERE}/usr/lib/monokit/monokit" "$@"
APPRUN
chmod +x "${APPDIR}/AppRun"

# Create desktop file
cat > "${APPDIR}/${APP_ID}.desktop" << DESKTOP
[Desktop Entry]
Type=Application
Name=Monokit
Comment=Teletype-style drum synthesizer
Exec=monokit
Icon=${APP_ID}
Terminal=true
Categories=Audio;Music;
Keywords=synthesizer;drum;audio;
DESKTOP
cp "${APPDIR}/${APP_ID}.desktop" "${APPDIR}/usr/share/applications/"

# Create a simple icon (or copy if exists)
if [ -f "${REPO_ROOT}/assets/icon.png" ]; then
    cp "${REPO_ROOT}/assets/icon.png" "${APPDIR}/${APP_ID}.png"
else
    # Create a placeholder icon using ImageMagick if available
    if command -v convert &> /dev/null; then
        convert -size 256x256 xc:'#1a1a2e' -fill '#e94560' \
            -font DejaVu-Sans-Bold -pointsize 72 \
            -gravity center -annotate 0 'MK' \
            "${APPDIR}/${APP_ID}.png"
    else
        # Minimal 1x1 PNG as fallback
        echo "Warning: No icon found and ImageMagick not available"
        printf '\x89PNG\r\n\x1a\n' > "${APPDIR}/${APP_ID}.png"
    fi
fi
cp "${APPDIR}/${APP_ID}.png" "${APPDIR}/usr/share/icons/hicolor/256x256/apps/"

# Create AppStream metainfo
cat > "${APPDIR}/usr/share/metainfo/${APP_ID}.metainfo.xml" << METAINFO
<?xml version="1.0" encoding="UTF-8"?>
<component type="console-application">
  <id>${APP_ID}</id>
  <name>Monokit</name>
  <summary>Teletype-style scripting frontend for a monophonic drum synthesizer</summary>
  <metadata_license>MIT</metadata_license>
  <project_license>MIT</project_license>
  <description>
    <p>
      Monokit is a Teletype-inspired terminal interface for a complex monophonic
      drum/percussion synthesizer built on SuperCollider. Features dual oscillators,
      FM synthesis, 16 Plaits engines, full effects chain, and pattern sequencing.
    </p>
  </description>
  <url type="homepage">https://github.com/stolmine/monokit</url>
  <provides>
    <binary>monokit</binary>
  </provides>
  <categories>
    <category>Audio</category>
    <category>Music</category>
  </categories>
</component>
METAINFO

# Build AppImage
echo "Building AppImage..."
cd "${BUILD_DIR}"
ARCH=${ARCH} ${APPIMAGETOOL} "${APPDIR}" "${APP_NAME}-${VERSION}-${ARCH}.AppImage"

echo ""
echo "=== AppImage built successfully ==="
echo "Location: ${BUILD_DIR}/${APP_NAME}-${VERSION}-${ARCH}.AppImage"
echo ""
echo "To test:"
echo "  chmod +x ${BUILD_DIR}/${APP_NAME}-${VERSION}-${ARCH}.AppImage"
echo "  ${BUILD_DIR}/${APP_NAME}-${VERSION}-${ARCH}.AppImage"
