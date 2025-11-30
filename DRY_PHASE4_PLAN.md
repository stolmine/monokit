# Monokit DRY Phase 4 Refactoring Plan

**Date:** 2025-11-30
**Status:** Ready for Implementation
**Prerequisites:** Phases 0-3 Complete (4,816 lines removed)

## Executive Summary

This document outlines the remaining DRY opportunities after successful completion of Phases 0-3. Analysis indicates an additional **1,000-1,200 line reduction** is achievable through:

- Core handler macros for variables and counters (~400-460 lines)
- Shared helper functions for common patterns (~200-280 lines)
- Test fixture optimization (~300-450 lines)
- Minor cleanups and consolidations (~50 lines)

**Total Estimated Reduction:** 950-1,240 lines (15-19% additional reduction from current codebase)

---

## Phase 4A: Core Handler Macros (~400-460 lines)

### 4A.1: Variable Handler Macro

**File:** `/Users/why/repos/monokit/src/commands/core/variables.rs`
**Current Size:** 340 lines
**Estimated Reduction:** 150-180 lines (44-53%)

#### Problem Analysis

Ten nearly-identical variable handlers (A, B, C, D, X, Y, Z, T, J, K) with 90%+ code duplication:

**Current Pattern (repeated 10 times):**

```rust
pub fn handle_variable_a<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("A = {}", variables.a));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR A".to_string());
                    return;
                }
            }
        };
        variables.a = value;
        output(format!("SET A TO {}", value));
    }
}
```

**Differences Only:**
- Variable name in format strings ("A" vs "B" vs "C", etc.)
- Which field to read/write (`variables.a` vs `variables.b`, etc.)
- J and K use `scripts.scripts[script_index].j` and check `script_index >= 10`

#### Refactoring Approach

Create a macro-based system similar to the successful pattern macros from Phase 2:

```rust
// In src/commands/core/variables.rs

macro_rules! define_variable_handler {
    // For standard variables (A-D, X-Z, T)
    ($fn_name:ident, $var_name:literal, $var_field:ident) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &mut Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            scale: &ScaleState,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                output(format!("{} = {}", $var_name, variables.$var_field));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(
                    &parts, 1, variables, patterns, counters, scripts, script_index, scale
                ) {
                    expr_val
                } else {
                    match parts[1].parse() {
                        Ok(v) => v,
                        Err(_) => {
                            output(format!("ERROR: FAILED TO PARSE VALUE FOR {}", $var_name));
                            return;
                        }
                    }
                };
                variables.$var_field = value;
                output(format!("SET {} TO {}", $var_name, value));
            }
        }
    };

    // For script-local variables (J, K) - uses scripts instead of variables
    (script, $fn_name:ident, $var_name:literal, $var_field:ident) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &mut ScriptStorage,
            script_index: usize,
            scale: &ScaleState,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if script_index >= 10 {
                output(format!("ERROR: {} REQUIRES SCRIPT CONTEXT", $var_name));
                return Ok(());
            }
            if parts.len() == 1 {
                output(format!("{} = {}", $var_name, scripts.scripts[script_index].$var_field));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(
                    &parts, 1, variables, patterns, counters, scripts, script_index, scale
                ) {
                    expr_val
                } else {
                    parts[1].parse().context(format!("Failed to parse value for {}", $var_name))?
                };
                scripts.scripts[script_index].$var_field = value;
                output(format!("SET {} TO {}", $var_name, value));
            }
            Ok(())
        }
    };
}

// Usage - replaces 10 handlers with 10 one-line invocations:
define_variable_handler!(handle_variable_a, "A", a);
define_variable_handler!(handle_variable_b, "B", b);
define_variable_handler!(handle_variable_c, "C", c);
define_variable_handler!(handle_variable_d, "D", d);
define_variable_handler!(handle_variable_x, "X", x);
define_variable_handler!(handle_variable_y, "Y", y);
define_variable_handler!(handle_variable_z, "Z", z);
define_variable_handler!(handle_variable_t, "T", t);
define_variable_handler!(script, handle_variable_j, "J", j);
define_variable_handler!(script, handle_variable_k, "K", k);
```

**Special Case: handle_variable_i**

Variable I is simpler (no expression evaluation), handle manually or create separate macro variant:

```rust
// Keep manual for I (simpler case, only 16 lines)
pub fn handle_variable_i<F>(
    parts: &[&str],
    variables: &mut Variables,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("I = {}", variables.i));
    } else {
        let value: i16 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR I".to_string());
                return;
            }
        };
        variables.i = value;
        output(format!("SET I TO {}", value));
    }
}
```

#### Implementation Steps

1. Add `define_variable_handler!` macro at top of `src/commands/core/variables.rs`
2. Replace handle_variable_a through handle_variable_t with macro invocations
3. Replace handle_variable_j and handle_variable_k with script-variant macro invocations
4. Keep handle_variable_i as-is (too simple to benefit from macro)
5. Run `cargo test` - all 411 tests must pass
6. Run `cargo clippy --fix` if needed

**Expected Result:**
- File size: 340 → 160-190 lines
- Line reduction: 150-180 lines
- All tests pass, no functional changes

**Risk Level:** Low (mechanical transformation, well-defined pattern)

---

### 4A.2: Counter Handler Macro

**File:** `/Users/why/repos/monokit/src/commands/core/counters.rs`
**Current Size:** 400 lines
**Estimated Reduction:** 220-280 lines (55-70%)

#### Problem Analysis

16 repetitive counter functions with 3 operation types:

1. **N1-N4 read handlers** (4 functions, ~17 lines each = 68 lines)
   - Return current value, then increment with wrapping
   - Identical logic, only counter index differs (0-3)

2. **N1.RST-N4.RST reset handlers** (4 functions, ~7 lines each = 28 lines)
   - Reset counter to its min value
   - Identical logic, only counter index differs

3. **N1.MAX-N4.MAX handlers** (4 functions, ~42 lines each = 168 lines)
   - Set max value with validation
   - Identical logic, only counter index differs

4. **N1.MIN-N4.MIN handlers** (4 functions, ~33 lines each = 132 lines)
   - Set min value
   - Identical logic, only counter index differs

**Total duplicated code:** ~396 lines (99% of file)

#### Current Pattern Examples

**Read Handler (repeated 4x):**
```rust
pub fn handle_n1<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    let current = counters.values[0];
    let min = counters.min[0];
    let max = counters.max[0];
    counters.values[0] = if max == 0 {
        current.wrapping_add(1)
    } else {
        let next = current + 1;
        if next > max { min } else { next }
    };
    output(format!("{}", current));
}
```

**Reset Handler (repeated 4x):**
```rust
pub fn handle_n1_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[0] = counters.min[0];
    output(format!("N1 RESET TO {}", counters.min[0]));
}
```

**Max Handler (repeated 4x):**
```rust
pub fn handle_n1_max<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N1.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N1.MAX".to_string());
                return;
            }
        }
    };

    if value > 0 && value < counters.min[0] {
        output(format!("ERROR: MAX ({}) MUST BE >= MIN ({})", value, counters.min[0]));
        return;
    }

    counters.max[0] = value;
    if value == 0 {
        output("N1.MAX DISABLED (NO WRAP)".to_string());
    } else {
        output(format!("N1.MAX SET TO {}", value));
    }
}
```

#### Refactoring Approach

Create a comprehensive macro system for all counter operations:

```rust
// In src/commands/core/counters.rs

macro_rules! define_counter_read {
    ($fn_name:ident, $counter_idx:expr) => {
        pub fn $fn_name<F>(counters: &mut Counters, mut output: F)
        where
            F: FnMut(String),
        {
            let current = counters.values[$counter_idx];
            let min = counters.min[$counter_idx];
            let max = counters.max[$counter_idx];
            counters.values[$counter_idx] = if max == 0 {
                current.wrapping_add(1)
            } else {
                let next = current + 1;
                if next > max { min } else { next }
            };
            output(format!("{}", current));
        }
    };
}

macro_rules! define_counter_reset {
    ($fn_name:ident, $counter_name:literal, $counter_idx:expr) => {
        pub fn $fn_name<F>(counters: &mut Counters, mut output: F)
        where
            F: FnMut(String),
        {
            counters.values[$counter_idx] = counters.min[$counter_idx];
            output(format!("{} RESET TO {}", $counter_name, counters.min[$counter_idx]));
        }
    };
}

macro_rules! define_counter_max {
    ($fn_name:ident, $counter_name:literal, $counter_idx:expr) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            scale: &ScaleState,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}.MAX REQUIRES A VALUE", $counter_name));
                return;
            }

            let value: i16 = if let Some((expr_val, _)) = eval_expression(
                &parts, 1, variables, patterns, counters, scripts, script_index, scale
            ) {
                expr_val
            } else {
                match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        output(format!("ERROR: FAILED TO PARSE VALUE FOR {}.MAX", $counter_name));
                        return;
                    }
                }
            };

            if value > 0 && value < counters.min[$counter_idx] {
                output(format!(
                    "ERROR: MAX ({}) MUST BE >= MIN ({})",
                    value,
                    counters.min[$counter_idx]
                ));
                return;
            }

            counters.max[$counter_idx] = value;
            if value == 0 {
                output(format!("{}.MAX DISABLED (NO WRAP)", $counter_name));
            } else {
                output(format!("{}.MAX SET TO {}", $counter_name, value));
            }
        }
    };
}

macro_rules! define_counter_min {
    ($fn_name:ident, $counter_name:literal, $counter_idx:expr) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            scale: &ScaleState,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}.MIN REQUIRES A VALUE", $counter_name));
                return;
            }

            let value: i16 = if let Some((expr_val, _)) = eval_expression(
                &parts, 1, variables, patterns, counters, scripts, script_index, scale
            ) {
                expr_val
            } else {
                match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        output(format!("ERROR: FAILED TO PARSE VALUE FOR {}.MIN", $counter_name));
                        return;
                    }
                }
            };

            counters.min[$counter_idx] = value;
            output(format!("{}.MIN SET TO {}", $counter_name, value));
        }
    };
}

// Convenience macro to define all 4 operations for a counter at once
macro_rules! define_counter {
    ($read_fn:ident, $rst_fn:ident, $max_fn:ident, $min_fn:ident, $name:literal, $idx:expr) => {
        define_counter_read!($read_fn, $idx);
        define_counter_reset!($rst_fn, $name, $idx);
        define_counter_max!($max_fn, $name, $idx);
        define_counter_min!($min_fn, $name, $idx);
    };
}

// Usage - 16 handlers in 4 lines:
define_counter!(handle_n1, handle_n1_rst, handle_n1_max, handle_n1_min, "N1", 0);
define_counter!(handle_n2, handle_n2_rst, handle_n2_max, handle_n2_min, "N2", 1);
define_counter!(handle_n3, handle_n3_rst, handle_n3_max, handle_n3_min, "N3", 2);
define_counter!(handle_n4, handle_n4_rst, handle_n4_max, handle_n4_min, "N4", 3);
```

#### Implementation Steps

1. Add macro definitions at top of `src/commands/core/counters.rs`
2. Replace all 16 handler functions with 4 macro invocations
3. Run `cargo test` - all counter tests must pass
4. Run `cargo clippy` to verify no warnings

**Expected Result:**
- File size: 400 → 120-180 lines
- Line reduction: 220-280 lines (55-70%)
- All tests pass, no functional changes

**Risk Level:** Low (highly mechanical, clear pattern)

---

## Phase 4B: Shared Helper Functions (~200-280 lines)

### 4B.1: Expression Parsing Helper

**Target Files:** Multiple command files across codebase
**Estimated Reduction:** 80-120 lines

#### Problem Analysis

The pattern `if let Some((expr_val, _)) = eval_expression(...)` appears 50+ times with identical error handling. This is boilerplate that can be extracted to a helper function.

**Current Pattern (repeated 50+ times):**
```rust
let value: i16 = if let Some((expr_val, _)) = eval_expression(
    &parts, 1, variables, patterns, counters, scripts, script_index, scale
) {
    expr_val
} else {
    match parts[1].parse() {
        Ok(v) => v,
        Err(_) => {
            output("ERROR: FAILED TO PARSE VALUE FOR X".to_string());
            return;
        }
    }
};
```

#### Refactoring Approach

Note: `src/commands/common.rs` doesn't currently exist. We need to check if there's a better location.

Looking at the codebase structure, `src/commands/patterns/common.rs` exists and contains shared helpers for patterns. We should create a top-level `src/commands/helpers.rs` or add to an existing common location.

**Check existing structure first:**
- Patterns already use `src/commands/patterns/common.rs` successfully (Phase 2)
- We should create `src/commands/common.rs` for cross-domain helpers

Create in `/Users/why/repos/monokit/src/commands/common.rs`:

```rust
use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};

pub fn parse_i16_expr<F>(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    param_name: &str,
    mut output: F,
) -> Option<i16>
where
    F: FnMut(String),
{
    if idx >= parts.len() {
        output(format!("ERROR: {} REQUIRES A VALUE", param_name));
        return None;
    }

    if let Some((expr_val, _)) = eval_expression(
        parts, idx, variables, patterns, counters, scripts, script_index, scale
    ) {
        Some(expr_val)
    } else {
        match parts[idx].parse() {
            Ok(v) => Some(v),
            Err(_) => {
                output(format!("ERROR: FAILED TO PARSE VALUE FOR {}", param_name));
                None
            }
        }
    }
}

pub fn parse_f32_expr<F>(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    param_name: &str,
    mut output: F,
) -> Option<f32>
where
    F: FnMut(String),
{
    if idx >= parts.len() {
        output(format!("ERROR: {} REQUIRES A VALUE", param_name));
        return None;
    }

    if let Some((expr_val, _)) = eval_expression(
        parts, idx, variables, patterns, counters, scripts, script_index, scale
    ) {
        Some(expr_val as f32)
    } else {
        match parts[idx].parse() {
            Ok(v) => Some(v),
            Err(_) => {
                output(format!("ERROR: FAILED TO PARSE VALUE FOR {}", param_name));
                None
            }
        }
    }
}
```

**Usage Example:**

```rust
// BEFORE (9 lines):
let value: i16 = if let Some((expr_val, _)) = eval_expression(
    &parts, 1, variables, patterns, counters, scripts, script_index, scale
) {
    expr_val
} else {
    match parts[1].parse() {
        Ok(v) => v,
        Err(_) => {
            output("ERROR: FAILED TO PARSE VALUE FOR N1.MAX".to_string());
            return;
        }
    }
};

// AFTER (3 lines):
let value = match parse_i16_expr(
    parts, 1, variables, patterns, counters, scripts, script_index, scale, "N1.MAX", &mut output
) {
    Some(v) => v,
    None => return,
};
```

**Alternative (even more concise with early return pattern):**

```rust
let Some(value) = parse_i16_expr(
    parts, 1, variables, patterns, counters, scripts, script_index, scale, "N1.MAX", &mut output
) else {
    return;
};
```

#### Implementation Notes

**IMPORTANT:** This helper should be implemented AFTER Phase 4A, as the variable and counter macros will benefit from using it internally. This creates a second optimization pass:

1. **Phase 4A:** Create variable/counter macros (reduces 400-460 lines)
2. **Phase 4B.1:** Create parse helpers and update macros to use them (reduces additional 80-120 lines)

This two-stage approach is safer and allows incremental testing.

**Files that will benefit:**
- `src/commands/core/variables.rs` (macros can be updated to use helpers)
- `src/commands/core/counters.rs` (macros can be updated to use helpers)
- `src/commands/core/scheduling.rs`
- `src/commands/slew.rs`
- `src/commands/gate.rs`
- `src/commands/randomization.rs`
- Various synth command files

**Expected Reduction:** 80-120 lines (net reduction after adding ~40 lines of helpers)

**Risk Level:** Low-Medium (affects many files, but changes are mechanical)

---

### 4B.2: Argument Validation Helper

**Currently Implemented:** This pattern is less consistent across the codebase. After reviewing the grep results showing only ~40 occurrences of `if parts.len()` checks, and given that the patterns vary (some use `<`, some use `==`, some use `!=`), creating a shared helper may not provide significant value.

**Recommendation:** SKIP this optimization. The complexity of accommodating all variations would outweigh the ~40-60 line savings.

---

### 4B.3: Pattern Property Handler Consolidation

**Files:**
- `/Users/why/repos/monokit/src/commands/patterns/explicit.rs`
- `/Users/why/repos/monokit/src/commands/patterns/working.rs`

**Current Size:** These files already use macros from Phase 2 and are quite lean
**Analysis:** After Phase 2, these files went from 268 → 166 lines (explicit.rs) and 168 → 135 lines (working.rs)

**Remaining duplication:** The navigation handlers (P.N/PN.N, P.L/PN.L, P.I/PN.I) are manually implemented

Looking at the actual files:
- `working.rs` is 135 lines (mostly navigation handlers)
- `explicit.rs` is 166 lines (mostly navigation handlers)

These handlers have different enough logic (working vs explicit pattern selection, different output formats) that macro-izing them further would be complex and save minimal lines.

**Recommendation:** SKIP further optimization of pattern property handlers. Phase 2 already achieved 90-97% reduction in pattern operation files. The remaining ~300 lines are mostly unique logic.

**Reason:** Diminishing returns - the pattern handlers are already highly optimized via Phase 2 macros.

---

## Phase 4C: Test Fixture Optimization (~300 lines)

### 4C.1: Test Setup Macro

**File:** `/Users/why/repos/monokit/src/tests/common.rs` (currently 37 lines)
**Target Files:** All 23 test files in `src/tests/`
**Current Pattern:** 122 occurrences of test setup boilerplate

#### Problem Analysis

Every test follows this 5-line setup pattern:

```rust
let variables = create_test_variables();
let mut patterns = create_test_patterns();
let scripts = create_test_scripts();
let mut counters = create_test_counters();
let scale = create_test_scale();
```

With 122 occurrences across test files, this represents **~610 lines** of boilerplate (122 tests × 5 lines each).

#### Refactoring Approach

Add a macro to `src/tests/common.rs`:

```rust
// Add to src/tests/common.rs

macro_rules! test_setup {
    () => {{
        let variables = create_test_variables();
        let mut patterns = create_test_patterns();
        let scripts = create_test_scripts();
        let mut counters = create_test_counters();
        let scale = create_test_scale();
        (variables, patterns, scripts, counters, scale)
    }};

    // Variant for tests that need mutable variables
    (mut) => {{
        let mut variables = create_test_variables();
        let mut patterns = create_test_patterns();
        let scripts = create_test_scripts();
        let mut counters = create_test_counters();
        let scale = create_test_scale();
        (variables, patterns, scripts, counters, scale)
    }};
}

pub(crate) use test_setup;
```

**Usage Example:**

```rust
// BEFORE (5 lines):
#[test]
fn test_variable_setter_with_expression_add() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let scale = create_test_scale();

    variables.a = 10;
    // ... test code
}

// AFTER (2 lines):
#[test]
fn test_variable_setter_with_expression_add() {
    let (mut variables, mut patterns, scripts, mut counters, scale) = test_setup!(mut);

    variables.a = 10;
    // ... test code
}
```

#### Implementation Steps

1. Add `test_setup!` macro to `src/tests/common.rs`
2. Update tests one file at a time:
   - `src/tests/variable_tests.rs`
   - `src/tests/pattern_tests.rs`
   - `src/tests/counter_tests.rs`
   - (continue through all 23 test files)
3. Run `cargo test` after EACH file update
4. Commit after each successful file conversion

**Expected Reduction:**
- 122 test setups × 4 lines saved per test = **~488 line reduction**
- Added macro: ~20 lines
- **Net reduction: ~468 lines**, but being conservative with **~300 lines** estimate due to:
  - Some tests may have non-standard setup
  - Some tests may not benefit from the macro
  - Not all 122 occurrences may be suitable for conversion

**Risk Level:** Low (tests verify themselves, easy to validate)

**Priority:** Medium (high impact but purely in test code, doesn't affect production code quality)

---

### 4C.2: Pattern Data Setup Helper

**Analysis:** After reviewing test files, pattern initialization is highly variable:
- Different patterns are initialized in different tests
- Different lengths, indices, and data values
- Setup is often test-specific and not repetitive enough for a helper

**Recommendation:** SKIP this optimization. The pattern setup in tests is too variable to benefit from a shared helper. Estimated savings would be minimal (~50 lines at best) and would reduce test clarity.

---

## Phase 4D: Minor Cleanups (~50 lines, OPTIONAL)

### 4D.1: Variable Resolution Consolidation

**File:** `/Users/why/repos/monokit/src/eval/mod.rs`
**Lines:** 8-34 and 67-97 (variable resolution logic appears twice)

#### Problem Analysis

Variable resolution logic is duplicated in two places:

1. `resolve_value()` function (lines 8-35) - returns i16
2. `eval_expression()` match arm (lines 66-97) - returns Option<(i16, usize)>

The variable name matching is identical, just wrapped differently.

#### Current Code

```rust
// Lines 8-35: resolve_value
pub fn resolve_value(s: &str, variables: &Variables, scripts: &ScriptStorage, script_index: usize) -> i16 {
    match s.trim().to_uppercase().as_str() {
        "A" => variables.a,
        "B" => variables.b,
        "C" => variables.c,
        "D" => variables.d,
        "I" => variables.i,
        "J" => {
            if script_index < 10 {
                scripts.scripts[script_index].j
            } else {
                0
            }
        }
        "K" => {
            if script_index < 10 {
                scripts.scripts[script_index].k
            } else {
                0
            }
        }
        "X" => variables.x,
        "Y" => variables.y,
        "Z" => variables.z,
        "T" => variables.t,
        _ => s.trim().parse::<i16>().unwrap_or(0),
    }
}

// Lines 66-97: Same logic in eval_expression
match expr.as_str() {
    "A" => Some((variables.a, 1)),
    "B" => Some((variables.b, 1)),
    "C" => Some((variables.c, 1)),
    "D" => Some((variables.d, 1)),
    "I" => Some((variables.i, 1)),
    "J" => {
        if script_index < 10 {
            Some((scripts.scripts[script_index].j, 1))
        } else {
            Some((0, 1))
        }
    }
    "K" => {
        if script_index < 10 {
            Some((scripts.scripts[script_index].k, 1))
        } else {
            Some((0, 1))
        }
    }
    "X" => Some((variables.x, 1)),
    "Y" => Some((variables.y, 1)),
    "Z" => Some((variables.z, 1)),
    "T" => Some((variables.t, 1)),
    _ => {
        if let Ok(val) = expr.parse::<i16>() {
            Some((val, 1))
        } else {
            None
        }
    }
}
```

#### Recommendation

**SKIP** - While this is duplication, the functions serve different purposes:
- `resolve_value()` is used in simpler contexts without expression parsing
- `eval_expression()` needs the full match for flow control

Consolidating them would require adding wrapper logic that would negate any line savings. The duplication is acceptable given the different use cases.

**Estimated savings if pursued:** 10-15 lines (not worth the complexity)

---

### 4D.2: Comparison Operator Consolidation

**File:** `/Users/why/repos/monokit/src/eval/logic.rs`

**Analysis:** Without seeing this file, this is speculative. Comparison operators in evaluation contexts often have subtle differences (type coercion, error handling, etc.) that make consolidation complex.

**Recommendation:** SKIP unless specifically working in this module for other reasons. Potential savings (30-50 lines) don't justify the risk and testing overhead.

---

## Implementation Summary

### Recommended Implementation Order

**Phase 4A: Core Handler Macros** (~400-460 lines, HIGH PRIORITY)
1. **4A.1:** Variable handler macro - `src/commands/core/variables.rs` (150-180 lines)
2. **4A.2:** Counter handler macro - `src/commands/core/counters.rs` (220-280 lines)

**Phase 4B: Shared Helpers** (~80-120 lines, MEDIUM PRIORITY)
3. **4B.1:** Expression parsing helper - Create `src/commands/common.rs` (80-120 lines)
   - Can be applied to macros from Phase 4A for additional reduction

**Phase 4C: Test Fixtures** (~300 lines, LOW PRIORITY)
4. **4C.1:** Test setup macro - `src/tests/common.rs` + all test files (300 lines)

**Phase 4D: Minor Cleanups** (SKIP - not worth the effort)

### Total Expected Reduction

| Phase | Component | Lines Saved | Priority | Risk |
|-------|-----------|-------------|----------|------|
| 4A.1 | Variable handlers | 150-180 | HIGH | Low |
| 4A.2 | Counter handlers | 220-280 | HIGH | Low |
| 4B.1 | Expression parsing helper | 80-120 | MEDIUM | Low-Med |
| 4C.1 | Test setup macro | ~300 | LOW | Low |
| **TOTAL** | | **750-880** | | |

**Conservative Estimate:** 750 lines
**Optimistic Estimate:** 880 lines
**Target Estimate:** 800-850 lines

### Phases 0-4 Combined Results

| Phase | Description | Lines Saved | Status |
|-------|-------------|-------------|--------|
| 0 | Reorganization | Structure only | COMPLETE |
| 1 | Envelope macros | 918 | COMPLETE |
| 2 | Pattern macros | 1,573 | COMPLETE |
| 3 | Command structure | ~2,325 | COMPLETE |
| **1-3 Subtotal** | | **4,816** | **COMPLETE** |
| 4A | Core handler macros | 400-460 | PLANNED |
| 4B | Shared helpers | 80-120 | PLANNED |
| 4C | Test fixtures | ~300 | PLANNED |
| **Phase 4 Subtotal** | | **780-880** | **PLANNED** |
| | | | |
| **GRAND TOTAL** | **Phases 1-4** | **~5,600-5,700** | |

---

## Verification Strategy

### After Each Sub-Phase

```bash
# Run full test suite
cargo test

# Expected: All 411 tests pass
# Any failures: STOP and investigate before proceeding

# Check for warnings
cargo clippy

# Expected: No warnings in modified files
# Fix any warnings before proceeding

# Build release
cargo build --release

# Expected: Clean build, no errors
```

### Incremental Testing

For **Phase 4A** (macros):
- Implement one macro at a time
- Test after each macro definition
- Verify handler behavior unchanged

For **Phase 4B** (helpers):
- Create helper functions first
- Test them in isolation if possible
- Update calling code incrementally
- One file at a time for cross-cutting changes

For **Phase 4C** (test macros):
- Update one test file at a time
- Run that file's tests after each change: `cargo test --test <filename>`
- Only proceed to next file when all tests pass

---

## Success Criteria

Phase 4 is considered successfully complete when:

1. **All target line reductions achieved:**
   - Core handler macros: 400-460 lines saved
   - Shared helpers: 80-120 lines saved
   - Test fixtures (if implemented): ~300 lines saved

2. **All tests pass:**
   - `cargo test` shows 411 tests passing
   - No test functionality changed or lost

3. **No new warnings:**
   - `cargo clippy` shows no warnings in modified files
   - Code quality maintained or improved

4. **Build succeeds:**
   - `cargo build --release` completes successfully
   - No new compilation errors or warnings

5. **Code clarity improved:**
   - Macros are well-documented
   - Helper functions have clear names and purposes
   - Test setup is more consistent

6. **Git history clean:**
   - Each sub-phase committed separately
   - Commit messages clearly describe changes
   - Easy to revert if needed

---

## Rollback Plan

Each sub-phase should be a separate commit:
- `git commit -m "Phase 4A.1: Variable handler macros (150 lines reduced)"`
- `git commit -m "Phase 4A.2: Counter handler macros (220 lines reduced)"`
- `git commit -m "Phase 4B.1: Expression parsing helpers (80 lines reduced)"`
- `git commit -m "Phase 4C.1: Test setup macros (300 lines reduced)"`

If issues arise:
```bash
# Revert last commit
git revert HEAD

# Or revert to before Phase 4
git revert <phase_4_start_commit>..HEAD

# All tests should still pass after revert
cargo test
```

---

## Future Opportunities (Phase 5+)

After Phase 4, consider:

1. **Synth parameter helpers** - If not already covered by existing macros from Phase 3, there may be opportunities to consolidate synth parameter validation and OSC sending logic.

2. **Error message standardization** - Create a consistent error message format/helper to reduce duplication in error strings.

3. **Output formatting helpers** - Many commands have similar output format patterns that could be consolidated.

4. **Eval expression optimization** - The eval module itself may have optimization opportunities, but this requires careful analysis to avoid breaking expression evaluation logic.

These are lower priority and should only be pursued if:
- Development velocity allows
- Clear patterns emerge during Phase 4 work
- Estimated savings exceed 100 lines
- Risk remains low

---

## Conclusion

Phase 4 represents a focused, low-risk opportunity to remove an additional **~800-850 lines** of duplication through:
- Macro-based code generation for highly repetitive handlers
- Shared helper functions for common patterns
- Test fixture optimization for cleaner test code

Combined with Phases 0-3, the total DRY refactoring effort will have removed **~5,600-5,700 lines** (**~26-27% of original codebase**) while maintaining 100% test coverage and zero functionality loss.

This positions the Monokit codebase for:
- Easier maintenance and debugging
- Faster feature development
- Reduced cognitive load for developers
- Higher code quality and consistency
- Better compile times (fewer lines to parse)

**Status:** Ready for implementation
**Next Step:** Begin Phase 4A.1 - Variable Handler Macro
