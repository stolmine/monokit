# Grid Mode Expansion & Sampler Plan

Design document for GRID.MODE expansion and unified sample playback system.

**Status:** Planning
**Date:** January 2026

---

## Overview

Expand GRID.MODE to include mixer and FX visualization, and add a sample playback voice with a unified interface for both sliced loops and one-shot kits.

---

## Part 1: GRID.MODE Expansion

### Current Modes

| Mode | Name | Description |
|------|------|-------------|
| 0 | Parameter Grid | 6x6 voice parameter grid |
| 1 | Scope | Oscilloscope (dedicated page) |
| 2 | EQ/Comp Viz | 3-band EQ + compressor meters |

### Proposed Additions

| Mode | Name | Description |
|------|------|-------------|
| 3 | Mixer | Voice levels, mutes, panning |
| 4 | FX Viz | Effects chain visualization |
| 5 | Sampler | Sample info, slice position, playback state |

### Mode 3: Mixer

**Visual Layout (30 chars wide):**
```
OSC ████████·· -6  ‹·› M·
PLA ██████···· -12 ‹›· ··
NOS ███······· -18 ·‹› ··
SMP ████████·· -6  ··› ··
MIX ██████████  0  ·‹·
```

**Elements per row:**
- Voice name (3 chars)
- Level meter (10 chars)
- dB readout (4 chars)
- Pan indicator (3 chars): `‹··` (L), `·‹·` (C), `··›` (R)
- Mute indicator (2 chars): `M·` = muted, `··` = active

**Commands:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| VOL.OSC | VO | 0-16383 | Complex oscillator volume |
| VOL.PLA | VP | 0-16383 | Plaits volume |
| VOL.NOS | VN | 0-16383 | Noise volume |
| VOL.SMP | VS | 0-16383 | Sampler volume |
| PAN.OSC | PO | -8192 to 8191 | Complex osc pan (center=0) |
| PAN.PLA | PP | -8192 to 8191 | Plaits pan |
| PAN.NOS | PNN | -8192 to 8191 | Noise pan |
| PAN.SMP | PS | -8192 to 8191 | Sampler pan |
| MUTE.OSC | MO | 0/1 | Mute complex oscillator |
| MUTE.PLA | MPL | 0/1 | Mute Plaits |
| MUTE.NOS | MN | 0/1 | Mute noise |
| MUTE.SMP | MS | 0/1 | Mute sampler |

**Notes:**
- PAN.NOS uses PNN alias to avoid conflict with PN.* pattern namespace
- MUTE.PLA uses MPL alias to avoid conflict with existing MP (route to pitch)

### Mode 4: FX Visualization

**Visual Layout (30 chars):**
```
DLY ████······ 40% ·‹· FB30
REV ██········ 20% ·‹·
CHR ··········  0%
DST ██████···· 60%     TYPE2
FLT ████████·· 80% LP 800Hz
```

**Elements:**
- Effect name (3 chars)
- Wet/dry or amount meter (10 chars)
- Percentage (4 chars)
- Effect-specific params (variable)

**Effects to visualize:**
- Delay (wet, feedback, pan)
- Reverb (wet)
- Chorus/Flanger (amount)
- Distortion (amount, type)
- Filter (cutoff, type)
- Beat repeat (when active)
- Clouds (when active)

### Mode 5: Sampler

**Visual Layout (30 chars wide):**
```
KIT  drums/808/
SLOT 03/16      ████████████··
RATE +0st  DIR► LOOP· GATE·
ATK 10ms   REL 100ms
PAN ·‹·    VOL ████████·· -6
```

**Row breakdown:**
1. **Source** - Kit/file name (truncated if needed)
2. **Slot** - Current slot/total + position bar showing playhead
3. **Playback** - Pitch offset, direction (►/◄), loop flag, gate state
4. **Envelope** - Attack and release times
5. **Output** - Pan position, volume meter with dB

**Alternative for slice mode:**
```
FILE amen.wav
SLICE 04/16     ████████████··
RATE +0st  DIR► LOOP· GATE·
ATK 10ms   REL 100ms
PAN ·‹·    VOL ████████·· -6
```

**Live state indicators:**
- `DIR►` = forward, `DIR◄` = reverse
- `LOOP●` = looping active, `LOOP·` = one-shot
- `GATE●` = currently playing, `GATE·` = idle
- Playhead bar shows position within current slot

---

## Part 2: Unified Sampler

### Core Concept

Single `KIT` command loads samples, single `STR` command triggers them. Path type determines behavior:

```
KIT breaks/amen.wav     # File → load and auto-slice
KIT drums/808/          # Directory → load as one-shot kit
```

Both result in **slots** that `STR` can trigger identically:

```
STR 0                   # Trigger slot 0
STR RND 0 15            # Random slot
STR PN.NEXT 0           # Pattern-driven selection
```

### Internal Representation

```rust
struct SamplerState {
    mode: SamplerMode,          // Slice or Kit
    slots: Vec<SampleSlot>,     // Up to 128 slots
    current_slot: usize,
    // For slice mode:
    buffer_id: Option<u32>,     // Single buffer
    // For kit mode:
    buffers: Vec<u32>,          // Multiple buffers
}

enum SamplerMode {
    Slice,  // One buffer, multiple regions
    Kit,    // Multiple buffers, one region each
}

struct SampleSlot {
    buffer_id: u32,
    start_frame: usize,
    end_frame: usize,
}
```

### Slot Limit

**128 slots maximum** for MIDI compatibility (0-127).

### Commands

#### Loading

| Command | Description |
|---------|-------------|
| KIT \<path\> | Load file (slice) or directory (kit) |
| KIT.LOAD \<path\> | Preload to RAM without switching |
| KIT.INFO | Show current kit info |
| KIT.LS | List available kits in library path |

#### Triggering

| Command | Range | Description |
|---------|-------|-------------|
| STR \<n\> | 0-127 | Trigger slot n |
| STR | - | Re-trigger current slot |

`STR` supports full expression evaluation:
```
STR RND 0 15
STR PN.NEXT 0
STR EITH 0 4 8 12
STR TOG 0 8
STR A              # Variable
```

#### Slicing (Slice Mode Only)

| Command | Range | Description |
|---------|-------|-------------|
| S.SLICE \<n\> | 2-128 | Divide into n equal slices |
| S.ONSET | - | Auto-detect transients |
| S.ONSET.TH \<n\> | 0-16383 | Onset detection threshold |

#### Playback Parameters

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| S.RATE | SR | 0-16383 | Playback rate (8192=1x, 0=0.25x, 16383=4x) |
| S.PITCH | SPT | -24 to +24 | Pitch offset in semitones |
| S.FINE | SFN | -100 to +100 | Fine pitch in cents |
| S.DIR | SD | 0/1 | Direction (0=fwd, 1=rev) |
| S.LOOP | SL | 0/1 | Loop mode |
| S.START | SST | 0-16383 | Start offset within slice (0=beginning) |
| S.LEN | SLE | 0-16383 | Loop length (0=full slice, else portion) |

#### Envelope Parameters

Uses ADSR envelope with variable sustain level for gate mode support.

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| S.ATK | SA | 0-16383 | Attack time (0=0ms, 16383=2000ms) |
| S.DEC | SDC | 0-16383 | Decay time (0=0ms, 16383=2000ms) |
| S.REL | SRE | 0-16383 | Release time (0=0ms, 16383=2000ms) |
| S.SUST | SSU | 0/1 | Sustain mode (0=one-shot/perc, 1=hold while gate) |

**Note:** t_gate trigger signal is separate from S.SUST sustain mode parameter.
- S.SUST=0: envelope decays immediately after attack (percussive)
- S.SUST=1: envelope holds at sustain level until gate released

#### Output Parameters

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| S.VOL | SV | 0-16383 | Sample volume |
| S.PAN | SP | -8192 to 8191 | Pan position (0=center) |
| S.FX | SFX | 0/1/2 | FX routing (0=dry, 1=post-filter, 2=post-all) |

#### Modulation Parameters

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| S.RATEMOD | SRM | 0-16383 | Rate modulation amount from envelope |
| S.PITCHMOD | SPM | 0-16383 | Pitch modulation amount from envelope |

#### Parameter Summary

**Total: 18 parameters**

| Category | Parameters |
|----------|------------|
| Pitch | S.RATE, S.PITCH, S.FINE |
| Playback | S.DIR, S.LOOP, S.START, S.LEN |
| Envelope | S.ATK, S.DEC, S.REL, S.SUST |
| Output | S.VOL, S.PAN, S.FX |
| Modulation | S.RATEMOD, S.PITCHMOD |
| Selection | STR (slot), S.SLICE/S.ONSET |

### Sampler FX Chain

The sampler has its own independent FX chain, separate from the main voice effects.

#### Signal Flow

```
Sample Playback
    ↓
[Decimator] ───────── bit crush + rate reduction
    ↓
[Disintegrator] ───── probability-based glitch
    ↓
[Sampler Filter] ──── independent LP/HP/BP (post-crush filtering)
    ↓
S.VOL / S.PAN
    ↓
→ Main Mix (uses global DLY/REV/CL for spatial FX)
```

**Rationale:** Filter after bit crush allows smoothing crushed artifacts or emphasizing harmonics created by decimation. Spatial effects (delay, reverb, clouds) use existing global FX chain.

#### Naming Convention

All sampler FX use `SF.*` prefix (Sampler FX) to avoid conflicts:

**Sampler Filter - DFM1 (SF.F*)**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| SF.CUT | SFC | 0-16383 | Filter cutoff frequency |
| SF.RES | SFQ | 0-16383 | Filter resonance |
| SF.TYPE | SFT | 0/1 | Filter type (0=LP, 1=HP) |

**Decimator (SF.D*)**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| SF.BITS | SFB | 1-24 | Bit depth (24=clean, 1=destroyed) |
| SF.RATE | SFR | 0-16383 | Sample rate reduction (0=off, 16383=extreme) |
| SF.DECI | SFD | 0-16383 | Decimator mix (0=bypass) |

**Disintegrator (SF.G*)**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| SF.PROB | SFP | 0-16383 | Glitch probability (0=off) |
| SF.MULT | SFM | 0-16383 | Glitch multiplier |
| SF.GLIT | SFG | 0-16383 | Disintegrator mix (0=bypass) |

#### FX Parameter Summary

| Category | Params | Commands |
|----------|--------|----------|
| Filter | 3 | SF.CUT, SF.RES, SF.TYPE |
| Decimator | 3 | SF.BITS, SF.RATE, SF.DECI |
| Disintegrator | 3 | SF.PROB, SF.MULT, SF.GLIT |
| **Total** | **9** | |

**Note:** Spatial effects (delay, reverb, granular) use the global FX chain (DLY, REV, CL commands).

#### FX Presets (Optional - Future)

Quick preset commands for common settings:

| Command | Description |
|---------|-------------|
| SF.RST | Reset all sampler FX to defaults |
| SF.LOFI | Lo-fi preset (8-bit, rate reduced, gentle filter) |
| SF.TAPE | Tape saturation preset (smooth decimator, filtered) |
| SF.MANGLE | Destruction preset (all FX engaged) |

#### Design Decisions

1. **FX Architecture** - Sampler has own destructive FX (SF.*), uses global spatial FX
2. **Filter type** - Reuse DFM1 (same as main voice filter)
3. **Mono/Stereo** - Samples converted to mono on load, stereo via S.PAN after FX chain
4. **Panning** - Applied in monokit_main (not in sampler SynthDef) per convention
5. **Modulation** - No envelope→FX modulation; FX params are sequenceable via commands
6. **Dry/Wet** - Per-effect mix controls (SF.DECI, SF.GLIT); filter always on
7. **"Bypass" filter** - Open cutoff fully (SF.CUT 16383) rather than true bypass
8. **Envelope** - ADSR with variable sustain; S.SUST=0 for percussive, S.SUST=1 for gate hold
9. **Spatial FX** - Use global DLY/REV/CL commands (no dedicated sampler reverb/delay)

### File System & Sample Logistics

#### Default Library Location

```
~/.config/monokit/samples/
```

Same location as other monokit config (scenes, presets, config.toml). Created on first run if missing.

#### Library Structure

```
~/.config/monokit/samples/
├── breaks/
│   ├── amen.wav
│   ├── think.wav
│   └── funky_drummer.wav
├── drums/
│   ├── 808/
│   │   ├── 00_kick.wav
│   │   ├── 01_snare.wav
│   │   └── 02_hat.wav
│   └── 909/
│       ├── 00_kick.wav
│       └── ...
└── oneshots/
    └── ...
```

#### Configuration

| Setting | Config Key | Default | Description |
|---------|------------|---------|-------------|
| Library path | `sample_path` | `~/.config/monokit/samples/` | Root sample directory |
| Buffer memory | `sample_buffer_mb` | 64 | Max MB for sample buffers |
| Auto-slice count | `sample_auto_slice` | 16 | Default slice count for loops |

Stored in `~/.config/monokit/config.toml`:
```toml
[sampler]
sample_path = "~/.config/monokit/samples/"
sample_buffer_mb = 64
sample_auto_slice = 16
```

#### Commands for Library Management

| Command | Description |
|---------|-------------|
| S.PATH | Query current library path |
| S.PATH \<path\> | Set library path (persists to config) |
| S.LS | List directories in library root |
| S.LS \<dir\> | List contents of directory |
| S.INFO | Show current kit info (name, slots, memory) |
| S.MEM | Show buffer memory usage |

#### Path Resolution Order

When `KIT <path>` is called:

1. **Absolute path** - `/Users/name/samples/kick.wav` → use directly
2. **Relative to library** - `breaks/amen.wav` → prepend library path
3. **Search library** - `amen.wav` → recursive search in library
4. **Search current dir** - `./amen.wav` → relative to working directory

#### Supported Formats

| Format | Extensions | Notes |
|--------|------------|-------|
| WAV | .wav | Preferred, all bit depths |
| AIFF | .aif, .aiff | Mac standard |
| FLAC | .flac | If SC supports it |

**Conversion on load:**
- Stereo → mono (sum to mono)
- Any sample rate → internal rate (via BufRateScale)
- Any bit depth → 32-bit float (SC internal)

#### Directory Loading (Kit Mode)

Files sorted alphanumerically, mapped to slots 0-127:

```
drums/808/
  00_kick.wav    → slot 0
  01_snare.wav   → slot 1
  02_hat.wav     → slot 2
  clap.wav       → slot 3 (alpha after numbers)
  rim.wav        → slot 4
```

**Naming convention (optional but recommended):**
- `00_name.wav` to `99_name.wav` for explicit ordering
- Or just rely on alphabetical sort

**Limits:**
- Max 128 samples per kit (slots 0-127)
- Excess files ignored with warning

#### File Loading (Slice Mode)

Single file loaded to buffer, auto-sliced:

```
KIT breaks/amen.wav
→ Loads to buffer
→ Auto-slices to 16 (or config default)
→ S.SLICE <n> to re-slice
```

#### Buffer Memory Management

**Allocation strategy:**
- Pre-allocate buffer pool on startup (configurable size)
- Track usage per kit
- Warn at 80% capacity
- Error if exceeded

**Memory estimation:**
```
1 second mono @ 44.1kHz = ~176KB
10 second loop = ~1.7MB
Kit of 16 one-shots (~0.5s each) = ~1.4MB
```

**Commands:**
```
S.MEM           → "Buffer: 12.4MB / 64MB (19%)"
S.FREE          → Free current kit buffers
```

#### Scene Persistence

Sample state saved with scene:

```json
{
  "sampler": {
    "kit_path": "drums/808/",
    "kit_mode": "directory",
    "num_slots": 16,
    "slice_count": null,
    "current_slot": 0,
    "playback": { ... },
    "fx": { ... }
  }
}
```

**On scene load:**
1. Check if kit path exists
2. If relative path, resolve against current library
3. If not found, search library for matching name
4. If still not found, warn but load scene without samples
5. Restore all parameters

#### Error Handling

| Situation | Behavior |
|-----------|----------|
| File not found | Error message, no change to current kit |
| Invalid format | Error message, skip file |
| Buffer full | Error message, partial load with warning |
| Empty directory | Warning, kit with 0 slots |
| Path outside library | Allowed (absolute paths work) |

### SuperCollider Implementation

#### SynthDef: monokit_sampler

```supercollider
SynthDef(\monokit_sampler, {
    arg out = 21,
        bufnum = 0,
        t_gate = 0,
        rate = 1,
        startFrame = 0,
        endFrame = -1,
        direction = 1,
        loop = 0,
        volume = 8192,
        pan = 0;

    var sig, env, frames, phasor, volNorm, panNorm;

    frames = BufFrames.kr(bufnum);
    endFrame = Select.kr(endFrame < 0, [endFrame, frames]);

    phasor = Phasor.ar(
        trig: t_gate,
        rate: rate * BufRateScale.kr(bufnum) * direction,
        start: startFrame,
        end: endFrame,
        resetPos: Select.kr(direction > 0, [endFrame, startFrame])
    );

    sig = BufRd.ar(1, bufnum, phasor, loop, 4);

    env = EnvGen.ar(
        Env.asr(0.001, 1, 0.01),
        gate: t_gate,
        doneAction: 0
    );

    volNorm = volume / 16383;
    panNorm = pan / 8192;

    sig = sig * env * volNorm;
    sig = Pan2.ar(sig, panNorm);

    Out.ar(out, sig);
}).add;
```

#### Buffer Management

- Pre-allocate buffer pool (128 buffers)
- Load via `/b_allocRead`
- Track buffer→slot mapping
- Free unused buffers on kit switch

### Scene Persistence

```json
{
  "sampler": {
    "mode": "kit",
    "path": "drums/808/",
    "num_slots": 16,
    "current_slot": 0,
    "slice_count": null,
    "playback": {
      "rate": 1.0,
      "direction": 1,
      "loop": false,
      "volume": 8192,
      "pan": 0
    }
  }
}
```

---

## Part 3: Voice Architecture Update

### Current Voices

| Node | Voice | Bus |
|------|-------|-----|
| 1000 | monokit_noise | 18 |
| 1001 | monokit_mod | 17 |
| 1002 | monokit_primary | 16 |
| 1003 | monokit_main | Mix + FX |
| 1004 | monokit_plaits | 19/20 |

### With Sampler

| Node | Voice | Bus |
|------|-------|-----|
| 1000 | monokit_noise | 18 |
| 1001 | monokit_mod | 17 |
| 1002 | monokit_primary | 16 |
| 1003 | monokit_main | Mix + FX |
| 1004 | monokit_plaits | 19/20 |
| 1005 | monokit_sampler | 21 |

### Per-Voice Control in monokit_main

Need to add:
- Volume control per voice before mix
- Pan control per voice
- Mute switches

---

## Implementation Phases

### Phase 1: Mixer Mode [~1 week]

- [ ] Add GRID.MODE 3 rendering
- [ ] Implement VOL.* commands per voice
- [ ] Implement PAN.* commands per voice
- [ ] Implement MUTE.* commands
- [ ] Update monokit_main SynthDef for per-voice control
- [ ] Level metering via SendReply

### Phase 2: Basic Sampler [~1 week]

- [ ] Create monokit_sampler SynthDef
- [ ] KIT command with path detection
- [ ] STR command with expression support
- [ ] Buffer allocation/management
- [ ] Integration with monokit_main

### Phase 3: Slicing [~1 week]

- [ ] S.SLICE equal division
- [ ] S.ONSET transient detection
- [ ] Slice→slot mapping
- [ ] Scene persistence

### Phase 4: Polish [~1 week]

- [ ] KIT.LOAD preloading
- [ ] GRID.MODE 4 FX viz
- [ ] Sample library browsing (KIT.LS)
- [ ] Help system updates
- [ ] Documentation

---

## Open Questions

1. **PN alias conflict** - PAN.NOS aliases to PN which conflicts with pattern next. Alternatives: `PNN`, `PANN`, or different scheme entirely.

2. **Trigger behavior** - Should `STR` also trigger the synth voices, or be sample-only? Current design: sample-only, use `TR` for synths.

3. **Polyphony** - Can multiple sample slots play simultaneously? Initial: no (monophonic like other voices). Future: optional 4-voice polyphony.

4. **Effects routing** - Should sampler go through filter/distortion or bypass to reverb/delay only? Recommend: configurable via `S.FX` command.

5. **Kit switching latency** - Need to measure actual buffer load times. If >50ms, preloading becomes important.

---

## UI Integration

### Header S Trigger Indicator

Add `S` indicator to header next to existing `H|P` section:
- Shows `S` when sampler triggers (STR command)
- Follows same activity pattern as other header indicators
- Brief flash on trigger, same timing as H|P

```
Header layout: ... H|P|S ...
- H = Help page indicator
- P = Pattern activity
- S = Sampler trigger (NEW)
```

## Success Criteria

- [ ] GRID.MODE 3 shows usable mixer with real-time levels
- [ ] Can load directory as kit and trigger with `STR 0-N`
- [ ] Can load WAV file and slice with `S.SLICE`
- [ ] Same patterns work regardless of kit vs slice mode
- [ ] Kit switching is fast enough for live use (<100ms)
- [ ] Scene save/load preserves sampler state
- [ ] Header shows S indicator on STR trigger
