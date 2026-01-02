# MONOKIT COMMAND DEBUG OUTPUT INVENTORY

## Summary

Total Command Files: 63
- NEW Pattern (ctx.output): 4 files
- OLD Pattern (debug_level): 21 files  
- NO OUTPUT: 38 files
- MIXED (both patterns): 1 file

### Adoption Status

| Pattern | Files | Count | Status |
|---------|-------|-------|--------|
| ctx.output(OutputCategory::) | 4 | ~118 uses | Modern (100% NEW) |
| debug_level >= TIER_ or >= N | 21 | ~198 uses | Legacy |
| No debug output | 38 | - | Silent |
| Mixed patterns | 1 | 3 ctx + 2 debug | Transitional |

---

## DETAILED INVENTORY BY CATEGORY

### CORE COMMANDS

#### 1. Variables (src/commands/core/variables.rs)
- **Pattern**: NEW (ctx.output)
- **Usage Count**: 8 occurrences
- **Categories Used**: Query, Confirm, Error
- **Status**: FULLY MIGRATED
- **Commands**:
  - handle_variable_a/b/c/d/x/y/z/t (8 uses of ctx.output)
  - handle_variable_j/k (script context)
  - handle_variable_i (direct output)

#### 2. Counters (src/commands/core/counters.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 9 occurrences
- **Tier Levels Used**: TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS
- **Status**: LEGACY - needs migration
- **Commands**:
  - handle_n1/n2/n3/n4 (counter read/reset/max/min)
  - Macros: define_counter_read, define_counter_reset, define_counter_max, define_counter_min

#### 3. Scheduling/Delays (src/commands/core/scheduling.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 9 occurrences
- **Tier Levels Used**: TIER_ESSENTIAL, TIER_CONFIRMS
- **Status**: LEGACY - needs migration
- **Commands**:
  - handle_del, handle_del_clr, handle_del_x, handle_del_r
  - Uses TIER_CONFIRMS in del/del_x/del_r
  - Uses TIER_ESSENTIAL in del_clr

#### 4. Sync (src/commands/core/sync.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 8 occurrences
- **Tier Level**: TIER_CONFIRMS (always)
- **Status**: LEGACY - needs migration
- **Commands**:
  - handle_sync (line 29)
  - handle_sync_seq (line 46)
  - handle_sync_tog (line 63)
  - handle_sync_pat (line 80)

#### 5. Scale/Quantize (src/commands/core/scale.rs)
- **Pattern**: OLD (debug_level >= 2)
- **Usage Count**: 6 occurrences
- **Status**: LEGACY - magic number hardcoded (no TIER constants)
- **Commands**:
  - handle_q_root (line 92)
  - handle_q_scale (line 139)
  - handle_q_bit (line 170)
  - Uses hardcoded "debug_level >= 2"

---

### PATTERN COMMANDS

#### 1. Working/Explicit Patterns (src/commands/patterns/working.rs, explicit.rs)
- **Pattern**: NEW (ctx.output)
- **Usage Count**: 17 + 18 = 35 occurrences
- **Status**: FULLY MIGRATED
- **Details**:
  - working.rs: 17 ctx.output calls
  - explicit.rs: 18 ctx.output calls
  - Both use modern OutputCategory pattern

#### 2. Pattern Macros (src/commands/patterns/macros.rs)
- **Pattern**: MIXED (46 ctx.output + 10 debug_level)
- **Usage Count**: 56 total occurrences
- **Status**: PARTIALLY MIGRATED
- **Details**:
  - define_pattern_op_1val: mixed (ctx.output for results, debug_level in helper)
  - define_pattern_op_noarg: mixed usage
  - define_pattern_op_2val: primarily ctx.output
  - Helper functions use debug_level checks
  - Output via ctx.output(OutputCategory::Error/Confirm)

#### 3. Pattern Common (src/commands/patterns/common.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 3 occurrences
- **Status**: LEGACY - needs migration
- **Details**:
  - parse_pattern_num function checks debug_level >= TIER_ERRORS
  - Used by pattern operations

---

### SYNTH COMMANDS

#### Oscillator & Filter
- **oscillator.rs**: NO OUTPUT
- **filter.rs**: NO OUTPUT
- **noise.rs**: NO OUTPUT

#### Effects

##### Beat Repeat (src/commands/synth/effects/beat_repeat.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 8 occurrences
- **Tier Level**: TIER_CONFIRMS (line 57, 97)
- **Status**: LEGACY - needs migration
- **Commands**: handle_br_len, handle_br_rev

##### Clouds (src/commands/synth/effects/clouds.rs)
- **Pattern**: MIXED (1 ctx.output + 2 debug_level)
- **Usage Count**: 3 occurrences
- **Status**: PARTIALLY MIGRATED
- **Details**:
  - handle_cl_trig: uses ctx.output (NEW pattern)
  - handle_cl_fb: uses debug_level >= TIER_CONFIRMS (OLD pattern)
  - Macros for cl_pitch, cl_pos, cl_size, etc. use debug_level

##### Pitch Shift (src/commands/synth/effects/pitch_shift.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 10 occurrences
- **Status**: LEGACY - needs migration

##### Compressor, Delay, EQ, LoFi, Reverb, Ring Mod
- **NO OUTPUT** for all

#### Envelopes

##### Common (src/commands/synth/envelopes/common.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 4 occurrences
- **Status**: LEGACY - needs migration

##### Amp, Disc, Feedback, Filter, FM, Pitch
- **NO OUTPUT** for all

#### Output (src/commands/synth/output.rs)
- **Pattern**: OLD (debug_level >= 2 and TIER_QUERIES)
- **Usage Count**: 5 occurrences
- **Status**: LEGACY - mixed hardcoded and TIER constants
- **Details**:
  - handle_vol: debug_level >= 2
  - handle_pan: debug_level >= 2
  - handle_vca: debug_level >= 2 and TIER_QUERIES
  - Magic number hardcoding mixed with TIER constants

#### Modulation, Resonator, Source Levels
- **NO OUTPUT** for all

#### Synth Common (src/commands/synth/common.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 14 occurrences
- **Tier Levels**: TIER_CONFIRMS, TIER_ERRORS, TIER_ESSENTIAL
- **Status**: LEGACY - needs migration
- **Macros**: define_int_param, define_int_param_ms, define_float_param

#### Plaits

##### Params (src/commands/synth/plaits/params.rs)
- **Pattern**: OLD (debug_level >= 2)
- **Usage Count**: 2 occurrences
- **Status**: LEGACY - hardcoded magic number

##### Engine
- **NO OUTPUT**

#### Discontinuity
- **NO OUTPUT**

---

### SYSTEM COMMANDS

#### Metro (src/commands/system/metro.rs)
- **Pattern**: NEW (ctx.output)
- **Usage Count**: 5 occurrences
- **Status**: FULLY MIGRATED
- **Commands**: handle_tr, handle_pltr

#### Misc (src/commands/system/misc.rs)
- **Pattern**: MIXED (4 ctx.output + 20 debug_level)
- **Usage Count**: 24 occurrences
- **Status**: HEAVILY LEGACY
- **Details**:
  - Macros use debug_level >= TIER_ patterns
  - Some functions use ctx.output
  - define_bool_toggle, define_enum_select use direct output()
  - handle_tr, handle_vol, handle_pltr use ctx.output

#### Preset (src/commands/system/preset.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 8 occurrences
- **Tier Level**: TIER_CONFIRMS
- **Status**: LEGACY - needs migration

#### Scene (src/commands/system/scene.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 8 occurrences
- **Tier Level**: TIER_CONFIRMS
- **Status**: LEGACY - needs migration

#### Audio, MIDI, SC
- **NO OUTPUT** for all

---

### GATE & RANDOMIZATION

#### Gate (src/commands/gate.rs)
- **Pattern**: OLD (debug_level >= 2)
- **Usage Count**: 14 occurrences
- **Status**: LEGACY - hardcoded magic number
- **Details**:
  - handle_gate, handle_aenv_gate, handle_penv_gate, etc.
  - All use "debug_level >= 2"
  - No TIER constants used

#### Randomization (src/commands/randomization.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 25 occurrences
- **Tier Levels**: TIER_CONFIRMS, TIER_QUERIES, TIER_ERRORS, TIER_ESSENTIAL
- **Status**: LEGACY - needs migration
- **Commands**: handle_rnd_voice, handle_rnd_osc, handle_rnd_fm, handle_rnd_mod, etc.

#### Slew (src/commands/slew.rs)
- **Pattern**: OLD (debug_level >= TIER_)
- **Usage Count**: 4 occurrences
- **Status**: LEGACY - needs migration

---

### ROOT LEVEL

#### Validate (src/commands/validate.rs, validate_expr.rs)
- **NO OUTPUT**

#### Aliases (src/commands/aliases.rs)
- **NO OUTPUT**

#### Context (src/commands/context.rs)
- **NO OUTPUT** (contains ExecutionContext definition)

#### Main Dispatcher (src/commands/mod.rs)
- **Pattern**: OLD (debug_level references)
- **Usage Count**: 189 occurrences
- **Status**: Extracts fields from context; legacy parameter passing

---

## MIGRATION SUMMARY TABLE

| Category | File | Pattern | Count | Priority |
|----------|------|---------|-------|----------|
| Core | variables.rs | NEW | 8 | DONE |
| Core | counters.rs | OLD | 9 | HIGH |
| Core | scheduling.rs | OLD | 9 | HIGH |
| Core | sync.rs | OLD | 8 | HIGH |
| Core | scale.rs | OLD (magic #) | 6 | HIGH |
| Patterns | working.rs | NEW | 17 | DONE |
| Patterns | explicit.rs | NEW | 18 | DONE |
| Patterns | macros.rs | MIXED | 56 | MEDIUM |
| Patterns | common.rs | OLD | 3 | MEDIUM |
| Synth | common.rs | OLD | 14 | MEDIUM |
| Synth | output.rs | OLD (magic #) | 5 | HIGH |
| Synth | beat_repeat.rs | OLD | 8 | MEDIUM |
| Synth | clouds.rs | MIXED | 3 | MEDIUM |
| Synth | pitch_shift.rs | OLD | 10 | MEDIUM |
| Synth | envelopes/common.rs | OLD | 4 | MEDIUM |
| Synth | plaits/params.rs | OLD (magic #) | 2 | LOW |
| System | metro.rs | NEW | 5 | DONE |
| System | misc.rs | MIXED | 24 | HIGH |
| System | preset.rs | OLD | 8 | HIGH |
| System | scene.rs | OLD | 8 | HIGH |
| Gate | gate.rs | OLD (magic #) | 14 | HIGH |
| Randomization | randomization.rs | OLD | 25 | HIGH |
| Slew | slew.rs | OLD | 4 | MEDIUM |
| Plaits | - | NO OUTPUT | - | DONE |
| Effects | delay/eq/lofi/reverb/ring_mod | NO OUTPUT | - | DONE |
| Envelopes | amp/disc/feedback/filter/fm/pitch | NO OUTPUT | - | DONE |
| Oscillator | - | NO OUTPUT | - | DONE |
| Filter | - | NO OUTPUT | - | DONE |
| Noise | - | NO OUTPUT | - | DONE |

---

## KEY FINDINGS

### Most Urgent Migrations (HIGH Priority)
1. **Randomization** (25 uses) - Core feature set
2. **System/Misc** (24 uses) - Multiple patterns mixed
3. **Gate** (14 uses) - Magic number hardcoding
4. **Synth/Common** (14 uses) - Affects many synth commands
5. **System/Preset & Scene** (16 uses combined) - Configuration critical
6. **Scale** (6 uses) - Magic number hardcoding
7. **Output** (5 uses) - Magic number hardcoding

### Pattern Issues Found
- **Magic Numbers**: scale.rs, gate.rs, output.rs, plaits/params.rs all use "debug_level >= 2"
- **Mixed Usage**: macros.rs, clouds.rs, misc.rs use both patterns
- **Legacy TIER constants**: Many files import but inconsistently apply TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS, TIER_ESSENTIAL

### Inconsistent Output Behavior
- OLD pattern: Conditional output (if debug_level check passes)
- NEW pattern: Always outputs via ctx.output (debug level applied in context)
- NO OUTPUT: 38 files silently execute without feedback

### Context Integration Status
- Variables: COMPLETE - uses ExecutionContext properly
- Patterns (working/explicit): COMPLETE - uses ExecutionContext properly
- Metro: COMPLETE - uses ExecutionContext properly
- All others: INCOMPLETE - use old parameter passing pattern
