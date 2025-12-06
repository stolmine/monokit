#!/bin/bash
# Clean up test bundles, keeping only release versions

DIST_DIR="dist/bundle"

if [ ! -d "$DIST_DIR" ]; then
    echo "No bundle directory found"
    exit 0
fi

echo "Current bundles:"
ls -d "${DIST_DIR}"/monokit-* 2>/dev/null | while read dir; do
    SIZE=$(du -sh "$dir" | awk '{print $1}')
    echo "  $(basename "$dir") ($SIZE)"
done

echo ""
read -p "Remove all test bundles (keep only semver releases)? [y/N] " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Remove test bundles (anything with -test, -rec-test, -dev, etc.)
    rm -rf "${DIST_DIR}"/monokit-*-test*
    rm -rf "${DIST_DIR}"/monokit-*rec-test*
    rm -rf "${DIST_DIR}"/monokit-dev-*
    rm -rf "${DIST_DIR}"/*.tar.gz
    rm -rf "${DIST_DIR}"/*.sha256
    
    echo ""
    echo "Remaining bundles:"
    ls -d "${DIST_DIR}"/monokit-* 2>/dev/null | while read dir; do
        SIZE=$(du -sh "$dir" | awk '{print $1}')
        echo "  $(basename "$dir") ($SIZE)"
    done || echo "  (none)"
fi
