#!/bin/bash
# Analyze error test results

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "================================"
echo "ERROR TEST ANALYSIS"
echo "================================"
echo ""

# Check for dump files
DUMP_COUNT=$(ls "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null | wc -l | tr -d ' ')

if [ "$DUMP_COUNT" -eq 0 ]; then
    echo "No dump files found. Run the tests first:"
    echo "  ./repl_tests/run_error_tests.sh"
    exit 1
fi

echo "Found $DUMP_COUNT dump files"
echo ""

# Count errors per file
echo "ERRORS PER TEST:"
echo "----------------"
for f in "$SCRIPT_DIR"/dump_error_*.txt; do
    name=$(basename "$f" .txt | sed 's/dump_//')
    errors=$(grep -c "ERROR" "$f" 2>/dev/null || echo "0")
    printf "  %-35s %3s errors\n" "$name" "$errors"
done

echo ""
echo "UNIQUE ERROR MESSAGES:"
echo "----------------------"
grep -h "ERROR" "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null | sort | uniq -c | sort -rn | head -30

echo ""
echo "SILENT FAILURES (commands with no output):"
echo "-------------------------------------------"
# Look for test markers without following ERROR
for f in "$SCRIPT_DIR"/dump_error_*.txt; do
    name=$(basename "$f" .txt | sed 's/dump_//')
    # Count TEST: markers
    tests=$(grep -c "TEST:" "$f" 2>/dev/null || echo "0")
    # Count DONE markers
    dones=$(grep -c "DONE" "$f" 2>/dev/null || echo "0")
    if [ "$tests" -ne "$dones" ]; then
        echo "  $name: $tests tests started, $dones completed (possible crash/silent fail)"
    fi
done

echo ""
echo "POTENTIAL ISSUES TO INVESTIGATE:"
echo "---------------------------------"

# Check for specific patterns
if grep -q "thread.*panic" "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null; then
    echo "  - PANICS detected (search: 'thread.*panic')"
fi

if grep -q "FAILED TO PARSE" "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null; then
    echo "  - Parse failures detected"
fi

if grep -q "FAILED TO EVALUATE" "$SCRIPT_DIR"/dump_error_*.txt 2>/dev/null; then
    echo "  - Expression evaluation failures detected"
fi

# Check for commands that produced no error when they should have
echo ""
echo "Commands that may have failed silently (no ERROR after TEST:):"
for f in "$SCRIPT_DIR"/dump_error_*.txt; do
    # This is a rough heuristic - look for TEST: not followed by ERROR before next TEST: or DONE
    awk '/TEST:/{test=$0} /ERROR/{test=""} /DONE/{if(test!="") print "  " FILENAME ": " test; test=""}' "$f"
done

echo ""
echo "Full results in: $SCRIPT_DIR/dump_error_*.txt"
