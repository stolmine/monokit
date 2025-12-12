# NV Parameter Freeze Test Results

## Test Date
2025-12-12

## Test Methodology
Automated test suite using Poll.kr to monitor RAW parameter values vs SMOOTH (Lag.kr-processed) values across three scenarios.

## Test Scenarios

### Test 1: WITH ncVal line (BUG STATE)
Current monokit_server.scd implementation with the envelope fix:
```supercollider
ncVal = nc.clip(-8, 8);  // Line 201 - CONTAMINATION POINT
```

### Test 2: WITHOUT ncVal line (POTENTIAL FIX)
Removed the ncVal assignment entirely, hardcoded -4 in envelope:
```supercollider
// ncVal = nc.clip(-8, 8);  // REMOVED
noiseEnv = EnvGen.kr(Env.perc(..., -4), trig);  // Hardcoded
```

### Test 3: WITH Lag.kr(0) workaround
Added Lag.kr(0) force-dynamic evaluation BEFORE smoothing:
```supercollider
nvDynamic = Lag.kr(nv, 0);
npDynamic = Lag.kr(np, 0);
nmDynamic = Lag.kr(nm, 0);
neDynamic = Lag.kr(ne, 0);
pvDynamic = Lag.kr(pv, 0);
mvDynamic = Lag.kr(mv, 0);

// Then use dynamic versions in smoothing:
nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
```

## Critical Finding: NO PARAMETERS ARE FROZEN

### Detailed Analysis

#### Test 1 Results (WITH ncVal - BUG STATE)
ALL parameters responded correctly:

**NV (Noise Volume):**
- RAW_NV: 0 → SMOOTH_NV: 0 ✓
- RAW_NV: 8192 → SMOOTH_NV: 0.500031 ✓
- RAW_NV: 16383 → SMOOTH_NV: 1.0 ✓
- RAW_NV: 4096 → SMOOTH_NV: 0.250015 ✓
- **STATUS: WORKING CORRECTLY**

**NE (Noise Envelope Amount):**
- RAW_NE: 16383 → SMOOTH_NE: 1.0 ✓
- RAW_NE: 0 → SMOOTH_NE: 0 ✓
- RAW_NE: 8192 → SMOOTH_NE: 0.500031 ✓
- **STATUS: WORKING CORRECTLY**

**NP (Noise → Primary FM):**
- RAW_NP: 0 → SMOOTH_NP: 0 ✓
- RAW_NP: 16383 → SMOOTH_NP: 1.0 ✓
- RAW_NP: 8192 → SMOOTH_NP: 0.500031 ✓
- **STATUS: WORKING CORRECTLY**

**PV (Primary Volume - Control):**
- RAW_PV: 16383 → SMOOTH_PV: 1.0 ✓
- RAW_PV: 0 → SMOOTH_PV: 0 ✓
- RAW_PV: 8192 → SMOOTH_PV: 0.500031 ✓
- **STATUS: WORKING CORRECTLY**

#### Test 2 Results (WITHOUT ncVal)
**STATUS: IDENTICAL BEHAVIOR TO TEST 1**
- All parameters responded correctly
- No difference observed from removing ncVal line

#### Test 3 Results (WITH Lag.kr(0) workaround)
**STATUS: IDENTICAL BEHAVIOR TO TEST 1**
- All parameters responded correctly
- Lag.kr(0) workaround had no effect (as expected, since nothing was frozen)

## Conclusion: The "Freeze" is an Auditory Illusion

### What We Discovered
The Poll.kr monitoring proves definitively that:
1. **NV parameter is NOT frozen** - it tracks changes perfectly (0 → 0.5 → 1.0)
2. **All noise parameters work correctly** - NE, NP, NM all respond
3. **ncVal line is NOT causing parameter freezing**

### The Real Problem: Audio-Rate Multiplication Issue

The issue is NOT parameter freezing. Looking at the signal flow:

```supercollider
sig = (primaryOsc * pvSmooth) + (noiseShaped * nvSmooth);
```

**Current behavior:**
- `nvSmooth` changes correctly (confirmed by Poll.kr)
- `noiseShaped` is generated correctly
- But `noiseShaped * nvSmooth` audio multiplication may have issues

### Possible Causes

1. **Audio-Rate vs Control-Rate interaction**
   - `noiseShaped` is audio-rate
   - `nvSmooth` is control-rate
   - Multiplication might not be interpolating properly

2. **Buffer size / block boundary issues**
   - Control-rate signals update at block boundaries (64 samples @ 44.1kHz)
   - Could cause stepping artifacts that sound like "frozen" parameter

3. **Noise generator state**
   - WhiteNoise/PinkNoise/BrownNoise might need re-initialization
   - Previous envelope fix may have affected noise generation timing

4. **Signal chain optimization**
   - SuperCollider optimizer might be pre-computing noise * 0 and caching it
   - Even though nvSmooth changes, the audio multiplication might be optimized away

## Recommended Next Steps

### 1. Test Audio Multiplication Directly
Create a test that monitors the ACTUAL AUDIO OUTPUT, not just parameter values:

```supercollider
// Monitor the actual multiplied signal
Poll.ar(Impulse.ar(10), noiseShaped * nvSmooth, "AUDIO_OUT");
```

### 2. Force Audio-Rate Processing
Try promoting nvSmooth to audio-rate:

```supercollider
nvSmoothAR = K2A.ar(nvSmooth);
sig = (primaryOsc * pvSmooth) + (noiseShaped * nvSmoothAR);
```

### 3. Check Noise Generator
Verify noise generator is actually producing signal:

```supercollider
Poll.ar(Impulse.ar(10), noiseShaped, "NOISE_SHAPED");
```

### 4. Bypass Volume Control
Test if noise works when volume is directly applied:

```supercollider
// Instead of parameter multiplication
sig = (primaryOsc * pvSmooth) + (noiseRaw * noiseEnv * 0.5);  // Direct value
```

## Files
- Test suite: `/tmp/automated_freeze_test.scd`
- This report: `/Users/why/repos/monokit/docs/NV_PARAMETER_TEST_RESULTS.md`
