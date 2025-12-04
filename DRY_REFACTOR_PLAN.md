# DRY Refactoring Plan for Monokit

## Overview

Comprehensive reorganization and DRY refactoring to create a logical, maintainable command structure.

**Goals:**
1. Organize commands by logical domain (Phase 0)
2. Consolidate scattered parameters into proper locations (Phase 0)
3. Apply DRY macros to eliminate boilerplate (Phases 1-3)

**Expected Results:**
- Clear, logical file organization
- ~4,000+ line reduction through macro consolidation
- Easier to add new commands (single location per domain)
- No loss of functionality (558 tests must pass throughout)

**Last Updated:** 2025-11-30

---

## Phase 0: Codebase Reorganization [COMPLETE]

### Current Problems

1. **Scattered envelope parameters:**
   - FBA, FBD in `oscillator.rs` (should be in `envelopes/feedback.rs`)
   - FE, FED in `filter.rs` (should be in `envelopes/filter.rs`)
   - DD in `discontinuity.rs` (should be in `envelopes/disc.rs`)

2. **Naming confusion:**
   - `delay.rs` handles command scheduling (DEL), not synth delay
   - `synth_params/delay.rs` has actual delay parameters

3. **Fragmented effects:**
   - EQ in separate `eq.rs` instead of with other effects
   - VOL in `misc.rs` instead of with output params
   - PAN in `effects.rs` separate from VOL

4. **Dead code:**
   - `envelopes/global.rs` - handlers not exported or routed
   - MODE handlers in all envelope files - deprecated, not exported
   - GATE handlers in `gate.rs` - not routed in dispatcher

5. **Flat structure in synth_params:**
   - 13 files at same level, hard to navigate
   - Related effects not grouped together

### Target Directory Structure

```
src/commands/
├── mod.rs                      # Main dispatcher
├── aliases.rs                  # Alias resolution
├── validate.rs                 # Validation helpers
│
├── core/                       # Language primitives
│   ├── mod.rs
│   ├── variables.rs            # A-D, X-Z, T, I, J, K
│   ├── counters.rs             # N1-N4 with MIN/MAX/RST
│   ├── math_ops.rs             # ADD, SUB, MUL, DIV, MOD, MAP
│   ├── random_ops.rs           # RND, RRND, TOSS, EITH, TOG
│   ├── scale.rs                # Q, Q.ROOT, Q.SCALE, Q.BIT
│   └── scheduling.rs           # DEL, DEL.CLR, DEL.X, DEL.R (renamed from delay.rs)
│
├── patterns/                   # Pattern operations (already well-organized)
│   ├── mod.rs
│   ├── common.rs               # NEW: shared impl functions
│   ├── working.rs              # P.N, P.L, P.I, P.HERE, P.NEXT, P.PREV
│   ├── working_math.rs         # P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE
│   ├── working_manip.rs        # P.PUSH, P.POP, P.INS, P.RM, P.REV, P.ROT, P.SHUF, P.SORT, P.RND
│   ├── working_query.rs        # P.MIN, P.MAX, P.SUM, P.AVG, P.FND
│   ├── explicit.rs             # PN.N, PN.L, PN.I, PN.HERE, PN.NEXT, PN.PREV
│   ├── explicit_math.rs        # PN.ADD, PN.SUB, etc.
│   ├── explicit_manip.rs       # PN.PUSH, PN.POP, etc.
│   └── explicit_query.rs       # PN.MIN, PN.MAX, etc.
│
├── system/                     # System/session commands
│   ├── mod.rs
│   ├── metro.rs                # M, M.BPM, M.ACT, M.SCRIPT
│   ├── scene.rs                # SAVE, LOAD, SCENES, DELETE
│   ├── recording.rs            # REC, REC.STOP, REC.PATH (extract from misc.rs)
│   └── misc.rs                 # TR, RST, SCRIPT, THEME, DEBUG, PRINT, HELP, CLEAR
│
├── synth/                      # Synth parameters (renamed from synth_params)
│   ├── mod.rs
│   ├── param_macro.rs          # NEW: DRY macro for param handlers
│   │
│   ├── oscillator.rs           # PF, PW, MF, MW, FB (core oscillator only)
│   ├── discontinuity.rs        # DC, DM (amount and mode only)
│   ├── modulation.rs           # TK, MB, MP, MD, MT, MA, FM, MX, MM, ME
│   ├── filter.rs               # FC, FQ, FT, FK, MF.F (core filter only)
│   ├── resonator.rs            # RF, RD, RM, RK
│   ├── output.rs               # NEW: VOL, PAN (consolidated)
│   ├── slew.rs                 # SLEW, SLEW.ALL
│   │
│   ├── envelopes/              # ALL envelope params consolidated
│   │   ├── mod.rs
│   │   ├── common.rs           # NEW: shared macro for DEC, AMT, ATK, CRV
│   │   ├── amp.rs              # AD, AENV.ATK, AENV.CRV
│   │   ├── pitch.rs            # PD, PA, PENV.ATK, PENV.CRV
│   │   ├── fm.rs               # FD, FA, FMEV.ATK, FMEV.CRV
│   │   ├── disc.rs             # DD, DA, DENV.ATK, DENV.CRV (DD moved from discontinuity.rs)
│   │   ├── feedback.rs         # FBD, FBA, FBEV.ATK, FBEV.CRV (FBD/FBA moved from oscillator.rs)
│   │   └── filter.rs           # FED, FE, FLEV.ATK, FLEV.CRV (FED/FE moved from filter.rs)
│   │
│   └── effects/                # Time-based and processing effects
│       ├── mod.rs
│       ├── common.rs           # NEW: shared macro for effect params
│       ├── delay.rs            # DT, DF, DLP, DW, DS, D.MODE, D.TAIL
│       ├── reverb.rs           # RV, RP, RH, RW, R.MODE, R.TAIL
│       ├── lofi.rs             # LB, LS, LM (extract from effects.rs)
│       ├── ring_mod.rs         # RGF, RGW, RGM (extract from effects.rs)
│       ├── compressor.rs       # CT, CR, CA, CL, CM (extract from effects.rs)
│       ├── eq.rs               # EL, EM, EF, EQ, EH
│       ├── beat_repeat.rs      # BR.ACT, BR.LEN, BR.REV, BR.WIN, BR.MIX
│       └── pitch_shift.rs      # PS.MODE, PS.SEMI, PS.GRAIN, PS.MIX, PS.TARG
│
└── randomization.rs            # RND.VOICE, RND.OSC, RND.FM, RND.MOD, RND.ENV, etc.
```

### Parameter Consolidation Map

**Move TO `synth/envelopes/disc.rs`:**
| Command | FROM | OSC Param |
|---------|------|-----------|
| DD | discontinuity.rs | dd |

**Move TO `synth/envelopes/feedback.rs`:**
| Command | FROM | OSC Param |
|---------|------|-----------|
| FBA | oscillator.rs | fba |
| FBD | oscillator.rs | fbd |
| FBEV.AMT | (alias for FBA) | fba |

**Move TO `synth/envelopes/filter.rs`:**
| Command | FROM | OSC Param |
|---------|------|-----------|
| FE | filter.rs | fe |
| FED | filter.rs | fed |

**Move TO `synth/output.rs` (NEW FILE):**
| Command | FROM | OSC Param |
|---------|------|-----------|
| VOL | misc.rs | volume |
| PAN | effects.rs | pan |

### Files to Delete

| File | Reason |
|------|--------|
| `synth_params/envelopes/global.rs` | Dead code - handlers not exported or routed |
| `gate.rs` | Deprecated - GATE commands removed in envelope simplification |

### Files to Rename

| Current | New | Reason |
|---------|-----|--------|
| `delay.rs` | `core/scheduling.rs` | Handles DEL command scheduling, not synth delay |
| `synth_params/` | `synth/` | Shorter, clearer |
| `synth_params/effects.rs` | Split into `synth/effects/*.rs` | One file per effect type |

### Phase 0 Migration Steps

1. **Create new directory structure:**
   ```bash
   mkdir -p src/commands/core
   mkdir -p src/commands/system
   mkdir -p src/commands/synth/effects
   ```

2. **Move and rename files (no code changes yet):**
   - `delay.rs` → `core/scheduling.rs`
   - `synth_params/*` → `synth/*`
   - Create `synth/effects/` and split `effects.rs`

3. **Consolidate envelope params:**
   - Move DD handler from `discontinuity.rs` → `envelopes/disc.rs`
   - Move FBA, FBD handlers from `oscillator.rs` → `envelopes/feedback.rs`
   - Move FE, FED handlers from `filter.rs` → `envelopes/filter.rs`

4. **Create output.rs:**
   - Move VOL from `misc.rs`
   - Move PAN from `effects.rs`

5. **Delete dead code:**
   - Delete `envelopes/global.rs`
   - Delete `gate.rs`
   - Delete MODE handlers from all envelope files

6. **Update mod.rs files:**
   - Update all module declarations
   - Update dispatcher routing

7. **Run tests:** `cargo test` (all 411 must pass)

---

## Phase 1: Envelope Handler DRY [COMPLETE]

### Phase 1 Implementation Summary

**Created:**
- `synth/envelopes/common.rs` - Shared `define_envelope!` macro and `define_param!` integration

**Files Refactored (6 envelope files):**
- `synth/envelopes/amp.rs`
- `synth/envelopes/pitch.rs`
- `synth/envelopes/fm.rs`
- `synth/envelopes/disc.rs`
- `synth/envelopes/feedback.rs`
- `synth/envelopes/filter.rs`

**Dead Code Removed:**
- Deleted `synth/envelopes/global.rs` (handlers not exported or routed)
- Removed all deprecated `*_mode` handlers from all 6 envelope files

**Results:**
- Line reduction: 918 lines (81% decrease in envelope code)
- All 558 tests pass
- No loss of functionality
- Cleaner, more maintainable envelope implementation

### After Phase 0, Envelope Structure Will Be:

| Envelope | File | Commands |
|----------|------|----------|
| Amp | `envelopes/amp.rs` | AD, AENV.ATK, AENV.CRV |
| Pitch | `envelopes/pitch.rs` | PD, PA, PENV.ATK, PENV.CRV |
| FM | `envelopes/fm.rs` | FD, FA, FMEV.ATK, FMEV.CRV |
| Disc | `envelopes/disc.rs` | DD, DA, DENV.ATK, DENV.CRV |
| Feedback | `envelopes/feedback.rs` | FBD, FBA, FBEV.ATK, FBEV.CRV |
| Filter | `envelopes/filter.rs` | FED, FE, FLEV.ATK, FLEV.CRV |

**Each file has 4 handlers with identical patterns:**
- Decay (DEC): int, 1-10000 ms
- Amount (AMT): float, 0-16 (varies by envelope)
- Attack (ATK): int, 1-10000 ms
- Curve (CRV): float, -8.0 to 8.0

### Create `synth/envelopes/common.rs`

```rust
use crate::eval::eval_expression;
use crate::types::*;
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

/// Macro to generate all 4 envelope handlers for a given envelope type
macro_rules! define_envelope {
    (
        $prefix:literal,           // "aenv", "penv", etc.
        $label:literal,            // "AMP", "PITCH", etc.
        $dec_fn:ident,             // handle_ad, handle_pd, etc.
        $amt_fn:ident,             // handle_pa, handle_fa, etc. (or none for amp)
        $atk_fn:ident,             // handle_aenv_atk, etc.
        $crv_fn:ident,             // handle_aenv_crv, etc.
        $amt_max:expr              // 16.0 for most, 16383 for filter
    ) => {
        // Generate decay handler
        define_param!($dec_fn, concat!($prefix, "_dec"), int, 1, 10000, "MS",
                      concat!($label, " DECAY"));

        // Generate amount handler (if not amp envelope)
        define_param!($amt_fn, concat!($prefix, "_amt"), float, 0.0, $amt_max, "",
                      concat!($label, " ENV AMOUNT"));

        // Generate attack handler
        define_param!($atk_fn, concat!($prefix, "_atk"), int, 1, 10000, "MS",
                      concat!($label, " ENV ATTACK"));

        // Generate curve handler
        define_param!($crv_fn, concat!($prefix, "_crv"), float, -8.0, 8.0, "",
                      concat!($label, " ENV CURVE"));
    };
}

pub(crate) use define_envelope;
```

### Usage in Each Envelope File

```rust
// envelopes/pitch.rs
use super::common::define_envelope;

define_envelope!(
    "penv", "PITCH",
    handle_pd, handle_pa, handle_penv_atk, handle_penv_crv,
    16.0
);
```

---

## Phase 2: Pattern Operation DRY (~1,300 line reduction)

### Create `patterns/common.rs`

```rust
pub enum PatternSelector {
    Working,
    Explicit(usize),
}

impl PatternSelector {
    pub fn get_mut<'a>(&self, patterns: &'a mut PatternStorage) -> &'a mut Pattern {
        match self {
            PatternSelector::Working => &mut patterns.patterns[patterns.working],
            PatternSelector::Explicit(idx) => &mut patterns.patterns[*idx],
        }
    }

    pub fn index(&self, patterns: &PatternStorage) -> usize {
        match self {
            PatternSelector::Working => patterns.working,
            PatternSelector::Explicit(idx) => *idx,
        }
    }
}

// Shared implementations
pub fn pattern_add_impl(selector: PatternSelector, val: i16, patterns: &mut PatternStorage) {
    let pattern = selector.get_mut(patterns);
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_add(val);
    }
}

// ... similar for SUB, MUL, DIV, MOD, SCALE, PUSH, POP, INS, RM, REV, ROT, SHUF, SORT, RND
// ... and MIN, MAX, SUM, AVG, FND
```

### Thin Wrappers

```rust
// working_math.rs - becomes ~30 lines total
pub fn handle_pattern_add<F>(...) -> Result<()> {
    let val = parse_i16_expr(parts, 1, ...)?;
    common::pattern_add_impl(PatternSelector::Working, val, patterns);
    output(format!("ADDED {} TO PATTERN {}", val, patterns.working));
    Ok(())
}

// explicit_math.rs - becomes ~40 lines total
pub fn handle_pn_add<F>(...) -> Result<()> {
    let pat = parse_pattern_number(parts, 1, ...)?;
    let val = parse_i16_expr(parts, 2, ...)?;
    common::pattern_add_impl(PatternSelector::Explicit(pat), val, patterns);
    output(format!("ADDED {} TO PATTERN {}", val, pat));
    Ok(())
}
```

---

## Phase 3: Synth Parameter DRY (~2,000 line reduction)

### Create `synth/param_macro.rs`

```rust
/// Core macro for simple synth parameters
macro_rules! define_param {
    // Integer parameter
    ($fn:ident, $osc:literal, int, $min:expr, $max:expr, $unit:literal, $desc:literal) => {
        pub fn $fn<F>(
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
            param_impl_int(parts, variables, patterns, counters, scripts,
                          script_index, metro_tx, debug_level, scale, &mut output,
                          $osc, $min, $max, $unit, $desc)
        }
    };

    // Float parameter
    ($fn:ident, $osc:literal, float, $min:expr, $max:expr, $unit:literal, $desc:literal) => {
        // Similar with f32
    };
}

fn param_impl_int<F>(..., osc: &str, min: i32, max: i32, unit: &str, desc: &str) -> Result<()> {
    // Single implementation used by all int params
}

pub(crate) use define_param;
```

### Usage in Effect Files

```rust
// synth/effects/delay.rs
use super::super::param_macro::define_param;

define_param!(handle_dt, "dt", int, 1, 2000, "MS", "DELAY TIME");
define_param!(handle_df, "df", int, 0, 16383, "", "DELAY FEEDBACK");
define_param!(handle_dlp, "dlp", int, 100, 20000, "HZ", "DELAY LP CUTOFF");
define_param!(handle_dw, "dw", int, 0, 16383, "", "DELAY WET");
define_param!(handle_ds, "ds", int, 0, 1, "", "DELAY SYNC");

// Keep manual for mode handlers that need string output
pub fn handle_d_mode<F>(...) -> Result<()> {
    // Manual implementation with mode names
}
```

---

## Testing Strategy

### Before Each Phase
```bash
cargo test           # All 558 tests must pass
cargo clippy         # No warnings
cargo build --release
```

### Test Coverage by Area
- Envelope tests: 74 tests
- Pattern tests: 73 tests
- Buffer effects: 16 tests
- FX randomization: 10 tests
- Math/expression: 42 tests
- Variables/counters: 22 tests

### Incremental Validation
- Move ONE file at a time
- Run `cargo test` after each move
- Commit after each successful migration
- Tag commits for easy rollback

---

## Results Summary

| Phase | Area | Estimated Reduction | Actual Reduction | Status |
|-------|------|---------------------|------------------|--------|
| 0 | Reorganization | Cleaner structure | Cleaner structure | COMPLETE |
| 1 | Envelopes | ~841 lines | **918 lines (81%)** | **COMPLETE** |
| 2 | Patterns | ~1,423 lines | **1,573 lines (78% of wrappers)** | **COMPLETE** |
| 3 | Synth Params | ~2,197 lines | **2,325 lines** | **COMPLETE** |
| 4 | Variables/Counters/Tests | ~800-850 lines | **1,126 lines** | **COMPLETE** |
| **Total** | | **~5,600-5,700** | **~5,942 lines (28%)** | **COMPLETE** |

**Phase 1 Notes:**
- Achieved 918 line reduction vs 841 estimated (109% of estimate)
- Better than expected due to removal of global.rs and mode handlers
- All 558 tests pass with no loss of functionality

**Phase 2 Notes:**
- Created `patterns/common.rs` (902 lines) with `PatternRef` enum, shared implementations, AND macro system
- Aggressive macro-based refactoring: macros generate both P.* and PN.* handlers from single definition
- Helper functions: `parse_pattern_num`, `parse_i16_expr`, `parse_usize_expr`
- Macro types: `define_pattern_op_1val!`, `define_pattern_op_noarg!`, `define_pattern_op_2val!`, `define_pattern_op_idx!`, `define_pattern_op_idx_val!`, `define_pattern_query!`, `define_pattern_query_1val!`, `define_pattern_nav!`, `define_pattern_pop!`, `define_pattern_rnd!`
- **Wrapper code reduced from 2023 → 450 lines (1573 line reduction, 78%)**
- Individual file reductions:
  - working_math.rs: 228 → 13 lines (94%)
  - explicit_math.rs: 294 → 8 lines (97%)
  - working_manip.rs: 269 → 15 lines (94%)
  - explicit_manip.rs: 420 → 11 lines (97%)
  - working_query.rs: 94 → 9 lines (90%)
  - explicit_query.rs: 197 → 7 lines (96%)
  - working.rs: 168 → 135 lines (nav handlers only, 20%)
  - explicit.rs: 268 → 166 lines (nav handlers only, 38%)
- All 558 tests pass with no loss of functionality

---

## Rollback Plan

- Each phase is a separate branch
- Merge to main only after full test pass
- Keep old file paths as git history
- Tag releases before major changes

---

## Implementation Order

1. **Phase 0A:** Create directory structure, move files (no code changes)
2. **Phase 0B:** Consolidate envelope params (move handlers between files)
3. **Phase 0C:** Create output.rs, delete dead code
4. **Phase 0D:** Update all mod.rs and dispatcher
5. **Phase 1:** Apply envelope macros
6. **Phase 2:** Apply pattern common.rs
7. **Phase 3:** Apply synth param macros

Each step: test → commit → proceed
