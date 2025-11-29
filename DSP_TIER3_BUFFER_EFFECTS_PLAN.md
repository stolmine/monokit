# Monokit DSP Tier 3: Buffer-Based Effects Implementation Plan

## STATUS: PLANNING PHASE

This document outlines the implementation plan for Tier 3 DSP effects: Beat Repeat and Pitch Shift. These are advanced buffer-based effects that require careful synchronization with the metro system and precise grain-level DSP.

---

## Architecture Overview

### PROPOSED Signal Flow

The new effects will be inserted after Compressor/Pan, before Delay:

```
Osc → Discontinuity → Lo-Fi → SVF → Ring Mod → Resonator → Amp → Compressor → Pan →
  ┌─────────────────────────┐
  │ BEAT REPEAT (NEW)       │ ← Tier 3
  └───────────┬─────────────┘
              ↓
  ┌─────────────────────────┐
  │ PITCH SHIFT (NEW)       │ ← Tier 3
  └───────────┬─────────────┘
              ↓
→ Delay → EQ → Reverb → Out
```

**Rationale for placement:**
- **After Pan**: We want to capture the full stereo image including panning
- **Before Delay/Reverb**: Beat repeat should not capture delay tails (cleaner rhythmic loops)
- **Pitch Shift after Beat Repeat**: Allows pitch shifting of repeated material

---

## 1. Beat Repeat Effect

### Concept

Beat Repeat captures a segment of incoming audio into a buffer and loops it rhythmically, synchronized to the metro interval. This is inspired by Ableton Live's Beat Repeat and similar stutter/glitch effects.

### Parameters

| Param | SC Arg | Range | Default | Type | Description |
|-------|--------|-------|---------|------|-------------|
| BR.ACT | br_act | 0-1 | 0 | Int | Beat repeat active (0=off, 1=on) |
| BR.LEN | br_len | 0-7 | 2 | Int | Loop length division (see table below) |
| BR.REV | br_rev | 0-1 | 0 | Int | Reverse playback (0=forward, 1=reverse) |
| BR.WIN | br_win | 1-50 | 5 | Int | Crossfade window size (ms) |
| BR.MIX | br_mix | 0-16383 | 8192 | Int | Dry/wet mix (0=dry, 16383=wet) |

**BR.LEN Division Table:**
```
0 = 1/16  (metro_interval / 16)
1 = 1/8   (metro_interval / 8)
2 = 1/4   (metro_interval / 4)  [DEFAULT]
3 = 1/2   (metro_interval / 2)
4 = 1x    (metro_interval)
5 = 2x    (metro_interval * 2)
6 = 4x    (metro_interval * 4)
7 = 8x    (metro_interval * 8)
```

### SuperCollider Implementation

**UGen Choices:**
- **LocalBuf**: For sample storage (avoids global buffer allocation)
- **BufWr**: For writing incoming signal to buffer
- **BufRd**: For reading with cubic interpolation
- **Phasor**: For playback position control
- **EnvGen + Env.sine**: For Hann-window crossfading at loop boundaries

**Implementation Strategy:**

```supercollider
// Add to SynthDef args:
br_act = 0,        // Beat repeat active
br_len = 2,        // Loop length division (0-7)
br_len_ms = 250,   // Loop length in ms (sent from Rust)
br_rev = 0,        // Reverse playback
br_win = 5,        // Crossfade window (ms)
br_mix = 8192,     // Mix amount

// Add to var declarations:
var beatRepeatBuf, brBufSize, brWritePos, brReadPos, brLoopLen;
var brReadRate, brRepeated, brMix, brWindowSize, brCrossfade;
var brActive;

// After Pan (between line 211 and 213):
// Beat Repeat Buffer Processing
brActive = br_act.clip(0, 1);
brLoopLen = br_len_ms / 1000;  // Convert ms to seconds
brBufSize = (brLoopLen * SampleRate.ir).ceil;

// Create local buffer for beat repeat
beatRepeatBuf = LocalBuf(brBufSize.max(4410), 2);  // Min 100ms stereo

// Write incoming signal to buffer (continuous recording)
brWritePos = Phasor.ar(0, 1, 0, brBufSize);
BufWr.ar(sigL, beatRepeatBuf, brWritePos, 0);  // Channel 0
BufWr.ar(sigR, beatRepeatBuf, brWritePos, 1);  // Channel 1

// Read position (loops when beat repeat is active)
// Forward: 0 → brBufSize, Reverse: brBufSize → 0
brReadRate = Select.kr(br_rev.clip(0, 1), [1, -1]);
brReadPos = Phasor.ar(
    brActive,           // Reset trigger when activated
    brReadRate,         // Rate (forward/reverse)
    0,                  // Start
    brBufSize,          // End
    Select.kr(br_rev.clip(0, 1), [0, brBufSize])  // Initial phase
);

// Crossfade window (Hann window at loop boundaries)
brWindowSize = (br_win / 1000 * SampleRate.ir).clip(1, brBufSize * 0.5);
brCrossfade = 1 - (
    min(brReadPos, brBufSize - brReadPos) / brWindowSize
).clip(0, 1).sqrt;  // Smooth fade at edges

// Read from buffer with cubic interpolation
brRepeated = [
    BufRd.ar(1, beatRepeatBuf, brReadPos, 0, 4),  // Left, cubic interp
    BufRd.ar(1, beatRepeatBuf, brReadPos, 1, 4)   // Right, cubic interp
];

// Mix: when inactive, pass through; when active, crossfade
brMix = (br_mix / 16383).clip(0, 1);
sigL = Select.ar(brActive, [
    sigL,                                              // Inactive: pass through
    (sigL * (1 - brMix)) + (brRepeated[0] * brMix)    // Active: mix
]);
sigR = Select.ar(brActive, [
    sigR,
    (sigR * (1 - brMix)) + (brRepeated[1] * brMix)
]);
```

**Metro Interval Synchronization:**

Since SuperCollider is stateless, the Rust metro thread must calculate the loop length in milliseconds and send it via OSC whenever:
1. Metro interval changes (M or M.BPM commands)
2. BR.LEN division changes
3. Beat repeat is activated

Formula (Rust side):
```rust
let loop_length_ms = match br_len {
    0 => metro_interval / 16,  // 1/16
    1 => metro_interval / 8,   // 1/8
    2 => metro_interval / 4,   // 1/4
    3 => metro_interval / 2,   // 1/2
    4 => metro_interval,       // 1x
    5 => metro_interval * 2,   // 2x
    6 => metro_interval * 4,   // 4x
    7 => metro_interval * 8,   // 8x
    _ => metro_interval / 4,   // Default to 1/4
};
```

**OSC Message:**
```
/monokit/param "br_len_ms" <float_value>
```

### Edge Cases and Gotchas

1. **Buffer Size Limits**: LocalBuf is limited by server memory. Maximum practical size: ~10 seconds stereo at 48kHz = ~960,000 samples. At 8x multiplier with metro_interval=1000ms, we need 8 seconds = safe.

2. **Sample Rate Mismatch**: Always use `SampleRate.ir` for buffer size calculation, not hardcoded 48000.

3. **Division by Zero**: When BR.LEN=0 (1/16), ensure metro_interval is at least 16ms to avoid tiny buffers.

4. **Crossfade Artifacts**: Window size (BR.WIN) must be less than half the loop length. Enforce: `brWindowSize.clip(1, brBufSize * 0.5)`.

5. **Clicks on Activation**: Use `Lag.kr(br_act, 0.01)` for smooth activation transitions.

6. **Metro Interval Changes**: When metro interval changes during active repeat, buffer may contain stale data from previous length. This is acceptable (creates interesting glitches) but document it.

---

## 2. Pitch Shift Effect

### Concept

Dual-mode pitch shifting: granular (preserves formants, classic pitch shift) and frequency shift (Bode modulator, metallic/robotic). Can target the main signal or only the beat repeat buffer output.

### Parameters

| Param | SC Arg | Range | Default | Type | Description |
|-------|--------|-------|---------|------|-------------|
| PS.MODE | ps_mode | 0-1 | 0 | Int | Pitch shift mode (0=granular, 1=frequency) |
| PS.SEMI | ps_semi | -24 to +24 | 0 | Int | Semitone shift amount |
| PS.GRAIN | ps_grain | 5-100 | 20 | Int | Grain size for granular mode (ms) |
| PS.MIX | ps_mix | 0-16383 | 0 | Int | Dry/wet mix |
| PS.TARG | ps_targ | 0-1 | 0 | Int | Target (0=main signal, 1=repeat only) |

### SuperCollider Implementation

**UGen Choices:**
- **PitchShift.ar**: Granular time-domain pitch shifter (mode 0)
- **FreqShift.ar**: Single-sideband frequency shifter (mode 1)

**Implementation Strategy:**

```supercollider
// Add to SynthDef args:
ps_mode = 0,       // Mode (0=granular, 1=freq shift)
ps_semi = 0,       // Semitone shift (-24 to +24)
ps_grain = 20,     // Grain size (ms, granular mode only)
ps_mix = 0,        // Mix amount
ps_targ = 0,       // Target (0=main, 1=repeat buffer only)

// Add to var declarations:
var psRatio, psGrainSize, psShifted, psMix;
var psInputL, psInputR;

// After Beat Repeat (insert before Delay):
// Pitch Shift Processing

// Select input source: main signal or just the beat-repeated portion
// For ps_targ=1, we need to isolate the beat repeat component
psInputL = Select.ar(ps_targ.clip(0, 1), [
    sigL,                      // 0: Process full signal (including any beat repeat)
    brRepeated[0] * brMix      // 1: Process only the beat repeat output
]);
psInputR = Select.ar(ps_targ.clip(0, 1), [
    sigR,
    brRepeated[1] * brMix
]);

// Convert semitones to pitch ratio: 2^(semitones/12)
psRatio = (ps_semi / 12).midiratio;  // Built-in conversion

// Grain size in seconds (for granular mode)
psGrainSize = (ps_grain / 1000).clip(0.005, 0.1);

// Apply pitch shifting based on mode
psShifted = Select.ar(ps_mode.clip(0, 1), [
    // Mode 0: Granular PitchShift
    [
        PitchShift.ar(
            psInputL,           // Input
            psGrainSize,        // Window size
            psRatio,            // Pitch ratio
            0,                  // Pitch dispersion (0 = no randomization)
            0.1                 // Time dispersion (slight randomization)
        ),
        PitchShift.ar(
            psInputR,
            psGrainSize,
            psRatio,
            0,
            0.1
        )
    ],
    // Mode 1: Frequency Shift (Bode modulator)
    [
        FreqShift.ar(psInputL, ps_semi * 100),  // Hz offset (100Hz per semitone approx)
        FreqShift.ar(psInputR, ps_semi * 100)
    ]
]);

// Mix dry/wet
psMix = (ps_mix / 16383).clip(0, 1);

// For ps_targ=0: mix shifted signal with dry
// For ps_targ=1: add shifted repeat back to main signal (dry is unaffected)
sigL = Select.ar(ps_targ.clip(0, 1), [
    (sigL * (1 - psMix)) + (psShifted[0] * psMix),    // Main signal mix
    sigL + (psShifted[0] * psMix)                      // Add shifted repeat
]);
sigR = Select.ar(ps_targ.clip(0, 1), [
    (sigR * (1 - psMix)) + (psShifted[1] * psMix),
    sigR + (psShifted[1] * psMix)
]);
```

### Pitch Ratio Calculation

SuperCollider provides `.midiratio` which converts a MIDI note interval to a frequency ratio:
- `semitones.midiratio` = `2.pow(semitones / 12)`
- Example: 12 semitones (octave) = ratio 2.0
- Example: -12 semitones = ratio 0.5

### Edge Cases and Gotchas

1. **PitchShift Artifacts**: Granular pitch shifting introduces metallic artifacts at large shifts (>12 semitones). Document recommended range: ±12 semitones for musical use.

2. **FreqShift Non-Harmonicity**: Frequency shifting destroys harmonic relationships. A major chord becomes dissonant. This is intentional (ring mod-like effect).

3. **Grain Size vs Pitch Accuracy**: Smaller grains (5ms) = better transient response but more artifacts. Larger grains (50ms+) = smoother but smeared transients. Default 20ms is a good balance.

4. **FreqShift Aliasing**: At high semitone values (±24), FreqShift can alias frequencies above Nyquist. Consider warning in documentation.

5. **Target Mode Complexity**: When `ps_targ=1` and beat repeat is inactive, pitch shift has no input. Behavior: silence (acceptable).

6. **CPU Load**: PitchShift is expensive (~5-8% per voice). FreqShift is cheaper (~2%). Document CPU impact.

---

## 3. Rust Command Handlers

### New Parameters to Add

**Beat Repeat (5 handlers):**
```rust
// File: src/commands/synth_params.rs

pub fn handle_br_act() -> Result<()>
// Range: 0-1 (Int)
// Validation: Clip to 0-1
// Also sends br_len_ms when activated

pub fn handle_br_len() -> Result<()>
// Range: 0-7 (Int)
// Validation: Clip to 0-7
// Sends updated br_len_ms to SC

pub fn handle_br_rev() -> Result<()>
// Range: 0-1 (Int)
// Validation: Clip to 0-1

pub fn handle_br_win() -> Result<()>
// Range: 1-50 (Int, ms)
// Validation: Clip to 1-50

pub fn handle_br_mix() -> Result<()>
// Range: 0-16383 (Int)
// Validation: Clip to 0-16383
```

**Pitch Shift (5 handlers):**
```rust
pub fn handle_ps_mode() -> Result<()>
// Range: 0-1 (Int)
// Validation: Clip to 0-1
// Display: 0=GRANULAR, 1=FREQ_SHIFT

pub fn handle_ps_semi() -> Result<()>
// Range: -24 to +24 (Int)
// Validation: Clip to -24..=24

pub fn handle_ps_grain() -> Result<()>
// Range: 5-100 (Int, ms)
// Validation: Clip to 5-100

pub fn handle_ps_mix() -> Result<()>
// Range: 0-16383 (Int)
// Validation: Clip to 0-16383

pub fn handle_ps_targ() -> Result<()>
// Range: 0-1 (Int)
// Validation: Clip to 0-1
// Display: 0=MAIN, 1=REPEAT_ONLY
```

### Metro Interval Synchronization Handler

**New State Variables** (add to main loop or metro thread):
```rust
// In metro thread or main state
let mut br_len_division: usize = 2;  // Default to 1/4
let mut metro_interval: u64 = 500;   // Default 500ms

fn calculate_and_send_br_len(
    metro_interval: u64,
    division: usize,
    metro_tx: &Sender<MetroCommand>
) -> Result<()> {
    let loop_length_ms = match division {
        0 => metro_interval / 16,
        1 => metro_interval / 8,
        2 => metro_interval / 4,
        3 => metro_interval / 2,
        4 => metro_interval,
        5 => metro_interval * 2,
        6 => metro_interval * 4,
        7 => metro_interval * 8,
        _ => metro_interval / 4,
    };

    metro_tx.send(MetroCommand::SendParam(
        "br_len_ms".to_string(),
        OscType::Float(loop_length_ms as f32)
    ))?;

    Ok(())
}
```

**Update Locations:**
1. `handle_m()` in metro_cmds.rs: Call `calculate_and_send_br_len()` after setting interval
2. `handle_m_bpm()`: Same as above
3. `handle_br_len()`: Call with current metro_interval
4. `handle_br_act()`: Call when activating (ensures sync)

### Command Match Cases

Add to `src/commands/mod.rs`:
```rust
"BR.ACT" => {
    synth_params::handle_br_act(&parts, metro_interval, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"BR.LEN" => {
    synth_params::handle_br_len(&parts, metro_interval, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"BR.REV" => {
    synth_params::handle_br_rev(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"BR.WIN" => {
    synth_params::handle_br_win(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"BR.MIX" => {
    synth_params::handle_br_mix(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"PS.MODE" => {
    synth_params::handle_ps_mode(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"PS.SEMI" => {
    synth_params::handle_ps_semi(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"PS.GRAIN" => {
    synth_params::handle_ps_grain(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"PS.MIX" => {
    synth_params::handle_ps_mix(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
"PS.TARG" => {
    synth_params::handle_ps_targ(&parts, variables, patterns, scripts, script_index, metro_tx, *debug_level, output)?;
}
```

---

## 4. OSC Parameter Mappings

### New OSC Messages

```
/monokit/param "br_act" <int>      // 0-1
/monokit/param "br_len" <int>      // 0-7
/monokit/param "br_len_ms" <float> // Calculated by Rust, sent to SC
/monokit/param "br_rev" <int>      // 0-1
/monokit/param "br_win" <int>      // 1-50
/monokit/param "br_mix" <int>      // 0-16383

/monokit/param "ps_mode" <int>     // 0-1
/monokit/param "ps_semi" <int>     // -24 to +24
/monokit/param "ps_grain" <int>    // 5-100
/monokit/param "ps_mix" <int>      // 0-16383
/monokit/param "ps_targ" <int>     // 0-1
```

### Complete Parameter Count (After Tier 3)

**Total parameters: 77**
- Existing (Tier 1+2): 66 parameters
- Tier 3 Beat Repeat: 6 parameters (5 user-facing + 1 internal br_len_ms)
- Tier 3 Pitch Shift: 5 parameters

---

## 5. Testing Scenarios

### Beat Repeat Tests

**Basic Functionality:**
```
# Test 1: Activate beat repeat at 1/4 note (default)
M 500          # 500ms metro (120 BPM quarter note)
BR.ACT 1       # Activate
TR             # Trigger note
# Expected: Signal loops every 125ms (1/4 of 500ms)
```

**Division Tests:**
```
# Test 2: Different divisions
BR.LEN 0       # 1/16 note
# Expected: Very short loop (31.25ms at 500ms metro)

BR.LEN 4       # 1x (full metro interval)
# Expected: 500ms loop

BR.LEN 7       # 8x multiplier
# Expected: 4 second loop
```

**Reverse Playback:**
```
# Test 3: Reverse
BR.REV 1       # Reverse
PF 440         # Sawtooth or pitched sound
PW 2           # Saw wave
TR
# Expected: Backward playback, distinctive reversed sound
```

**Crossfade Window:**
```
# Test 4: Crossfade artifacts
BR.WIN 1       # 1ms window (sharp loop)
# Expected: Click at loop boundary

BR.WIN 20      # 20ms window
# Expected: Smooth loop, no clicks
```

**Mix Control:**
```
# Test 5: Dry/wet balance
BR.MIX 0       # All dry
# Expected: Pass through

BR.MIX 8192    # 50/50 mix
# Expected: Both original and looped signal

BR.MIX 16383   # Full wet
# Expected: Only looped signal
```

**Metro Sync Changes:**
```
# Test 6: Metro interval changes during active repeat
BR.ACT 1
BR.LEN 4       # 1x metro
M 500          # Set to 500ms
TR
M 1000         # Change to 1000ms mid-loop
# Expected: Buffer size doesn't update (contains 500ms of old data)
# Behavior: Glitchy/interesting, document as feature not bug
```

### Pitch Shift Tests

**Granular Mode:**
```
# Test 7: Granular pitch shift up
PS.MODE 0      # Granular
PS.SEMI 12     # Octave up
PS.MIX 16383   # Full wet
PF 220         # A3
TR
# Expected: A4 (440Hz), slight metallic artifacts
```

**Frequency Shift Mode:**
```
# Test 8: Frequency shift (non-harmonic)
PS.MODE 1      # Freq shift
PS.SEMI 12     # ~1200 Hz shift (12 * 100)
TR
# Expected: Inharmonic, robotic/metallic timbre
```

**Grain Size:**
```
# Test 9: Grain size effect on transients
PS.GRAIN 5     # Small grain
TR             # Sharp attack
# Expected: Better transient preservation, more grainy

PS.GRAIN 100   # Large grain
TR
# Expected: Smoother, smeared attack
```

**Target Mode:**
```
# Test 10: Pitch shift beat repeat only
BR.ACT 1
PS.TARG 1      # Target repeat only
PS.SEMI 7      # Fifth up
PS.MIX 16383
TR
# Expected: Dry signal unchanged, beat repeat pitched up
```

**Extreme Shifts:**
```
# Test 11: Extreme pitch shift
PS.SEMI 24     # Two octaves up
# Expected: Granular mode: very metallic, chipmunk-like
#           Freq shift mode: severe aliasing, distorted

PS.SEMI -24    # Two octaves down
# Expected: Granular: dark, muddy
#           Freq shift: aliasing in opposite direction
```

### Combined Effects Tests

**Beat Repeat + Pitch Shift:**
```
# Test 12: Combined stutter and pitch
BR.ACT 1
BR.LEN 1       # 1/8 note
PS.MODE 0
PS.SEMI 5      # Fourth up
PS.TARG 0      # Apply to full signal
PS.MIX 8192    # 50/50
TR
# Expected: Stuttering, half pitched up
```

**Interaction with Delay:**
```
# Test 13: Beat repeat before delay
BR.ACT 1
DT 500         # Delay after beat repeat
DW 8000        # Moderate delay mix
DF 8000        # Feedback
# Expected: Stuttered signal goes into delay
# Delay tails should NOT be captured by beat repeat
```

**Interaction with Reverb:**
```
# Test 14: Clean beat repeat (no tail capture)
RW 12000       # Large reverb
RV 12000       # Long decay
BR.ACT 1
TR
# Expected: Beat repeat buffer is clean (pre-reverb)
# Reverb tails are NOT looped
```

### CPU Performance Tests

```
# Test 15: CPU monitoring
BR.ACT 1       # Beat repeat on
PS.MODE 0      # Granular pitch shift
PS.MIX 16383   # Full wet

# Monitor SC server CPU
# Expected increase:
# - Beat Repeat: ~3-5% (buffer ops + interpolation)
# - Pitch Shift Granular: ~5-8%
# - Pitch Shift Freq: ~2-3%
# Total with both: ~10-15% per voice
```

---

## 6. Implementation Phases

### Phase 1: Beat Repeat Core (3-4 hours)

1. Add beat repeat parameters to SynthDef
2. Implement LocalBuf buffer allocation
3. Add BufWr continuous recording
4. Implement Phasor-based playback
5. Add forward/reverse switching
6. Test basic looping (before metro sync)

**Deliverable:** Beat repeat works with hardcoded loop length

### Phase 2: Metro Sync Integration (2-3 hours)

1. Add `br_len_ms` parameter to SynthDef
2. Implement `calculate_and_send_br_len()` in Rust
3. Hook into M and M.BPM handlers
4. Add BR.LEN command with recalculation
5. Test tempo sync with various divisions

**Deliverable:** Beat repeat synced to metro interval

### Phase 3: Beat Repeat Crossfade (1-2 hours)

1. Implement Hann window calculation
2. Add crossfade at loop boundaries
3. Add BR.WIN parameter
4. Test click-free looping

**Deliverable:** Smooth, click-free beat repeat

### Phase 4: Pitch Shift Core (2-3 hours)

1. Add pitch shift parameters to SynthDef
2. Implement granular mode (PitchShift.ar)
3. Implement frequency shift mode (FreqShift.ar)
4. Add semitone to ratio conversion
5. Add mode switching
6. Test both modes independently

**Deliverable:** Dual-mode pitch shifting

### Phase 5: Pitch Shift Target Mode (1-2 hours)

1. Expose beat repeat buffer output as variable
2. Implement PS.TARG signal routing
3. Add dry/wet mixing for both target modes
4. Test interactions with beat repeat

**Deliverable:** Pitch shift can target beat repeat buffer

### Phase 6: Rust Commands (3-4 hours)

1. Add 10 command handlers (5 beat repeat + 5 pitch shift)
2. Add 10 match cases in mod.rs
3. Add validation rules in validate.rs
4. Implement metro sync recalculation logic
5. Test all commands from TUI

**Deliverable:** Full command interface in Rust

### Phase 7: Testing & Documentation (2-3 hours)

1. Run all 15 test scenarios
2. Measure CPU performance
3. Document edge cases and limitations
4. Update help text
5. Create usage examples

**Deliverable:** Production-ready Tier 3 effects

**Total Estimated Time:** 14-21 hours

---

## 7. Files to Modify

### SuperCollider
- `/Users/why/repos/monokit/sc/monokit_server.scd`
  - Add 11 parameters to SynthDef args
  - Add ~60 lines of beat repeat implementation
  - Add ~40 lines of pitch shift implementation
  - Insert between Pan and Delay sections

### Rust
- `/Users/why/repos/monokit/src/commands/synth_params.rs`
  - Add 10 new command handlers (~200 lines)

- `/Users/why/repos/monokit/src/commands/mod.rs`
  - Add 10 new match cases (~30 lines)

- `/Users/why/repos/monokit/src/commands/metro_cmds.rs`
  - Add `calculate_and_send_br_len()` function (~20 lines)
  - Update `handle_m()` to call sync function (~3 lines)
  - Update `handle_m_bpm()` to call sync function (~3 lines)

- `/Users/why/repos/monokit/src/commands/validate.rs`
  - Add 10 validation rules (~30 lines)

**Total Files:** 4
**Total Lines of Code:** ~390 lines (SC: ~100, Rust: ~290)

---

## 8. Technical Deep Dive

### Beat Repeat: Buffer Management

**LocalBuf vs Global Buffer:**

We use `LocalBuf` instead of global `Buffer.alloc` because:
1. **No async allocation**: LocalBuf is synth-local, no server communication
2. **Automatic cleanup**: Freed when synth is freed
3. **No buffer number management**: No risk of buffer ID conflicts
4. **Deterministic timing**: No waiting for /b_alloc done message

**Buffer Size Calculation:**

```
bufSize = (loop_length_seconds * sample_rate)
```

At maximum division (8x) and metro interval (e.g., 2000ms for slow tempo):
```
8 * 2.0 seconds = 16 seconds
16 * 48000 samples = 768,000 samples per channel
Stereo: 1,536,000 samples total
Memory: ~6 MB (at 32-bit float)
```

This is well within LocalBuf limits (typically 100+ MB available).

**Write/Read Pointer Phase Relationship:**

```
Write pointer: continuous, wraps at buffer size
Read pointer: resets when br_act triggers, loops independently
Phase offset: no fixed relationship (creates "capture" effect)
```

When beat repeat activates, the read pointer captures whatever is currently in the buffer (last N milliseconds of audio).

### Beat Repeat: Crossfade Windowing

**Hann Window (Raised Cosine):**

Traditional formula: `w(n) = 0.5 * (1 - cos(2π * n / N))`

Our implementation uses distance from loop edges:
```supercollider
distance_from_edge = min(position, bufSize - position)
fade_factor = (distance_from_edge / windowSize).clip(0, 1)
crossfade = 1 - fade_factor.sqrt  // sqrt for equal-power
```

This creates smooth fade-in at start and fade-out at end of loop.

**COLA (Constant Overlap-Add) Property:**

Hann window with 50% overlap has perfect COLA property: overlapping windows sum to unity. This ensures no amplitude modulation at loop boundaries.

Research sources:
- [COLA Examples](https://ccrma.stanford.edu/~jos/sasp/COLA_Examples.html)
- [Choosing the right overlap for a window function](https://dsp.stackexchange.com/questions/13436/choosing-the-right-overlap-for-a-window-function)

### Pitch Shift: Granular vs Frequency Shift

**Granular Pitch Shift (PitchShift.ar):**

Algorithm: Time-domain granular synthesis with variable playback rate
- Grain duration: 5-100ms (adjustable via PS.GRAIN)
- Overlap: 4:1 (75% overlap, hardcoded in PitchShift)
- Window: Triangular envelope (hardcoded)
- Interpolation: Linear (hardcoded)

Formant preservation: Achieved by maintaining grain size while changing playback rate. Higher pitch = faster playback through same-sized grains = formants relatively preserved.

Limitations:
- Metallic artifacts at large shifts (>±12 semitones)
- Formant preservation not perfect (not true vocoder)
- Time dispersion parameter adds slight randomness (reduces periodicity artifacts)

**Frequency Shift (FreqShift.ar):**

Algorithm: Single-sideband modulation (Hilbert transform + quadrature modulation)
- All frequency components shifted by fixed Hz amount
- Harmonic relationships destroyed
- Result: inharmonic, metallic, "ring mod" character

Example:
```
Input: 100Hz, 200Hz, 300Hz (harmonic series)
Shift: +700 Hz
Output: 800Hz, 900Hz, 1000Hz (no longer harmonic)
```

This is intentional for creative effects (robotization, chorusing, etc).

Research sources:
- [PitchShift SuperCollider Help](https://doc.sccode.org/Classes/PitchShift.html)
- [FreqShift SuperCollider Help](https://doc.sccode.org/Classes/FreqShift.html)
- [Implementing a Pitch Shifter in SuperCollider](https://reading.supply/@ben/implementing-a-pitch-shifter-in-supercollider-Z0fcAX)

### Pitch Shift: Semitone to Ratio Conversion

Musical pitch is exponential: each octave doubles frequency.

Formula: `ratio = 2^(semitones / 12)`

SuperCollider provides `.midiratio` which implements this:
```supercollider
12.midiratio      // = 2.0 (octave up)
-12.midiratio     // = 0.5 (octave down)
7.midiratio       // = 1.498 (perfect fifth up)
```

For FreqShift mode, we approximate with linear Hz offset:
```supercollider
ps_semi * 100     // 100 Hz per "semitone"
```

This is NOT musically accurate (frequency shift is non-harmonic by design) but provides a useful parameter range.

### Buffer-Based Effects: CPU Considerations

**CPU Cost Breakdown:**

Beat Repeat:
- LocalBuf allocation: negligible (one-time)
- BufWr (continuous writing): ~1-2%
- BufRd with cubic interpolation: ~2-3%
- Phasor and windowing math: ~1%
- **Total: ~4-6% per voice**

Pitch Shift (Granular):
- PitchShift.ar internal grain synthesis: ~5-8%
- Varies with grain size (smaller = more expensive)
- **Total: ~5-8% per voice**

Pitch Shift (Frequency):
- FreqShift.ar (Hilbert + modulation): ~2-3%
- **Total: ~2-3% per voice**

Combined maximum (both effects active, granular mode): ~14% per voice

Research sources:
- [Granular Synthesis Module](https://documentation.dspconcepts.com/awe-designer/8.D.2.6/granular-synthesis-module)
- [DSP Labs Granular Synthesis](https://lcav.gitbook.io/dsp-labs/granular-synthesis/implementation)

---

## 9. Example Usage Patterns

### Stutter Effect (Classic Beat Repeat)
```
M.BPM 128      # Set tempo
BR.LEN 1       # 1/8 note
BR.MIX 16383   # Full wet
BR.ACT 1       # Activate
PF 440
TR
# Expected: Classic stutter/glitch effect
```

### Reverse Stutter
```
BR.LEN 2       # 1/4 note
BR.REV 1       # Reverse
BR.WIN 10      # 10ms crossfade
BR.ACT 1
TR
# Expected: Reversed stuttering
```

### Pitched-Up Beat Repeat
```
BR.ACT 1
BR.LEN 3       # 1/2 note
PS.TARG 1      # Target beat repeat only
PS.MODE 0      # Granular
PS.SEMI 12     # Octave up
PS.MIX 16383   # Full wet
TR
# Expected: Dry signal normal, beat repeat pitched up
```

### Frequency-Shifted Glitch
```
BR.ACT 1
BR.LEN 0       # 1/16 note (very short)
PS.MODE 1      # Frequency shift
PS.SEMI 7      # ~700 Hz shift
PS.TARG 0      # Full signal
PS.MIX 8192    # 50/50 mix
TR
# Expected: Glitchy, inharmonic, metallic texture
```

### Long Loop with Granular Time-Stretch Effect
```
M 2000         # Slow metro (30 BPM quarter note)
BR.LEN 7       # 8x = 16 second loop
BR.MIX 12000   # 73% wet
PS.GRAIN 80    # Large grain size
PS.SEMI -5     # Fourth down
PS.MIX 8192    # 50/50
TR
# Expected: Long, evolving, pitched-down texture
```

---

## 10. Known Limitations & Future Enhancements

### Current Limitations

1. **No Probability/Randomization**: Beat repeat always loops. No stutter probability parameter (yet).

2. **Fixed Grain Overlap**: PitchShift.ar uses hardcoded 4:1 overlap. Cannot adjust for different textures.

3. **No Gate/Envelope Sync**: Beat repeat doesn't retrigger envelopes. It loops the audio buffer, not the note.

4. **Single Buffer**: Only one beat repeat buffer per voice. Cannot have multiple simultaneous loops.

5. **No Tempo Map**: Rust metro is simple interval-based. No support for tempo curves or swing.

6. **Linear Frequency Shift Mapping**: PS.SEMI in freq shift mode is approximate (100 Hz/semitone).

### Future Enhancements (Post-Tier 3)

1. **Beat Repeat Probability** (`BR.PROB`): 0-100% chance of repeat triggering on metro tick.

2. **Gate Retrigger** (`BR.GATE`): Optionally retrigger note envelopes on loop.

3. **Multi-Buffer Slots** (`BR.SLOT`): 2-4 independent beat repeat buffers with switching.

4. **Stutter Fade-Out** (`BR.FADE`): Auto-fade repeat when deactivating (vs sudden stop).

5. **Pitch Shift Formant Control**: Add true formant preservation (requires spectral processing or PV_* UGens).

6. **Tempo-Synced Grain Size**: Lock PS.GRAIN to metro divisions.

7. **Dual Pitch Shift**: Two parallel pitch shifters for detuned chorus effect.

8. **Buffer Freeze**: Separate from beat repeat, infinite sustain of current buffer (granular cloud).

---

## 11. Research References

### Beat Repeat & Buffer Techniques
- [PlayBuf 2017](https://github.com/supercollider/supercollider/wiki/PlayBuf-2017) - SuperCollider buffer playback discussion
- [LocalBuf](https://depts.washington.edu/dxscdoc/Help/Classes/LocalBuf.html) - LocalBuf documentation
- [Stutter tutorial](https://sccode.org/1-50T) - SuperCollider stutter effect example
- [Buffer crossfader](https://fredrikolofsson.com/f0blog/buffer-xfader/) - Crossfading technique

### Pitch Shifting
- [PitchShift](https://doc.sccode.org/Classes/PitchShift.html) - PitchShift UGen reference
- [FreqShift](https://doc.sccode.org/Classes/FreqShift.html) - FreqShift UGen reference
- [Implementing a Pitch Shifter in SuperCollider](https://reading.supply/@ben/implementing-a-pitch-shifter-in-supercollider-Z0fcAX) - Implementation guide

### Granular Synthesis
- [Granular Synthesis Module](https://documentation.dspconcepts.com/awe-designer/8.D.2.6/granular-synthesis-module) - Grain size and overlap
- [DSP Labs Granular Synthesis](https://lcav.gitbook.io/dsp-labs/granular-synthesis/implementation) - Implementation details
- [Granular synthesis](https://en.wikipedia.org/wiki/Granular_synthesis) - Theory overview

### Windowing & Crossfading
- [COLA Examples](https://ccrma.stanford.edu/~jos/sasp/COLA_Examples.html) - Constant overlap-add windowing
- [Choosing the right overlap](https://dsp.stackexchange.com/questions/13436/choosing-the-right-overlap-for-a-window-function) - Window overlap discussion
- [Hanning Window](https://www.sciencedirect.com/topics/engineering/hanning-window) - Hann window characteristics

### Tempo Sync
- [TempoClock](https://doc.sccode.org/Classes/TempoClock.html) - SuperCollider tempo clock
- [Pattern Guide Cookbook 05](https://doc.sccode.org/Tutorials/A-Practical-Guide/PG_Cookbook05_Using_Samples.html) - Tempo-synced sample playback

---

## 12. Parameter Summary Table

| Effect | Params | SC Args | Rust Handlers | Match Cases | Total LoC (SC) | Total LoC (Rust) |
|--------|--------|---------|---------------|-------------|----------------|------------------|
| Beat Repeat | 6 | br_act, br_len, br_len_ms, br_rev, br_win, br_mix | 5 | 5 | ~60 | ~150 |
| Pitch Shift | 5 | ps_mode, ps_semi, ps_grain, ps_mix, ps_targ | 5 | 5 | ~40 | ~140 |
| **TOTALS** | **11** | **11** | **10** | **10** | **~100** | **~290** |

---

## 13. Validation Rules

Add to `/Users/why/repos/monokit/src/commands/validate.rs`:

```rust
// Beat repeat commands
"BR.ACT" => parts.len() >= 2,
"BR.LEN" => parts.len() >= 2,
"BR.REV" => parts.len() >= 2,
"BR.WIN" => parts.len() >= 2,
"BR.MIX" => parts.len() >= 2,

// Pitch shift commands
"PS.MODE" => parts.len() >= 2,
"PS.SEMI" => parts.len() >= 2,
"PS.GRAIN" => parts.len() >= 2,
"PS.MIX" => parts.len() >= 2,
"PS.TARG" => parts.len() >= 2,
```

---

## 14. Testing Checklist

### Beat Repeat Tests
- [ ] BR.ACT activation/deactivation (smooth transitions)
- [ ] BR.LEN all divisions (0-7) sync correctly to metro
- [ ] BR.REV forward/reverse switching (no pops)
- [ ] BR.WIN crossfade prevents clicks (1ms vs 20ms)
- [ ] BR.MIX dry/wet balance (0, 8192, 16383)
- [ ] Metro interval changes during active repeat (glitch behavior acceptable)
- [ ] Extreme metro intervals (50ms, 5000ms) don't crash
- [ ] Buffer doesn't capture delay/reverb tails

### Pitch Shift Tests
- [ ] PS.MODE switching between granular and freq shift
- [ ] PS.SEMI range -24 to +24 (test extremes)
- [ ] PS.GRAIN size effect (5ms vs 100ms) on transients
- [ ] PS.MIX dry/wet balance
- [ ] PS.TARG mode 0 (main signal) vs 1 (repeat only)
- [ ] Freq shift inharmonicity (verify non-musical behavior)
- [ ] Granular artifacts at ±12 semitones (document)
- [ ] CPU usage within expected range (~14% combined max)

### Integration Tests
- [ ] Beat repeat + pitch shift combined
- [ ] Beat repeat → pitch shift → delay signal flow
- [ ] PS.TARG=1 with BR.ACT=0 (silence, expected)
- [ ] PS.TARG=1 with BR.ACT=1 (only repeat pitched)
- [ ] Multiple rapid parameter changes (stability)
- [ ] Pattern/script automation of all parameters

---

## End of Implementation Plan

This plan provides a complete roadmap for implementing Tier 3 buffer-based effects in Monokit. The beat repeat and pitch shift effects leverage SuperCollider's built-in UGens while maintaining tight integration with the Rust metro system for tempo synchronization. All research has been conducted using authoritative sources, and the implementation strategy balances audio quality, CPU efficiency, and creative flexibility.

**Key Innovations:**
1. Metro-synced beat repeat with 8 division options
2. Dual-mode pitch shifting (musical + inharmonic)
3. Target routing (pitch shift main signal OR beat repeat buffer)
4. Smooth crossfading for click-free looping
5. Efficient LocalBuf management

**Next Steps:**
1. Review this plan for technical accuracy
2. Begin Phase 1: Beat Repeat Core implementation
3. Iteratively test each phase before proceeding
4. Document any deviations from plan during implementation
