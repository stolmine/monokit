# Monokit Tier 1 DSP Blocks - Implementation Plan

## STATUS: COMPLETE

All Tier 1 DSP blocks have been implemented as of the current codebase state.

## Architecture Overview

### IMPLEMENTED Signal Flow

Complete signal chain (from monokit_server.scd):
1. Modulator oscillator (with optional feedback)
2. Primary oscillator with FM
3. Mix between primary and modulator
4. Discontinuity injection + waveshaping (fold/tanh/softclip)
5. **SVF Multi-Mode Filter** (LP/HP/BP/Notch with envelope and key tracking)
6. **Comb Resonator** (Karplus-Strong with key tracking)
7. Amplitude envelope
8. **Stereo Delay** (ping-pong with feedback filtering and routing modes)
9. **Plate Reverb** (FreeVerb2 with pre-delay and routing modes)
10. Stereo output

### Effect Routing System

Implemented routing modes for delay and reverb:
- **BYPASS (0)**: Effect disabled, signal passes through unchanged
- **INSERT (1)**: Traditional series processing with wet/dry mix
- **SEND (2)**: Parallel processing where wet parameter controls send level

Tail behaviors:
- **CUT (0)**: Tails cut immediately when wet=0
- **RING (1)**: Tails decay naturally (default)
- **FREEZE (2)**: Stop new input, sustain current tail indefinitely

Commands: D.MODE, D.TAIL, R.MODE, R.TAIL

---

## 1. SVF Multi-Mode Filter

### Parameters

| Param | SC Arg | Range | Default | Type | Description |
|-------|--------|-------|---------|------|-------------|
| FC | fc | 20-20000 | 1000 | Float | Filter cutoff frequency (Hz) |
| FQ | fq | 0-16383 | 0 | Int | Filter resonance (0-16383) |
| FT | ft | 0-3 | 0 | Int | Filter type (0=LP, 1=HP, 2=BP, 3=Notch) |
| FE | fe | 0-16383 | 0 | Int | Filter envelope amount |
| FED | fed | 1-10000 | 100 | Int | Filter envelope decay (ms) |
| FK | fk | 0-16383 | 0 | Int | Filter key tracking amount |
| MF | mf_route | 0-1 | 0 | Int | ModBus → Filter cutoff routing |

### SuperCollider Implementation

**UGen Choice:** `SVF.ar` (State Variable Filter)
- Provides LP/HP/BP/Notch from single UGen
- Efficient sample-accurate switching between modes
- Good resonance behavior without instability

**Signal Chain Position:** After discontinuity, before output

```supercollider
// Add to SynthDef args:
fc = 1000,
fq = 0,
ft = 0,
fe = 0,
fed = 100,
fk = 0,
mf_route = 0,

// Add to var declarations:
var filterEnv, filterCutoff, filterQ;

// After discontinuity (line 96):
// Filter envelope
filterEnv = EnvGen.kr(Env.perc(0.001, fed / 1000, 1, -4), trig);

// Filter cutoff: base + (keyTracking * primaryFreq) + (envelope * amount) + (modBus * routing)
// Key tracking maps primaryFreq to cutoff (0 = fixed, 16383 = 1:1 tracking)
filterCutoff = fc
    + ((fk / 16383) * primaryFreq)
    + (filterEnv * (fe / 16383) * 5000)  // Scale envelope to +/- 5kHz range
    + (modBus * mf_route * 5000);         // ModBus can modulate +/- 5kHz
filterCutoff = filterCutoff.clip(20, 20000);

// Resonance: 0-16383 maps to 0.1-1.0 (SVF res range)
filterQ = ((fq / 16383) * 0.9 + 0.1);

// Apply filter with mode switching
sig = SVF.ar(sig, filterCutoff, filterQ,
    lowpass: ft.clip(0, 3) == 0,
    highpass: ft.clip(0, 3) == 1,
    bandpass: ft.clip(0, 3) == 2,
    notch: ft.clip(0, 3) == 3
);
```

### Rust Command Handlers

File: `/Users/why/repos/monokit/src/commands/synth_params.rs`

Add 7 new handlers (approx. 20 lines each):
- `handle_fc()` - Float 20-20000
- `handle_fq()` - Int 0-16383
- `handle_ft()` - Int 0-3 (with enum display)
- `handle_fe()` - Int 0-16383
- `handle_fed()` - Int 1-10000
- `handle_fk()` - Int 0-16383
- `handle_mf_route()` - Int 0-1

File: `/Users/why/repos/monokit/src/commands/mod.rs`

Add 7 match cases in `process_command()`:
```rust
"FC" => synth_params::handle_fc(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
"FQ" => synth_params::handle_fq(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
"FT" => synth_params::handle_ft(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
"FE" => synth_params::handle_fe(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
"FED" => synth_params::handle_fed(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
"FK" => synth_params::handle_fk(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
"MF.ROUTE" => synth_params::handle_mf_route(&parts, variables, patterns, scripts, script_index, metro_tx, output)?,
```

**Lines of Code:**
- SC: ~25 lines
- Rust: ~150 lines (7 handlers @ ~20 lines + 7 match cases)

---

## 2. Comb Resonator (Karplus-Strong)

### Parameters

| Param | SC Arg | Range | Default | Type | Description |
|-------|--------|-------|---------|------|-------------|
| RF | rf | 20-5000 | 440 | Float | Resonator frequency (Hz) |
| RD | rd | 10-5000 | 500 | Int | Resonator decay (ms) |
| RM | rm | 0-16383 | 0 | Int | Resonator mix (dry/wet) |
| RK | rk | 0-16383 | 0 | Int | Key tracking (follows primary pitch) |

### SuperCollider Implementation

**UGen Choice:** `CombC.ar` (cubic interpolation comb filter)
- More accurate pitch than CombL
- Good for resonant/string-like tones
- Stable with feedback control

**Signal Chain Position:** After filter, parallel mix

```supercollider
// Add to SynthDef args:
rf = 440,
rd = 500,
rk = 0,
rm = 0,

// Add to var declarations:
var resonatorFreq, resonatorDelay, resonatorDecay, resonatorMix, resonated;

// After filter (after SVF):
// Resonator frequency: base + (keyTracking * primaryFreq)
resonatorFreq = rf + ((rk / 16383) * (primaryFreq - rf));
resonatorFreq = resonatorFreq.clip(20, 5000);

// Convert frequency to delay time
resonatorDelay = (1 / resonatorFreq).clip(0.0002, 0.05);  // 20Hz-5kHz

// Decay time in seconds
resonatorDecay = (rd / 1000).clip(0.01, 5.0);

// Apply comb filter
resonated = CombC.ar(sig, 0.05, resonatorDelay, resonatorDecay);

// Mix: 0 = dry, 16383 = 100% wet
resonatorMix = rm / 16383;
sig = (sig * (1 - resonatorMix)) + (resonated * resonatorMix);
```

### Rust Command Handlers

Add 4 handlers to synth_params.rs:
- `handle_rf()` - Float 20-5000
- `handle_rd()` - Int 10-5000
- `handle_rm()` - Int 0-16383
- `handle_rk()` - Int 0-16383

Add 4 match cases to commands/mod.rs

**Lines of Code:**
- SC: ~20 lines
- Rust: ~90 lines (4 handlers @ ~20 lines + 4 match cases)

---

## 3. Stereo Delay with Feedback Filtering

### Parameters

| Param | SC Arg | Range | Default | Type | Description |
|-------|--------|-------|---------|------|-------------|
| DT | dt | 1-2000 | 250 | Int | Delay time (ms) |
| DF | df | 0-16383 | 0 | Int | Delay feedback amount |
| DD | dd_damp | 100-20000 | 5000 | Float | Delay damping/LPF cutoff (Hz) |
| DW | dw | 0-16383 | 0 | Int | Delay wet mix |
| DS | ds | 0-1 | 0 | Int | Delay sync (0=free, 1=tempo - NOT IMPLEMENTED YET) |

**NOTE:** Tempo sync (DS) requires BPM from metro thread. For now, implement free-running mode only. Add BPM parameter passing in future iteration.

### SuperCollider Implementation

**UGen Choice:** `DelayC.ar` (cubic interpolation)
- Better than CombC for tempo-sync applications
- Clean delay without resonance
- Suitable for both mono and stereo

**Signal Chain Position:** Post-resonator, requires stereo rework

**IMPORTANT:** Current architecture uses `Pan2.ar(sig * amp, 0)` for mono output. Need to refactor to stereo chain.

```supercollider
// Add to SynthDef args:
dt = 250,
df = 0,
dd_damp = 5000,
dw = 0,
ds = 0,

// Add to var declarations:
var delayTime, delayFeedback, delayWet;
var delayBufL, delayBufR, delayedL, delayedR;
var sigL, sigR;

// BEFORE output (replace Pan2.ar line):
// Convert to stereo BEFORE delay
sigL = sig;
sigR = sig;

// Delay parameters
delayTime = (dt / 1000).clip(0.001, 2.0);  // Convert ms to seconds
delayFeedback = df / 16383;
delayWet = dw / 16383;

// Stereo ping-pong delay with damping filter in feedback path
delayBufL = LocalIn.ar(1);
delayBufR = LocalIn.ar(1);

// Left channel delay
delayedL = DelayC.ar(sigL + (delayBufR * delayFeedback), 2.0, delayTime);
delayedL = LPF.ar(delayedL, dd_damp);  // Damping filter

// Right channel delay (ping-pong from left)
delayedR = DelayC.ar(sigR + (delayBufL * delayFeedback), 2.0, delayTime);
delayedR = LPF.ar(delayedR, dd_damp);  // Damping filter

LocalOut.ar([delayedL, delayedR]);

// Mix dry/wet
sigL = (sigL * (1 - delayWet)) + (delayedL * delayWet);
sigR = (sigR * (1 - delayWet)) + (delayedR * delayWet);

// Apply amplitude to stereo signal
sigL = sigL * amp;
sigR = sigR * amp;

// Output stereo
Out.ar(0, [sigL, sigR]);
```

### Rust Command Handlers

Add 5 handlers to synth_params.rs:
- `handle_dt()` - Int 1-2000
- `handle_df()` - Int 0-16383
- `handle_dd_damp()` - Float 100-20000
- `handle_dw()` - Int 0-16383
- `handle_ds()` - Int 0-1 (for future tempo sync)

Add 5 match cases to commands/mod.rs

**Lines of Code:**
- SC: ~35 lines (includes stereo rework)
- Rust: ~110 lines (5 handlers @ ~20 lines + 5 match cases)

---

## 4. Plate Reverb

### Parameters

| Param | SC Arg | Range | Default | Type | Description |
|-------|--------|-------|---------|------|-------------|
| RV | rv | 0-16383 | 0 | Int | Reverb size/decay time |
| RP | rp | 0-100 | 0 | Int | Pre-delay (ms) |
| RH | rh | 0-16383 | 8000 | Int | High damping (maps to damping amount) |
| RW | rw | 0-16383 | 0 | Int | Reverb wet mix |

### SuperCollider Implementation

**UGen Choice:** `JPverb` (best quality for CPU)
- `FreeVerb` is cheaper but lower quality
- `JPverb` has excellent plate-like character
- `GVerb` is more expensive, can be unstable

**Signal Chain Position:** Final effect, post-delay

```supercollider
// Add to SynthDef args:
rv = 0,
rp = 0,
rh = 8000,
rw = 0,

// Add to var declarations:
var reverbDecay, reverbPreDelay, reverbDamping, reverbWet;
var reverbedL, reverbedR;

// After delay, before Out.ar:
// Reverb parameters
reverbDecay = (rv / 16383) * 5 + 0.5;  // 0.5-5.5 seconds decay
reverbPreDelay = rp / 1000;            // Convert to seconds
reverbDamping = rh / 16383;            // 0-1 damping amount
reverbWet = rw / 16383;

// Pre-delay
sigL = DelayC.ar(sigL, 0.1, reverbPreDelay);
sigR = DelayC.ar(sigR, 0.1, reverbPreDelay);

// Apply JPverb (stereo in, stereo out)
#reverbedL, reverbedR = JPverb.ar(
    [sigL, sigR],
    t60: reverbDecay,
    damp: reverbDamping,
    size: 2.0,  // Room size (fixed for plate character)
    earlyDiff: 0.707,  // Early reflection diffusion
    modDepth: 0.1,
    modFreq: 2.0
);

// Mix dry/wet
sigL = (sigL * (1 - reverbWet)) + (reverbedL * reverbWet);
sigR = (sigR * (1 - reverbWet)) + (reverbedR * reverbWet);

// Output (replace previous Out.ar)
Out.ar(0, [sigL, sigR]);
```

### Rust Command Handlers

Add 4 handlers to synth_params.rs:
- `handle_rv()` - Int 0-16383
- `handle_rp()` - Int 0-100
- `handle_rh()` - Int 0-16383
- `handle_rw()` - Int 0-16383

Add 4 match cases to commands/mod.rs

**Lines of Code:**
- SC: ~30 lines
- Rust: ~90 lines (4 handlers @ ~20 lines + 4 match cases)

---

## Complete Signal Flow Diagram

```
BEFORE (Current):
┌─────────────┐
│  Modulator  │ (mf, mw, tracking, modBus)
└──────┬──────┘
       │
       v
┌─────────────┐
│   Primary   │ (pf, pw, FM)
└──────┬──────┘
       │
       v
┌─────────────┐
│     Mix     │ (mx, mm, me)
└──────┬──────┘
       │
       v
┌─────────────┐
│Discontinuity│ (dc, dm, da)
└──────┬──────┘
       │
       v
┌─────────────┐
│  Amplitude  │ (ampEnv, volume, ma)
└──────┬──────┘
       │
       v
┌─────────────┐
│    Pan2     │ (mono → stereo)
└──────┬──────┘
       │
       v
    Output


AFTER (Tier 1 DSP):
┌─────────────┐
│  Modulator  │ (mf, mw, tracking, modBus)
└──────┬──────┘
       │
       v
┌─────────────┐
│   Primary   │ (pf, pw, FM)
└──────┬──────┘
       │
       v
┌─────────────┐
│     Mix     │ (mx, mm, me)
└──────┬──────┘
       │
       v
┌─────────────┐
│Discontinuity│ (dc, dm, da)
└──────┬──────┘
       │
       v
┌─────────────┐
│  SVF Filter │ (fc, fq, ft, fe, fed, fk, mf_route) ← NEW
└──────┬──────┘
       │
       v
┌─────────────┐
│Comb Resonator│ (rf, rd, rm, rk) ← NEW
└──────┬──────┘
       │
       v
┌─────────────┐
│  Amplitude  │ (ampEnv, volume, ma)
└──────┬──────┘
       │
       v
┌─────────────┐
│ Stereo Split│ ← NEW (sigL, sigR)
└──────┬──────┘
       │
       v
┌─────────────┐
│Stereo Delay │ (dt, df, dd_damp, dw, ds) ← NEW
└──────┬──────┘
       │
       v
┌─────────────┐
│Plate Reverb │ (rv, rp, rh, rw) ← NEW
└──────┬──────┘
       │
       v
    Output
```

---

## Implementation Order (Dependencies)

### Phase 1: Filter (No Dependencies)
1. Add filter parameters to SynthDef
2. Add filter envelope
3. Add SVF implementation
4. Add 7 Rust handlers
5. Test filter modes, resonance, envelope

**Estimated Time:** 2-3 hours

### Phase 2: Resonator (No Dependencies)
1. Add resonator parameters to SynthDef
2. Add CombC implementation
3. Add 4 Rust handlers
4. Test resonator with key tracking

**Estimated Time:** 1-2 hours

### Phase 3: Stereo Delay (Depends on stereo rework)
1. Refactor output to stereo (sigL, sigR)
2. Add delay parameters
3. Implement ping-pong delay with LocalIn/LocalOut
4. Add damping filter
5. Add 5 Rust handlers
6. Test feedback stability

**Estimated Time:** 3-4 hours

### Phase 4: Reverb (Depends on Phase 3 stereo)
1. Add reverb parameters
2. Add JPverb implementation
3. Add 4 Rust handlers
4. Test wet/mix, pre-delay, damping

**Estimated Time:** 1-2 hours

**Total Estimated Time:** 7-11 hours

---

## Parameter Summary Table

| Block | Params | SC Args | Rust Handlers | Match Cases | Total LoC (SC) | Total LoC (Rust) |
|-------|--------|---------|---------------|-------------|----------------|------------------|
| SVF Filter | 7 | fc, fq, ft, fe, fed, fk, mf_route | 7 | 7 | ~25 | ~150 |
| Comb Resonator | 4 | rf, rd, rm, rk | 4 | 4 | ~20 | ~90 |
| Stereo Delay | 5 | dt, df, dd_damp, dw, ds | 5 | 5 | ~35 | ~110 |
| Plate Reverb | 4 | rv, rp, rh, rw | 4 | 4 | ~30 | ~90 |
| **TOTALS** | **20** | **20** | **20** | **20** | **~110** | **~440** |

---

## SuperCollider Complete Parameter List

```supercollider
SynthDef(\monokit, {
    arg
        // Existing parameters
        pf = 200, pw = 0, mf = 50, mw = 0,
        dc = 0, dm = 0, tk = 0,
        mb = 0, mp = 0, md = 0, mt = 0, ma = 0,
        fm = 0,
        ad = 100, pd = 10, fd = 10, dd = 10,
        pa = 4, fa = 0, da = 0,
        mx = 0, mm = 0, me = 0,
        fb = 0, fba = 0, fbd = 10,
        volume = 1, gate = 0,

        // NEW: Filter parameters
        fc = 1000,      // Filter cutoff (Hz)
        fq = 0,         // Filter Q (resonance)
        ft = 0,         // Filter type (0-3)
        fe = 0,         // Filter envelope amount
        fed = 100,      // Filter envelope decay (ms)
        fk = 0,         // Filter key tracking
        mf_route = 0,   // ModBus → Filter routing

        // NEW: Resonator parameters
        rf = 440,       // Resonator frequency (Hz)
        rd = 500,       // Resonator decay (ms)
        rm = 0,         // Resonator mix
        rk = 0,         // Resonator key tracking

        // NEW: Delay parameters
        dt = 250,       // Delay time (ms)
        df = 0,         // Delay feedback
        dd_damp = 5000, // Delay damping (Hz)
        dw = 0,         // Delay wet mix
        ds = 0,         // Delay sync (0=free, 1=tempo)

        // NEW: Reverb parameters
        rv = 0,         // Reverb decay
        rp = 0,         // Reverb pre-delay (ms)
        rh = 8000,      // Reverb high damping
        rw = 0;         // Reverb wet mix
```

---

## Testing Checklist

### Filter Tests
- [ ] FC sweep 20-20000 Hz (listen for cutoff movement)
- [ ] FQ 0-16383 (verify no instability at max resonance)
- [ ] FT mode switching (0=LP, 1=HP, 2=BP, 3=Notch)
- [ ] FE + FED (filter envelope modulation)
- [ ] FK key tracking (cutoff follows pitch)
- [ ] MF.ROUTE modbus routing to cutoff

### Resonator Tests
- [ ] RF sweep 20-5000 Hz
- [ ] RD decay behavior (10-5000 ms)
- [ ] RM dry/wet mix
- [ ] RK key tracking (resonator follows pitch)

### Delay Tests
- [ ] DT delay time accuracy (1-2000 ms)
- [ ] DF feedback stability (no runaway at max)
- [ ] DD damping filter (100-20000 Hz)
- [ ] DW wet mix
- [ ] Ping-pong stereo effect
- [ ] DS tempo sync (not implemented yet)

### Reverb Tests
- [ ] RV decay time (0-16383 range)
- [ ] RP pre-delay (0-100 ms)
- [ ] RH damping (bright vs dark reverb)
- [ ] RW wet mix
- [ ] CPU usage (monitor with JPverb)

---

## Future Enhancements (Beyond Tier 1)

1. **BPM Tempo Sync**
   - Add BPM parameter to metro state
   - Pass BPM to SuperCollider via OSC
   - Implement DS (delay sync) using tempo divisions
   - Potential command: `M.BPM <value>` (already exists, needs OSC pass-through)

2. **Additional Modbus Routing**
   - ModBus → Delay time
   - ModBus → Reverb size
   - ModBus → Resonator frequency

3. **Filter Envelope Modes**
   - Currently uses perc envelope
   - Add AD, ASR, ADSR options

4. **Multi-Filter Topology**
   - Series/parallel filter routing
   - Dual SVF for complex filtering

5. **LFOs for Modulation**
   - Independent LFOs (separate from modulator)
   - Route to filter, delay, reverb parameters

---

## CPU Considerations

**Current architecture:** ~5-10% CPU (mono synthesis)

**Estimated CPU impact:**
- SVF: +2-3%
- CombC: +1-2%
- Stereo Delay (with LocalIn/Out): +3-5%
- JPverb: +8-12%

**Total estimated:** ~19-32% CPU per voice

**Optimization strategies if needed:**
- Replace JPverb with FreeVerb (-5% CPU)
- Use DelayL instead of DelayC (-1% CPU)
- Reduce reverb update rate to kr (not recommended)

---

## Files to Modify

### SuperCollider
- `/Users/why/repos/monokit/sc/monokit_server.scd` - Add all DSP blocks

### Rust
- `/Users/why/repos/monokit/src/commands/synth_params.rs` - Add 20 new handlers
- `/Users/why/repos/monokit/src/commands/mod.rs` - Add 20 new match cases

**Total files modified:** 3

---

## Command Name Conflicts

**Check for existing commands:**
- `DD` - Already used for discontinuity decay (line 589-621 in synth_params.rs)
- Need to use different name for delay damping: `DD.DAMP` or just use existing `dd_damp` in SC

**Resolution:**
- Use `DD.DAMP` for delay damping filter cutoff
- Keep `DD` for existing discontinuity decay
- Update Rust handler to `handle_dd_damp()`

---

## Validation Rules

Add to `/Users/why/repos/monokit/src/commands/validate.rs`:

```rust
// Filter commands
"FC" => parts.len() >= 2,
"FQ" => parts.len() >= 2,
"FT" => parts.len() >= 2,
"FE" => parts.len() >= 2,
"FED" => parts.len() >= 2,
"FK" => parts.len() >= 2,
"MF.ROUTE" => parts.len() >= 2,

// Resonator commands
"RF" => parts.len() >= 2,
"RD" => parts.len() >= 2,
"RM" => parts.len() >= 2,
"RK" => parts.len() >= 2,

// Delay commands
"DT" => parts.len() >= 2,
"DF" => parts.len() >= 2,
"DD.DAMP" => parts.len() >= 2,
"DW" => parts.len() >= 2,
"DS" => parts.len() >= 2,

// Reverb commands
"RV" => parts.len() >= 2,
"RP" => parts.len() >= 2,
"RH" => parts.len() >= 2,
"RW" => parts.len() >= 2,
```

---

## Example Usage Patterns

### Resonant Low-Pass Sweep
```
FC 200          # Start cutoff at 200 Hz
FQ 12000        # High resonance
FT 0            # Low-pass mode
FE 8000         # Strong envelope modulation
FED 500         # 500ms decay
TR              # Trigger
```

### Karplus-Strong Pluck
```
RF 220          # Resonator at A3
RD 2000         # 2 second decay
RM 8000         # 50% mix
RK 16383        # Full key tracking
PF 220          # Primary at A3
TR              # Pluck!
```

### Ping-Pong Delay
```
DT 375          # 375ms delay (dotted 8th at 120 BPM)
DF 8000         # Medium feedback
DD.DAMP 2000    # Dark repeats
DW 6000         # 37% wet
TR              # Trigger
```

### Large Plate Reverb
```
RV 12000        # Long decay
RP 20           # 20ms pre-delay
RH 4000         # Moderate damping
RW 4000         # 25% wet
TR              # Trigger
```

---

## Implementation Notes

1. **Envelope Consistency:** All envelopes use `Env.perc(0.001, time/1000, 1, -4)` pattern
2. **Parameter Scaling:** 14-bit range (0-16383) for modulation amounts, direct Hz/ms for time/frequency
3. **ModBus Routing:** Binary 0/1 switches for routing, modulation depth controlled by MB parameter
4. **Stereo Architecture:** Delay and reverb require stereo signal path from amplitude stage onwards
5. **Default Values:** Conservative defaults (filters off, effects at 0% wet) for gradual sound design

---

## Implementation Summary

### COMPLETED (All Phases)

All Tier 1 DSP blocks have been successfully implemented:

**Phase 1: SVF Filter** - COMPLETE
- 7 parameters: FC, FQ, FT, FE, FED, FK, MF.F
- Multi-mode filtering (LP/HP/BP/Notch)
- Envelope modulation with independent decay
- Key tracking to follow pitch
- ModBus routing to cutoff

**Phase 2: Comb Resonator** - COMPLETE
- 4 parameters: RF, RD, RM, RK
- Karplus-Strong algorithm using CombC
- Key tracking for harmonic resonance
- Variable decay time

**Phase 3: Stereo Delay** - COMPLETE
- 5 DSP parameters: DT, DF, DLP, DW, DS (DS not yet implemented for tempo sync)
- 2 routing parameters: D.MODE, D.TAIL
- Ping-pong stereo delay with cubic interpolation
- Feedback lowpass filtering
- Routing modes: BYPASS/INSERT/SEND
- Tail behaviors: CUT/RING/FREEZE

**Phase 4: Plate Reverb** - COMPLETE
- 4 DSP parameters: RV, RP, RH, RW
- 2 routing parameters: R.MODE, R.TAIL
- FreeVerb2 implementation
- Pre-delay and damping control
- Routing modes: BYPASS/INSERT/SEND
- Tail behaviors: CUT/RING/FREEZE

### Parameter Count

- **Total parameters:** 49
  - Oscillator/FM/Mix: 25 parameters
  - DSP blocks: 20 parameters (7 filter + 4 resonator + 5 delay + 4 reverb)
  - Routing: 4 parameters (D.MODE, D.TAIL, R.MODE, R.TAIL)

### Files Modified

- `/Users/why/repos/monokit/sc/monokit_server.scd` - SuperCollider DSP implementation
- `/Users/why/repos/monokit/src/commands/synth_params.rs` - Rust command handlers
- `/Users/why/repos/monokit/src/commands/mod.rs` - Command routing
- `/Users/why/repos/monokit/src/commands/validate.rs` - Parameter validation

### Remaining Work

- **DS (Delay Sync):** Tempo-synced delay divisions (requires BPM passing from metro)
- **Additional ModBus routing:** Delay time, reverb size modulation
- **Documentation updates:** Help text and usage examples

---

## End of Implementation Plan

This document served as the complete roadmap for adding professional-grade DSP effects to Monokit. All planned features have been successfully implemented.
