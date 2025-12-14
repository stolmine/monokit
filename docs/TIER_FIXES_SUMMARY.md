# Debug Tier Fixes Summary

## Overview

Fixed tier violations and missing tier checks that were causing unwanted output at tier 0 (TIER_SILENT). All commands now properly respect debug level settings.

## Changes Made

### 1. SLEW / SLEW.ALL - Fixed Tier Violation ✓
**File**: `src/commands/slew.rs`

**Problem**: Used `debug_level >= 1` (TIER_ERRORS) for parameter confirmations
**Fix**: Changed to `debug_level >= TIER_CONFIRMS || out_cfm` (tier 4)
**Impact**: SLEW confirmations now only appear at tier 4 instead of flooding tier 1

**Changes**:
- Added `TIER_CONFIRMS` import
- Added `out_cfm: bool` parameter to both functions
- Changed condition from `>= 1` to `>= TIER_CONFIRMS || out_cfm`
- Updated call sites in `src/commands/mod.rs` to pass `out_cfm`

---

### 2. PRINT - Fixed Tier Violation ✓
**File**: `src/commands/system/misc.rs`

**Problem**: Used `debug_level >= TIER_ERRORS || out_ess` (tier 1) for user output
**Fix**: Changed to `debug_level >= TIER_ESSENTIAL || out_ess` (tier 2)
**Impact**: PRINT output now appears at tier 2 (essential) instead of tier 1 (errors)

**Changes**:
- Added `TIER_ESSENTIAL` import
- Changed two output conditions from `>= TIER_ERRORS` to `>= TIER_ESSENTIAL`

---

### 3. Scene Commands - Added Missing Tier Gates ✓
**File**: `src/commands/system/scene.rs`

**Problem**: Called `output()` directly without any tier checks
**Fix**: Added appropriate tier gating to all scene commands
**Impact**: Scene operations now respect debug level settings

**Changes**:
- **SAVE**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around confirmation
- **LOAD**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around confirmation
- **SCENES**: Wrapped entire function body in `if debug_level >= TIER_QUERIES || out_qry`
- **DELETE**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around confirmation

---

### 4. Preset Commands - Added Missing Tier Gates ✓
**File**: `src/commands/system/preset.rs`

**Problem**: Called `output()` directly without any tier checks
**Fix**: Added appropriate tier gating to all preset commands
**Impact**: Preset operations now respect debug level settings

**Changes**:
- **PSET**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around confirmation
- **PSET.SAVE**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around confirmation
- **PSET.DEL**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around confirmation
- **PSETS**: Wrapped entire function body in `if debug_level >= TIER_QUERIES || out_qry`

---

### 5. Recording Commands - Added Missing Tier Gates ✓
**File**: `src/commands/system/misc.rs`

**Problem**: Called `output()` directly without any tier checks
**Fix**: Added appropriate tier gating to all recording commands
**Impact**: Recording operations now respect debug level settings

**Changes**:
- **REC**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around "RECORDING STARTED"
- **REC.STOP**: Added `if debug_level >= TIER_ESSENTIAL || out_ess` around "RECORDING STOPPED"
- **REC.PATH**: Added `if debug_level >= TIER_CONFIRMS || out_cfm` around path confirmation

---

## Tier Assignment Summary

### Commands Now at TIER_ESSENTIAL (2)
- **PRINT** - User output/expression evaluation
- **SAVE**, **LOAD**, **DELETE** - Scene management confirmations
- **PSET**, **PSET.SAVE**, **PSET.DEL** - Preset management confirmations
- **REC**, **REC.STOP** - Recording control notifications

### Commands Now at TIER_QUERIES (3)
- **SCENES** - List all scenes
- **PSETS** - List all presets

### Commands Now at TIER_CONFIRMS (4)
- **SLEW**, **SLEW.ALL** - Parameter slew time confirmations
- **REC.PATH** - Recording path setting confirmation

---

## Testing

Build completed successfully:
```bash
cargo build
# Result: Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
# 158 warnings (unused imports), no errors
```

---

## Before vs After Behavior

### Tier 0 (TIER_SILENT) - DEBUG 0
**Before**: SLEW, PRINT, scene/preset/recording commands all produced output
**After**: Only config commands and errors produce output

### Tier 1 (TIER_ERRORS) - DEBUG 1
**Before**: SLEW and PRINT flooded output at tier 1
**After**: Only actual error messages appear at tier 1

### Tier 2 (TIER_ESSENTIAL) - DEBUG 2
**Before**: Scene/preset/recording confirmations always appeared
**After**: Confirmations respect tier 2 threshold

### Tier 3 (TIER_QUERIES) - DEBUG 3
**Before**: List commands (SCENES, PSETS) always appeared
**After**: List commands respect tier 3 threshold

### Tier 4 (TIER_CONFIRMS) - DEBUG 4
**Before**: SLEW appeared at tier 1, REC.PATH had no gating
**After**: All parameter confirmations properly gated at tier 4

---

## Files Modified

1. `src/commands/slew.rs` - Fixed SLEW/SLEW.ALL tier violations
2. `src/commands/system/misc.rs` - Fixed PRINT tier + added recording tier gates
3. `src/commands/system/scene.rs` - Added all scene command tier gates
4. `src/commands/system/preset.rs` - Added all preset command tier gates
5. `src/commands/mod.rs` - Updated SLEW call sites with `out_cfm` parameter

---

## Related Documentation

- `docs/DEBUG_TIERS.md` - Complete tier classification of all commands
- `docs/TIER_0_COMMANDS.md` - Analysis of true tier 0 (silent) commands
- `src/types.rs:17-21` - Tier constant definitions
- `src/commands/context.rs:79-110` - ExecutionContext tier filtering logic

---

## Notes

### Commands Still Without Tier Gating
The following command types still output unconditionally (by design):
1. **Config toggle commands** - Use `define_bool_toggle` and `define_enum_select` macros that always output
2. **Error messages** - All commands output errors unconditionally (correct behavior)

### Future Improvements
If config commands should also respect tier 0, the `define_bool_toggle` and `define_enum_select` macros would need to be refactored to use `ExecutionContext.should_output()` or manual tier checks.
