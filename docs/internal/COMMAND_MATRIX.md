# MONOKIT COMMAND INVENTORY - QUICK REFERENCE MATRIX

## ADOPTION STATUS BY FILE

| File | Pattern | Count | Status | Priority | Notes |
|------|---------|-------|--------|----------|-------|
| **CORE COMMANDS** |
| variables.rs | NEW | 8 | DONE | - | Full ctx.output |
| counters.rs | OLD | 9 | LEGACY | HIGH | Nested macros, TIER constants |
| scheduling.rs | OLD | 9 | LEGACY | HIGH | del/del_x/del_r/del_clr handlers |
| sync.rs | OLD | 8 | LEGACY | HIGH | Simple TIER_CONFIRMS pattern |
| scale.rs | OLD | 6 | LEGACY | HIGH | Magic number (>= 2) |
| **PATTERN COMMANDS** |
| working.rs | NEW | 17 | DONE | - | Full ctx.output |
| explicit.rs | NEW | 18 | DONE | - | Full ctx.output |
| macros.rs | MIXED | 56 | PARTIAL | MEDIUM | 46 ctx.output, 10 debug_level |
| common.rs | OLD | 3 | LEGACY | MEDIUM | Helper function |
| **SYSTEM COMMANDS** |
| metro.rs | NEW | 5 | DONE | - | handle_tr, handle_pltr |
| misc.rs | MIXED | 24 | HEAVY LEGACY | HIGH | Macros + some ctx.output |
| preset.rs | OLD | 8 | LEGACY | HIGH | TIER_CONFIRMS pattern |
| scene.rs | OLD | 8 | LEGACY | HIGH | TIER_CONFIRMS pattern |
| audio.rs | NONE | 0 | DONE | - | No output |
| midi.rs | NONE | 0 | DONE | - | No output |
| sc.rs | NONE | 0 | DONE | - | No output |
| **SYNTH - COMMON** |
| synth/common.rs | OLD | 14 | LEGACY | MEDIUM | define_int_param macros |
| **SYNTH - EFFECTS** |
| effects/beat_repeat.rs | OLD | 8 | LEGACY | MEDIUM | TIER_CONFIRMS |
| effects/clouds.rs | MIXED | 3 | PARTIAL | MEDIUM | 1 ctx.output, 2 debug_level |
| effects/pitch_shift.rs | OLD | 10 | LEGACY | MEDIUM | Pitch effect parameters |
| effects/compressor.rs | NONE | 0 | DONE | - | No output |
| effects/delay.rs | NONE | 0 | DONE | - | No output |
| effects/eq.rs | NONE | 0 | DONE | - | No output |
| effects/lofi.rs | NONE | 0 | DONE | - | No output |
| effects/reverb.rs | NONE | 0 | DONE | - | No output |
| effects/ring_mod.rs | NONE | 0 | DONE | - | No output |
| **SYNTH - ENVELOPES** |
| envelopes/common.rs | OLD | 4 | LEGACY | MEDIUM | Envelope parameters |
| envelopes/amp.rs | NONE | 0 | DONE | - | No output |
| envelopes/disc.rs | NONE | 0 | DONE | - | No output |
| envelopes/feedback.rs | NONE | 0 | DONE | - | No output |
| envelopes/filter.rs | NONE | 0 | DONE | - | No output |
| envelopes/fm.rs | NONE | 0 | DONE | - | No output |
| envelopes/pitch.rs | NONE | 0 | DONE | - | No output |
| **SYNTH - OTHER** |
| oscillator.rs | NONE | 0 | DONE | - | No output |
| filter.rs | NONE | 0 | DONE | - | No output |
| noise.rs | NONE | 0 | DONE | - | No output |
| modulation.rs | NONE | 0 | DONE | - | No output |
| resonator.rs | NONE | 0 | DONE | - | No output |
| source_levels.rs | NONE | 0 | DONE | - | No output |
| discontinuity.rs | NONE | 0 | DONE | - | No output |
| plaits/params.rs | OLD | 2 | LEGACY | LOW | Magic number (>= 2) |
| plaits/engine.rs | NONE | 0 | DONE | - | No output |
| output.rs | OLD | 5 | LEGACY | HIGH | Magic number (>= 2) |
| **GATE & RANDOMIZATION** |
| gate.rs | OLD | 14 | LEGACY | HIGH | Magic number (>= 2) all handlers |
| randomization.rs | OLD | 25 | LEGACY | HIGH | Multiple TIER patterns |
| slew.rs | OLD | 4 | LEGACY | MEDIUM | TIER pattern |
| **UTILITIES** |
| context.rs | - | - | - | - | ExecutionContext definition |
| common.rs | - | - | - | - | General utilities |
| validate.rs | NONE | 0 | DONE | - | No output |
| validate_expr.rs | NONE | 0 | DONE | - | No output |
| aliases.rs | NONE | 0 | DONE | - | No output |
| **DISPATCHER** |
| mod.rs | OLD | 189 | LEGACY | - | Parameter extraction & routing |

---

## STATISTICS

### By Pattern
- NEW (ctx.output): 4 files, 118 total uses, 100% modern
- OLD (debug_level >= TIER_): 21 files, 198 total uses, legacy
- MIXED: 1 file (macros.rs), 56 uses, transitional
- MAGIC NUMBERS (>= N): 4 files, 27 total uses, problematic
- NO OUTPUT: 38 files, fully silent

### By Completion Status
- FULLY MIGRATED (NEW): 6 files (variables, working, explicit, metro)
- PARTIALLY MIGRATED (MIXED): 3 files (macros, clouds, misc)
- LEGACY (OLD): 21 files
- SILENT (NO OUTPUT): 38 files
- INFRASTRUCTURE: 5 files

### By Priority
- HIGH: 14 files, 172 uses
  - randomization (25), gate (14), misc (24), counters (9), scheduling (9), sync (8), preset (8), scene (8), scale (6), output (5)
- MEDIUM: 9 files, 98 uses
  - macros (56), synth/common (14), pitch_shift (10), beat_repeat (8), patterns/common (3), envelopes/common (4), clouds (3)
- LOW: 1 file, 2 uses
  - plaits/params
- DONE: 44 files

### Total Command Uses
- With Output: 316 uses
- Without Output: 0 uses in 38 files
- Migration Target: 223 uses (21 OLD + 3 MIXED - fully migrated parts)

---

## COMMAND HANDLER FUNCTION COUNT

| File | Handlers | Macros | Total |
|------|----------|--------|-------|
| variables.rs | 10 | 0 | 10 |
| counters.rs | 4 | 4 | 8 |
| scheduling.rs | 4 | 0 | 4 |
| sync.rs | 4 | 0 | 4 |
| scale.rs | 3 | 0 | 3 |
| working.rs | Many | 0 | - |
| explicit.rs | Many | 0 | - |
| macros.rs | 0 | 3 | 3 |
| common.rs (patterns) | Many | 0 | - |
| metro.rs | 2 | 0 | 2 |
| misc.rs | Many | 2 | - |
| preset.rs | Many | 0 | - |
| scene.rs | Many | 0 | - |
| synth/common.rs | 0 | 3 | 3 |
| beat_repeat.rs | 2 | 0 | 2 |
| clouds.rs | 9 | 0 | 9 |
| pitch_shift.rs | Many | 0 | - |
| envelopes/common.rs | 0 | 3 | 3 |
| output.rs | 3 | 0 | 3 |
| plaits/params.rs | 2 | 0 | 2 |
| gate.rs | 14 | 0 | 14 |
| randomization.rs | 10+ | 0 | 10+ |
| slew.rs | 3 | 0 | 3 |

---

## MAGIC NUMBER OCCURRENCES

Files using hardcoded debug_level checks (>= 2):
```
src/commands/core/scale.rs
  - Line 92: handle_q_root
  - Line 139: handle_q_scale  
  - Line 170: handle_q_bit
  Total: 3 uses

src/commands/gate.rs
  - handle_gate (line 40)
  - handle_aenv_gate (line 80)
  - handle_penv_gate (similar)
  - handle_fenv_gate (similar)
  - handle_disc_gate (similar)
  - handle_disc_aenv_gate (similar)
  - handle_disc_penv_gate (similar)
  Total: ~8-10 uses (multiple similar handlers)

src/commands/synth/output.rs
  - handle_vol (line 31)
  - handle_pan (line 70)
  - handle_vca (line 31)
  Total: 3 uses (some use TIER_QUERIES too)

src/commands/synth/plaits/params.rs
  - 2 uses of hardcoded checks
```

Should be replaced with:
```rust
const TIER_FEEDBACK = 2;  // or use existing TIER_CONFIRMS
```

---

## OUTPUT CATEGORY USAGE

From NEW pattern implementations (ctx.output):

| Category | Count | Files |
|----------|-------|-------|
| Error | ~40 | variables, patterns, metro, macros, clouds |
| Query | ~20 | variables, patterns, output |
| Confirm | ~50 | variables, patterns, metro, macros, clouds |
| (implicit Essential) | ~8 | Various |

---

## TIER CONSTANT USAGE IN LEGACY CODE

| Constant | Used In | Count |
|----------|---------|-------|
| TIER_ERRORS | counters, randomization, patterns/common | 3 files |
| TIER_QUERIES | counters, randomization, output | 3 files |
| TIER_CONFIRMS | sync, counters, scheduling, beat_repeat, synth/common, randomization, preset, scene, misc | 9 files |
| TIER_ESSENTIAL | scheduling, randomization, synth/common, misc | 4 files |

Most common: TIER_CONFIRMS (used in 9 files for >= comparison)

---

## DEPENDENCIES GRAPH

### Files requiring ctx updates:
1. patterns/macros.rs depends on patterns/common.rs
2. patterns/common.rs (parse_pattern_num) uses debug_level checks
3. synth/effects/clouds.rs has internal macros using debug_level
4. system/misc.rs has macro definitions using debug_level

### Files that call deprecated handlers:
- mod.rs (dispatcher) - calls all handlers with old signatures
- Would need updates to dispatcher once handlers migrated

---

## RECOMMENDED MIGRATION ORDER

1. **Standalone Simple Files** (can migrate independently):
   - sync.rs (8 uses) - only uses TIER_CONFIRMS
   - scale.rs (6 uses) - replace magic numbers
   - plaits/params.rs (2 uses) - replace magic numbers
   - slew.rs (4 uses) - simple tier checks

2. **Coupled Pairs** (migrate together):
   - counters.rs + patterns/common.rs (9+3=12 uses)
   - gate.rs standalone (14 uses with magic numbers)
   - output.rs standalone (5 uses with magic numbers)

3. **Large Refactors** (complex changes needed):
   - synth/common.rs (14 uses in 3 macros)
   - synth/effects/* (8+10+4=22 uses spread across handlers)
   - randomization.rs (25 uses, all handlers)
   - preset.rs + scene.rs (16 uses together)

4. **Complex Macros** (most challenging):
   - patterns/macros.rs (56 uses in 3 main macros)
   - system/misc.rs (24 uses in 2 macros + some functions)
   - clouds.rs (mixed pattern transition needed)

