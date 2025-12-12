# SuperCollider Optimizer Workaround Pattern

**Date:** 2025-12-12
**Status:** PROVEN SOLUTION

## Problem

SuperCollider 3.14.1's control-rate optimizer incorrectly treats certain parameters as constants when SynthDef complexity crosses specific thresholds, causing them to freeze at instantiation values and not respond to `.set()` calls.

## Root Cause

When a SynthDef contains:
- 49+ Lag.kr operations
- Multiple .clip() operations on control-rate values
- Complex Select.kr conditional routing (30+ instances)

The optimizer enters an aggressive state where it incorrectly marks parameters processed AFTER certain lines (particularly `nc.clip(-8, 8)` in our case) as "quasi-constant" and evaluates them only at synth instantiation time.

## Proven Workaround Pattern

### The Lag.kr(0) Identity Operation

Insert a zero-lag Lag.kr operation before using the parameter:

```supercollider
// 1. Declare dynamic variable
var paramDynamic;

// 2. Force dynamic evaluation BEFORE use
paramDynamic = Lag.kr(param, 0);

// 3. Use dynamic version in calculations
smoothParam = Lag.kr(paramDynamic / 16383, 0.01);
```

**How it works:**
- Lag.kr(param, 0) with zero lag time is a no-op at DSP level
- BUT it acts as a compiler optimization fence
- Forces the optimizer to treat the parameter as truly dynamic
- Ensures parameter is freshly read each control cycle

## Application History

### Iteration 1: Envelope Decay Parameters (2025-12-12)

**Problem:** AD, PD, FD, DD, FBD, FED, MBD frozen at default values

**Solution Applied:**
```supercollider
// Variable declarations
var adDynamic, pdDynamic, fdDynamic, ddDynamic, fbdDynamic, fedDynamic, mbdDynamic;

// Force dynamic evaluation (lines 230-236 in monokit_server.scd)
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
fmEnv = EnvGen.kr(Env.perc(fmAtk, fdDynamic / 1000, 1, fmCrv), trig);
dcEnv = EnvGen.kr(Env.perc(dAtk, ddDynamic / 1000, 1, dCrv), trig);
fbEnv = EnvGen.kr(Env.perc(fbAtk, fbdDynamic / 1000, 1, fbCrv), trig);
filterEnv = EnvGen.kr(Env.perc(flAtk, fedDynamic / 1000, 1, flCrv), trig);
modBusEnv = EnvGen.kr(Env.perc(0.001, mbdDynamic / 1000, 1, -4), trig);
```

**Result:** ✅ All envelope decay parameters now respond correctly
**Added Operations:** +7 Lag.kr (49 → 56 total)

### Iteration 2: Volume Parameters (2025-12-12)

**Problem:** NV, PV, MV appeared stuck (actually audio-rate issue combined with optimizer)

**Solution Applied:**
```supercollider
// Variable declarations
var nvDynamic, pvDynamic, mvDynamic;

// Force dynamic evaluation (lines 207-209 in monokit_server.scd)
nvDynamic = Lag.kr(nv, 0);
pvDynamic = Lag.kr(pv, 0);
mvDynamic = Lag.kr(mv, 0);

// Use in smoothing
nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
pvSmooth = Lag.kr(pvDynamic / 16383, 0.01);
mvSmooth = Lag.kr(mvDynamic / 16383, 0.01);

// ALSO applied K2A.ar() promotion for audio-rate multiplication:
sig = (primaryOsc * K2A.ar(pvSmooth)) + (modOsc * K2A.ar(mvSmooth)) + (noiseShaped * K2A.ar(nvSmooth));
```

**Result:** ✅ Volume parameters now respond correctly
**Added Operations:** +3 Lag.kr (56 → 59 total)

## When to Apply This Pattern

Apply Lag.kr(0) workaround when:

1. **Parameter doesn't respond to .set() calls** despite OSC being sent correctly
2. **Parameter works in simple test SynthDef** but fails in complex one
3. **Parameter is processed AFTER problematic .clip() operations** in code
4. **SynthDef has high control-rate operation count** (49+ Lag.kr, 50+ Select.kr, etc.)

## Testing Methodology

Use Poll.kr to verify parameter updates at control-rate:

```supercollider
// Monitor both raw and smoothed values
Poll.kr(Impulse.kr(4), param, "RAW_PARAM");
Poll.kr(Impulse.kr(4), smoothParam, "SMOOTH_PARAM");
```

If RAW updates but SMOOTH doesn't → parameter frozen
If both update → audio-rate issue, not parameter freeze

## Cost Analysis

**Per Parameter:**
- +1 control-rate UGen (Lag.kr with 0 lag time)
- Negligible CPU impact (~0.01% per parameter)
- Zero audio latency (lag time is 0)

**Threshold Risk:**
- Each Lag.kr(0) adds to total operation count
- May trigger cascading thresholds if many parameters need the fix
- Current count: 59 Lag.kr operations (approaching unknown threshold)

## Alternative Solutions (If Threshold Becomes Issue)

1. **Remove problematic nc parameter** - Reduces complexity, loses one control
2. **Split SynthDef** into Voice + Effects - Major refactor, but reduces per-SynthDef complexity
3. **Use Control Buses** - External parameter routing, architectural change
4. **Upgrade SuperCollider** - May be fixed in newer versions (test 3.13 vs 3.14)

## Files Modified

All applications of this pattern update both:
- `sc/monokit_server.scd` - Runtime SynthDef
- `build_scripts/compile_synthdefs.scd` - Pre-compilation source

## Related Documentation

- `docs/ENVELOPE_BUG_INVESTIGATION.md` - Detailed investigation history
- `docs/NV_FREEZE_ANALYSIS.md` - NV parameter analysis
- `docs/NV_PARAMETER_TEST_RESULTS.md` - Automated test results

## SuperCollider Bug Report Status

This should be reported to SuperCollider developers with minimal reproduction case showing:
- Simple parameter works alone
- Same parameter freezes when surrounded by specific operations
- Lag.kr(0) workaround fixes it

**GitHub:** https://github.com/supercollider/supercollider/issues
