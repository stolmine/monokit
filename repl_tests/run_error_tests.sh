#!/bin/bash
# Monokit Error Test Runner
# Uses --run batch mode to execute test scenes

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MONOKIT_DIR="$(dirname "$SCRIPT_DIR")"
WAIT_TIME=3000  # milliseconds to wait for metro to complete

# Test scenes to run
TESTS=(
    "test_error_unknown_commands"
    "test_error_argument_count"
    "test_error_range_values"
    "test_error_division_math"
    "test_error_type_mismatches"
    "test_error_pattern_indices"
    "test_error_script_indices"
    "test_error_seq_syntax"
    "test_error_control_flow"
    "test_error_semicolon_edge"
)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "================================"
echo "MONOKIT ERROR TEST RUNNER"
echo "================================"
echo ""

# Check if monokit is built
if [ ! -f "$MONOKIT_DIR/target/release/monokit" ]; then
    echo -e "${YELLOW}Building monokit in release mode...${NC}"
    cd "$MONOKIT_DIR"
    cargo build --release
fi

# Ensure test scenes are in ~/.config/monokit/scenes/
echo "Copying test scenes to ~/.config/monokit/scenes/..."
mkdir -p ~/.config/monokit/scenes
cp "$SCRIPT_DIR"/test_error_*.json ~/.config/monokit/scenes/

# Clean old dump files
echo "Cleaning old dump files..."
rm -f "$SCRIPT_DIR"/dump_error_*.txt

# Run all tests
echo ""
echo "Running ${#TESTS[@]} test scenes..."
echo ""

PASSED=0
FAILED=0

for test in "${TESTS[@]}"; do
    echo -n "Running $test... "

    # Run in batch mode, capture output
    output=$("$MONOKIT_DIR/target/release/monokit" --run "$test" $WAIT_TIME 2>&1)

    # Check if dump file was created
    dump_file="$SCRIPT_DIR/dump_${test}.txt"
    if [ -f "$dump_file" ]; then
        error_count=$(grep -c "ERROR" "$dump_file" 2>/dev/null || echo "0")
        echo -e "${GREEN}DONE${NC} ($error_count errors captured)"
        ((PASSED++))
    else
        # Check if there were errors in output
        if echo "$output" | grep -q "ERROR"; then
            error_count=$(echo "$output" | grep -c "ERROR" || echo "0")
            echo -e "${YELLOW}PARTIAL${NC} ($error_count errors, no dump file)"
        else
            echo -e "${RED}FAILED${NC} (no output)"
        fi
        ((FAILED++))
    fi
done

echo ""
echo "================================"
echo "RESULTS: $PASSED passed, $FAILED failed"
echo "================================"
echo ""

# Summary
echo "Dump files created:"
ls -la "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null || echo "  (none)"

echo ""
echo "To analyze results:"
echo "  ./repl_tests/analyze_error_tests.sh"
echo ""
echo "Quick error summary:"
grep -h "ERROR" "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null | sort | uniq -c | sort -rn | head -20
