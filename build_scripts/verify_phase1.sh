#!/bin/bash
# Phase 1 Verification Script
# Verifies all success criteria for SynthDef pre-compilation phase

set -e

echo "=== Phase 1: SynthDef Pre-compilation Verification ==="
echo ""

# Check if synthdefs directory exists
if [ ! -d "../sc/synthdefs" ]; then
    echo "❌ FAIL: synthdefs directory does not exist"
    exit 1
fi
echo "✓ synthdefs directory exists"

# Check if all 3 .scsyndef files exist
FILES=("monokit.scsyndef" "monokit_spectrum.scsyndef" "monokit_scope.scsyndef")
for file in "${FILES[@]}"; do
    if [ ! -f "../sc/synthdefs/$file" ]; then
        echo "❌ FAIL: $file not found"
        exit 1
    fi

    # Check file size (must be >1KB = 1024 bytes)
    size=$(stat -f %z "../sc/synthdefs/$file")
    if [ "$size" -lt 1024 ]; then
        echo "❌ FAIL: $file is too small ($size bytes)"
        exit 1
    fi

    # Verify it's a valid SuperCollider synth definition file
    if ! file "../sc/synthdefs/$file" | grep -q "SuperCollider3 Synth Definition"; then
        echo "❌ FAIL: $file is not a valid SuperCollider synth definition"
        exit 1
    fi

    echo "✓ $file is valid ($size bytes)"
done

echo ""
echo "=== Compilation Test ==="

# Test headless compilation
cd "$(dirname "$0")"
if ! /Applications/SuperCollider.app/Contents/MacOS/sclang -D compile_synthdefs.scd >/dev/null 2>&1; then
    echo "❌ FAIL: Headless compilation failed"
    exit 1
fi
echo "✓ Headless compilation works (sclang -D)"

echo ""
echo "=== Loading Test ==="

# Test loading (with timeout)
timeout 15s /Applications/SuperCollider.app/Contents/MacOS/sclang test_load_synthdefs.scd 2>&1 | grep -q "SUCCESS"
if [ $? -eq 0 ]; then
    echo "✓ SynthDefs load successfully into scsynth"
else
    echo "❌ FAIL: SynthDef loading test failed"
    exit 1
fi

echo ""
echo "=== SUCCESS CRITERIA VERIFICATION ==="
echo "✓ compile_synthdefs.scd compiles all 3 SynthDefs"
echo "✓ .scsyndef files are valid and loadable"
echo "✓ Build script can run headlessly (sclang -D)"
echo ""
echo "🎉 Phase 1 complete! All success criteria met."
