# Comprehensive Test Fix Plan

## Overview

This plan addresses all issues identified in:
1. **ERROR_TEST_REPORT.md** - Critical bugs (P0), high priority (P1), medium priority (P2)
2. **TEST_FINDINGS.md** - Feature gaps and enhancements

## Workflow (Repeat for Each Phase)

1. Make corrections (highest priority first)
2. Write/run test scripts as necessary
3. Examine dumps to verify fixes
4. Repeat until all issues resolved

---

## PHASE 1: Error Test Report Issues

### P0-1: Semicolon Splits Quoted Strings (CRITICAL)

**Bug**: `PRINT "hello;world"` splits into two commands, breaking valid scripts.

**Files**:
- `src/app/script_exec/interactive.rs` (line ~117)
- `src/app/script_exec/mod.rs` (line ~216)

**Fix**: Replace `split(';')` with quote-aware splitting:
```rust
fn split_respecting_quotes(cmd: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for c in cmd.chars() {
        match c {
            '"' | '\'' if !in_quote => {
                in_quote = true;
                quote_char = c;
                current.push(c);
            }
            c if c == quote_char && in_quote => {
                in_quote = false;
                current.push(c);
            }
            ';' if !in_quote => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}
```

**Test**: `PRINT "hello;world"` should output `HELLO;WORLD`

---

### P0-2: Negative Values Not Validated (CRITICAL)

**Bug**: `PF -1` accepted silently, could cause undefined SC behavior.

**Files**: `src/commands/synth/common.rs` - macros

**Fix**: Add negative check in `define_float_param!` macro:
```rust
if $min >= 0.0 && value < 0.0 {
    output(format!("ERROR: {} MUST BE NON-NEGATIVE", $display_name));
    return Ok(());
}
```

**Test**: `PF -1` should output error

---

### P1-1: Pattern Bounds Bug in Eval

**Bug**: `pat <= 3` should be `pat <= 5` (6 patterns exist: 0-5)

**File**: `src/eval/patterns.rs` - lines 22, 36, 54, 67, 80, 93

**Fix**: Change all `if pat <= 3` to `if pat <= 5`

**Test**: `PRINT PN.NEXT 5` should work

---

### P1-2: Bool Params Accept Values > 1

**Bug**: `BR.REV 2` accepted silently instead of error.

**File**: `src/commands/synth/effects/beat_repeat.rs`

**Fix**: Replace `.clamp(0, 1)` with explicit validation:
```rust
if value != 0 && value != 1 {
    output("ERROR: VALUE MUST BE 0 OR 1".to_string());
    return Ok(());
}
```

Also check: `pitch_shift.rs` for PS.MODE, PS.TARG

**Test**: `BR.REV 2` should output error

---

### P1-3: DEL Time Upper Bound Not Enforced

**Bug**: `DEL 17000: TR` should error (max 16000ms)

**File**: `src/commands/core/scheduling.rs`

**Note**: Code exists at lines 56-59 but may be bypassed. Check type conversion issue with `as u64`.

**Fix**: Add negative check before upper bound check:
```rust
if delay_ms < 0 {
    output("ERROR: DELAY CANNOT BE NEGATIVE".to_string());
    return Ok(());
}
```

**Test**: `DEL 17000: TR` should output error

---

### P1-4: DEL.X Count of 0 Accepted

**Bug**: `DEL.X 0 100: TR` should error (count must be >= 1)

**File**: `src/commands/core/scheduling.rs`

**Note**: Validation exists at lines 154-157. Verify code path.

**Test**: `DEL.X 0 100: TR` should output error

---

### P1-5: SEQ Errors Not Descriptive

**Bug**: All SEQ syntax errors become "UNKNOWN COMMAND: SEQ"

**File**: `src/commands/validate.rs`

**Fix**: Add specific SEQ validation before unknown command fallthrough:
```rust
"SEQ" => {
    let remaining = &trimmed[3..].trim_start();
    if !remaining.starts_with('"') && !remaining.starts_with('\'') {
        return Err(anyhow!("SEQ pattern must be quoted"));
    }
    let quote_char = remaining.chars().next().unwrap();
    if !remaining[1..].contains(quote_char) {
        return Err(anyhow!("SEQ has unclosed quote"));
    }
    // Validate inner pattern structure
    Ok(())
}
```

**Test**: `SEQ "x x x` should output "SEQ has unclosed quote"

---

### P2-1: Extra Arguments Silently Ignored

**Bug**: `TOSS 1`, `TR 1`, `RST 1` silently ignore extra args

**File**: `src/commands/validate.rs`

**Note**: Validation code exists. Verify it's being called.

**Test**: `TOSS 1` should output error about extra arguments

---

### Phase 1 Verification

```bash
cargo build --release
./repl_tests/run_error_tests.sh
./repl_tests/analyze_error_tests.sh
```

---

## PHASE 2: Test Findings Issues (After Phase 1 Complete)

### HIGH-1: Pattern Queries Not Expression-Compatible

**Issue**: `P.N`, `P.L`, `P.MIN`, `P.MAX`, `P.SUM`, `P.AVG`, `P.FND` (and PN.*) work as commands but not in expressions.

**File**: `src/eval/patterns.rs`

**Fix**: Add handlers in `eval_pattern_expression`:
```rust
"P.N" => Some((patterns.working as i16, 1)),
"P.L" => {
    let pattern = &patterns.patterns[patterns.working];
    Some((pattern.length as i16, 1))
}
"P.MIN" => {
    let pattern = &patterns.patterns[patterns.working];
    let slice = &pattern.data[..pattern.length];
    Some((*slice.iter().min().unwrap_or(&0), 1))
}
// etc.
```

**Test**: `A P.MIN`, `PRINT P.MAX`, `IF GT P.MIN 50: TR`

---

### MEDIUM-1: Metro Query Not Expression-Compatible

**Issue**: `PRINT M` fails - M query is command, not expression.

**File**: `src/eval/mod.rs`

**Fix**: Complex - requires passing metro_interval to eval context or special handling in PRINT.

**Test**: `PRINT M`, `A M`

---

### MEDIUM-2: M.SCRIPT M Doesn't Parse

**Issue**: `M.SCRIPT M` should set metro to call M script (index 8)

**File**: `src/commands/system/metro.rs`

**Fix**: Add special handling:
```rust
let value: usize = if parts[1].to_uppercase() == "M" {
    8
} else {
    parts[1].parse()?
};
```

**Test**: `M.SCRIPT M` equivalent to `M.SCRIPT 8`

---

### LOW-1: Output Formatting Inconsistencies

**Issue**: `AD 100` says "SET AMP DECAY TO 100 MS" but `DD 100` just says "DD 100"

**Files**: Various command handlers

**Fix**: Standardize all confirmations to "SET {PARAM} TO {VALUE} {UNIT}"

---

## Summary Checklist

### Phase 1 (Errors - Do First)
- [x] P0-1: Semicolon splits quoted strings
- [x] P0-2: Negative values not validated
- [x] P1-1: Pattern bounds bug (pat <= 3 -> <= 5)
- [x] P1-2: Bool params accept values > 1
- [x] P1-3: DEL time upper bound
- [x] P1-4: DEL.X count of 0
- [x] P1-5: SEQ errors not descriptive
- [x] P2-1: Extra arguments silently ignored

### Phase 2 (Findings - Do Second)
- [x] HIGH-1: Pattern queries expression-compatible
- [x] MEDIUM-1: Metro query expression-compatible
- [x] MEDIUM-2: M.SCRIPT M parsing
- [x] LOW-1: Output formatting consistency

---

## Key Files

| Priority | File | Issues |
|----------|------|--------|
| P0 | `src/app/script_exec/interactive.rs` | Semicolon splitting |
| P0 | `src/app/script_exec/mod.rs` | Semicolon splitting |
| P0 | `src/commands/synth/common.rs` | Negative validation |
| P1 | `src/eval/patterns.rs` | Bounds bug, expression support |
| P1 | `src/commands/core/scheduling.rs` | DEL bounds |
| P1 | `src/commands/synth/effects/beat_repeat.rs` | Bool validation |
| P1 | `src/commands/validate.rs` | SEQ errors, extra args |
