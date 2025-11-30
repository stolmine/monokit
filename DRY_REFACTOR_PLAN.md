# DRY Refactoring Plan for Monokit

## Overview

Refactor three major areas of code duplication to reduce ~4,500 lines to ~1,500 lines (~67% reduction).

**Decisions:**
- Use declarative macros for code generation
- Keep individual handler functions (thin wrappers) for incremental migration
- Order: Envelopes → Patterns → Synth Params

---

## Phase 1: Envelope Handlers (~650 line reduction) [START HERE]

### Current State
- 7 envelope types × 3 handlers each (ATK, CRV, MODE) = 21 functions
- 100% identical logic, only strings differ
- Additional handlers: AD, PD, FD, PA, FA, DA (envelope-specific, keep as-is)

### Approach: Macro-Generated Handlers

**Create `src/commands/synth_params/envelopes/common.rs`:**

```rust
use crate::eval::eval_expression;
// ... other imports

/// Generic envelope attack handler
pub fn handle_envelope_atk_impl<F>(
    prefix: &str,       // "aenv", "penv", etc.
    label: &str,        // "AMP ENV", "PITCH ENV", etc.
    cmd_prefix: &str,   // "AENV", "PENV", etc.
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("ERROR: {}.ATK REQUIRES A TIME VALUE (1-10000 MS)", cmd_prefix));
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1].parse().context(format!("Failed to parse {} attack time", label))?
    };
    if !(1..=10000).contains(&value) {
        output(format!("ERROR: {} ATTACK MUST BE BETWEEN 1 AND 10000 MS", label));
        return Ok(());
    }
    metro_tx.send(MetroCommand::SendParam(format!("{}_atk", prefix), OscType::Int(value)))?;
    if debug_level >= 2 {
        output(format!("SET {} ATTACK TO {} MS", label, value));
    }
    Ok(())
}

// Similar for handle_envelope_crv_impl, handle_envelope_mode_impl
```

**Macro for generating wrapper functions:**

```rust
macro_rules! envelope_handlers {
    ($prefix:literal, $label:literal, $cmd_prefix:literal, $atk:ident, $crv:ident, $mode:ident) => {
        pub fn $atk<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            metro_tx: &Sender<MetroCommand>,
            debug_level: u8,
            scale: &ScaleState,
            output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            handle_envelope_atk_impl($prefix, $label, $cmd_prefix, parts, variables, patterns,
                                     counters, scripts, script_index, metro_tx, debug_level, scale, output)
        }

        // Similar for $crv, $mode
    };
}

// Usage in each file:
envelope_handlers!("aenv", "AMP ENV", "AENV", handle_aenv_atk, handle_aenv_crv, handle_aenv_mode);
```

### Files to Modify
1. Create: `src/commands/synth_params/envelopes/common.rs` (~120 lines)
2. Refactor each envelope file to use macro:
   - `amp.rs`: Keep handle_ad, macro-generate ATK/CRV/MODE
   - `pitch.rs`: Keep handle_pd, handle_pa, macro-generate ATK/CRV/MODE
   - `fm.rs`: Keep handle_fd, handle_fa, macro-generate ATK/CRV/MODE
   - `disc.rs`: Keep handle_da, macro-generate ATK/CRV/MODE
   - `feedback.rs`: macro-generate all (no special handlers)
   - `filter.rs`: macro-generate all (no special handlers)
   - `global.rs`: Keep handle_env_dec, macro-generate ATK/CRV/MODE
3. Update `mod.rs` to export from common

### Migration Steps
1. Create common.rs with impl functions
2. Add macro definition
3. Migrate one envelope file at a time
4. Run `cargo test` after each file
5. Commit when all envelope files done

---

## Phase 2: Pattern Operations (~850 line reduction)

### Current State
- 16 operation pairs: P.* (working) vs PN.* (explicit)
- 90% identical code, only pattern selection differs
- P.* uses `patterns.working`, PN.* parses pattern number from args

### Approach: PatternSelector Enum + Shared Implementations

**Create `src/commands/patterns/common.rs`:**

```rust
pub enum PatternSelector {
    Working,
    Explicit(usize),
}

impl PatternSelector {
    pub fn get_index(&self, patterns: &PatternStorage) -> usize {
        match self {
            PatternSelector::Working => patterns.working,
            PatternSelector::Explicit(idx) => *idx,
        }
    }
}

/// Parse pattern number with expression evaluation
pub fn parse_pattern_number(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
) -> Result<usize> {
    let pat: usize = if let Some((expr_val, _)) = eval_expression(parts, idx, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[idx].parse().context("Failed to parse pattern number")?
    };
    if pat > 5 {
        anyhow::bail!("Pattern number must be 0-5");
    }
    Ok(pat)
}

/// Core ADD implementation
pub fn pattern_add_impl<F>(
    selector: PatternSelector,
    val: i16,
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let idx = selector.get_index(patterns);
    let pattern = &mut patterns.patterns[idx];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_add(val);
    }
    output(format!("ADDED {} TO PATTERN {}", val, idx));
}

// Similar for: pattern_sub_impl, pattern_mul_impl, pattern_div_impl, pattern_mod_impl,
// pattern_scale_impl, pattern_push_impl, pattern_pop_impl, pattern_ins_impl,
// pattern_rm_impl, pattern_rev_impl, pattern_rot_impl, pattern_shuf_impl,
// pattern_sort_impl, pattern_rnd_impl, pattern_min_impl, pattern_max_impl,
// pattern_sum_impl, pattern_avg_impl, pattern_fnd_impl
```

**Wrapper functions become thin:**

```rust
// working_math.rs
pub fn handle_pattern_add<F>(/* full signature */) -> Result<()> {
    let val = parse_i16_expr(parts, 1, /* ... */)?;
    pattern_add_impl(PatternSelector::Working, val, patterns, output);
    Ok(())
}

// explicit_math.rs
pub fn handle_pn_add<F>(/* full signature */) -> Result<()> {
    let pat = parse_pattern_number(parts, 1, /* ... */)?;
    let val = parse_i16_expr(parts, 2, /* ... */)?;
    pattern_add_impl(PatternSelector::Explicit(pat), val, patterns, output);
    Ok(())
}
```

### Files to Modify
1. Create: `src/commands/patterns/common.rs` (~300 lines)
2. Refactor:
   - `working_math.rs`: 6 operations → thin wrappers
   - `explicit_math.rs`: 6 operations → thin wrappers
   - `working_manip.rs`: 9 operations → thin wrappers
   - `explicit_manip.rs`: 9 operations → thin wrappers
   - `working_query.rs`: 5 operations → thin wrappers (if exists)
   - `explicit_query.rs`: 5 operations → thin wrappers (if exists)
3. Update `mod.rs` exports

### Migration Steps
1. Create common.rs with PatternSelector and helper functions
2. Add one impl function at a time
3. Migrate corresponding P.* and PN.* wrappers together
4. Run tests after each operation pair
5. Commit when all pattern ops done

---

## Phase 3: Synth Parameter Handlers (~2,000 line reduction)

### Current State
- 50+ handlers across 12 files in `src/commands/synth_params/`
- Each handler: 25-40 lines of nearly identical code
- Only differences: param name, type (i32/f32), range, OSC name, error messages

### Approach: Declarative Macro for Parameter Definitions

**Create `src/commands/synth_params/param_macro.rs`:**

```rust
/// Macro to define a synth parameter handler
macro_rules! define_param {
    // Integer parameter
    ($fn_name:ident, $osc_name:literal, int, $min:expr, $max:expr, $unit:literal, $desc:literal) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            metro_tx: &Sender<MetroCommand>,
            debug_level: u8,
            scale: &ScaleState,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A VALUE ({}-{}{})",
                    stringify!($fn_name).to_uppercase().trim_start_matches("HANDLE_"),
                    $min, $max, if $unit.is_empty() { "" } else { concat!(" ", $unit) }));
                return Ok(());
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as i32
            } else {
                parts[1].parse().context(concat!("Failed to parse ", $desc))?
            };
            if !($min..=$max).contains(&value) {
                output(format!("ERROR: {} MUST BE BETWEEN {} AND {}{}",
                    $desc.to_uppercase(), $min, $max,
                    if $unit.is_empty() { "".to_string() } else { format!(" {}", $unit) }));
                return Ok(());
            }
            metro_tx.send(MetroCommand::SendParam($osc_name.to_string(), OscType::Int(value)))?;
            if debug_level >= 2 {
                output(format!("SET {} TO {}{}", $desc.to_uppercase(), value,
                    if $unit.is_empty() { "".to_string() } else { format!(" {}", $unit) }));
            }
            Ok(())
        }
    };

    // Float parameter - similar structure
    ($fn_name:ident, $osc_name:literal, float, $min:expr, $max:expr, $unit:literal, $desc:literal) => {
        // ... similar but with f32
    };
}

pub(crate) use define_param;
```

**Usage in each module:**

```rust
// filter.rs
use super::param_macro::define_param;

define_param!(handle_fc, "fc", float, 20.0, 20000.0, "HZ", "filter cutoff");
define_param!(handle_fq, "fq", int, 0, 16383, "", "filter resonance");
define_param!(handle_ft, "ft", int, 0, 3, "", "filter type");
define_param!(handle_fe, "fe", int, 0, 16383, "", "filter envelope amount");
define_param!(handle_fed, "fed", int, 1, 10000, "MS", "filter envelope decay");
define_param!(handle_fk, "fk", int, 0, 16383, "", "filter key tracking");
define_param!(handle_mf_f, "mf_f", int, 0, 1, "", "modbus to filter routing");
```

### Exceptions (Keep Manual)
- `beat_repeat.rs` - extra parameters (metro_interval, br_len state)
- Handlers with enum mode descriptions (d_mode, pw, mw) - add macro variant with mode_names

### Files to Modify
1. Create: `src/commands/synth_params/param_macro.rs` (~100 lines)
2. Refactor each module:
   - `filter.rs`: 7 handlers → 7 macro calls
   - `effects.rs`: 12 handlers → 12 macro calls
   - `delay.rs`: 5 handlers → 5 macro calls (keep d_mode, d_tail manual)
   - `reverb.rs`: 4 handlers → 4 macro calls (keep r_mode, r_tail manual)
   - `oscillator.rs`: ~7 handlers → macro calls
   - `modulation.rs`: ~10 handlers → macro calls
   - `discontinuity.rs`: 3 handlers → macro calls
   - `resonator.rs`: 4 handlers → macro calls
   - `eq.rs`: 5 handlers → macro calls
   - `pitch_shift.rs`: 5 handlers → macro calls
3. Keep as-is: `beat_repeat.rs`

---

## Testing Strategy

- All 334 existing tests must pass after each phase
- Run `cargo test` after each module migration
- Run `cargo clippy` to catch any issues
- Manual smoke test with SC server after each phase complete

---

## Estimated Results

| Phase | Area | Before | After | Reduction |
|-------|------|--------|-------|-----------|
| 1 | Envelopes | ~1,142 lines | ~450 lines | ~692 lines |
| 2 | Patterns | ~1,215 lines | ~500 lines | ~715 lines |
| 3 | Synth Params | ~2,930 lines | ~800 lines | ~2,130 lines |
| **Total** | | ~5,287 lines | ~1,750 lines | **~3,537 lines (67%)** |

---

## Critical Files to Read Before Implementation

### Phase 1 (Envelopes)
- `src/commands/synth_params/envelopes/amp.rs` - reference implementation
- `src/commands/synth_params/envelopes/mod.rs` - module structure
- `src/tests/envelope_tests.rs` - test coverage

### Phase 2 (Patterns)
- `src/commands/patterns/working_math.rs` - P.* reference
- `src/commands/patterns/explicit_math.rs` - PN.* reference
- `src/tests/pattern_ops_tests.rs` - test coverage

### Phase 3 (Synth Params)
- `src/commands/synth_params/filter.rs` - typical param handlers
- `src/commands/synth_params/beat_repeat.rs` - exception case
- `src/commands/mod.rs` - dispatch routing
