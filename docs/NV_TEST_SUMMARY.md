# NV Parameter Investigation Summary

## Executive Summary

Comprehensive automated testing reveals that **NO PARAMETERS ARE FROZEN**. The Poll.kr monitoring proves that NV, NE, NP, and PV all track changes perfectly at the control-rate level. The issue appears to be an audio-rate multiplication or signal flow problem, not parameter freezing.

## Test Results Overview

### Test 1: WITH ncVal line (Current Bug State)
- NV parameter: **WORKING** (0 → 0.5 → 1.0 transitions perfectly)
- NE parameter: **WORKING** (0 → 0.5 → 1.0 transitions perfectly)
- NP parameter: **WORKING** (0 → 0.5 → 1.0 transitions perfectly)
- PV parameter: **WORKING** (0 → 0.5 → 1.0 transitions perfectly)

### Test 2: WITHOUT ncVal line
- **IDENTICAL BEHAVIOR** to Test 1
- Removing ncVal had **NO EFFECT**
- All parameters still working correctly

### Test 3: WITH Lag.kr(0) workaround
- **IDENTICAL BEHAVIOR** to Test 1
- Lag.kr(0) workaround had **NO EFFECT** (as expected)
- All parameters still working correctly

## Critical Discovery: Auditory Illusion

What sounds like a "frozen parameter" is actually an **audio signal flow issue**, not frozen parameter values. The control-rate parameters are updating correctly, but something downstream in the audio path is not responding.

## Root Cause Theories

### Theory 1: Control-Rate to Audio-Rate Multiplication
```supercollider
noiseMultiplied = noiseShaped * nvSmooth;
//                ^audio-rate  ^control-rate
```

**Problem:** Control-rate signals update at block boundaries (64 samples). When multiplying CR * AR, the interpolation might not be working correctly, causing stepping or freezing artifacts.

**Solution:** Promote nvSmooth to audio-rate before multiplication:
```supercollider
nvSmoothAR = K2A.ar(nvSmooth);
noiseMultiplied = noiseShaped * nvSmoothAR;
```

### Theory 2: Optimizer Pre-Computation
SuperCollider's optimizer might be detecting:
1. Initial state: `noiseShaped * 0` (when nvSmooth = 0)
2. Optimization: "This always equals zero, cache it!"
3. Bug: Even when nvSmooth changes, the cached zero is used

**Solution:** Force dynamic evaluation with Lag.kr(0) or use a different signal path.

### Theory 3: Noise Generator State
The noise generators (WhiteNoise, PinkNoise, BrownNoise) might not be properly initialized or reset when transitioning from zero volume.

**Solution:** Use a gate to restart noise generator when NV > 0.

### Theory 4: Select.ar/Select.kr Issue
The envelope gate mode uses Select.kr:
```supercollider
noiseShaped = noiseRaw * Select.kr(ngVal, [
    1,
    ((1 - neSmooth) + (neSmooth * noiseEnv))
]);
```

When in drone mode (ng=0), this multiplies by 1, but the Select might be causing optimization issues.

**Solution:** Replace Select with simple multiplication.

## Next Steps

### Immediate Action: Run Audio-Rate Test
Execute `/tmp/audio_multiplication_test.scd` to monitor actual audio output:
```bash
/Applications/SuperCollider.app/Contents/MacOS/sclang /tmp/audio_multiplication_test.scd
```

This will:
1. Monitor `NOISE_MULTIPLIED` at audio rate
2. Test K2A.ar promotion as potential fix
3. Confirm if multiplication is the issue

### If Audio Test Confirms Multiplication Issue

Apply K2A.ar fix to monokit_server.scd:

```supercollider
// Promote volume controls to audio-rate BEFORE final multiplication
nvSmoothAR = K2A.ar(nvSmooth);
pvSmoothAR = K2A.ar(pvSmooth);
mvSmoothAR = K2A.ar(mvSmooth);

// Use audio-rate multiplication for all volume controls
primarySig = primaryOsc * pvSmoothAR;
modulatorSig = modOsc * mvSmoothAR;
noiseSig = noiseShaped * nvSmoothAR;
```

### Alternative Approaches

1. **Use .ar instead of Lag.kr**
   ```supercollider
   nvSmooth = Lag.ar(nv / 16383, 0.01);  // Audio-rate smoothing
   ```

2. **Force signal evaluation**
   ```supercollider
   noiseMultiplied = (noiseShaped * nvSmooth) + DC.ar(0);  // Force evaluation
   ```

3. **Bypass volume parameter entirely for testing**
   ```supercollider
   sig = noiseShaped * 0.5;  // Fixed value - does this work?
   ```

## Files Generated

### Test Scripts
- `/tmp/nv_param_test.scd` - Original test suite setup
- `/tmp/automated_freeze_test.scd` - Automated parameter monitoring test
- `/tmp/audio_multiplication_test.scd` - Audio-rate signal debugging test

### Documentation
- `/Users/why/repos/monokit/docs/NV_PARAMETER_TEST_RESULTS.md` - Detailed test results
- `/Users/why/repos/monokit/docs/NV_TEST_SUMMARY.md` - This summary document

## Conclusion

The ncVal line (line 201) is **NOT** causing parameter freezing. The envelope fix was correct. The bug is somewhere in the audio signal path, most likely in the control-rate to audio-rate multiplication step.

**Recommendation:** Run the audio multiplication test next to pinpoint the exact location of the signal flow issue, then apply the K2A.ar fix if confirmed.
