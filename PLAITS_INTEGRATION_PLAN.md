# Plaits Voice Integration Plan

Research and design notes for integrating Mutable Instruments Plaits into monokit as a concurrent voice alongside the existing HD2 complex oscillator.

---

## Overview

**MiPlaits** is part of the mi-UGens collection by Volker Böhm, porting Mutable Instruments Eurorack modules to SuperCollider.

- **Repository**: [github.com/v7b1/mi-UGens](https://github.com/v7b1/mi-UGens)
- **Author**: Volker Böhm (not affiliated with Mutable Instruments)
- **License**: GPL-3.0 (SC port), MIT (original MI code)

---

## Technical Specification

### UGen Signature

```supercollider
MiPlaits.ar(pitch, engine, harm, timbre, morph, trigger, level,
            fm_mod, timb_mod, morph_mod, decay, lpg_colour, mul, add)
```

### Parameters

| Parameter | Description | Range | Default |
|-----------|-------------|-------|---------|
| `pitch` | MIDI note value | 0-127 | 60.0 |
| `engine` | Synthesis model | 0-15 | 0 |
| `harm` | Harmonics | 0.0-1.0 | 0.1 |
| `timbre` | Tonal character | 0.0-1.0 | 0.5 |
| `morph` | Model morphing | 0.0-1.0 | 0.5 |
| `trigger` | Gate/trigger input | trig | - |
| `level` | Amplitude + LPG brightness | 0.0-1.0 | - |
| `fm_mod` | FM depth | -1.0 to 1.0 | 0 |
| `timb_mod` | Timbre mod depth | -1.0 to 1.0 | 0 |
| `morph_mod` | Morph mod depth | -1.0 to 1.0 | 0 |
| `decay` | Internal env decay | 0.0-1.0 | 0.5 |
| `lpg_colour` | LPG character | 0.0-1.0 | 0.5 |

### Output

**Dual-channel** (not stereo):
- `[0]` Main signal
- `[1]` AUX - variant/byproduct of main signal

### Synthesis Engines (0-15)

| Engine | Type | Harm Control | Timbre Control | Morph Control |
|--------|------|--------------|----------------|---------------|
| 0 | Virtual Analog | Detuning | PWM | Variable saw |
| 1 | Waveshaper | Waveform | Fold amount | Asymmetry |
| 2 | Phase Mod (FM) | Freq ratio | Mod index | Feedback |
| 3 | Grain | Pitch random | Density | Duration |
| 4 | Additive | Bump count | Harmonic idx | Bump shape |
| 5 | Wavetable | Bank select | Row index | Column index |
| 6 | Chord | Chord type | Inversion | Waveform |
| 7 | Speech | Algorithm | Formants | Phoneme |
| 8 | Swarm | Pitch random | Density | Duration |
| 9 | Filtered Noise | Filter type | Clock freq | Resonance |
| 10 | Particle | Freq random | Density | Filter type |
| 11 | String (physical) | Inharmonicity | Brightness | Decay |
| 12 | Modal (physical) | Inharmonicity | Brightness | Decay |
| 13 | Bass Drum | Harmonics | Brightness | Decay |
| 14 | Snare | Harmonics | Brightness | Decay |
| 15 | Hi-Hat | Harmonics | Brightness | Decay |

**Note**: Physical/drum models (11-15) disable internal LPG. Trigger acts as synthesis trigger; level functions as accent.

---

## Proposed Command Structure

### Plaits-Specific Commands

| Command | Description | Range | SC Param |
|---------|-------------|-------|----------|
| `PL.ENG` | Engine/model | 0-15 | engine |
| `PL.HARM` | Harmonics | 0-16383 | harm |
| `PL.TIMB` | Timbre | 0-16383 | timbre |
| `PL.MORPH` | Morph | 0-16383 | morph |
| `PL.DEC` | Internal decay | 0-16383 | decay |
| `PL.LPG` | LPG colour | 0-16383 | lpg_colour |
| `PL.AUX` | AUX output mix | 0-16383 | (mix ratio) |
| `PL.VOL` | Plaits volume | 0-16383 | (amplitude) |

### Pitch Control Options

**Option A: Shared pitch**
- `PF` controls both HD2 and Plaits
- `PL.DETUNE <cents>` - Plaits offset from PF

**Option B: Independent pitch**
- `PF` - HD2 pitch (existing)
- `PL.PF` - Plaits pitch (separate)

**Recommendation**: Start with Option A (shared + detune) for simplicity.

### Modulation Commands (Future)

| Command | Description |
|---------|-------------|
| `PL.FM` | FM mod depth |
| `PL.TM` | Timbre mod depth |
| `PL.MM` | Morph mod depth |

### Randomization

| Command | Description |
|---------|-------------|
| `RND.PL` | Randomize all Plaits params |
| `RND.PL.ENG` | Random engine only |

---

## Concurrent Voice Architecture

### Design Goals

1. HD2 and Plaits run simultaneously
2. Shared trigger (TR fires both)
3. Independent volume control
4. Shared effects chain (delay, reverb, etc.)
5. Minimal CPU overhead

### Signal Flow Options

**Option A: Early Mix (Simplest)**
```
HD2 ────────┐
            ├──> Filter ──> Effects ──> Out
Plaits ─────┘
```
- Both voices hit shared SVF filter
- Simple but Plaits loses internal LPG character

**Option B: Post-Filter Mix (Recommended)**
```
HD2 ──> SVF Filter ──┐
                     ├──> Effects ──> Out
Plaits (internal LPG)┘
```
- HD2 goes through existing SVF filter
- Plaits uses its internal LPG (preserves character)
- Both hit shared delay/reverb/compressor

**Option C: Parallel Effects (CPU Heavy)**
```
HD2 ──> Filter ──> Effects ──┐
                             ├──> Out
Plaits ──> Effects ──────────┘
```
- Maximum flexibility but doubles effect processing
- Not recommended for monokit's design

### SuperCollider Implementation

Single SynthDef approach (fits current architecture):

```supercollider
SynthDef(\monokit, {
    arg out=0, gate=0,
        // ... existing HD2 args ...
        // Plaits args
        plVol=0, plEng=0, plHarm=0.5, plTimb=0.5, plMorph=0.5,
        plDec=0.5, plLpg=0.5, plAux=0, plDetune=0;

    var trig = Trig.kr(gate, 0.001);

    // Existing HD2 signal chain
    var hd2Sig = /* ... existing oscillator/filter code ... */;

    // Plaits voice
    var plPitch = (pf * pow(2, plDetune/1200)).cpsmidi; // detune in cents
    var plaitsSig = MiPlaits.ar(
        pitch: plPitch,
        engine: plEng,
        harm: plHarm,
        timbre: plTimb,
        morph: plMorph,
        trigger: trig,
        level: gate,
        decay: plDec,
        lpg_colour: plLpg
    );

    // Mix main and aux outputs
    var plaitsOut = (plaitsSig[0] * (1 - plAux)) + (plaitsSig[1] * plAux);

    // Mix voices (after HD2 filter, before shared effects)
    var mixed = (hd2Sig * volume) + (plaitsOut * plVol);

    // ... existing effects chain (delay, reverb, etc.) ...
}).add;
```

### Triggering Behavior

**Shared Trigger (Recommended for MVP)**
- `TR` command fires both voices
- Plaits trigger input receives same gate
- Simpler mental model, drum-focused

**Independent Triggers (Future Enhancement)**
- `TR` - HD2 only
- `PL.TR` - Plaits only
- `TR.ALL` or just `TR` with flag - Both
- More complex but allows polyrhythmic layering

---

## Distribution

### Extension Installation

mi-UGens must be installed in SC's plugin path:

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/SuperCollider/Extensions/` |
| Linux | `~/.local/share/SuperCollider/Extensions/` |
| Windows | `%LOCALAPPDATA%/SuperCollider/Extensions/` |

### Bundled Distribution

For standalone monokit distribution:

```
monokit.app/Contents/
├── MacOS/monokit
└── Resources/
    ├── scsynth
    ├── plugins/
    │   ├── MiPlaits.scx      # Plaits UGen
    │   └── (other plugins)
    └── synthdefs/
        └── monokit.scsyndef
```

Launch scsynth with explicit plugin path:
```bash
./scsynth -u 57110 -U ./plugins/
```

### Platform-Specific Builds

mi-UGens requires separate compilation per platform:
- macOS: `.scx` (Intel and Apple Silicon separately)
- Linux: `.so` (x86_64, ARM)
- Windows: `.scx`

Pre-built binaries available at [github.com/v7b1/mi-UGens/releases](https://github.com/v7b1/mi-UGens/releases)

---

## Licensing

### License Chain

| Component | License | Commercial Use |
|-----------|---------|----------------|
| MI Plaits (original) | MIT | Yes |
| mi-UGens (SC port) | GPL-3.0 | Yes (with conditions) |
| SuperCollider | GPL-3.0 | Yes (with conditions) |

### Why Monokit is Safe

Current architecture uses **process separation** (Rust CLI + scsynth via OSC):
- This is an "aggregate" under GPL, not a derivative work
- Rust CLI can use any license
- Only SC components must remain GPL-3.0

### Required Attribution

Include in distribution:
1. GPL-3.0 license text for SC components
2. MIT license text for original MI code
3. Attribution to:
   - Emilie Gillet / Mutable Instruments (original algorithms)
   - Volker Böhm / mi-UGens (SuperCollider port)
   - SuperCollider project

### Precedent

Commercial products using MI code:
- Arturia MicroFreak
- Korg Prologue
- VCV Rack (Audible Instruments)

All cite MI's permissive licensing as enabling their use.

---

## Implementation Phases

### Phase 1: Basic Integration
- [ ] Add MiPlaits to monokit_server.scd
- [ ] Implement PL.ENG, PL.HARM, PL.TIMB, PL.MORPH, PL.VOL
- [ ] Shared trigger (TR fires both voices)
- [ ] Shared pitch (PF controls both)
- [ ] Mix after HD2 filter, before effects

### Phase 2: Extended Control
- [ ] PL.DEC, PL.LPG for internal envelope control
- [ ] PL.AUX for aux output mixing
- [ ] PL.DETUNE for pitch offset
- [ ] RND.PL randomization

### Phase 3: Advanced Features
- [ ] Independent pitch (PL.PF)
- [ ] Modulation inputs (PL.FM, PL.TM, PL.MM)
- [ ] Per-voice panning (PL.PAN)
- [ ] Consider: Independent triggers (PL.TR)

### Phase 4: Distribution
- [ ] Bundle mi-UGens with monokit releases
- [ ] Platform-specific plugin builds
- [ ] License/attribution documentation
- [ ] Installation automation

---

## CPU Considerations

- MiPlaits is relatively efficient (comparable to 2-4 standard oscillators)
- Physical modeling engines (11-15) are heavier
- Drum engines (13-15) optimized for percussive use
- Internal LPG reduces need for external envelope/filter

**Recommendation**: Default to simpler engines (0-6) for layering, reserve physical models (11-15) for solo Plaits use.

---

## Open Questions

1. **Envelope independence**: Should Plaits have external envelope options or always use internal LPG?
2. **Effect routing**: Any need for per-voice effect sends?
3. **Preset format**: Should presets store both voice configurations or treat them separately?
4. **Voice muting**: `PL.MUTE` command or just `PL.VOL 0`?
5. **Crossfade mode**: `MIX` command for crossfade vs independent `VOL`/`PL.VOL`?

---

## References

- [mi-UGens GitHub](https://github.com/v7b1/mi-UGens)
- [MiPlaits SC Help](https://s4ntp.org/NTMI/SCHelp/Classes/MiPlaits.html)
- [Plaits Manual](https://pichenettes.github.io/mutable-instruments-documentation/modules/plaits/manual/)
- [MI Open Source](https://pichenettes.github.io/mutable-instruments-documentation/modules/plaits/open_source/)

---

*Document created: December 2025*
*Status: Research/Planning*
