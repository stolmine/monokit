# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## ✅ Completed Features

### Core Voice & DSP
- **HD2-style dual oscillator** with FM, discontinuity, and complex modulation
- **Full DSP signal chain:** Oscillators → FM → Mix → Discontinuity → Lo-Fi → SVF Filter → Ring Mod → Comb Resonator → Amp → Compressor → Pan → Stereo Delay → 3-Band EQ → Plate Reverb
- **Extended discontinuity modes** (0-6: fold, tanh, softclip, hard, asym, rectify, crush)
- **66 real-time parameters** controlling all aspects of voice and FX

### Effects System
- **Tier 1 DSP blocks:** SVF Filter, Comb Resonator, Stereo Delay, Plate Reverb
- **Tier 2 DSP blocks:** Lo-Fi, Ring Modulator, Compressor, 3-Band EQ, Pan
- **Effect routing modes:** BYPASS/INSERT/SEND with CUT/RING/FREEZE tail behaviors

### Language & Scripting (PLAN.md - All Phases Complete)
- **Page-based interface:** LIVE, SCRIPT 1-8, Metro (M), Init (I), Pattern (P), Help
- **Script storage:** 8 lines per script with local J, K variables
- **Pattern system:** 6 patterns × 64 steps with comprehensive operations (P, PN, P.L, P.I, P.N, P.NEXT, P.PREV, P.HERE)
- **Variables:** A, B, C, D, X, Y, Z, T (global), I (loop), J, K (per-script)
- **Control flow:** IF/ELIF/ELSE, L (loop), PROB, EVERY, SKIP with PRE separator (`:`)
- **Comparison operators:** EQ, NE, GT, LT, GTE, LTE, EZ, NZ (both prefix and infix)
- **Sub-command separator:** `;` for multiple commands per line
- **Scene persistence:** SAVE/LOAD system for scripts + patterns
- **MAP operator:** Range mapping with clamping
- **TOG generator:** Toggle between values on each trigger
- **N1-N4 counters:** Auto-increment variables with MIN/MAX/RST control

### Modulation & Routing
- **ModBus routing** to filter cutoff (MF_F parameter)
- **Envelope system:** PA (pitch), FA (FM), DA (discontinuity) amounts with dedicated decay times
- **Tracking system:** TK (key tracking), MB (mod bus), MP/MD/MT/MA (routing switches)
- **Global parameter slew** via SLEW.ALL with SC-side Lag.kr smoothing

### Infrastructure
- **Dedicated metro thread** with absolute timing (no cumulative drift)
- **Recording system:** WAV int24 output with timestamped files
- **OSC protocol:** CLI → SuperCollider communication
- **Theme system:** RGB color support with dark/light variants and system detection

---

## Phase 1: Core Utilities

**Focus:** Quick wins that add immediate value with minimal dependencies

### Slewing & Interpolation [Medium]
- [x] `SLEW.ALL <ms>` - Global slew time for all parameters ✓
- [x] SC-side Lag.kr smoothing for 30+ parameters ✓
- [x] `SLEW <param> <ms>` - Per-parameter slew override ✓
- [ ] `LERP <var> <target> <steps>` - Interpolate variable over N ticks (optional)
- [ ] `A.LERP <target> <ms>` - Time-based interpolation (optional)

### Envelope Shaping [Medium]
- [ ] `AD.CURVE <-8 to 8>` - Amp envelope curve control (log/linear/exp)
- [ ] `PD.CURVE`, `FD.CURVE`, `DD.CURVE` - Per-envelope curve shaping
- [ ] `AA <ms>` - Amp attack time
- [ ] `PAT <ms>` - Pitch attack time
- [ ] `ENV.MODE <0-2>` - Envelope modes (0=perc/AD, 1=ASR, 2=ADSR)

---

## Phase 2: Pattern Expansion

**Focus:** Building on existing pattern infrastructure with Teletype-inspired operations

### Pattern Manipulation [Medium]
- [ ] `P.PUSH <val>` - Push value, shift pattern
- [ ] `P.POP` - Pop and return last value
- [ ] `P.INS <idx> <val>` - Insert at index
- [ ] `P.RM <idx>` - Remove at index
- [ ] `P.REV` - Reverse pattern
- [ ] `P.ROT <n>` - Rotate pattern by n positions
- [ ] `P.SHUF` - Shuffle pattern randomly
- [ ] `P.SORT` - Sort pattern ascending

### Pattern Math [Low]
- [ ] `P.ADD <val>` - Add to all values
- [ ] `P.SUB <val>` - Subtract from all values
- [ ] `P.MUL <val>` - Multiply all values
- [ ] `P.DIV <val>` - Divide all values
- [ ] `P.MOD <val>` - Modulo all values
- [ ] `P.SCALE <min> <max>` - Scale pattern to range

### Pattern Queries [Low]
- [ ] `P.MIN` - Get minimum value
- [ ] `P.MAX` - Get maximum value
- [ ] `P.SUM` - Sum of all values
- [ ] `P.AVG` - Average of all values
- [ ] `P.FND <val>` - Find index of value

### Pattern Storage Expansion [Low]
- [x] Increase from 4 to 6 pattern slots ✓
- [ ] Increase to 8 or 16 pattern slots (optional)
- [ ] Optional: Bank system `P.BANK <0-3>` for 4 banks of 4
- [ ] Optional: Increase max length from 64 to 128 or 256

### Randomization System [Medium]
- [ ] `RND.VOICE` - Randomize oscillator/FM parameters within musical ranges
- [ ] `RND.OSC` - Randomize oscillator params only
- [ ] `RND.FM` - Randomize FM-related params
- [ ] `RND.MOD` - Randomize modulation routing
- [ ] `RND.ENV` - Randomize envelope times and amounts
- [ ] `RND.FX` - Randomize all effect parameters
- [ ] `RND.FILT`, `RND.DLY`, `RND.VERB` - Per-effect randomization
- [ ] `RND.P` - Randomize working pattern values
- [ ] `RND.P <min> <max>` - Randomize within range
- [ ] `RND.PN <n>`, `RND.PALL` - Specific or all patterns

---

## Phase 3: Musical Features

**Focus:** Musical utilities and external sync capabilities

### Scale Quantization [Medium]
- [ ] `Q <note>` - Quantize note to current scale
- [ ] `Q.SCALE <0-11>` - Set scale (major, minor, dorian, etc.)
- [ ] `Q.ROOT <0-11>` - Set root note (C=0, C#=1, etc.)
- [ ] `PF N Q A` - Quantize variable to scale, convert to frequency
- [ ] 12 scale types: Chromatic, Major, Minor, Dorian, Phrygian, Lydian, Mixolydian, Pentatonic Major/Minor, Blues, Whole Tone, Diminished

### Mini Notation / Inline Sequencing [High]
- [ ] `SEQ "x _ x _"` - Simple trigger pattern notation
- [ ] `SEQ "200 400 300 _"` - Value sequence notation
- [ ] `PF SEQ "C3 E3 G3 C4"` - Note name support
- [ ] `TR SEQ "x _ [x x] _"` - Subdivision brackets
- [ ] Pattern syntax: `x` (trigger), `_` (rest), `[a b]` (subdivision), `<a b>` (alternation), `?` (random), `*n` (repeat)

### DAW / MIDI Clock Sync [High]
- [ ] `M.SYNC <0-2>` - Sync mode (0=internal, 1=MIDI clock, 2=MIDI clock + transport)
- [ ] Auto-detect MIDI clock from connected devices
- [ ] Follow external tempo (24 PPQN standard)
- [ ] Start/stop follows MIDI transport commands
- [ ] `M.SEND <0|1>` - Send MIDI clock out
- [ ] `M.DIV <1-16>` - Divide incoming clock
- [ ] `M.MUL <1-4>` - Multiply incoming clock
- [ ] `PLAY`, `STOP`, `PAUSE`, `RST.POS` - Transport control
- [ ] Optional: `LINK <0|1>` - Ableton Link support

### Additional ModBus Routing (from CONCEPT.md) [Medium]
- [ ] ModBus → delay time routing
- [ ] ModBus → reverb size routing
- [ ] ModBus → resonator frequency routing

### Tempo-Synced Delay (from CONCEPT.md) [Low]
- [ ] `DS` parameter - Delay time sync to metro (divisions: 1/4, 1/8, 1/16, etc.)

---

## Phase 4: Modulation System

**Focus:** New synthesis infrastructure with LFO routing matrix

### LFO System [High]
- [ ] 2-4 LFO units (L1, L2, L3, L4)
- [ ] `L1.RATE <hz>` - LFO frequency (0.01-100 Hz)
- [ ] `L1.WAVE <0-4>` - Waveform (sin, tri, saw, square, random)
- [ ] `L1.AMP <0-16383>` - Amplitude/depth
- [ ] `L1.PHASE <0-360>` - Phase offset
- [ ] `L1.SYNC <0|1>` - Sync to metro
- [ ] `L1.DEST <param>` - Set destination parameter
- [ ] `L1.AMT <0-16383>` - Modulation amount
- [ ] `L1.SLEW <ms>` - Slew/lag on LFO output
- [ ] `L1.QUANT <steps>` - Quantize LFO to N steps
- [ ] SC implementation: New UGens, routing matrix, phase sync

---

## Phase 5: UI/Feedback

**Focus:** Visual enhancements and real-time parameter monitoring

### Variable Display [Medium]
- [ ] Dedicated UI area showing A, B, C, D, X, Y, Z, T values
- [ ] Real-time updates as variables change
- [ ] Color coding for recently changed variables

### Parameter Feedback [Medium]
- [ ] Show last-changed parameter and value
- [ ] Flash/highlight on parameter change
- [ ] Optional: Parameter history log

### Activity Indicators (KO II Style) [Medium]
- [ ] Page icon highlighting when script executes
- [ ] M icon pulses on metro tick
- [ ] Script 1-8 icons flash when called
- [ ] Live screen event visualization (trigger, metro, pattern, script icons)
- [ ] Decay/fade animations
- [ ] Per-parameter activity dots with brightness/color coding

### CPU & Audio Metering [Medium]
- [ ] `CPU` - Display SuperCollider server CPU usage
- [ ] Status bar CPU meter with warning at >80%
- [ ] `METER` - Audio output level metering
- [ ] Peak hold display
- [ ] Clip indicator

### Optional: Waveform Preview [Medium]
- [ ] Mini oscilloscope on Live page
- [ ] Real-time output waveform display
- [ ] Optional: Spectrum analyzer

---

## Phase 6: Advanced DSP

**Focus:** Major architectural changes requiring deep SuperCollider work

### Additional Voice Types [Very High]
- [ ] `VOICE <0-N>` - Voice type selector
- [ ] Voice 0: Current HD2-style complex oscillator (default)
- [ ] Voice 1: FM (DX-style 4-operator)
- [ ] Voice 2: Subtractive (classic analog)
- [ ] Voice 3: Wavetable
- [ ] Voice 4: Physical modeling (Karplus-Strong)
- [ ] Voice 5: Noise/percussion focused
- [ ] Full SynthDef implementation per voice type
- [ ] Parameter mapping and compatibility layer

### Plaits Macro Voice [Very High]
- [ ] Import Mutable Instruments Plaits algorithms
- [ ] `PLAITS.MODEL <0-15>` - Select Plaits model
- [ ] `PLAITS.TIMBRE` - Timbre control
- [ ] `PLAITS.MORPH` - Morph control
- [ ] `PLAITS.HARM` - Harmonics control
- [ ] Dependencies: SC Plaits UGen or full port of algorithms

### Optional Polyphony [Very High]
- [ ] `POLY <1-8>` - Number of voices
- [ ] Voice allocation: round-robin or lowest
- [ ] Per-voice detuning
- [ ] Unison mode with spread
- [ ] State management per voice
- [ ] Complex routing and mixing requirements

---

## Phase 7: Distribution

**Focus:** Packaging and deployment infrastructure

### Unified Installer [High]
- [ ] Single installer package bundling:
  - Rust CLI binary
  - SuperCollider runtime (scsynth)
  - SC SynthDefs
  - Default config/themes
- [ ] Platform-specific installers:
  - macOS: .pkg or Homebrew formula
  - Linux: .deb, .rpm, AppImage
  - Windows: .msi or portable .exe
- [ ] Auto-start SC server on launch
- [ ] No manual SuperCollider installation required
- [ ] Dependency management and version checking

---

## Implementation Notes

### Design Principles
- Maintain CLI-first philosophy throughout
- All new parameters must support expression evaluation
- Keep commands terse (Teletype-inspired)
- Consider CPU impact for real-time features
- UI features should be optional/toggleable
- Never expose secrets, keys, or sensitive data
- Fail fast with clear errors

### Dependencies Between Phases
- **Phase 2** can proceed independently (builds on existing pattern system)
- **Phase 3** scale quantization needed before mini notation (note name parsing)
- **Phase 4** LFO system may inform additional Phase 3 modulation routing
- **Phase 5** UI features can be implemented incrementally, no blocking dependencies
- **Phase 6** requires all features complete and stable before voice architecture changes
- **Phase 7** requires all features complete and tested before packaging

### Complexity Legend
- **[Low]** - 1-3 days, minimal dependencies, straightforward implementation
- **[Medium]** - 1-2 weeks, moderate complexity, some new infrastructure
- **[High]** - 2-4 weeks, significant new systems, external dependencies
- **[Very High]** - 4+ weeks, major architectural changes, deep domain expertise required

---

## Contributing

Feature requests and suggestions should be added to `future_ideas.md` for consideration. All contributions should maintain the project's terse command syntax and CLI-native philosophy.
