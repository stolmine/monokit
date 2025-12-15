# MiClouds Granular Effect Integration

Design document for adding MiClouds (Mutable Instruments Clouds) as a global effect.

**Status:** Planning
**Complexity:** [Low-Medium] - 5-6 days
**Dependencies:** mi-UGens already bundled (MiClouds.scx included with MiPlaits)

---

## Overview

Add MiClouds granular processor as a global effect in the monokit signal chain. MiClouds creates textures and soundscapes by combining multiple overlapping, delayed, transposed and enveloped segments of audio (grains) captured from a recording buffer.

**Key difference from other effects:** MiClouds continuously records incoming audio into a buffer. Triggers (CL.TRIG) play back grains from this continuously-updating buffer.

---

## MiClouds.ar Parameters

Based on mi-UGens documentation:

| Parameter | Range | Description |
|-----------|-------|-------------|
| `pit` | 0.0-1.0 | Pitch transposition of grains |
| `pos` | 0.0-1.0 | Position in recording buffer (grain source) |
| `size` | 0.0-1.0 | Duration of individual grains |
| `dens` | 0.0-1.0 | Frequency at which grains are generated |
| `tex` | 0.0-1.0 | Envelope shape variations for grains |
| `drywet` | 0.0-1.0 | Balance between processed and dry signal |
| `in_gain` | 0.0-2.0+ | Input signal strength |
| `spread` | 0.0-1.0 | Stereo width distribution |
| `rvb` | 0.0-1.0 | Internal reverb contribution |
| `fb` | 0.0-1.0 | Feedback amount (>0.3 risky!) |
| `freeze` | 0/1 | Halts buffer recording, grains from frozen audio |
| `mode` | 0-3 | Processing mode (grain/pitch/loop/spectral) |
| `lofi` | 0.0-1.0 | Sample rate/bit depth reduction |
| **`trig`** | **gate** | **Trigger to play back grains from buffer** |

---

## Effects Chain Placement

**Recommended position: Pre-delay (after compressor/pan, before beat repeat)**

```
Signal Flow:
  Compressor → Pan → MiClouds → Beat Repeat → Pitch Shift →
  Delay → EQ → Reverb → Output
```

**Rationale:**
- Clouds textures can be further processed by beat repeat/delay
- Traditional send/return style (reverb last)
- Allows beat repeat to work on clouded output
- Clouds acts as early texture generator

---

## SynthDef Integration

### Parameter Additions

Add to monokit_main SynthDef after pan, before beat repeat:

```supercollider
SynthDef(\monokit_main, {
    arg // ... existing params ...

        // MiClouds parameters
        t_cl_trig = 0,    // Trigger to play back grains
        cl_pitch = 8192,  // Pitch (0-16383, 8192=center/no transpose)
        cl_pos = 8192,    // Buffer position (0-16383)
        cl_size = 8192,   // Grain size (0-16383)
        cl_dens = 8192,   // Density (0-16383)
        cl_tex = 8192,    // Texture (0-16383)
        cl_wet = 0,       // Wet mix (0-16383)
        cl_gain = 8192,   // Input gain (0-16383, 8192=1.0)
        cl_spread = 8192, // Stereo spread (0-16383)
        cl_rvb = 0,       // Internal reverb (0-16383)
        cl_fb = 0,        // Feedback (0-16383)
        cl_freeze = 0,    // Freeze (0/1)
        cl_mode = 0,      // Mode (0-3)
        cl_lofi = 0;      // Lo-fi (0-16383)

    var // ... existing vars ...
        cloudsL, cloudsR, cloudsActive, cloudsWet;

    // ... existing signal processing ...

    // Pan control (existing)
    panPos = (panSmooth / 16383).clip(-1, 1);
    #sigL, sigR = Pan2.ar(sig, panPos);

    // MiClouds Granular Processing
    cloudsWet = Lag.kr(cl_wet / 16383, 0.01);
    cloudsActive = cloudsWet > 0;

    #cloudsL, cloudsR = Select.ar(cloudsActive, [
        [sigL, sigR],  // Bypass when wet=0
        MiClouds.ar(
            [sigL, sigR],  // inputArray - audio input as array
            pit: (cl_pitch / 16383).clip(0, 1),
            pos: (cl_pos / 16383).clip(0, 1),
            size: (cl_size / 16383).clip(0, 1),
            dens: (cl_dens / 16383).clip(0, 1),
            tex: (cl_tex / 16383).clip(0, 1),
            drywet: cloudsWet,
            in_gain: (cl_gain / 16383).clip(0, 2),
            spread: (cl_spread / 16383).clip(0, 1),
            rvb: (cl_rvb / 16383).clip(0, 1),
            fb: (cl_fb / 16383).clip(0, 1),
            freeze: cl_freeze.clip(0, 1),
            mode: cl_mode.clip(0, 3).round,
            lofi: (cl_lofi / 16383).clip(0, 1),
            trig: t_cl_trig  // Trigger input for grain playback
        )
    ]);

    sigL = cloudsL;
    sigR = cloudsR;

    // Beat Repeat (existing - continues from here)
    // ...
}).add;
```

---

## Rust Command Layer

### New File: `src/commands/synth/effects/clouds.rs`

```rust
// MiClouds granular effect commands

use crate::commands::context::ExecutionContext;
use crate::osc::OscType;

// Trigger command
pub fn handle_clouds_trigger(
    ctx: &mut ExecutionContext,
    output: &mut Vec<String>,
) -> Result<(), String> {
    ctx.sender.send(("/n_set", vec![
        OscType::Int(1006),  // monokit_main node
        OscType::String("t_cl_trig".to_string()),
        OscType::Int(1),
    ]))?;

    ctx.output(OutputCategory::Confirm, "GRAINS TRIGGERED".to_string(), output);
    Ok(())
}

// Parameter commands (using standard int parameter macro)
// CL.PITCH, CL.POS, CL.SIZE, etc. - 0-16383 range
```

### Commands & Aliases

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| CL.TRIG | CLTR | - | Trigger grain playback from buffer |
| CL.PITCH | CLP, CLPT | 0-16383 | Grain pitch (8192=no transpose) |
| CL.POS | CLO, CLPS | 0-16383 | Buffer position for grain source |
| CL.SIZE | CLS, CLSZ | 0-16383 | Grain duration |
| CL.DENS | CLD, CLDS | 0-16383 | Grain generation frequency |
| CL.TEX | CLT, CLTX | 0-16383 | Grain envelope variation |
| CL.WET | CLW | 0-16383 | Wet/dry mix |
| CL.GAIN | CLG | 0-16383 | Input gain (8192=unity) |
| CL.SPREAD | CLSP | 0-16383 | Stereo spread |
| CL.RVB | CLR, CLRV | 0-16383 | Internal reverb |
| CL.FB | CLF | 0-16383 | Feedback (>10000 risky!) |
| CL.FREEZE | CLFZ | 0/1 | Freeze buffer recording |
| CL.MODE | CLM | 0-3 | 0=grain 1=pitch 2=loop 3=spectral |
| CL.LOFI | CLLO | 0-16383 | Lo-fi sample rate reduction |

### Trigger Behavior

**CL.TRIG (t_gate parameter):**
- Triggers grain playback from the continuously-recording buffer
- Buffer is always recording input audio (~1 second loop)
- Each trigger generates new grains based on current parameters
- Essential for Clouds to produce output
- Can be triggered rhythmically via Metro scripts
- Works with pattern system (IF ER 3 8 I: CL.TRIG)

**Freeze mode interaction:**
- When CL.FREEZE 0: Buffer continuously records, grains play from live audio
- When CL.FREEZE 1: Buffer stops recording (frozen), grains play from frozen audio
- Freeze + varying CL.POS = scan through different positions in frozen buffer

---

## Processing Modes (CL.MODE)

**Mode 0: Granular (default)**
- Classic granular synthesis
- Best for: Textures, pads, drones, atmospheres
- CPU: Moderate

**Mode 1: Pitch Shifter/Time Stretch**
- More deterministic pitch shifting
- Best for: Melodic shifts, formant preservation
- Alternative to built-in pitch shift effect
- CPU: Moderate-High

**Mode 2: Looping Delay**
- Rhythmic delay-like behavior
- Grains sync to create repeating patterns
- Best for: Rhythmic textures, delay-like effects
- CPU: Moderate

**Mode 3: Spectral Madness**
- FFT-based spectral processing
- Best for: Experimental sounds, glitches, drones
- CPU: High (most intensive mode)

---

## Integration with Existing Systems

### Pattern System

```
# Rhythmic grain triggering
CL.WET 8192
CL.SIZE 4000
IF ER 5 8 I: CL.TRIG    # Euclidean rhythm triggers grains

# Scanning through frozen buffer
CL.FREEZE 1
CL.POS P.NEXT           # Pattern controls buffer position
CL.TRIG                 # Generate grains at each position
```

### Metro Scripts

```
# Metro: Continuous grain generation
M1: CL.TRIG
M2: CL.POS RND 0 16383  # Random buffer position

# Init: Setup Clouds texture
I1: CL.WET 8192
I2: CL.SIZE 6000
I3: CL.DENS 10000
I4: CL.MODE 0
```

### Variables & Expressions

```
# Store grain size in variable
A RND 2000 12000
CL.SIZE A
CL.TRIG

# Conditional processing
IF GT A 8000: CL.MODE 0   # Granular for large grains
IF LT A 8000: CL.MODE 3   # Spectral for small grains
CL.TRIG
```

### RND.FX Integration

```rust
// Add to RND.FX handler
set_param("cl_pitch", rng.gen_range(0..16383));
set_param("cl_pos", rng.gen_range(0..16383));
set_param("cl_size", rng.gen_range(4096..12288));  // Bias toward medium
set_param("cl_dens", rng.gen_range(4096..12288));
set_param("cl_tex", rng.gen_range(0..16383));
set_param("cl_wet", rng.gen_range(0..8192));       // Max 50% wet
set_param("cl_gain", 8192);                        // Keep at unity
set_param("cl_spread", rng.gen_range(0..16383));
set_param("cl_rvb", rng.gen_range(0..8192));       // Max 50% reverb
set_param("cl_fb", rng.gen_range(0..6553));        // Max ~40% to be safe
set_param("cl_freeze", 0);                         // Don't randomize freeze
set_param("cl_mode", rng.gen_range(0..4));
set_param("cl_lofi", rng.gen_range(0..4096));      // Light lo-fi
// Note: RND.FX doesn't trigger, user must CL.TRIG manually
```

### RST Command Coverage

```rust
// Add to RST handler
set_param("cl_pitch", 8192);   // Center/no transpose
set_param("cl_pos", 8192);     // Middle of buffer
set_param("cl_size", 8192);    // Medium grain size
set_param("cl_dens", 8192);    // Medium density
set_param("cl_tex", 8192);     // Neutral envelope
set_param("cl_wet", 0);        // Bypassed
set_param("cl_gain", 8192);    // Unity gain
set_param("cl_spread", 8192);  // Medium spread
set_param("cl_rvb", 0);        // No reverb
set_param("cl_fb", 0);         // No feedback
set_param("cl_freeze", 0);     // Not frozen
set_param("cl_mode", 0);       // Granular mode
set_param("cl_lofi", 0);       // No lo-fi
```

---

## Scene Persistence

### Scene JSON Addition

```json
{
  "clouds": {
    "pitch": 8192,
    "pos": 8192,
    "size": 8192,
    "dens": 8192,
    "tex": 8192,
    "wet": 0,
    "gain": 8192,
    "spread": 8192,
    "rvb": 0,
    "fb": 0,
    "freeze": 0,
    "mode": 0,
    "lofi": 0
  }
}
```

**Note:** CL.TRIG is not persisted (it's a trigger, not a state parameter)

---

## Performance Considerations

### CPU Usage

**Estimated CPU cost by mode:**
- Mode 0 (Granular): 5-8%
- Mode 1 (Pitch): 7-10%
- Mode 2 (Loop): 6-9%
- Mode 3 (Spectral): 10-15% (most intensive)

**Density impact:**
- Low density (0-5000): Minimal additional cost
- Medium density (5000-11000): Moderate cost
- High density (11000-16383): Maximum cost (more grains = more CPU)

**Optimization:**
- When CL.WET = 0, effect fully bypassed (no CPU cost)
- Consider using lower densities for real-time performance
- Mode 3 (spectral) should be used sparingly

### Memory Usage

- Internal buffer: ~88KB per instance (1 second at 44.1kHz stereo)
- Automatically allocated by MiClouds UGen
- No manual buffer management required
- Fixed size, not configurable

### Integration Challenges

**1. Feedback Warning**
- CL.FB > 10000 (~60%) can cause runaway gain/self-oscillation
- Add validation warning: "CL.FB >10000 may self-oscillate!"
- Consider clamping max to 13107 (~80%) for safety
- Help system should note this clearly

**2. Internal Reverb vs Plate Reverb**
- Clouds has internal reverb (CL.RVB)
- Using both CL.RVB and main reverb (RV) can muddy the mix
- Recommendation: Use one or the other, not both maxed
- CL.RVB is lighter/faster, plate reverb is deeper/richer

**3. Freeze + Delay Feedback**
- CL.FREEZE 1 + delay feedback can create infinite textures
- Interesting but can overwhelm the mix
- Document this interaction in manual

**4. Grain Trigger Timing**
- Unlike TR (which triggers voices), CL.TRIG triggers grain playback
- Buffer is always recording (unless frozen), so timing of triggers controls grain density
- In freeze mode, buffer is static (frozen buffer scanning)
- In normal mode, buffer continuously updates with live audio

---

## Implementation Phases

### Phase 1: Basic Integration [~2 days]

**SuperCollider:**
- [ ] Add 14 parameters to monokit_main SynthDef (13 + t_cl_trig)
- [ ] Integrate MiClouds.ar after pan, before beat repeat
- [ ] Verify mi-UGens bundle includes MiClouds.scx
- [ ] Test bypass behavior (CL.WET 0)

**Testing:**
- [ ] Manual test: CL.WET 8192, CL.TRIG
- [ ] Verify grain generation
- [ ] Test mode switching (0-3)

**Deliverable:** MiClouds active in signal chain with trigger support

### Phase 2: Command Implementation [~2 days]

**Rust:**
- [ ] Create `src/commands/synth/effects/clouds.rs`
- [ ] Implement CL.TRIG command (t_gate trigger)
- [ ] Implement 13 parameter commands (CL.PITCH, CL.POS, etc.)
- [ ] Add parameter validation and range clipping
- [ ] Register all commands in mod.rs
- [ ] Add short-form aliases (CLTR, CLP, CLO, etc.)
- [ ] Expression support for all parameters

**Testing:**
- [ ] Test each command individually
- [ ] Test expression evaluation (CL.POS RND 0 16383)
- [ ] Verify trigger confirmation output

**Deliverable:** All Clouds parameters controllable from REPL

### Phase 3: Integration & Polish [~1 day]

**Features:**
- [ ] Add to RND.FX randomization
- [ ] Add to RST defaults
- [ ] Scene save/load support
- [ ] Help system documentation (all commands)
- [ ] Feedback warning (CL.FB >10000)

**Testing:**
- [ ] Pattern integration (CL.TRIG with ER, PROB, etc.)
- [ ] Metro script triggering
- [ ] Variable usage (A CL.SIZE, etc.)
- [ ] Scene load/save with Clouds state

**Deliverable:** Full integration with existing systems

### Phase 4: Documentation & Examples [~1 day]

**Documentation:**
- [ ] Update MANUAL.md with Clouds section
- [ ] Add to COMMAND_REFERENCE.md
- [ ] Update ARCHITECTURE.md signal flow diagram
- [ ] Document all 4 modes with use cases
- [ ] CPU performance characteristics
- [ ] Effect interaction notes (reverb, delay, freeze)

**Example Scenes:**
- [ ] Granular texture generation
- [ ] Pitch shimmer effect
- [ ] Rhythmic freeze scanning
- [ ] Spectral experimentation

**Deliverable:** Complete documentation and examples

---

## Total Effort Estimate: 5-6 days [Low-Medium]

**Breakdown:**
- SuperCollider integration: 2 days
- Rust command layer: 2 days
- Integration & polish: 1 day
- Documentation: 1 day

**Why relatively quick:**
- ✓ mi-UGens already bundled (MiClouds.scx included)
- ✓ No complex buffer management (automatic)
- ✓ Standard parameter pattern (14 params, 0-16383 range)
- ✓ Fits cleanly into existing effects chain
- ✓ Similar to beat repeat/pitch shift implementation
- ✓ Well-documented UGen with clear parameters

---

## Creative Use Cases & Examples

### Example 1: Texture Generation

```
# Setup granular texture
CL.MODE 0        # Granular mode
CL.SIZE 4000     # Small grains
CL.DENS 12000    # Dense cloud
CL.TEX 10000     # Varied envelopes
CL.WET 8192      # 50/50 mix

# Trigger to create evolving texture
TR               # Trigger voice
CL.TRIG          # Play back grains from buffer
```

### Example 2: Pitch Shimmer

```
# Shimmer pad effect
CL.MODE 1        # Pitch shift mode
CL.PITCH 12000   # Up ~3 semitones
CL.SIZE 8000     # Medium grains
CL.RVB 8000      # Internal reverb for wash
CL.WET 6000      # Blend with dry

# Metro triggers grains
M1: CL.TRIG
```

### Example 3: Rhythmic Freeze Scanning

```
# Freeze buffer and scan
CL.FREEZE 1      # Capture current audio
CL.SIZE 2000     # Tiny grains for glitchy sound
CL.WET 16383     # 100% wet

# Pattern-based buffer scanning
P0 0 4000 8000 12000 16383
CL.POS P.NEXT
CL.TRIG          # Trigger grains at each position
```

### Example 4: Euclidean Grain Rhythm

```
# Setup
CL.MODE 0
CL.SIZE 6000
CL.DENS 8000
CL.WET 10000

# Euclidean rhythm triggers grains
IF ER 5 8 I: CL.TRIG
```

### Example 5: Spectral Feedback Madness

```
# Experimental mode
CL.MODE 3        # Spectral processing
CL.FB 8000       # High feedback (careful!)
CL.LOFI 6000     # Lo-fi crush for grit
CL.DENS 16383    # Maximum density
CL.WET 12000     # Heavy wet mix

# Random triggering
PROB 50: CL.TRIG
```

### Example 6: Freeze + Position Modulation

```
# Capture interesting moment
TR               # Play voice
CL.FREEZE 1      # Freeze that audio

# Metro scans frozen buffer
M1: CL.POS RND 0 16383
M2: CL.TRIG
```

---

## Help System Documentation

Add to `src/help/effects.rs`:

```
--- CLOUDS (Granular Effect) ---
CL.TRIG   Trigger grain playback
CL.PITCH  Grain pitch (0-16383, 8192=center)
CL.POS    Buffer position (0-16383)
CL.SIZE   Grain size (0-16383)
CL.DENS   Grain density (0-16383)
CL.TEX    Grain texture (0-16383)
CL.WET    Wet/dry mix (0-16383)
CL.GAIN   Input gain (0-16383, 8192=1x)
CL.SPREAD Stereo spread (0-16383)
CL.RVB    Internal reverb (0-16383)
CL.FB     Feedback (0-16383, >10000 risky!)
CL.FREEZE Freeze buffer (0/1)
CL.MODE   0=grain 1=pitch 2=loop 3=spectral
CL.LOFI   Lo-fi reduction (0-16383)

Aliases: CLTR, CLP/CLPT, CLO/CLPS, CLS/CLSZ,
         CLD/CLDS, CLT/CLTX, CLW, CLG, CLSP,
         CLR/CLRV, CLF, CLFZ, CLM, CLLO

Note: Buffer continuously records input audio.
      CL.TRIG plays back grains from buffer.
      Use in Metro for continuous grain playback.
```

---

## Testing Checklist

### Unit Tests
- [ ] CL.TRIG sends correct OSC message (t_cl_trig = 1)
- [ ] All parameter commands validate ranges (0-16383)
- [ ] CL.MODE clips to 0-3
- [ ] CL.FREEZE clips to 0-1
- [ ] Expression evaluation in all parameters
- [ ] Short aliases resolve correctly

### Integration Tests
- [ ] CL.TRIG in pattern context (IF ER 3 8 I: CL.TRIG)
- [ ] Metro script triggering (M1: CL.TRIG)
- [ ] Variable assignment (A RND 0 16383, CL.POS A)
- [ ] RND.FX randomizes Clouds (except freeze)
- [ ] RST resets all Clouds parameters
- [ ] Scene save/load preserves Clouds state

### Audio Tests
- [ ] Mode 0: Granular texture generation
- [ ] Mode 1: Pitch shifting accuracy
- [ ] Mode 2: Looping delay behavior
- [ ] Mode 3: Spectral processing
- [ ] Freeze mode: Buffer captured and static
- [ ] Position scanning: Different buffer regions
- [ ] Feedback: Verify <10000 safe, >10000 risky
- [ ] Internal reverb: Works independently of plate reverb
- [ ] Wet/dry mix: Smooth crossfade
- [ ] Bypass: CL.WET 0 = no processing

### Performance Tests
- [ ] CPU usage per mode (0-3)
- [ ] High density impact (16383 vs 4096)
- [ ] Multiple effects active (Clouds + Beat Repeat + Delay)
- [ ] Verify bypass optimization (CL.WET 0)

---

## Documentation Updates Required

- **MANUAL.md** - Add "MiClouds Granular Effect" section with:
  - Overview of granular processing
  - All 14 commands with descriptions
  - Mode explanations (0-3)
  - Trigger workflow explanation
  - Example patches for each mode
  - Performance notes (CPU, interaction with other effects)

- **COMMAND_REFERENCE.md** - Add CL.* commands to effects section

- **ARCHITECTURE.md** - Update signal flow diagram:
  ```
  Compressor → Pan → [MiClouds] → Beat Repeat → Pitch Shift →
  Delay → EQ → Reverb
  ```

- **ROADMAP.md** - Move "MiClouds Integration" from backlog to completed

- **CHANGELOG.md** - Document in release notes:
  - New MiClouds granular effect
  - 14 commands (CL.TRIG + 13 parameters)
  - 4 processing modes
  - Pattern/Metro integration

---

## Conclusion

MiClouds adds professional granular processing capability to monokit with relatively low implementation effort. The key insight is that **MiClouds continuously records audio into a buffer** and requires explicit triggering (CL.TRIG) to play back grains - without triggers, the effect records but produces no output.

**Advantages:**
- ✓ Powerful granular synthesis in effects chain
- ✓ 4 distinct modes expand sonic palette
- ✓ Freeze mode enables creative sound design
- ✓ Pattern-based triggering for rhythmic textures
- ✓ Internal reverb reduces need for heavy plate reverb
- ✓ Complements existing effects (Beat Repeat → Clouds → Delay)

**Implementation simplicity:**
- ✓ mi-UGens already bundled
- ✓ No manual buffer management
- ✓ Straightforward parameter mapping
- ✓ Fits cleanly into effects chain

**Total effort: 5-6 days**

The trigger-based grain playback workflow fits naturally into monokit's command philosophy and enables creative rhythmic/pattern-based grain triggering that goes beyond typical "always-on" granular effects.
