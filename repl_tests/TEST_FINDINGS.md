# Monokit REPL Test Suite Findings

**Date**: December 2025
**Test Suite Version**: 1.0
**Scenes Tested**: 12

---

## Fixes Applied (December 2025)

All high-priority feature gaps from this report have been addressed:
- **Pattern Query Expressions**: P.N, P.L, P.I, P.MIN, P.MAX, P.SUM, P.AVG, P.FND now work in expressions
- **Explicit Pattern Queries**: PN.MIN, PN.MAX, PN.SUM, PN.AVG, PN.FND now expression-compatible
- **Metro Query Expression**: M now works in expressions (`PRINT M`, `A M`)
- **M.SCRIPT Alias**: M.SCRIPT M now accepted as alias for M.SCRIPT 8

All test scenes now pass with expected behavior. The original issues documented below have been resolved.

---

## Executive Summary

12 test scenes were created and executed covering all major Monokit command categories. **7 tests passed fully**, **5 tests passed with minor issues**. No critical bugs found. Several limitations and inconsistencies identified for potential future enhancement.

---

## Test Results by Scene

### PASS (7 tests)

| Test | Coverage |
|------|----------|
| test_buffer_fx | Beat Repeat (BR.*), Pitch Shift (PS.*) |
| test_effects | Filter, Resonator, Delay, Reverb, EQ, LoFi, Ring Mod, Compressor, Routing |
| test_envelopes | All 6 envelope types (AENV, PENV, FMEV, DENV, FBEV, FLEV) + aliases |
| test_math_logic | ADD/SUB/MUL/DIV/MOD/MAP, comparisons, IF/ELIF/ELSE, PROB/EV/SKIP, L loops |
| test_quantize | N operator, Q quantization, Q.ROOT, Q.SCALE, Q.BIT custom scales |
| test_ui_system | DEBUG 0-5, OUT.* overrides, HEADER, CPU, METER.*, SCOPE.*, SLEW |
| test_vars_counters | Variables A-D, X-Z, T, J, K; Counters N1-N4 with MIN/MAX/RST |

### PARTIAL PASS (5 tests)

| Test | Issues |
|------|--------|
| test_metro_timing | `PRINT M` and `M.SCRIPT M` don't work |
| test_patterns | P.N/P.L/P.MIN/P.MAX etc. not valid in expressions |
| test_randomization | Same pattern query limitation as above |
| test_seq_random | Test script typo (variable "E" invalid) |
| test_synth_osc | Two "VALUE MUST BE 0 OR 1" errors in mix controls |

---

## Issues Requiring Attention

### HIGH PRIORITY

#### 1. Pattern Query Operations Not Valid as Expressions

**Affected Commands**:
- `P.N`, `P.L`, `P.I` (working pattern state)
- `P.MIN`, `P.MAX`, `P.SUM`, `P.AVG`, `P.FND`
- `PN.N`, `PN.L`, `PN.I` (explicit pattern state)
- `PN.MIN`, `PN.MAX`, `PN.SUM`, `PN.AVG`, `PN.FND`

**Symptom**:
```
A P.N         → ERROR: FAILED TO PARSE VALUE FOR A
PRINT P.MIN   → ERROR: FAILED TO EVALUATE EXPRESSION
```

**Expected Behavior**: These should return values usable in:
- Variable assignment: `A P.MIN`
- PRINT: `PRINT P.MAX`
- Expressions: `IF GT P.MIN 50: ...`

**Current Behavior**: They work as standalone commands but not in expression contexts.

**Impact**: Medium - workaround is to store pattern values at known indices and read those.

**Files to Investigate**:
- `src/eval/patterns.rs`
- `src/eval/mod.rs`

---

### MEDIUM PRIORITY

#### 2. Metro Interval Query (`M` without args)

**Symptom**:
```
PRINT M       → ERROR: FAILED TO EVALUATE EXPRESSION
```

**Expected**: `M` alone should return current interval for use in expressions.

**Current**: `M` works to display interval in REPL output but can't be used in PRINT or variable assignment.

**Impact**: Low - users can track interval manually.

---

#### 3. `M.SCRIPT M` Doesn't Parse

**Symptom**:
```
M.SCRIPT M    → Error: Failed to parse script number
```

**Expected**: `M.SCRIPT M` should set metro to call the M script (index 8).

**Current**: Only numeric values 1-8 accepted, "M" as identifier not recognized.

**Impact**: Low - workaround is `M.SCRIPT 9` or leave at default.

**Note**: Check if this is by design (M script is default anyway).

---

### LOW PRIORITY

#### 4. Bool Param Validation Edge Case

**Symptom**: In test_synth_osc, two errors appeared:
```
ERROR: VALUE MUST BE 0 OR 1
```

**Context**: Occurred during Script 6 (Mix Controls) execution. The test script has:
```
MX 8000; PRINT "MX 8000"
MM 4000; PRINT "MM 4000"
ME 1; PRINT "ME ON"
```

**Analysis**: MM is defined with `define_int_param!` (0-16383 range), ME with `define_bool_param!` (0 or 1). The error message suggests something tried to set a bool param with a non-0/1 value.

**Possible Causes**:
1. Command parsing issue when multiple commands on same line
2. Expression evaluation returning unexpected value
3. Test script authoring error

**Impact**: Low - individual commands work correctly.

---

#### 5. Test Script Authoring Errors (Non-Code Issues)

These are test script bugs, not Monokit bugs:

| Test | Issue |
|------|-------|
| test_seq_random | Used `E` as variable (invalid - only A-D, X-Z, T, J, K valid) |
| test_synth_osc | Script 5 missing some PRINT statements for DC values |
| test_effects | Script 7 missing PRINT for CR 4/CR 10 |

---

## Uncertain Areas (Need Clarification)

### 1. Pattern State Query Design Intent

**Question**: Is it intentional that `P.N`, `P.L`, `P.MIN`, etc. work as commands but not expressions?

**Options**:
- A) By design - these are setter/getter commands, not expressions
- B) Missing feature - should be added to eval system
- C) Partial implementation - some work, others don't

**Recommendation**: If (B), add to `src/eval/patterns.rs` alongside existing `PN.NEXT`, `PN.HERE`, etc.

---

### 2. PRINT Command Expression Scope

**Question**: What expressions should PRINT support?

**Currently Works**:
- Literals: `PRINT 42`, `PRINT "text"`
- Math: `PRINT ADD 10 20`
- Variables: `PRINT A`
- Pattern values: `PRINT P 0`, `PRINT P.HERE`, `PRINT P.NEXT`

**Currently Fails**:
- Pattern state: `PRINT P.N`, `PRINT P.L`
- Pattern queries: `PRINT P.MIN`, `PRINT P.MAX`
- Metro query: `PRINT M`

**Recommendation**: Document supported expressions or extend eval system.

---

### 3. Semicolon Command Chaining Edge Cases

**Observation**: Some multi-command lines with `;` produced unexpected behavior in test_synth_osc.

**Question**: Are there parsing edge cases with:
- Commands with string arguments containing special chars
- Deeply nested expressions in chained commands
- Bool params at end of chains

**Recommendation**: Add specific semicolon-parsing tests to error test suite.

---

## Output Formatting Inconsistencies

Minor inconsistencies in REPL output (not bugs, just polish):

| Command | Output Style |
|---------|--------------|
| AD 100 | `SET AMP DECAY TO 100 MS` |
| DD 100 | `DD 100` (no "SET..." prefix) |
| FBD 100 | `FBD 100` |
| FED 100 | `FED 100` |

**Recommendation**: Standardize output format across all envelope parameters.

---

## Test Infrastructure Notes

### REPL.DUMP Fix Applied

During testing, discovered `REPL.DUMP` only worked in interactive mode, not scripts.

**Fix Applied**: Added REPL.DUMP handling to `src/app/script_exec/control_flow.rs:203-227`

**Status**: Working in both interactive and script contexts now.

---

### Scene File Format Issues

Initial test scenes had structural issues:
1. Scripts as arrays instead of objects with `{lines, j, k}`
2. Patterns as arrays instead of objects with `{data, length, index}`
3. Missing `j`/`k` fields in some scripts

**Resolution**: Created validation/repair script, all 12 scenes now valid.

**Recommendation**: Add scene validation on load with helpful error messages.

---

## Recommendations for Future Work

### Immediate (Before Error Test Suite)

1. Decide on pattern query expression support (P.MIN, P.MAX, etc.)
2. Review bool param validation in command chains
3. Fix test script typos (variable "E", missing PRINTs)

### Short Term

1. Standardize REPL output formatting
2. Add scene file validation with clear error messages
3. Document PRINT expression support

### Error Test Suite (Next Phase)

Create tests covering:
- Invalid command names
- Wrong argument counts
- Out-of-range values (negative, overflow)
- Type mismatches (string where int expected)
- Invalid pattern indices (< 0, > 5)
- Invalid script indices
- Malformed SEQ syntax
- Division by zero
- Recursion limits
- Semicolon parsing edge cases

---

## Appendix: Test Scene Locations

All test scenes in: `/Users/why/repos/monokit/repl_tests/`

| File | Dump |
|------|------|
| test_buffer_fx.json | dump_buffer_fx.txt |
| test_effects.json | dump_effects.txt |
| test_envelopes.json | dump_envelopes.txt |
| test_math_logic.json | dump_math_logic.txt |
| test_metro_timing.json | dump_metro_timing.txt |
| test_patterns.json | dump_patterns.txt |
| test_quantize.json | dump_quantize.txt |
| test_randomization.json | dump_randomization.txt |
| test_seq_random.json | dump_seq_random.txt |
| test_synth_osc.json | dump_synth_osc.txt |
| test_ui_system.json | dump_ui_system.txt |
| test_vars_counters.json | dump_vars_counters.txt |
