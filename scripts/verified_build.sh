#!/bin/bash
# Verified Build Script - Ensures synthdefs and bundle are in sync
# Exit on any error

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== MONOKIT VERIFIED BUILD ===${NC}"
echo ""

# Step 1: Clean environment
echo -e "${YELLOW}STEP 1: Cleaning environment${NC}"
pkill -9 sclang 2>/dev/null || true
pkill -9 scsynth 2>/dev/null || true
sleep 2

# Step 2: Clean old synthdefs
echo -e "${YELLOW}STEP 2: Removing old synthdefs${NC}"
rm -f sc/synthdefs/*.scsyndef
echo "  Removed old .scsyndef files"

# Step 3: Compile SynthDefs
echo -e "${YELLOW}STEP 3: Compiling SynthDefs${NC}"
# Kill any lingering sclang/scsynth processes to avoid race conditions
pkill -9 sclang 2>/dev/null || true
pkill -9 scsynth 2>/dev/null || true
sleep 2
timeout 30 /Applications/SuperCollider.app/Contents/MacOS/sclang build_scripts/compile_synthdefs.scd > /tmp/synthdef_compile.log 2>&1

# Step 4: Verify synthdefs were created
echo -e "${YELLOW}STEP 4: Verifying SynthDef compilation${NC}"

# Check all required synthdefs
REQUIRED_SYNTHDEFS=(
    "monokit_noise.scsyndef"
    "monokit_mod.scsyndef"
    "monokit_primary.scsyndef"
    "monokit_main.scsyndef"
    "monokit_spectrum.scsyndef"
    "monokit_scope.scsyndef"
    "monokit_recorder.scsyndef"
)

for synthdef in "${REQUIRED_SYNTHDEFS[@]}"; do
    if [ ! -f "sc/synthdefs/$synthdef" ]; then
        echo -e "${RED}ERROR: $synthdef was not created!${NC}"
        echo "Compilation log:"
        tail -50 /tmp/synthdef_compile.log
        exit 1
    fi

    SYNTHDEF_SIZE=$(wc -c < "sc/synthdefs/$synthdef")
    if [ "$SYNTHDEF_SIZE" -lt 100 ]; then
        echo -e "${RED}ERROR: $synthdef is suspiciously small ($SYNTHDEF_SIZE bytes)${NC}"
        exit 1
    fi

    echo "  ✓ $synthdef created ($SYNTHDEF_SIZE bytes)"
done

# Get MD5 of main synthdef for manifest
SOURCE_MD5=$(md5 -q sc/synthdefs/monokit_main.scsyndef)
echo "  Main SynthDef MD5: $SOURCE_MD5"

# Step 5: Build bundle
echo -e "${YELLOW}STEP 5: Building bundle${NC}"
./scripts/bundle.sh > /tmp/bundle.log 2>&1

# Step 6: Verify bundle was created
echo -e "${YELLOW}STEP 6: Verifying bundle${NC}"
if [ ! -f "dist/bundle/monokit-dev-aarch64-apple-darwin/monokit" ]; then
    echo -e "${RED}ERROR: Bundle executable not found!${NC}"
    echo "Bundle log:"
    tail -50 /tmp/bundle.log
    exit 1
fi

# Verify all synthdefs were copied to bundle
for synthdef in "${REQUIRED_SYNTHDEFS[@]}"; do
    if [ ! -f "dist/bundle/monokit-dev-aarch64-apple-darwin/Resources/synthdefs/$synthdef" ]; then
        echo -e "${RED}ERROR: $synthdef not copied to bundle!${NC}"
        exit 1
    fi
    echo "  ✓ $synthdef in bundle"
done

# Step 7: Verify MD5 checksums match
echo -e "${YELLOW}STEP 7: Verifying MD5 checksums${NC}"
BUNDLE_MD5=$(md5 -q dist/bundle/monokit-dev-aarch64-apple-darwin/Resources/synthdefs/monokit_main.scsyndef)
echo "  Source MD5: $SOURCE_MD5"
echo "  Bundle MD5: $BUNDLE_MD5"

if [ "$SOURCE_MD5" != "$BUNDLE_MD5" ]; then
    echo -e "${RED}ERROR: MD5 checksums do not match!${NC}"
    echo "  Source and bundle synthdefs are different!"
    exit 1
fi

echo -e "  ${GREEN}✓ Checksums match${NC}"

# Step 8: Verify timestamps
echo -e "${YELLOW}STEP 8: Verifying timestamps${NC}"
SOURCE_TIME=$(stat -f %m sc/synthdefs/monokit_main.scsyndef)
BUNDLE_TIME=$(stat -f %m dist/bundle/monokit-dev-aarch64-apple-darwin/Resources/synthdefs/monokit_main.scsyndef)

if [ "$BUNDLE_TIME" -lt "$SOURCE_TIME" ]; then
    echo -e "${RED}ERROR: Bundle synthdef is older than source!${NC}"
    exit 1
fi

echo "  ✓ Bundle is up to date"

# Step 9: Final verification
echo ""
echo -e "${GREEN}=== BUILD VERIFICATION COMPLETE ===${NC}"
echo ""
echo "Bundle location: dist/bundle/monokit-dev-aarch64-apple-darwin"
echo "SynthDef MD5:    $SOURCE_MD5"
echo "Bundle size:     $(du -sh dist/bundle/monokit-dev-aarch64-apple-darwin | cut -f1)"
echo ""
echo -e "${GREEN}✓ All checks passed - bundle is ready for testing${NC}"
echo ""

# Create a build manifest
cat > dist/bundle/BUILD_MANIFEST.txt <<EOF
Build Date: $(date)
Multi-Synth Architecture: 4 source synths + utilities
Main SynthDef MD5: $SOURCE_MD5

SynthDefs Included:
  - monokit_noise.scsyndef (noise generator)
  - monokit_mod.scsyndef (modulator oscillator)
  - monokit_primary.scsyndef (primary oscillator with FM)
  - monokit_main.scsyndef (effects processor)
  - monokit_spectrum.scsyndef (spectrum analyzer)
  - monokit_scope.scsyndef (oscilloscope)
  - monokit_recorder.scsyndef (audio recorder)

Verification: PASSED
EOF

echo "Build manifest written to dist/bundle/BUILD_MANIFEST.txt"
