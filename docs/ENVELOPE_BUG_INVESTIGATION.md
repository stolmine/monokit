# Envelope Bug Investigation

**Date:** 2025-12-11
**Status:** In Progress
**Bug:** Envelope decay parameters (AD, PD, FD, DD) don't respond to changes in v0.3.4

## Summary

After v0.3.4 release, all envelope decay parameters (AD, PD, FD, DD) stopped responding to changes. The filter envelope decay (FED) works correctly. VCA 0 (drone mode) sounds normal, VCA 1 (gated mode) has the issue.

## Key Findings

### Confirmed Facts

1. **OSC messages ARE being sent correctly** - Debug logging in metro.rs confirmed `[DEBUG] SendParam: ad = Int(5000)` is transmitted
2. **The issue is NOT in Rust code** - OSC communication works (triggers work, filter env works, VCA switching works)
3. **The issue is in the SynthDef itself** - A simple test SynthDef with the same envelope pattern works correctly
4. **The compiled .scsyndef file exhibits the bug** - Loading monokit.scsyndef and testing shows broken envelopes
5. **Using .add() with full monokit body ALSO breaks** - Not a compilation issue
6. **The bug appears somewhere between lines 181-195** in the SynthDef body

### Binary Search Results

We systematically tested adding code from monokit_server.scd:

| Lines | Content | Result |
|-------|---------|--------|
| 1-151 | Args + var declarations + partial smoothing | WORKS |
| 1-181 | + All slew-based Lag.kr smoothings | WORKS |
| 1-195 | + Noise parameter smoothing | BREAKS |
| 1-218 | + Envelope attack/curve resolution | BREAKS |

### Narrowing Down Lines 182-195

```supercollider
// Lines 182-195 - Noise parameters smoothing
nwVal = nw.clip(0, 2).round;           // WORKS
naSmooth = Lag.kr(na, 0.01);           // WORKS
ndSmooth = Lag.kr(nd, 0.01);           // WORKS
ncVal = nc.clip(-8, 8);                // <<<< BREAKS ENVELOPES
neSmooth = Lag.kr(ne / 16383, 0.01);   // WORKS (when ncVal removed)
npSmooth = Lag.kr(np / 16383, 0.01);   // WORKS (when ncVal removed)
nmSmooth = Lag.kr(nm / 16383, 0.01);   // (not tested individually)
nvSmooth = Lag.kr(nv / 16383, 0.01);   // (not tested individually)
ngVal = ng.clip(0, 1).round;           // (not tested individually)
pvSmooth = Lag.kr(pv / 16383, 0.01);   // WORKS
mvSmooth = Lag.kr(mv / 16383, 0.01);   // WORKS
```

### The Culprit Line

```supercollider
ncVal = nc.clip(-8, 8);
```

This single line, when added to the SynthDef, causes ALL envelope decay parameters to stop updating on retrigger.

**However:** Removing just this line and hardcoding `-4` in its place did NOT fix the full synth in our final test. This suggests either:
1. There may be multiple trigger points
2. The sed replacement didn't work correctly
3. There's something about the cumulative effect

## Tests That Worked

1. **Simple envelope test** (Env.perc with ad parameter):
   ```supercollider
   SynthDef(\envtest, { |t_gate = 0, ad = 100|
       var trig = Trig1.kr(t_gate, 0.001);
       var env = EnvGen.kr(Env.perc(0.001, ad / 1000, 1, -4), trig);
       var sig = SinOsc.ar(440) * env * 0.3;
       Out.ar(0, sig ! 2);
   }).add;
   ```
   ✅ AD changes audible on retrigger

2. **Medium complexity test** (monokit args + simplified body):
   ```supercollider
   SynthDef(\medium_test, { |t_gate = 0, [all monokit args], ad = 100, ...|
       // Just envelope + simple output
       trig = Trig1.kr(t_gate, 0.001);
       ampEnv = EnvGen.kr(Env.perc(0.001, ad / 1000, 1, -4), trig);
       sig = SinOsc.ar(pf) * pvSmooth;
       amp = Select.kr(vca_mode.clip(0, 1), [1.0, ampEnv]) * volumeSmooth;
       Out.ar(0, sig * amp ! 2);
   }).add;
   ```
   ✅ Works with full arg list but simplified body

3. **Slew system test** (all 30+ slew parameters): ✅ Works
4. **Full filter bank test** (14 filter types in Select.ar): ✅ Works
5. **Effects chain test** (delay, reverb, compressor, EQ): ✅ Works
6. **LocalBuf/beat repeat test**: ✅ Works
7. **Multiple envelopes test** (all 7 envelopes): ✅ Works

## Tests That Failed

1. **Compiled monokit.scsyndef** loaded via d_load: ❌ Breaks
2. **Full monokit SynthDef via .add()**: ❌ Breaks
3. **Partial monokit including `ncVal = nc.clip(-8, 8)`**: ❌ Breaks

## Technical Details

### nc Parameter
- **Purpose:** Noise envelope curve (-8 to +8)
- **Default:** -4
- **Arg position:** Line 117 in monokit_server.scd
- **Usage:** Controls the curve shape of the noise envelope

### Why nc.clip() Might Cause Issues

Theories:
1. **Name collision?** - `nc` is a very short name, could conflict with something internal (unlikely - search found nothing)
2. **Threshold effect?** - Some cumulative limit being hit at this exact point
3. **Graph ordering?** - Something about how scsynth orders/optimizes the UGen graph
4. **Control bus allocation?** - Internal limit being exceeded

### Envelope Code Pattern (All Use Same Pattern)

```supercollider
// These ALL use the same pattern:
pitchEnv = EnvGen.kr(Env.perc(pAtk, pd / 1000, 1, pCrv), trig);
ampEnv = EnvGen.kr(Env.perc(aAtk, ad / 1000, 1, aCrv), trig);
fmEnv = EnvGen.kr(Env.perc(fmAtk, fd / 1000, 1, fmCrv), trig);
dcEnv = EnvGen.kr(Env.perc(dAtk, dd / 1000, 1, dCrv), trig);
filterEnv = EnvGen.kr(Env.perc(flAtk, fed / 1000, 1, flCrv), trig);
noiseEnv = EnvGen.kr(Env.perc(naSmooth / 1000, ndSmooth / 1000, 1, ncVal), trig);
```

The difference is that `noiseEnv` uses `ncVal` which comes from `nc.clip(-8, 8)`.

## Files Modified During Investigation

- `/Users/why/repos/monokit/src/metro.rs` - Added debug logging (reverted)
- Various `/tmp/test_*.scd` files created for testing

## Attempted Fix: Remove nc Parameter

**Date:** 2025-12-11 evening

### What We Did

Removed the `nc` parameter entirely from both `monokit_server.scd` and `compile_synthdefs.scd`:

1. Commented out `nc = -4` in the arg list
2. Removed `ncVal` from var declarations
3. Removed `ncVal = nc.clip(-8, 8);` line
4. Hardcoded `-4` in the noise envelope: `Env.perc(naSmooth / 1000, ndSmooth / 1000, 1, -4)`

### Files Modified

- `/Users/why/repos/monokit/sc/monokit_server.scd` - Lines 117, 155, 201, 248
- `/Users/why/repos/monokit/build_scripts/compile_synthdefs.scd` - Lines 117, 152, 196, 237

### Result

❌ **Still broken** - Envelopes still don't respond after recompiling and bundling

### Conclusion

Simply removing `ncVal = nc.clip(-8, 8);` is NOT sufficient. This means:

1. **There may be additional trigger lines** we haven't found yet
2. **The issue may be cumulative** - the full SynthDef complexity itself causes the bug
3. **Something about the graph structure** after line ~180 breaks envelope parameter updates
4. **There may be a SuperCollider bug** related to SynthDef size/complexity

The fact that individual components work but the full SynthDef doesn't suggests this is either:
- A threshold/limit being hit in scsynth's UGen graph compiler
- A graph optimization bug that affects parameter routing
- Some interaction between multiple `.clip()` operations or control-rate operations

## Next Steps

1. **Test with SuperCollider IDE** - Load the SynthDef interactively and watch for warnings
2. **Use s.dumpOSC(true)** to verify OSC messages reach scsynth correctly
3. **Check scsynth server logs** for any internal errors/warnings
4. **Test on different SuperCollider version** (3.13 vs 3.14)
5. **Report to SuperCollider mailing list** with minimal reproduction case
6. **Consider architectural changes**:
   - Split into multiple SynthDefs
   - Use control buses for parameter routing
   - Reduce total number of control-rate operations

## Environment

- SuperCollider 3.14.1
- macOS Darwin 24.5.0
- monokit v0.3.4
- scsynth-direct mode (port 57110)

## RESOLUTION: Lag.kr(param, 0) Workaround

**Date:** 2025-12-12
**Status:** PARTIAL FIX - AD/PD/FD/DD working, but NEW ISSUE with NV parameter

### Fix Applied

Added `Lag.kr(param, 0)` identity operations to force dynamic evaluation of envelope decay parameters:

```supercollider
// Force dynamic evaluation of decay params (workaround for SC optimizer bug)
adDynamic = Lag.kr(ad, 0);
pdDynamic = Lag.kr(pd, 0);
fdDynamic = Lag.kr(fd, 0);
ddDynamic = Lag.kr(dd, 0);
fbdDynamic = Lag.kr(fbd, 0);
fedDynamic = Lag.kr(fed, 0);
mbdDynamic = Lag.kr(mbd, 0);

// Use in envelopes
pitchEnv = EnvGen.kr(Env.perc(pAtk, pdDynamic / 1000, 1, pCrv), trig);
ampEnv = EnvGen.kr(Env.perc(aAtk, adDynamic / 1000, 1, aCrv), trig);
// etc...
```

### Result
✅ **Envelope decay parameters now respond to changes**
❌ **NEW BUG: NV parameter (noise volume) appears to be stuck/frozen**

### NEW ISSUE: NV Parameter Frozen

**Symptoms:**
- Noise is always audible regardless of NV setting
- NV parameter does not respond to changes
- This is a recently added parameter (noise source integration in v0.3.4)

**Hypothesis:**
- May have hit ANOTHER complexity threshold
- The Lag.kr() additions pushed us over a different limit
- NV is one of the more recently added parameters - position in SynthDef may be significant
- Could be same optimizer bug affecting different parameters now

**Questions to Investigate:**
1. Are we hitting another SuperCollider optimizer threshold with the added Lag.kr() operations?
2. Does parameter order/position matter? NV was added recently with noise integration
3. Are there OTHER parameters that have also become frozen?
4. Is this the same optimizer bug manifesting differently, or a new issue?

**Testing Needed:**
- Test other recently added noise parameters (NW, NA, ND, NE, NP, NM, NG)
- Test voice-level parameters (PV, MV)
- Count total control-rate operations after fix
- Try applying same Lag.kr(0) workaround to noise parameters

## RESOLUTION: NV Parameter Audio Issue (2025-12-12)

After applying the Lag.kr(0) fix for envelope decays, NV (noise volume) parameter appeared stuck.

**Investigation Results:**
- Automated tests showed NV WAS updating at control-rate correctly
- The issue was NOT parameter freezing but an **audio-rate signal path problem**

**Solution Applied (Two-Part Fix):**

1. **K2A.ar() Audio-Rate Promotion** (Line 322):
   ```supercollider
   sig = (primaryOsc * K2A.ar(pvSmooth)) + (modOsc * K2A.ar(mvSmooth)) + (noiseShaped * K2A.ar(nvSmooth));
   ```
   Explicitly promotes control-rate volume parameters to audio-rate before multiplication.

2. **Lag.kr(0) Workaround for Volume Parameters** (Lines 207-213):
   ```supercollider
   // Force dynamic evaluation (same as envelope fix)
   nvDynamic = Lag.kr(nv, 0);
   pvDynamic = Lag.kr(pv, 0);
   mvDynamic = Lag.kr(mv, 0);

   // Use in smoothing
   nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
   pvSmooth = Lag.kr(pvDynamic / 16383, 0.01);
   mvSmooth = Lag.kr(mvDynamic / 16383, 0.01);
   ```

**Result:** ✅ NV parameter now responds correctly

**Total Lag.kr Operations:** 59 (was 49 in v0.3.4, +7 for envelopes, +3 for volumes)

## Related Files

- `build_scripts/compile_synthdefs.scd` - Compiled SynthDef source
- `sc/monokit_server.scd` - Runtime SynthDef source
- `sc/synthdefs/monokit.scsyndef` - Compiled binary SynthDef
- `docs/NV_FREEZE_ANALYSIS.md` - Detailed analysis of NV parameter freeze
- `/tmp/nv_param_test.scd` - Test suite for NV freeze
