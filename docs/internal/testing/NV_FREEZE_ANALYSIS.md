# NV Parameter Freeze - Technical Analysis

**Date:** 2025-12-12
**Status:** INVESTIGATING
**Related:** ENVELOPE_BUG_INVESTIGATION.md

## Problem Statement

After fixing the envelope decay parameter freeze (AD, PD, FD, DD) using `Lag.kr(param, 0)` workaround, the NV (noise volume) parameter is now frozen - noise is always audible regardless of NV setting.

## Control-Rate Complexity Analysis

### Total Control-Rate Operations Count

From `/Users/why/repos/monokit/sc/monokit_server.scd`:

- **Lag.kr operations:** 56 total
  - 30 for main parameter smoothing (lines 166-195)
  - 11 for noise parameter smoothing (lines 199-208)
  - 7 for envelope decay workaround (lines 230-236)
  - 8 for other uses (dmodeSmooth, rmodeSmooth, etc.)

- **Select.kr operations:** 52 total
  - 30 for per-parameter slew time selection
  - 6 for envelope attack/curve resolution
  - 16+ for various mode/routing selections

- **Clip/Round operations:** 89 total
  - Parameter validation throughout

### Added Lag.kr Operations (v0.3.4 → fix)

The envelope fix added **7 new Lag.kr(param, 0)** operations:
```supercollider
adDynamic = Lag.kr(ad, 0);   // Line 230
pdDynamic = Lag.kr(pd, 0);   // Line 231
fdDynamic = Lag.kr(fd, 0);   // Line 232
ddDynamic = Lag.kr(dd, 0);   // Line 233
fbdDynamic = Lag.kr(fbd, 0); // Line 234
fedDynamic = Lag.kr(fed, 0); // Line 235
mbdDynamic = Lag.kr(mbd, 0); // Line 236
```

**Total Lag.kr count increased from 49 → 56**

This may have pushed the SynthDef over another complexity threshold.

## NV Parameter Usage Analysis

### Declaration and Initialization
- **Arg position:** Line 121: `nv = 0` (Noise output level)
- **Var declaration:** Line 155: `nvSmooth` in noise source variables
- **Smoothing:** Line 205: `nvSmooth = Lag.kr(nv / 16383, 0.01);`

### Usage Points
1. **Line 318:** Gain compensation calculation
   ```supercollider
   totalLevel = (pvSmooth + mvSmooth + nvSmooth).max(0.001);
   ```

2. **Line 322:** Source mixing
   ```supercollider
   sig = (primaryOsc * pvSmooth) + (modOsc * mvSmooth) + (noiseShaped * nvSmooth);
   ```

### NV Position in Parameter List

NV is parameter #121 in the arg list (counting from line 18). It's one of the NEWER parameters added in v0.3.4 for noise source integration.

**Noise parameters (all added v0.3.4):**
- nw (line 114) - Noise waveform
- na (line 115) - Noise envelope attack
- nd (line 116) - Noise envelope decay
- nc (line 117) - Noise envelope curve
- ne (line 118) - Noise envelope amount
- np (line 119) - Noise → primary FM
- nm (line 120) - Noise → modulator FM
- **nv (line 121)** - Noise output level ← FROZEN
- ng (line 122) - Noise gate mode
- pv (line 123) - Primary volume
- mv (line 124) - Modulator volume

## Noise Parameter Smoothing Section (Lines 197-209)

This section performs control-rate operations on 11 parameters:

```supercollider
nwVal = nw.clip(0, 2).round;              // clip + round
naSmooth = Lag.kr(na, 0.01);              // Lag.kr
ndSmooth = Lag.kr(nd, 0.01);              // Lag.kr
ncVal = nc.clip(-8, 8);                   // clip ← KNOWN PROBLEMATIC
neSmooth = Lag.kr(ne / 16383, 0.01);      // division + Lag.kr
npSmooth = Lag.kr(np / 16383, 0.01);      // division + Lag.kr
nmSmooth = Lag.kr(nm / 16383, 0.01);      // division + Lag.kr
nvSmooth = Lag.kr(nv / 16383, 0.01);      // division + Lag.kr ← FROZEN
ngVal = ng.clip(0, 1).round;              // clip + round
pvSmooth = Lag.kr(pv / 16383, 0.01);      // division + Lag.kr
mvSmooth = Lag.kr(mv / 16383, 0.01);      // division + Lag.kr
```

**Note:** `ncVal = nc.clip(-8, 8)` was identified in the envelope bug investigation as the line that triggered the original envelope freeze.

## Hypothesis: Pattern Recognition

### Common Pattern in Frozen Parameters

**Original envelope bug:**
- Parameters: AD, PD, FD, DD, FBD, FED, MBD
- All used directly in `Env.perc(attack, DECAY / 1000, ...)`
- Fixed by wrapping in `Lag.kr(param, 0)`

**New NV bug:**
- Parameter: NV (and possibly NP, NM, NE, PV, MV?)
- All use pattern: `Lag.kr(param / 16383, 0.01)`
- All processed after the problematic `ncVal = nc.clip(-8, 8)` line

### Proximity to ncVal Line

The `ncVal = nc.clip(-8, 8)` line (201) appears to create a "contamination zone" where subsequent parameter operations become frozen:

```
197: // Noise parameters smoothing
198: nwVal = nw.clip(0, 2).round;          ← Before ncVal - likely OK
199: naSmooth = Lag.kr(na, 0.01);          ← Before ncVal - likely OK
200: ndSmooth = Lag.kr(nd, 0.01);          ← Before ncVal - likely OK
201: ncVal = nc.clip(-8, 8);               ← CONTAMINATION POINT
202: neSmooth = Lag.kr(ne / 16383, 0.01);  ← After ncVal - possibly frozen?
203: npSmooth = Lag.kr(np / 16383, 0.01);  ← After ncVal - possibly frozen?
204: nmSmooth = Lag.kr(nm / 16383, 0.01);  ← After ncVal - possibly frozen?
205: nvSmooth = Lag.kr(nv / 16383, 0.01);  ← After ncVal - CONFIRMED FROZEN
206: ngVal = ng.clip(0, 1).round;          ← After ncVal - possibly frozen?
207: pvSmooth = Lag.kr(pv / 16383, 0.01);  ← After ncVal - possibly frozen?
208: mvSmooth = Lag.kr(mv / 16383, 0.01);  ← After ncVal - possibly frozen?
```

## SuperCollider Optimizer Bug Theory

### Complexity Thresholds

Evidence suggests SuperCollider's optimizer has multiple complexity thresholds:

1. **First threshold (~195 lines):** Original envelope decay freeze
2. **Second threshold (~230+ lines with 56 Lag.kr ops):** NV parameter freeze

### Optimizer Behavior

When SynthDef complexity crosses a threshold, the optimizer appears to:
1. Incorrectly identify control-rate signals as constants
2. Cache parameter values at compile time
3. Fail to update cached values when parameters change

### Why Lag.kr(param, 0) Works

The `Lag.kr(param, 0)` workaround appears to:
- Force the optimizer to treat the signal as dynamic
- Prevent constant-folding optimization
- Maintain proper parameter update path

Identity operation (lag time = 0) has no audio effect but changes graph topology.

## Questions for Investigation

### 1. Which Parameters Are Frozen?

Test matrix:
- [ ] NV (noise volume) - **CONFIRMED FROZEN**
- [ ] NP (noise → primary FM)
- [ ] NM (noise → modulator FM)
- [ ] NE (noise envelope amount)
- [ ] PV (primary volume)
- [ ] MV (modulator volume)
- [ ] NG (noise gate mode)
- [ ] NW (noise waveform)
- [ ] NA (noise attack)
- [ ] ND (noise decay)

### 2. Is Position/Order Significant?

- Do parameters AFTER ncVal line freeze but BEFORE ncVal work?
- Does moving ncVal to END of section change which params freeze?

### 3. Does Same Fix Work?

Apply `Lag.kr(param, 0)` workaround to noise parameters:
```supercollider
nvDynamic = Lag.kr(nv, 0);
nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
```

### 4. Can We Remove ncVal?

Since nc (noise envelope curve) is rarely changed during performance:
- Could hardcode `-4` as default
- Remove nc parameter entirely
- Test if this eliminates the contamination point

### 5. Alternative: Capture Early

Similar to envelope amount fix (paCtl, faCtl, etc.):
```supercollider
// Capture noise params early with Lag.kr(0) workaround
nvCtl = Lag.kr(nv, 0);
npCtl = Lag.kr(np, 0);
nmCtl = Lag.kr(nm, 0);
neCtl = Lag.kr(ne, 0);

// Use captured values for smoothing
nvSmooth = Lag.kr(nvCtl / 16383, 0.01);
npSmooth = Lag.kr(npCtl / 16383, 0.01);
nmSmooth = Lag.kr(nmCtl / 16383, 0.01);
neSmooth = Lag.kr(neCtl / 16383, 0.01);
```

## Proposed Solutions

### Solution 1: Apply Lag.kr(0) to All Noise Parameters

Most direct fix, following pattern from envelope decay fix:

```supercollider
// Force dynamic evaluation of noise params (workaround for SC optimizer bug)
nvDynamic = Lag.kr(nv, 0);
npDynamic = Lag.kr(np, 0);
nmDynamic = Lag.kr(nm, 0);
neDynamic = Lag.kr(ne, 0);
pvDynamic = Lag.kr(pv, 0);
mvDynamic = Lag.kr(mv, 0);

// Use in smoothing
nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
npSmooth = Lag.kr(npDynamic / 16383, 0.01);
nmSmooth = Lag.kr(nmDynamic / 16383, 0.01);
neSmooth = Lag.kr(neDynamic / 16383, 0.01);
pvSmooth = Lag.kr(pvDynamic / 16383, 0.01);
mvSmooth = Lag.kr(mvDynamic / 16383, 0.01);
```

**Cost:** +6 Lag.kr operations (total 62)
**Risk:** May push us over ANOTHER threshold

### Solution 2: Remove nc Parameter

Simplest reduction, nc is rarely changed:

```supercollider
// Remove from args: nc = -4
// Remove from vars: ncVal
// Remove line: ncVal = nc.clip(-8, 8);
// Hardcode in noiseEnv: Env.perc(naSmooth / 1000, ndSmooth / 1000, 1, -4)
```

**Cost:** -1 parameter, -1 clip operation
**Risk:** Low, already tested this approach

### Solution 3: Aggressive Refactoring

Reduce overall complexity:
- Split into multiple SynthDefs
- Use control buses for parameter routing
- Reduce total control-rate operations
- Consider removing rarely-used features

**Cost:** Major architectural changes
**Risk:** High, breaks compatibility

### Solution 4: Systematic Early Capture

Apply Lag.kr(0) to ALL parameters at the start:

```supercollider
// Capture ALL control inputs early (workaround for SC optimizer bug)
// This creates a dynamic evaluation layer that prevents constant-folding
pfDynamic = Lag.kr(pf, 0);
pwDynamic = Lag.kr(pw, 0);
// ... etc for ALL 120+ parameters
```

**Cost:** +120 Lag.kr operations
**Risk:** Very high, likely to cause worse problems

## Recommended Approach

### Phase 1: Minimal Testing (15 min)

1. Create test SynthDef with ONLY noise parameters
2. Verify which specific parameters are frozen (NV, NP, NM, NE, PV, MV)
3. Test if removing `ncVal = nc.clip(-8, 8)` alone fixes the issue

### Phase 2: Targeted Fix (30 min)

Based on test results, apply Solution 1 or 2:
- If only NV frozen: Apply Lag.kr(0) to just NV
- If multiple frozen: Apply Lag.kr(0) to all affected params
- If removal fixes: Remove nc parameter entirely

### Phase 3: Verification (15 min)

1. Recompile SynthDefs
2. Bundle into app
3. Test all affected parameters respond
4. Verify no NEW parameters frozen

### Phase 4: Documentation (15 min)

1. Update ENVELOPE_BUG_INVESTIGATION.md with findings
2. Document total Lag.kr count and threshold observations
3. Note which parameters required the workaround
4. Add comments in source code explaining the workaround

## Files to Modify

Primary:
- `/Users/why/repos/monokit/sc/monokit_server.scd` - Runtime SynthDef
- `/Users/why/repos/monokit/build_scripts/compile_synthdefs.scd` - Compilation source

Testing:
- `/tmp/nv_test.scd` - Isolated test case

Documentation:
- `/Users/why/repos/monokit/docs/ENVELOPE_BUG_INVESTIGATION.md` - Update with NV findings
- `/Users/why/repos/monokit/docs/NV_FREEZE_ANALYSIS.md` - This file

## Related Issues

- Original envelope bug (AD, PD, FD, DD freeze)
- SuperCollider 3.14.1 optimizer behavior
- SynthDef complexity limits

## Environment

- SuperCollider 3.14.1
- macOS Darwin 24.5.0
- monokit v0.3.4+ (post-envelope-fix)
