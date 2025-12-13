# SuperCollider Optimizer Bug - Complete Summary

**Version:** monokit v0.3.4+
**SC Version:** SuperCollider 3.14.1
**Date:** 2025-12-12
**Status:** ONGOING INVESTIGATION

## Overview

We have encountered a SuperCollider 3.14.1 optimizer bug that causes control-rate parameters to be incorrectly treated as constants when SynthDef complexity crosses certain thresholds. The bug has manifested in two distinct phases:

### Phase 1: Envelope Decay Parameters (FIXED)
- **Affected:** AD, PD, FD, DD, FBD, FED, MBD
- **Symptom:** Envelope decay times frozen, don't respond to parameter changes
- **Fix:** Wrap parameters in `Lag.kr(param, 0)` before use
- **Status:** ✅ RESOLVED

### Phase 2: Noise Volume Parameters (CURRENT)
- **Affected:** NV (confirmed), possibly NP, NM, NE, PV, MV
- **Symptom:** Noise always audible regardless of NV setting
- **Cause:** Adding 7 Lag.kr(0) operations for Phase 1 fix pushed SynthDef over ANOTHER threshold
- **Status:** ⚠️ IN PROGRESS

## Technical Analysis

### Control-Rate Complexity Metrics

Current state of `/Users/why/repos/monokit/sc/monokit_server.scd`:

| Metric | Count | Notes |
|--------|-------|-------|
| Total parameters | ~124 | Main arg list |
| Lag.kr operations | 56 | Was 49 before envelope fix |
| Select.kr operations | 52 | For conditionals/routing |
| .clip() operations | ~60 | Parameter validation |
| .round() operations | ~15 | Integer parameters |
| EnvGen.kr generators | 7 | All envelopes |
| Total SynthDef lines | ~650 | Entire monokit synth |

### Complexity Thresholds Observed

Evidence suggests multiple thresholds exist:

**Threshold 1: ~49 Lag.kr operations**
- Crossed in v0.3.4 (noise source integration)
- Resulted in envelope decay parameter freeze
- Trigger line: `ncVal = nc.clip(-8, 8)` (line 201)
- Parameters affected: AD, PD, FD, DD, FBD, FED, MBD

**Threshold 2: ~56 Lag.kr operations**
- Crossed when applying Phase 1 fix
- Resulted in noise volume parameter freeze
- Same trigger line: `ncVal = nc.clip(-8, 8)` (line 201)
- Parameters affected: NV (confirmed), possibly NP, NM, NE, PV, MV

### Pattern Recognition

#### Common Symptoms
1. Parameters stop responding to OSC /param messages
2. Parameter appears "frozen" at initialization value
3. Audio output suggests parameter has a constant value
4. Trigger events (t_gate) still work correctly
5. Some parameters work, others don't - suggesting selective optimization

#### Contamination Zone Pattern

Both bugs show a "contamination zone" where parameters processed AFTER a specific line become frozen:

```supercollider
// Lines BEFORE this point: mostly OK
ncVal = nc.clip(-8, 8);  // ← CONTAMINATION POINT (line 201)
// Lines AFTER this point: parameters may freeze
```

Parameters that use values AFTER this line are subject to incorrect optimization.

#### Workaround Pattern

Wrapping parameters in `Lag.kr(param, 0)` forces dynamic evaluation:

```supercollider
// Instead of:
ampEnv = EnvGen.kr(Env.perc(aAtk, ad / 1000, 1, aCrv), trig);

// Use:
adDynamic = Lag.kr(ad, 0);  // Force dynamic evaluation
ampEnv = EnvGen.kr(Env.perc(aAtk, adDynamic / 1000, 1, aCrv), trig);
```

The `Lag.kr(param, 0)` is an identity operation (lag time = 0) with no audio effect, but it changes the UGen graph topology in a way that prevents the optimizer from constant-folding.

## Root Cause Theory

### Hypothesis: Constant Folding Optimizer Bug

SuperCollider's scsynth optimizer performs constant folding to optimize SynthDefs. When complexity crosses a threshold, the optimizer appears to:

1. **Incorrectly identify dynamic signals as constants**
   - Control-rate parameters become "frozen" at compile time
   - Parameter update path is optimized away

2. **Apply threshold-based heuristics**
   - Different optimization strategies at different complexity levels
   - Thresholds may be based on: total UGens, control-rate ops, graph depth

3. **Fail after specific graph patterns**
   - `nc.clip(-8, 8)` line acts as trigger
   - Subsequent operations become candidates for aggressive optimization

4. **Maintain trigger responsiveness**
   - TrigControl (t_gate) still works
   - Suggests optimization only affects scalar controls, not triggers

### Why Lag.kr(0) Works

The workaround exploits graph topology:

```
Before (optimized incorrectly):
  param → [CONSTANT FOLD] → usage

After (forces dynamic):
  param → Lag.kr(0) → [NO CONSTANT FOLD] → usage
```

Inserting `Lag.kr(0)` creates an explicit UGen node that:
- Cannot be constant-folded (it's a filter UGen)
- Forces param to be treated as time-varying
- Has zero audio impact (lag time = 0)
- Changes graph structure enough to prevent optimization

## Affected Parameters

### Phase 1: Envelope Decays (FIXED)
- ✅ AD - Amplitude envelope decay
- ✅ PD - Pitch envelope decay
- ✅ FD - FM envelope decay
- ✅ DD - Discontinuity envelope decay
- ✅ FBD - Feedback envelope decay
- ✅ FED - Filter envelope decay
- ✅ MBD - ModBus envelope decay

### Phase 2: Noise Parameters (TESTING NEEDED)
- ❌ NV - Noise output level (CONFIRMED FROZEN)
- ❓ NP - Noise → primary FM amount
- ❓ NM - Noise → modulator FM amount
- ❓ NE - Noise envelope amount
- ❓ PV - Primary oscillator volume
- ❓ MV - Modulator oscillator volume
- ❓ NG - Noise gate mode
- ✅ NW - Noise waveform (likely OK - processed before ncVal)
- ✅ NA - Noise attack (likely OK - processed before ncVal)
- ✅ ND - Noise decay (likely OK - processed before ncVal)

## Testing Protocol

### 1. Verify Affected Parameters

Use `/tmp/nv_param_test.scd` to test three scenarios:

**Test 1: With ncVal line** (reproduces bug)
```supercollider
~voice = Synth(\noise_param_test);
~voice.set(\t_gate, 1);  // Trigger
~voice.set(\pv, 0);      // Turn off primary osc
~voice.set(\nv, 8192);   // Set noise to 50%
~voice.set(\nv, 16383);  // Set noise to 100%
~voice.set(\nv, 0);      // Set noise to 0%
```

**Expected:** NV frozen (noise always audible) ← BUG

**Test 2: Without ncVal line** (tests removal)
```supercollider
~voice = Synth(\noise_no_nc);
// Same test sequence
```

**Expected:** NV works correctly

**Test 3: With Lag.kr(0) workaround**
```supercollider
~voice = Synth(\noise_lag_fix);
// Same test sequence
```

**Expected:** NV works correctly

### 2. Test All Suspect Parameters

For each parameter in Phase 2 list:
1. Set primary/modulator to 0 to isolate noise
2. Set noise to audible level
3. Trigger envelope
4. Change parameter value
5. Trigger again
6. Listen for change

### 3. Document Results

Create test matrix:

| Parameter | Frozen? | Position vs ncVal | Notes |
|-----------|---------|-------------------|-------|
| NV | YES | After | Confirmed |
| NP | ? | After | |
| NM | ? | After | |
| NE | ? | After | |
| PV | ? | After | |
| MV | ? | After | |
| NG | ? | After | |
| NW | ? | Before | Likely OK |
| NA | ? | Before | Likely OK |
| ND | ? | Before | Likely OK |

## Proposed Solutions

### Option 1: Apply Lag.kr(0) to Affected Parameters (RECOMMENDED)

**Pros:**
- Minimal code change
- Proven to work for envelope decays
- Surgical fix targeting specific parameters

**Cons:**
- Adds 6+ more Lag.kr operations (total ~62)
- Risk of triggering ANOTHER threshold
- Doesn't address root cause

**Implementation:**
```supercollider
// Force dynamic evaluation of noise params (workaround for SC optimizer bug)
nvDynamic = Lag.kr(nv, 0);
npDynamic = Lag.kr(np, 0);
nmDynamic = Lag.kr(nm, 0);
neDynamic = Lag.kr(ne, 0);
pvDynamic = Lag.kr(pv, 0);
mvDynamic = Lag.kr(mv, 0);

// Use dynamic versions in smoothing
nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
npSmooth = Lag.kr(npDynamic / 16383, 0.01);
nmSmooth = Lag.kr(nmDynamic / 16383, 0.01);
neSmooth = Lag.kr(neDynamic / 16383, 0.01);
pvSmooth = Lag.kr(pvDynamic / 16383, 0.01);
mvSmooth = Lag.kr(mvDynamic / 16383, 0.01);
```

**Files to modify:**
- `/Users/why/repos/monokit/sc/monokit_server.scd`
- `/Users/why/repos/monokit/build_scripts/compile_synthdefs.scd`

### Option 2: Remove nc Parameter

**Pros:**
- Eliminates contamination trigger line
- Reduces complexity by 1 parameter + 1 clip operation
- nc (noise curve) rarely changed during performance

**Cons:**
- Loses user control over noise envelope curve
- May not fully resolve issue if complexity is cumulative
- Already tested - DIDN'T work for Phase 1

**Implementation:**
```supercollider
// Remove from args
// nc = -4,  ← DELETE

// Remove from vars
// var ..., ncVal, ...  ← DELETE ncVal

// Remove line
// ncVal = nc.clip(-8, 8);  ← DELETE

// Hardcode in noiseEnv
Env.perc(naSmooth / 1000, ndSmooth / 1000, 1, -4)  // ← Use -4 directly
```

**Risk:** Did not fix envelope bug when tried, may not work here either.

### Option 3: Aggressive Refactoring

**Pros:**
- Addresses root cause (complexity)
- Future-proof against more thresholds
- Could improve performance

**Cons:**
- Major architectural changes required
- Breaks compatibility
- High development/testing cost
- May introduce new bugs

**Strategies:**
1. **Split into multiple SynthDefs**
   - Separate voice, effects, modulation into distinct synths
   - Use audio/control buses for routing
   - Reduces per-SynthDef complexity

2. **Use control buses for parameters**
   - Move parameter smoothing to separate control synth
   - Voice reads from control buses
   - Separates control-rate complexity

3. **Reduce rarely-used features**
   - Identify low-value parameters
   - Remove or simplify feature set
   - Focus on core functionality

4. **Upgrade SuperCollider**
   - Test on SC 3.13.x or SC 3.15.x (if available)
   - File bug report with SC developers
   - Wait for upstream fix

### Option 4: Hybrid Approach

**Pros:**
- Balances immediate fix with long-term solution
- Iterative risk management

**Cons:**
- More work
- Requires multiple testing cycles

**Implementation:**
1. **Phase A:** Apply Lag.kr(0) workaround (immediate)
   - Fix NV and other frozen parameters
   - Get system working again

2. **Phase B:** Remove nc parameter (reduction)
   - Test if eliminating trigger line helps
   - Document impact on complexity

3. **Phase C:** Monitor for new issues (validation)
   - Watch for additional parameters freezing
   - Document exact thresholds

4. **Phase D:** Plan refactoring (long-term)
   - Design multi-SynthDef architecture
   - Implement in v0.4.x or v0.5.x

## Recommended Action Plan

### Immediate (Today)

1. **Run test suite** (`/tmp/nv_param_test.scd`)
   - Confirm NV freeze
   - Test if removal of ncVal fixes
   - Test if Lag.kr(0) workaround fixes

2. **Test other suspect parameters**
   - NP, NM, NE, PV, MV
   - Document which are frozen

3. **Apply targeted fix**
   - If only NV frozen: Apply Lag.kr(0) to just NV
   - If multiple frozen: Apply Lag.kr(0) to all affected

4. **Recompile and verify**
   - Rebuild SynthDefs
   - Bundle into app
   - Test all parameters respond

### Short-term (This Week)

5. **Document findings**
   - Update investigation docs
   - Add comments to source code
   - Create workaround reference

6. **Consider nc removal**
   - Evaluate user impact
   - Test if removing nc helps overall
   - Make decision on keeping vs removing

7. **Monitor for new issues**
   - Watch for other parameters freezing
   - Document total Lag.kr count
   - Note any new thresholds

### Long-term (v0.4.x)

8. **Investigate SC versions**
   - Test on SC 3.13.x
   - Check SC 3.15.x roadmap
   - File upstream bug report with minimal reproduction

9. **Plan architectural changes**
   - Design multi-SynthDef structure
   - Prototype control bus routing
   - Evaluate performance impact

10. **Consider feature reduction**
    - Audit parameter usage
    - Identify rarely-used features
    - Prioritize core functionality

## Communication with SuperCollider Community

### Minimal Reproduction Case

To report upstream, create minimal SynthDef that reproduces:

```supercollider
SynthDef(\optimizer_bug, {
    arg ad = 100, nc = -4, nv = 0;
    var adSmooth, ncVal, nvSmooth;
    var trig, env, sig;

    // Many Lag.kr operations to reach threshold
    // ... [add ~50 Lag.kr calls]

    ncVal = nc.clip(-8, 8);  // ← Trigger line
    nvSmooth = Lag.kr(nv / 16383, 0.01);  // ← This freezes

    trig = Trig1.kr(t_gate, 0.001);
    env = EnvGen.kr(Env.perc(0.001, ad / 1000, 1, -4), trig);
    sig = WhiteNoise.ar() * nvSmooth * env;

    Out.ar(0, sig ! 2);
}).add;
```

### Information to Provide

- SuperCollider version: 3.14.1
- Platform: macOS Darwin 24.5.0
- SynthDef complexity: ~56 Lag.kr, ~52 Select.kr, ~124 parameters
- Symptom: Control-rate parameters treated as constants
- Workaround: Lag.kr(param, 0) forces dynamic evaluation
- Reproducibility: Consistent at complexity thresholds

## Files Reference

### Source Files
- `/Users/why/repos/monokit/sc/monokit_server.scd` - Runtime SynthDef
- `/Users/why/repos/monokit/build_scripts/compile_synthdefs.scd` - Compilation source

### Documentation
- `/Users/why/repos/monokit/docs/ENVELOPE_BUG_INVESTIGATION.md` - Phase 1 investigation
- `/Users/why/repos/monokit/docs/NV_FREEZE_ANALYSIS.md` - Phase 2 detailed analysis
- `/Users/why/repos/monokit/docs/SC_OPTIMIZER_BUG_SUMMARY.md` - This file

### Test Scripts
- `/tmp/nv_param_test.scd` - NV parameter freeze test suite

### Binary Artifacts
- `/Users/why/repos/monokit/sc/synthdefs/monokit.scsyndef` - Compiled SynthDef

## Conclusion

This is a complex optimizer bug in SuperCollider 3.14.1 that manifests when SynthDef complexity crosses certain thresholds. We have a working workaround (`Lag.kr(param, 0)`), but each application of the workaround increases complexity and risk of hitting new thresholds.

**Current Status:**
- Phase 1 (envelope decays): ✅ FIXED
- Phase 2 (noise volume): ⚠️ Testing fix, watching for cascading issues

**Risk Level:** 🔴 HIGH - Each fix may trigger new bugs

**Long-term Solution:** Architectural refactoring to reduce SynthDef complexity

---

*Last updated: 2025-12-12*
