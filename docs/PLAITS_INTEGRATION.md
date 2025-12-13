# Plaits Integration Documentation

## 1. Overview

### What is Plaits?
Plaits is a Mutable Instruments macro oscillator module, originally designed for Eurorack modular synthesis. It provides 16 different synthesis engines in a single module, ranging from virtual analog oscillators to physical modeling, granular synthesis, and percussion synthesis.

### Why Integrate Plaits?
The monokit synthesizer originally featured a dual-oscillator architecture (HD2) with primary and modulator oscillators plus noise. While powerful, this limited the sonic palette to FM-based synthesis and subtractive techniques. Plaits integration expands monokit's capabilities with:
- 16 distinct synthesis engines (wavetable, granular, modal, percussion, etc.)
- Dual outputs per engine (main + aux with different characteristics)
- Physical modeling capabilities not possible with HD2
- Percussion synthesis (kick, snare, hi-hat) for rhythm programming
- Additive and spectral synthesis options

### Integration Approach
Plaits was integrated as a **5th parallel sound source** that feeds into the pre-filter mix alongside HD2 oscillators and noise. This design ensures Plaits benefits from monokit's comprehensive filter and effects chain while maintaining independent control over triggering and output levels.

## 2. Architecture

### Signal Flow

```
Sound Sources (Parallel):
  monokit_noise    → Bus 18 ──┐
  monokit_mod      → Bus 17 ──┤
  monokit_primary  → Bus 16 ──┤
  monokit_plaits:              │
    - Main output  → Bus 19 ──┤──→ monokit_main (pre-filter mix)
    - AUX output   → Bus 20 ──┘         ↓
                                    SVF Filter
                                         ↓
                                    Effects Chain
                                    (Lofi, Ring Mod, Compressor,
                                     Delay, Reverb, EQ, Pitch Shift)
                                         ↓
                                      Output
```

### Key Design Decisions

**Pre-Filter Routing**
- Plaits feeds into monokit_main's pre-filter mix (line 1016 in compile_synthdefs.scd)
- This ensures Plaits passes through the same filter as HD2 sources
- Maintains consistent signal flow and sonic coherence

**Dual Output Architecture**
- Plaits provides two outputs per engine (main + aux)
- Each output has independent volume control (PLV, PAV)
- Different engines route different signals to aux (see Engine Reference)
- Enables layering strategies (e.g., main chord + aux arpeggio)

**Independent Trigger**
- PLTR command triggers Plaits independently from HD2's TR command
- Allows polyrhythmic patterns (HD2 on one rhythm, Plaits on another)
- Future enhancement: unified trigger system (TR.HD2, TR.PL, TR.ALL)

**Shared Pitch with Offset Capability**
- PF command controls both HD2 primary and Plaits pitch
- Future PL.DETUNE will allow pitch offset in cents
- Simplifies melodic sequences while preserving flexibility

**5-Synth Architecture**
The runtime uses 5 parallel SynthDefs:
1. **monokit_noise** (node 1001) - Noise generator with envelope
2. **monokit_mod** (node 1002) - Modulator oscillator
3. **monokit_primary** (node 1003) - Primary oscillator with FM
4. **monokit_plaits** (node 1004) - Plaits voice (NEW)
5. **monokit_main** (node 1000) - Effects processor reading all buses

## 3. Implementation Summary

### SuperCollider Side (Complete)

**SynthDef Creation**
- Created `monokit_plaits` SynthDef (lines 791-825 in compile_synthdefs.scd)
- Added to sc/monokit_server.scd for runtime instantiation
- Verified in verified_build.sh (8 SynthDefs now checked)

**Signal Routing**
- Updated `monokit_main` to read buses 19/20 via InFeedback.ar
- Mixed Plaits outputs into pre-filter signal (line 1016)
- Preserved audio bus architecture for low-latency feedback

**Build Pipeline**
- MiPlaits.scx bundled in bundle.sh (lines 179-188)
- Graceful degradation if mi-UGens not installed (warning only)
- Installation instructions included in bundle script output

### Rust Side (Complete - Debugging Required)

**Module Structure**
Created `src/commands/synth/plaits/` module:
- `mod.rs` - Module parent with public exports
- `engine.rs` - PL.ENG handler (0-15 validation)
- `params.rs` - All parameter handlers (HARM, TIMB, MORPH, DEC, LPG, volumes)

**Type System Updates**
- Added PLAITS_NODE_ID constant (1004) to src/types.rs
- Updated parameter routing in route_to_node() function
- Added plaits parameters to routing table (line 742)

**Command Integration**
- Added 10 Plaits commands to command dispatcher (src/commands/mod.rs)
- Implemented PLTR trigger in src/commands/system/misc.rs
- Implemented RND.PL randomizer in src/commands/randomization.rs
- All commands registered and compiled into binary

## 4. Commands Implemented

### Core Parameters

**PL.ENG** (0-15) - Engine Selection
- Selects one of 16 synthesis engines
- See Engine Reference for details on each engine
- Usage: `PL.ENG 0` (virtual analog), `PL.ENG 13` (bass drum), etc.

**PL.HARM** (0-16383) - Harmonics
- Controls harmonic content/overtone structure
- Engine-specific behavior (e.g., wavetable position, grain density)
- Usage: `PL.HARM 8192` (mid-range)

**PL.TIMB** (0-16383) - Timbre
- Controls spectral brightness/texture
- Engine-specific behavior (e.g., filter cutoff, formant shift)
- Usage: `PL.TIMB 12000` (bright)

**PL.MORPH** (0-16383) - Morph
- Morphs between engine variations/modes
- Engine-specific behavior (e.g., waveform mix, resonance)
- Usage: `PL.MORPH 4000` (subtle variation)

**PL.DEC** (0-16383) - Decay
- Controls amplitude envelope decay time
- Affects both outputs (main + aux)
- Usage: `PL.DEC 8000` (medium decay)

**PL.LPG** (0-16383) - Lowpass Gate Color
- Simulates vactrol-based lowpass gate
- Combines VCA and VCF behavior (Buchla-style)
- Usage: `PL.LPG 10000` (warm, closed sound)

### Volume Controls

**PLV** (0-16383) - Plaits Main Output Volume
- Controls level of main output (bus 19)
- Default: 8192 (50%)
- Usage: `PLV 16383` (full volume), `PLV 0` (mute main)

**PAV** (0-16383) - Plaits AUX Output Volume
- Controls level of aux output (bus 20)
- Default: 0 (muted)
- Usage: `PAV 8192` (50% aux), `PAV 16383` (full aux)

### Trigger

**PLTR** - Trigger Plaits
- Sends trigger to Plaits engine (t_gate parameter)
- Independent from HD2's TR command
- Usage: `PLTR` (trigger once)

### Randomization

**RND.PL** - Randomize All Plaits Parameters
- Randomizes: engine, harmonics, timbre, morph, decay, lpg
- Does NOT randomize: volumes (PLV, PAV) to prevent unexpected silence
- Usage: `RND.PL` (explore new sounds)

## 5. Plaits Engines Reference

| Engine | Type | Description | Main Output | AUX Output |
|--------|------|-------------|-------------|------------|
| 0 | Virtual Analog | Classic subtractive synthesis | Selected waveform | Sub-oscillator (-1 octave) |
| 1 | Waveshaper | Wavefolding and distortion | Shaped waveform | Fold variant |
| 2 | FM | Two-operator FM synthesis | Carrier + modulator | Modulator only |
| 3 | Grain Formant | Formant synthesis via grains | Grain cloud | Individual grains |
| 4 | Harmonic Oscillator | Additive synthesis | Full harmonic series | Odd harmonics only |
| 5 | Wavetable | Wavetable oscillator | Wavetable scan | Phase-shifted copy |
| 6 | Chords | Chord generator | Full chord | Arpeggio |
| 7 | Vowel/Speech | Vocal formant synthesis | Vowel formants | Raw pulse wave |
| 8 | Granular Cloud | Granular synthesis | Forward grains | Reversed grains |
| 9 | Filtered Noise | Resonant noise filter | Filtered output | Raw noise |
| 10 | Particle Noise | Particle-based synthesis | Particle cloud | Dust impulses |
| 11 | String Resonator | Karplus-Strong string | String resonance | Excitation signal |
| 12 | Modal Resonator | Physical modeling (bells, etc.) | Modal resonance | Strike/excitation |
| 13 | Bass Drum | 808-style bass drum | Drum body | Click/attack layer |
| 14 | Snare Drum | 808-style snare drum | Drum body | Snare rattle |
| 15 | Hi-Hat | 808-style hi-hat | Main cymbal | Sizzle layer |

### Engine Usage Examples

**Melodic Engines (0-8)**
```
PL.ENG 0; PLV 12000; PL.HARM 8000    # VA bass
PL.ENG 2; PLV 10000; PL.FM 12000     # FM bell (future PL.FM)
PL.ENG 5; PLV 10000; PL.HARM 10000   # Wavetable pad
PL.ENG 6; PLV 12000; PAV 8000        # Chord + arp layer
```

**Percussion Engines (13-15)**
```
PL.ENG 13; PLV 16383; PL.DEC 6000    # Punchy kick
PL.ENG 14; PLV 14000; PL.TIMB 12000  # Bright snare
PL.ENG 15; PLV 10000; PL.DEC 2000    # Crisp hi-hat
```

**Textural Engines (8-12)**
```
PL.ENG 8; PLV 8000; PAV 8000         # Granular cloud (fwd+rev)
PL.ENG 11; PLV 12000; PL.HARM 10000  # String pluck
PL.ENG 12; PLV 10000; PL.MORPH 8000  # Bell tones
```

## 6. Files Modified

### SuperCollider

**build_scripts/compile_synthdefs.scd**
- Lines 791-825: Added monokit_plaits SynthDef
  - Parameters: t_gate, pitch, detune, engine, harmonics, timbre, morph, decay, lpg, plv, pav
  - MiPlaits.ar integration with proper parameter scaling
  - Dual outputs to buses 19 (main) and 20 (aux)
- Lines 998-1016: Updated monokit_main to read Plaits buses
  - Added plaitsMainIn and plaitsAuxIn via InFeedback.ar
  - Mixed into pre-filter signal
- Line 1399: Updated SynthDef summary documentation

**sc/monokit_server.scd**
- Added monokit_plaits SynthDef (mirrored from compile_synthdefs.scd)
- Added synth instantiation at node 1004 with addAfter ordering
- Updated runtime architecture documentation

### Rust

**src/commands/synth/plaits/mod.rs** (NEW)
- Module parent with public exports
- Re-exports: handle_pl_eng, handle_pl_harm, handle_pl_timb, handle_pl_morph, handle_pl_dec, handle_pl_lpg, handle_plv, handle_pav

**src/commands/synth/plaits/engine.rs** (NEW)
- PL.ENG handler using define_mode_param! macro
- Validates engine range (0-15)
- Routes to "engine" parameter on PLAITS_NODE_ID

**src/commands/synth/plaits/params.rs** (NEW)
- All parameter handlers using define_int_param! macro
- Handlers: handle_pl_harm, handle_pl_timb, handle_pl_morph, handle_pl_dec, handle_pl_lpg, handle_plv, handle_pav
- Range validation (0-16383) for all parameters

**src/types.rs**
- Line 709: Added PLAITS_NODE_ID constant (1004)
- Line 742: Updated route_to_node() to route Plaits parameters
  - Routes: engine, harmonics, timbre, morph, decay, lpg, plv, pav

**src/commands/synth/mod.rs**
- Line 12: Added plaits module declaration
- Line 24: Added plaits re-export via pub use plaits::*

**src/commands/system/misc.rs**
- Lines 140-150: Added handle_pltr function
- Sends t_gate trigger to PLAITS_NODE_ID (1004)

**src/commands/randomization.rs**
- Lines 507-540: Added handle_rnd_pl function
- Randomizes: engine (0-15), harmonics, timbre, morph, decay, lpg
- Preserves: volumes (PLV, PAV) to avoid unexpected silence

**src/commands/mod.rs**
- Lines 300-324: Added Plaits command dispatch cases
  - PL.DEC, PL.ENG, PL.HARM, PL.LPG, PL.MORPH, PL.TIMB, PLTR, PLV, PAV
- Line 724: Added RND.PL dispatch case

### Build System

**scripts/bundle.sh**
- Lines 179-188: Added MiPlaits.scx bundling
- Checks for mi-UGens installation in ~/Library/Application Support/SuperCollider/Extensions
- Copies MiPlaits.scx to bundle Resources/plugins/
- Provides graceful warning if not found with installation link

**scripts/verified_build.sh**
- Lines 46-55: Updated REQUIRED_SYNTHDEFS array
- Added "monokit_plaits.scsyndef" verification
- Now checks 8 SynthDefs (was 7)
- Lines 156-164: Updated build summary documentation

## 7. Known Issues

### Commands Show as Unknown (Runtime Issue)

**Symptoms**
All Plaits commands (PL.ENG, PL.HARM, PLV, PLTR, etc.) are registered in code but show as "unknown command" at runtime.

**Evidence**
- Commands are present in src/commands/mod.rs dispatcher (verified)
- Module structure is correct (mod.rs → engine.rs + params.rs)
- Public exports are in place (pub use plaits::*)
- Parameter routing exists in types.rs (line 742)

**Possible Causes**
1. **Stale Binary Cache** - Old monokit binary running without new commands
2. **Module Import Issue** - plaits module not properly exported from synth module
3. **Feature Gate Issue** - Commands conditionally compiled out
4. **Macro Expansion Issue** - define_int_param/define_mode_param not generating code

**Debugging Steps**
1. Clean build: `cargo clean && cargo build --release`
2. Verify binary freshness: Check monokit binary timestamp vs. source files
3. Add debug logging to command dispatcher: Print all registered commands at startup
4. Test command matching: Add println! before match statement in mod.rs
5. Check macro expansion: `cargo expand commands::synth::plaits` to verify generated code
6. Verify module tree: Ensure plaits re-exports propagate to top-level commands module

**Temporary Workaround**
None - commands must be recognized for Plaits to function.

**Impact**
Plaits integration is complete in SuperCollider (SynthDef works, buses routed correctly) but unusable from Rust CLI until command registration issue is resolved.

## 8. Testing Recommendations

Once command recognition is resolved, test in this order:

### Basic Plaits Functionality
```
# Test VA engine with trigger
PL.ENG 0          # Virtual analog oscillator
PL.HARM 12000     # Add harmonics
PLV 10000         # Set volume
PLTR              # Trigger - should produce sound

# Test parameter changes
PL.TIMB 8000      # Change timbre
PL.MORPH 4000     # Morph waveform
PLTR              # Trigger again

# Test volume controls
PLV 16383         # Max main volume
PAV 8192          # Enable aux output
PLTR              # Should hear both outputs
```

### Engine Variety
```
# Test percussion engines
PL.ENG 13; PLTR   # Bass drum
PL.ENG 14; PLTR   # Snare drum
PL.ENG 15; PLTR   # Hi-hat

# Test melodic engines
PL.ENG 2; PLTR    # FM synthesis
PL.ENG 5; PLTR    # Wavetable
PL.ENG 6; PLTR    # Chords

# Test textural engines
PL.ENG 8; PLTR    # Granular cloud
PL.ENG 11; PLTR   # String resonator
PL.ENG 12; PLTR   # Modal resonator
```

### HD2 + Plaits Layering
```
# Kick from HD2, snare from Plaits
PF 60             # Low pitch for HD2
PV 16383          # HD2 primary volume
TR                # Trigger HD2 (kick)

PL.ENG 14         # Plaits snare engine
PLV 14000         # Plaits volume
PLTR              # Trigger Plaits (snare)

# Simultaneous triggers
TR                # Both should work independently
PLTR
```

### Randomization
```
# Explore random Plaits sounds
RND.PL            # Randomize all parameters
PLTR              # Trigger to hear result
RND.PL; PLTR      # Quick iteration
RND.PL; PLTR      # Keep exploring

# Randomize with fixed engine
PL.ENG 8          # Lock to granular engine
RND.PL            # Randomizes other params (not engine)
PLTR
```

### Filter Integration
```
# Test Plaits through filter
PL.ENG 0          # VA oscillator
PLV 12000
FT 4              # MoogFF filter
FC 2000           # Cutoff frequency
FQ 12000          # Resonance
PLTR              # Should hear filtered Plaits

# Swept filter
FC 500; PLTR
FC 1000; PLTR
FC 2000; PLTR
FC 4000; PLTR
```

### Effects Chain
```
# Test Plaits through reverb
PL.ENG 12         # Modal resonator
PLV 10000
RV 12000          # Reverb decay
RW 8000           # Reverb wet
PLTR              # Should hear reverb tail

# Test with delay
PL.ENG 6          # Chords
DT 8000           # Delay time
DF 8000           # Delay feedback
DW 8000           # Delay wet
PLTR              # Should hear delayed chords
```

## 9. Commits

Integration work completed across 3 commits:

**a598d9f** - build: add MiPlaits.scx to bundle pipeline
- Added mi-UGens detection and bundling
- MiPlaits.scx copied from Extensions to bundle
- Graceful warning if not installed

**12da4e0** - build: add Plaits verification to verified_build script
- Updated REQUIRED_SYNTHDEFS to include monokit_plaits
- Now verifies 8 SynthDefs (was 7)
- Updated build summary documentation

**c77f5be** - fix: correct postln string concatenation syntax
- Fixed SuperCollider syntax error in compile_synthdefs.scd
- Corrected string concatenation for postln statements

## 10. Future Enhancements

### Phase 2 Features (Not Yet Implemented)

**Independent Pitch Control**
- `PL.DETUNE` (cents) - Pitch offset from PF
- `PL.PF` (Hz) - Completely independent pitch
- Enables harmonic intervals (e.g., PF=C, PL.DETUNE=700 for perfect fifth)

**Modulation Inputs**
- `PL.FM` - FM amount to harmonics parameter
- `PL.TM` - Modulation to timbre parameter
- `PL.MM` - Modulation to morph parameter
- Enables dynamic timbral evolution

**Unified Trigger System**
- `TR.HD2` - Trigger only HD2 oscillators
- `TR.PL` - Trigger only Plaits (alias for PLTR)
- `TR.ALL` - Trigger both HD2 and Plaits simultaneously
- Simplifies layered percussion programming

**Per-Voice Effect Sends**
- `PL.RV` - Plaits-specific reverb send
- `PL.DL` - Plaits-specific delay send
- Independent effect levels for HD2 vs Plaits

**Plaits-Specific Presets**
- `PR.PL` - Save/recall Plaits parameters separately
- Enables mixing HD2 and Plaits presets
- Engine-specific parameter sets

### Architecture Improvements

**Trigger Envelope**
- Per-engine trigger envelope shapes
- Attack/decay controls for percussive engines
- Integration with main envelope system

**Output Routing Options**
- Route Plaits pre/post filter (switch)
- Direct output bypass (Plaits → Out, skip filter/FX)
- Stereo width control for dual outputs

**Engine-Specific Parameters**
- Expose engine-specific hidden parameters
- E.g., grain size for engine 8, formant frequency for engine 7
- Requires deep dive into MiPlaits implementation

## 11. References

### MiPlaits Documentation
- SuperCollider UGen: https://github.com/v7b1/mi-UGens
- Original Mutable Instruments: https://mutable-instruments.net/modules/plaits/

### Related Files
- SynthDef definitions: `/build_scripts/compile_synthdefs.scd`
- Runtime server: `/sc/monokit_server.scd`
- Command routing: `/src/types.rs`
- Command handlers: `/src/commands/synth/plaits/`
- Randomization: `/src/commands/randomization.rs`

### Integration Context
- Multi-synth architecture: Implemented in v0.3.5 (commit 89d035b)
- 5-synth design: noise, mod, primary, plaits, main
- Audio bus routing: Buses 16-20 for source signals
