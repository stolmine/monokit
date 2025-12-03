# Monokit REPL Error Test Suite - Implementation Plan

**Date**: December 2025
**Based on**: TEST_FINDINGS.md from previous test session
**Status**: Planning Complete

---

## Executive Summary

This plan outlines 10 test scenes covering all error categories identified in TEST_FINDINGS.md, plus additional validation gaps discovered during codebase exploration.

---

## Key Validation Findings from Codebase

**Validation Layer** (`src/commands/validate.rs`):
- Pre-validates commands before execution
- Checks argument counts, basic syntax
- Returns `anyhow::anyhow!()` errors with messages
- Unknown commands trigger "Unknown command: {}" error

**Error Message Patterns**:
1. `ERROR: {CMD} REQUIRES A VALUE` - missing argument
2. `ERROR: {CMD} REQUIRES A VALUE ({min}-{max})` - missing with range hint
3. `ERROR: {VALUE} MUST BE BETWEEN {min} AND {max}` - out of range
4. `ERROR: VALUE MUST BE 0 OR 1` - bool param error
5. `ERROR: FAILED TO PARSE VALUE FOR {param}` - parse failure
6. `ERROR: FAILED TO EVALUATE {operand}` - expression eval failure
7. `ERROR: DIVISION BY ZERO` / `ERROR: MODULO BY ZERO` - math errors
8. `ERROR: PATTERN NUMBER MUST BE 0-5` - pattern index error
9. `ERROR: SCRIPT NUMBER MUST BE 1-8` - script index error
10. `Unknown command: {cmd}` - command not found

---

## Test Scene Structure

### Scene 1: `test_error_unknown_commands.json`
**Purpose**: Test handling of invalid/unknown command names

| Test Case | Expected Error |
|-----------|----------------|
| `FOOBAR` | "Unknown command: FOOBAR" |
| `BLEEP` | "Unknown command: BLEEP" |
| `P.INVALID` | "Unknown command: P.INVALID" |
| `PN.INVALID` | "Unknown command: PN.INVALID" |
| `M.INVALID` | "Unknown command: M.INVALID" |
| `123CMD` | "Unknown command: 123CMD" |

---

### Scene 2: `test_error_argument_count.json`
**Purpose**: Test wrong argument counts (too few, too many)

**Missing Required Args:**

| Test Case | Expected Error |
|-----------|----------------|
| `ADD` | "ADD REQUIRES TWO OPERANDS" |
| `ADD 10` | "FAILED TO EVALUATE SECOND OPERAND" |
| `SUB` | "SUB REQUIRES TWO OPERANDS" |
| `MAP 1 2 3` | "MAP requires at least 5 arguments" |
| `RND` | "RND requires at least 1 argument" |
| `RRND 5` | "RRND requires at least 2 arguments" |
| `SCRIPT` | "SCRIPT takes exactly 1 argument" |
| `P.SCALE 0` | "P.SCALE NEEDS MIN AND MAX" |
| `PN` | "PN requires at least 2 arguments" |
| `PN.PUSH 0` | "PN.PUSH NEEDS PAT NUM AND VAL" |
| `SLEW PF` | "SLEW NEEDS PARAM NAME AND TIME" |
| `TOG 1` | "TOG requires at least 2 arguments" |
| `EITH 5` | "EITH requires at least 2 arguments" |

**Too Many Args:**

| Test Case | Expected Error |
|-----------|----------------|
| `TOSS 1` | "TOSS takes no arguments" |
| `TR 1` | "TR takes no arguments" |
| `RST 1` | "RST takes no arguments" |
| `P.HERE 1` | "P.HERE takes no arguments" |
| `SCENES 1` | "SCENES takes no arguments" |

---

### Scene 3: `test_error_range_values.json`
**Purpose**: Test out-of-range values (negative, overflow, zero where invalid)

**Synth Params:**

| Test Case | Expected Error |
|-----------|----------------|
| `PF 20000` | "MUST BE BETWEEN 0 AND 16383" |
| `PF -1` | "MUST BE BETWEEN 0 AND 16383" |
| `FC 30000` | "MUST BE BETWEEN 20 AND 20000" |
| `AD 10001` | Time param out of range |

**Mode Params:**

| Test Case | Expected Error |
|-----------|----------------|
| `D.MODE 5` | "D.MODE VALUE MUST BE 0-2" |
| `R.MODE -1` | "R.MODE VALUE MUST BE 0-2" |

**Metro:**

| Test Case | Expected Error |
|-----------|----------------|
| `M 0` | "INTERVAL MUST BE GREATER THAN 0" |
| `M.BPM 0` | "BPM MUST BE GREATER THAN 0" |
| `M.ACT 5` | "M.ACT VALUE MUST BE 0 OR 1" |
| `M.SCRIPT 0` | "M.SCRIPT VALUE MUST BE 1-8" |
| `M.SCRIPT 9` | "M.SCRIPT VALUE MUST BE 1-8" |

**Bool Params:**

| Test Case | Expected Error |
|-----------|----------------|
| `ME 5` | "VALUE MUST BE 0 OR 1" |
| `BR.REV 2` | "VALUE MUST BE 0 OR 1" |

**Delay:**

| Test Case | Expected Error |
|-----------|----------------|
| `DEL 17000: TR` | "DELAY TIME MAX 16000MS" |
| `DEL.X 0 100: TR` | "COUNT MUST BE AT LEAST 1" |

**SLEW:**

| Test Case | Expected Error |
|-----------|----------------|
| `SLEW PF 11000` | "SLEW TIME 0-10000 MS" |
| `SLEW.ALL 11000` | "SLEW TIME 0-10000 MS" |

**DEBUG:**

| Test Case | Expected Error |
|-----------|----------------|
| `DEBUG 6` | Should clamp or error (0-5 valid) |

---

### Scene 4: `test_error_pattern_indices.json`
**Purpose**: Test invalid pattern number errors

**Working Pattern (P.N) Bounds:**

| Test Case | Expected Error |
|-----------|----------------|
| `P.N -1` | Error or wrap? |
| `P.N 6` | "PATTERN NUMBER MUST BE 0-5" |
| `P.N 100` | "PATTERN NUMBER MUST BE 0-5" |

**Explicit Pattern (PN.*) Bounds:**

| Test Case | Expected Error |
|-----------|----------------|
| `PN -1 0 100` | Pattern index error |
| `PN 6 0 100` | "PATTERN NUMBER MUST BE 0-5" |
| `PN.L 6` | "PATTERN NUMBER MUST BE 0-5" |
| `PN.NEXT -1` | Error |
| `PN.PUSH 6 100` | "PATTERN NUMBER MUST BE 0-5" |
| `PN.ADD 6 10` | "PATTERN NUMBER MUST BE 0-5" |

**Pattern Value Index Bounds:**

| Test Case | Expected Error |
|-----------|----------------|
| `P 64 100` | Index out of bounds (max 63) |
| `P -1 100` | Negative index error |
| `P.INS 64 100` | Invalid index for insert |
| `P.RM 64` | Invalid index for remove |

---

### Scene 5: `test_error_script_indices.json`
**Purpose**: Test invalid script number errors

| Test Case | Expected Error |
|-----------|----------------|
| `SCRIPT 0` | "SCRIPT NUMBER MUST BE 1-8" |
| `SCRIPT 9` | "SCRIPT NUMBER MUST BE 1-8" |
| `SCRIPT -1` | Parse error or bounds check |
| `$ 0` | "SCRIPT NUMBER MUST BE 1-8" |
| `$ 100` | "SCRIPT NUMBER MUST BE 1-8" |
| `M.SCRIPT 0` | "M.SCRIPT VALUE MUST BE 1-8" |
| `M.SCRIPT 9` | "M.SCRIPT VALUE MUST BE 1-8" |
| `M.SCRIPT M` | "Failed to parse script number" (known issue) |

---

### Scene 6: `test_error_type_mismatches.json`
**Purpose**: Test type mismatch errors

**Parse Failures:**

| Test Case | Expected Error |
|-----------|----------------|
| `A hello` | "FAILED TO PARSE VALUE FOR A" |
| `B "text"` | "FAILED TO PARSE VALUE FOR B" |
| `PF abc` | Parse error |
| `M xyz` | Parse error |

**Invalid Variable Names:**

| Test Case | Expected Error |
|-----------|----------------|
| `E 100` | "Unknown command: E" |

**Math with Bad Operands:**

| Test Case | Expected Error |
|-----------|----------------|
| `ADD hello world` | "FAILED TO EVALUATE FIRST OPERAND" |
| `ADD 10 hello` | "FAILED TO EVALUATE SECOND OPERAND" |

**Known Expression Limitations:**

| Test Case | Expected Error | Notes |
|-----------|----------------|-------|
| `A P.N` | "FAILED TO PARSE VALUE FOR A" | Known issue |
| `B P.MIN` | "FAILED TO EVALUATE EXPRESSION" | Known issue |
| `C M` | Error | M query not expression-compatible |

---

### Scene 7: `test_error_division_math.json`
**Purpose**: Test division by zero and modulo by zero errors

| Test Case | Expected Error |
|-----------|----------------|
| `DIV 10 0` | "DIVISION BY ZERO" |
| `/ 100 0` | "DIVISION BY ZERO" |
| `MOD 10 0` | "MODULO BY ZERO" |
| `% 100 0` | "MODULO BY ZERO" |
| `A DIV 50 0` | "DIVISION BY ZERO" |
| `PRINT DIV 100 0` | "DIVISION BY ZERO" |
| `A 0; B DIV 100 A` | "DIVISION BY ZERO" |
| `P.DIV 0` | Check if handled |
| `PN.DIV 0 0` | Check if handled |

---

### Scene 8: `test_error_seq_syntax.json`
**Purpose**: Test malformed SEQ syntax errors

| Test Case | Expected Error |
|-----------|----------------|
| `SEQ"x x x"` | "SEQ requires space before quote" |
| `SEQ "x x x` | "SEQ has unclosed quote" |
| `SEQ ""` | Check behavior |
| `SEQ "<>"` | "Empty alternation <> not allowed" |
| `SEQ "{}"` | "Empty random choice {} not allowed" |
| `SEQ "{x"` | "Unclosed random choice bracket" |
| `SEQ "<x"` | "Unclosed alternation bracket" |
| `SEQ "x*abc"` | "Invalid repeat count: abc" |
| `SEQ "x>y"` | "Unexpected closing bracket >" |
| `SEQ "x}y"` | "Unexpected closing bracket }" |

---

### Scene 9: `test_error_control_flow.json`
**Purpose**: Test control flow syntax errors

**L Loop Errors:**

| Test Case | Expected Error |
|-----------|----------------|
| `L: TR` | "Failed to evaluate loop start value" |
| `L 1: TR` | "Failed to evaluate loop end value" |
| `L abc def: TR` | Eval error on bounds |

**DEL Command Errors:**

| Test Case | Expected Error |
|-----------|----------------|
| `DEL TR` | "DEL REQUIRES FORMAT: DEL <ms>: <cmd>" |
| `DEL 100` | Missing colon error |
| `DEL 100:` | "DEL REQUIRES A COMMAND AFTER COLON" |
| `DEL: TR` | "DEL REQUIRES A DELAY TIME" |
| `DEL abc: TR` | "FAILED TO PARSE DELAY TIME" |

**DEL.X/DEL.R Errors:**

| Test Case | Expected Error |
|-----------|----------------|
| `DEL.X TR` | "DEL.X FORMAT: DEL.X N MS: CMD" |
| `DEL.X 5: TR` | "DEL.X REQUIRES COUNT AND INTERVAL" |
| `DEL.X abc 100: TR` | "FAILED TO PARSE COUNT" |
| `DEL.R 5: TR` | "DEL.R REQUIRES COUNT AND INTERVAL" |

---

### Scene 10: `test_error_semicolon_edge.json`
**Purpose**: Test semicolon parsing edge cases (from TEST_FINDINGS)

| Test Case | Expected Behavior |
|-----------|-------------------|
| `;` | No-op |
| `;;` | No-op |
| `TR;;TR` | Both triggers execute |
| `PRINT "hello;world"` | Print whole string |
| `MX 8000; ME 1` | Check ME 1 gets correct context |
| `A 1; B ADD A 1; C ADD B 1` | Sequential evaluation |
| `A 1; FOOBAR; B 2` | A=1, error on FOOBAR, B=2? |

---

## Validation Gaps Discovered

### 1. Pattern Query Not Expression-Compatible (Known)
- `P.N`, `P.L`, `P.I`, `P.MIN`, `P.MAX`, `P.SUM`, `P.AVG`, `P.FND` work as commands but not in expression contexts
- Same for `PN.*` equivalents
- **File**: `src/eval/patterns.rs` handles `P.NEXT`, `P.PREV`, `P.HERE` but NOT `P.N`, `P.L`, etc.

### 2. Metro Query Not Expression-Compatible (Known)
- `M` without args shows interval but can't be used in expressions
- `M.SCRIPT M` fails because "M" identifier not recognized

### 3. Pattern Index Bug in eval/patterns.rs
- Code checks `pat <= 3` but there are 6 patterns (0-5)
- **Should be**: `pat <= 5` or `pat < 6`

### 4. Negative Value Handling
- Many places cast to `usize` without checking for negative values
- Could cause unexpected behavior or panics

### 5. Missing Validation for Some Commands
- `HEADER`, `FLASH`, `HL.COND` have inline validation not in validate.rs
- Inconsistent error handling approaches

---

## Recommendations for Error Handling Improvements

1. **Unify Error Message Format**
   - Standardize on `ERROR: {CMD} {MESSAGE}` format
   - Use consistent capitalization

2. **Fix Pattern Index Bug**
   - Change `pat <= 3` to `pat <= 5` in `src/eval/patterns.rs`

3. **Add Pattern Query to Eval System**
   - Add `P.N`, `P.L`, `P.MIN`, `P.MAX`, etc. to `eval_pattern_expression()`
   - Would resolve expression compatibility issues

4. **Add Negative Value Validation**
   - Before casting to `usize`, check for negative values
   - Return appropriate error instead of panicking

5. **Consider Recursion Limits**
   - Add depth tracking for SCRIPT calls to prevent infinite recursion

---

## Critical Files for Implementation

| File | Purpose |
|------|---------|
| `src/commands/validate.rs` | Central validation, arg count checks |
| `src/eval/mod.rs` | Expression evaluation entry point |
| `src/eval/patterns.rs` | Pattern expression eval (has pat <= 3 bug) |
| `src/commands/core/math_ops.rs` | Division/modulo by zero handling |
| `src/eval/seq.rs` | SEQ parsing error messages |

---

## Implementation Notes

1. Use same JSON scene format as existing `test_*.json` files
2. Each scene should have scripts that execute the error cases
3. Use `REPL.DUMP` to capture output for verification
4. Expected errors should be documented in each scene's notes
5. Some tests may require manual verification (panics vs errors)
