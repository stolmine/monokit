# MONOKIT COMMAND DEBUG OUTPUT - EXECUTIVE SUMMARY

## Overview

Complete inventory of all 63 command files in monokit, categorizing their debug output handling patterns.

**Key Finding**: Only 4 files (6%) use the modern `ctx.output()` pattern. 21 files (33%) use legacy `debug_level` checks. 38 files (60%) produce no output at all.

---

## Quick Stats

| Metric | Count |
|--------|-------|
| **Total Command Files** | 63 |
| **Files with Output** | 25 |
| **Files without Output** | 38 |
| **Total Output Calls** | 316 |
| **Modern Pattern (ctx.output)** | 4 files, 118 uses |
| **Legacy Pattern (debug_level >=)** | 21 files, 198 uses |
| **Mixed Pattern** | 1 file, 56 uses |
| **Magic Numbers (>= N)** | 4 files, 27 uses |

---

## Pattern Breakdown

### NEW PATTERN: `ctx.output(OutputCategory::X, message, &mut output)`

**Status**: Modern, centralized approach (DESIRED)

**Files (4 total)**:
1. `src/commands/core/variables.rs` (8 uses) - COMPLETE
2. `src/commands/patterns/working.rs` (17 uses) - COMPLETE
3. `src/commands/patterns/explicit.rs` (18 uses) - COMPLETE
4. `src/commands/system/metro.rs` (5 uses) - COMPLETE

**Characteristics**:
- Uses ExecutionContext's output method
- Specifies OutputCategory (Query, Confirm, Error, Essential)
- Debug level filtering centralized in ctx.output()
- Cleaner signatures, less parameter pollution
- Ready for audit trail & formatting upgrades

**Example**:
```rust
ctx.output(OutputCategory::Confirm, format!("SET {} TO {}", name, value), &mut output);
```

---

### OLD PATTERN: `if debug_level >= TIER_X { output(...) }`

**Status**: Legacy, distributed approach (NEEDS MIGRATION)

**Files (21 total, 198 uses)**:

**Core Commands** (34 uses):
- counters.rs (9)
- scheduling.rs (9)
- sync.rs (8)
- scale.rs (6) - uses magic number

**System Commands** (16 uses):
- preset.rs (8)
- scene.rs (8)

**Synth Commands** (56 uses):
- synth/common.rs (14)
- pitch_shift.rs (10)
- beat_repeat.rs (8)
- envelopes/common.rs (4)
- clouds.rs (2)
- plaits/params.rs (2) - uses magic number
- output.rs (3) - uses magic number

**Gate & Randomization** (39 uses):
- randomization.rs (25)
- gate.rs (14) - all use magic numbers

**Patterns** (3 uses):
- patterns/common.rs (3)

**Characteristics**:
- Direct debug_level comparison in handlers
- Conditional output (may not execute)
- Uses TIER constants or magic numbers
- Combined with explicit flags (out_err, out_qry, out_cfm, out_ess)
- Distributed logic, harder to audit

**Example**:
```rust
if debug_level >= TIER_CONFIRMS || out_cfm {
    output(format!("SET COUNTER TO {}", value));
}
```

---

### MIXED PATTERN: Both new and old in same file

**Status**: Partial migration, transition state

**Files (3 total, 83 uses)**:
1. `src/commands/patterns/macros.rs` (46 new + 10 old = 56 uses)
   - Macro-generated code uses ctx.output
   - Helper functions still use debug_level checks
   
2. `src/commands/system/misc.rs` (4 new + 20 old = 24 uses)
   - Some functions use ctx.output
   - Macros still use debug_level checks
   
3. `src/commands/synth/effects/clouds.rs` (1 new + 2 old = 3 uses)
   - handle_cl_trig uses ctx.output
   - handle_cl_fb uses debug_level

---

### NO OUTPUT: Silent execution

**Status**: Fully silent (may need output added)

**Files (38 total)**:
- Synth parameters (7): oscillator, filter, noise, modulation, resonator, source_levels, discontinuity
- Effects (6): compressor, delay, eq, lofi, reverb, ring_mod
- Envelopes (6): amp, disc, feedback, filter, fm, pitch
- System (3): audio, midi, sc
- Utilities (3): validate, validate_expr, aliases
- Infrastructure (2): context, common
- Plaits (1): engine

**Characteristics**:
- No user feedback on execution
- Silently set parameters/configuration
- May need output added (design decision)

---

## Priority Migration Matrix

### HIGHEST PRIORITY (HIGH) - 172 uses
**Rationale**: Core functionality, high usage, fixes magic numbers

1. **randomization.rs** (25 uses)
   - 10+ handlers with mixed TIER patterns
   - Core RND.* command set

2. **misc.rs** (24 uses)
   - System configuration commands
   - Mixed pattern (4 ctx.output + 20 debug_level)

3. **gate.rs** (14 uses)
   - ~14 handlers using magic number (>= 2)
   - Critical gate timing commands

4. **synth/common.rs** (14 uses)
   - 3 macros affecting many synth commands
   - Defines int_param, float_param patterns

5. **preset.rs** (8 uses) + **scene.rs** (8 uses)
   - Configuration loading/saving
   - System state critical

6. **counters.rs** (9 uses) + **scheduling.rs** (9 uses)
   - Core command processing
   - Nested macro complexity

7. **scale.rs** (6 uses) + **output.rs** (5 uses)
   - Magic number cleanup needed

### MEDIUM PRIORITY - 98 uses
**Rationale**: Useful features, manageable complexity

- patterns/macros.rs (56 uses) - Complex, multiple macros
- pitch_shift.rs (10 uses)
- beat_repeat.rs (8 uses)
- envelopes/common.rs (4 uses)
- patterns/common.rs (3 uses)
- clouds.rs (3 uses) - Partially done

### LOW PRIORITY - 2 uses
**Rationale**: Minor impact, low usage

- plaits/params.rs (2 uses) - Magic number replacement

---

## Magic Number Problem

4 files use hardcoded magic numbers instead of semantic constants:

```rust
if debug_level >= 2 {  // ← PROBLEMATIC
    output(...);
}
```

**Files affected**:
- scale.rs (3 uses)
- gate.rs (8-10 uses)
- output.rs (3 uses, some mixed)
- plaits/params.rs (2 uses)

**Total magic number uses**: ~27

**Recommendation**: Define constant
```rust
const TIER_FEEDBACK = 2;  // Or use existing TIER_CONFIRMS
```

---

## Inconsistencies Found

### 1. Handler Signature Variations
- **NEW**: Uses `ctx: &mut ExecutionContext`
- **OLD**: Uses individual parameters (debug_level, out_err, out_qry, etc.)
- **RESULT**: Inconsistent API across command module

### 2. Output Control
- **NEW**: Debug level filtering in ctx.output()
- **OLD**: Debug level check in handler, output conditional
- **RESULT**: Different behavior patterns, harder to trace

### 3. Tier Constants vs Magic Numbers
- **GOOD**: randomization.rs uses TIER_CONFIRMS, TIER_QUERIES, TIER_ERRORS, TIER_ESSENTIAL
- **BAD**: gate.rs always uses >= 2 (magic number)
- **RESULT**: Inconsistent semantic meaning

### 4. Output Callback Handling
- **NEW**: Passed to ctx.output() which decides execution
- **OLD**: Passed to handler, called conditionally
- **RESULT**: Different responsibility distribution

---

## Recommended Approach

### Phase 1: Quick Wins (2-3 weeks)
1. Fix magic numbers (scale.rs, gate.rs, output.rs, plaits/params.rs)
   - Replace `>= 2` with `>= TIER_CONFIRMS` or new constant
   - ~27 changes, low risk

2. Migrate simple handlers (sync.rs, slew.rs)
   - Only TIER_CONFIRMS pattern
   - ~12 uses, straightforward migration

### Phase 2: Core Migration (3-4 weeks)
3. Core commands (counters.rs, scheduling.rs)
   - More complex due to macros
   - ~18 uses, moderate risk

4. System config (preset.rs, scene.rs)
   - User-visible functionality
   - ~16 uses, moderate complexity

### Phase 3: Large Refactors (4-6 weeks)
5. Synth commands (synth/common.rs, effects/*)
   - Affects many parameter handlers
   - ~42 uses, high complexity

6. Randomization (25 uses)
   - Heavy usage, multiple tiers
   - ~25 uses, high complexity

### Phase 4: Final Cleanup (2-3 weeks)
7. Pattern macros (patterns/macros.rs, patterns/common.rs)
   - Macro code complexity
   - ~59 uses, very high complexity

8. System misc (misc.rs)
   - Mixed patterns, multiple macros
   - ~24 uses, high complexity

---

## Success Criteria

**Completion**: All 21 OLD pattern files migrated to NEW pattern

**Metrics to track**:
- Number of files using NEW pattern
- Number of files using OLD pattern
- Magic number occurrences
- Consistency of handler signatures
- Test coverage for output behavior

**Definition of "Done"**:
- All command handlers use `ctx.output(OutputCategory::*, ...)`
- No hardcoded magic numbers (>= 2)
- Consistent handler signatures across module
- All TIER constants used semantically
- No conditional output logic in handlers

---

## Documentation Files

Three reference documents created:

1. **COMMAND_INVENTORY.md** - Detailed catalog of all commands
   - Full breakdown by category
   - Line numbers and specific details
   - Current status of each file

2. **COMMAND_PATTERNS_DETAIL.md** - Technical reference
   - Pattern examples with code
   - Tier constants mapping
   - Migration checklist
   - Validation procedures

3. **COMMAND_MATRIX.md** - Quick reference table
   - All files in matrix format
   - Priority and status columns
   - Statistics and dependencies
   - Migration order recommendations

---

## Next Steps

1. **Review** this summary with team
2. **Plan** which phase to tackle first
3. **Assign** resources for Phase 1 (magic number fixes)
4. **Create** migration tracking in ROADMAP.md
5. **Begin** with simplest files in Phase 1

---

## Related Files

- `src/commands/context.rs` - ExecutionContext definition
- `src/types/mod.rs` - OutputCategory, TIER constants
- `src/commands/mod.rs` - Main dispatcher (189 debug_level references)

---

**Generated**: 2026-01-02  
**Scope**: All 63 command files in monokit  
**Pattern Count**: 316 debug output calls across codebase
