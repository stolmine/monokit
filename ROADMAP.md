# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## Completed Features

### Core Voice & DSP
- [x] HD2-style dual oscillator with FM, discontinuity, and complex modulation
- [x] Full DSP signal chain: Oscillators → FM → Mix → Discontinuity → Lo-Fi → SVF Filter → Ring Mod → Comb Resonator → Amp → Compressor → Pan → Beat Repeat → Pitch Shift → Stereo Delay → 3-Band EQ → Plate Reverb
- [x] Extended discontinuity modes (0-6: fold, tanh, softclip, hard, asym, rectify, crush)
- [x] 77 real-time parameters controlling all aspects of voice and FX

### Effects System
- [x] Tier 1 DSP blocks: SVF Filter, Comb Resonator, Stereo Delay, Plate Reverb
- [x] Tier 2 DSP blocks: Lo-Fi, Ring Modulator, Compressor, 3-Band EQ, Pan
- [x] Beat Repeat with buffer freeze and stereo operation
- [x] Pitch Shift with normal and granular modes
- [x] Effect routing modes: BYPASS/INSERT/SEND with CUT/RING/FREEZE tail behaviors

### Language & Scripting
- [x] Page-based interface: LIVE, SCRIPT 1-8, Metro (M), Init (I), Pattern (P), Help
- [x] Script storage: 8 lines per script with local J, K variables
- [x] Pattern system: 6 patterns × 64 steps with comprehensive operations (P, PN, P.L, P.I, P.N, P.NEXT, P.PREV, P.HERE)
- [x] Variables: A, B, C, D, X, Y, Z, T (global), I (loop), J, K (per-script)
- [x] Control flow: IF/ELIF/ELSE, L (loop), PROB, EVERY, SKIP with PRE separator (`:`)
- [x] Comparison operators: EQ, NE, GT, LT, GTE, LTE, EZ, NZ (both prefix and infix)
- [x] Sub-command separator: `;` for multiple commands per line
- [x] Scene persistence: SAVE/LOAD system for scripts + patterns
- [x] MAP operator: Range mapping with clamping
- [x] TOG generator: Toggle between two values on each trigger
- [x] N1-N4 counters: Auto-increment variables with MIN/MAX/RST control

### Modulation & Routing
- [x] ModBus routing to filter cutoff (MF_F parameter)
- [x] Envelope system with PREFIX.SUFFIX naming (AENV, PENV, FMEV, DENV, FBEV, FLEV)
- [x] Per-envelope control: ATK, DEC, CRV, MODE, GATE for each envelope type
- [x] Global envelope controls: ENV.ATK, ENV.DEC, ENV.CRV, ENV.MODE, GATE
- [x] Envelope amounts: PA (pitch), FA (FM), DA (discontinuity)
- [x] Tracking system: TK (key tracking), MB (mod bus), MP/MD/MT/MA (routing switches)
- [x] Global parameter slew: SLEW.ALL with SC-side Lag.kr smoothing
- [x] Per-parameter slew: SLEW <param> <ms> for individual control

### Infrastructure
- [x] Dedicated metro thread with absolute timing (no cumulative drift)
- [x] Recording system: WAV int24 output with timestamped files
- [x] OSC protocol: CLI → SuperCollider communication
- [x] Theme system: Named themes with RGB colors, 30+ pre-defined themes
- [x] Command alias system: 93 aliases mapping PREFIX.SUFFIX to terse forms

---

## Phase 1: Core Utilities [COMPLETE]

**Focus:** Quick wins that add immediate value with minimal dependencies

### Slewing & Interpolation [Medium] - COMPLETE
- [x] `SLEW.ALL <ms>` - Global slew time for all parameters
- [x] `SLEW <param> <ms>` - Per-parameter slew override
- [x] SC-side Lag.kr smoothing for 30+ parameters

**Future Extensions:**
- [ ] `LERP <var> <target> <steps>` - Interpolate variable over N ticks
- [ ] `A.LERP <target> <ms>` - Time-based interpolation (runs in background, updates each metro tick)

### Envelope Shaping [Medium] - COMPLETE
- [x] `ENV.ATK <ms>` - Global attack time
- [x] `ENV.DEC <ms>` - Global decay time
- [x] `ENV.CRV <-8 to 8>` - Global envelope curve control (log/linear/exp)
- [x] `ENV.MODE <0-2>` - Global envelope modes (0=AD, 1=ASR, 2=ADSR)
- [x] `GATE <ms>` - Global gate duration
- [x] Per-envelope overrides for all 6 envelope types (AENV, PENV, FMEV, DENV, FBEV, FLEV)

### MAP Operator [Low] - COMPLETE
- [x] `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Range mapping with clamping

**Future Extensions:**
- [ ] `MAPU` - Unclamped mapping (allows extrapolation beyond output range)
- [ ] `MAP01 <val> <out_min> <out_max>` - Map from 0-1 range (normalized inputs)
- [ ] `MAP7 <val> <out_min> <out_max>` - Map from 0-127 range (MIDI values)
- [ ] `MAP14 <val> <out_min> <out_max>` - Map from 0-16383 range (14-bit)
- [ ] `MAPC <val> <in_min> <in_max> <out_min> <out_max> <curve>` - Map with curve (-8 to 8)

### TOG Generator [Low] - COMPLETE
- [x] `TOG <a> <b>` - Toggle between two values on each trigger
- [x] State is per-script and per-line

**Future Extensions:**
- [ ] `TOG <a> <b> <c>` - Cycle through 3+ values
- [ ] `TOG.RST` - Reset toggle state

### Auto-Increment Counters [Low] - COMPLETE
- [x] `N1`, `N2`, `N3`, `N4` - Auto-increment on each read
- [x] `N1.MIN <n>` - Set minimum value (default 0)
- [x] `N1.MAX <n>` - Set maximum value (wraps to MIN, 0=disabled)
- [x] `N1.RST` - Reset counter to MIN value

**Future Extensions:**
- [ ] `N1.STEP <n>` - Set increment amount (currently always 1)
- [ ] `A.INC <n>` - Increment variable A by n each tick
- [ ] `A.DEC <n>` - Decrement variable A by n each tick
- [ ] `A.WRAP <min> <max>` - Wrap variable within range

### Pattern Storage [Low] - COMPLETE
- [x] Increased from 4 to 6 pattern slots (PN accepts 0-5)

---

## Phase 2: Pattern Expansion

**Focus:** Building on existing pattern infrastructure with Teletype-inspired operations

### Pattern Manipulation [Medium] - COMPLETE
- [x] `P.PUSH <val>` / `PN.PUSH <pat> <val>` - Push value, shift left
- [x] `P.POP` / `PN.POP <pat>` - Return last value
- [x] `P.INS <idx> <val>` / `PN.INS <pat> <idx> <val>` - Insert at index
- [x] `P.RM <idx>` / `PN.RM <pat> <idx>` - Remove at index
- [x] `P.REV` / `PN.REV <pat>` - Reverse pattern
- [x] `P.ROT <n>` / `PN.ROT <pat> <n>` - Rotate pattern
- [x] `P.SHUF` / `PN.SHUF <pat>` - Shuffle randomly
- [x] `P.SORT` / `PN.SORT <pat>` - Sort ascending
- [x] `P.RND [min] [max]` / `PN.RND <pat> [min] [max]` - Randomize values

### Pattern Math [Low] - COMPLETE
- [x] `P.ADD <val>` / `PN.ADD <pat> <val>` - Add to all (saturating)
- [x] `P.SUB <val>` / `PN.SUB <pat> <val>` - Subtract from all (saturating)
- [x] `P.MUL <val>` / `PN.MUL <pat> <val>` - Multiply all (saturating)
- [x] `P.DIV <val>` / `PN.DIV <pat> <val>` - Divide all (zero-safe)
- [x] `P.MOD <val>` / `PN.MOD <pat> <val>` - Modulo all (zero-safe)
- [x] `P.SCALE <min> <max>` / `PN.SCALE <pat> <min> <max>` - Scale to range

### Pattern Queries [Low] - COMPLETE
- [x] `P.MIN` / `PN.MIN <pat>` - Return minimum value
- [x] `P.MAX` / `PN.MAX <pat>` - Return maximum value
- [x] `P.SUM` / `PN.SUM <pat>` - Return sum of all values
- [x] `P.AVG` / `PN.AVG <pat>` - Return average (integer)
- [x] `P.FND <val>` / `PN.FND <pat> <val>` - Find index (-1 if not found)

### Randomization System [Medium]

**Voice Randomization:**
- [ ] `RND.VOICE` - Randomize all oscillator/FM parameters within musical ranges
- [ ] `RND.OSC` - Randomize oscillator params only (PF, PW, MF, MW)
- [ ] `RND.FM` - Randomize FM-related params (FM, FB, FBA, FBD)

**Modulation Randomization:**
- [ ] `RND.MOD` - Randomize modulation routing (MB, TK, MP, MD, MT, MA)
- [ ] `RND.ENV` - Randomize envelope times and amounts

**FX Randomization:**
- [ ] `RND.FX` - Randomize all effect parameters
- [ ] `RND.FILT` - Randomize filter (FC, FQ, FT, FE)
- [ ] `RND.DLY` - Randomize delay (DT, DF, DLP, DW)
- [ ] `RND.VERB` - Randomize reverb (RV, RP, RH, RW)

**Pattern Randomization:**
- [ ] `RND.P` - Randomize working pattern values
- [ ] `RND.P <min> <max>` - Randomize within range
- [ ] `RND.PN <n>` - Randomize specific pattern
- [ ] `RND.PALL` - Randomize all patterns

---

## Phase 3: Musical Features

**Focus:** Musical utilities and external sync capabilities

### Scale Quantization [Medium]
- [ ] `Q <note>` - Quantize note to current scale
- [ ] `Q.SCALE <0-11>` - Set scale type
- [ ] `Q.ROOT <0-11>` - Set root note (C=0, C#=1, etc.)
- [ ] `PF N Q A` - Quantize variable to scale, convert to frequency

**Scale Types:**
- 0=Chromatic, 1=Major, 2=Minor, 3=Dorian, 4=Phrygian, 5=Lydian
- 6=Mixolydian, 7=Pentatonic Major, 8=Pentatonic Minor
- 9=Blues, 10=Whole Tone, 11=Diminished

### Mini Notation / Inline Sequencing [High]
- [ ] `SEQ "x _ x _"` - Simple trigger pattern notation
- [ ] `SEQ "200 400 300 _"` - Value sequence notation
- [ ] `PF SEQ "C3 E3 G3 C4"` - Note name support
- [ ] `TR SEQ "x _ [x x] _"` - Subdivision brackets

**Pattern Syntax:**
- `x` = trigger/value
- `_` or `.` = rest
- `[a b]` = subdivision
- `<a b>` = alternation
- `?` = random inclusion
- `*n` = repeat n times

### DAW / MIDI Clock Sync [High]

**MIDI Clock Input:**
- [ ] `M.SYNC <0-2>` - Sync mode (0=internal, 1=MIDI clock, 2=MIDI clock + transport)
- [ ] Auto-detect MIDI clock from connected devices
- [ ] Follow external tempo (24 PPQN standard)
- [ ] Start/stop follows MIDI transport commands

**MIDI Clock Output:**
- [ ] `M.SEND <0|1>` - Send MIDI clock out
- [ ] Send start/stop/continue messages

**Clock Division/Multiplication:**
- [ ] `M.DIV <1-16>` - Divide incoming clock
- [ ] `M.MUL <1-4>` - Multiply incoming clock

**Transport Control:**
- [ ] `PLAY`, `STOP`, `PAUSE` - Playback control
- [ ] `RST.POS` - Reset to beginning

**Optional:**
- [ ] `LINK <0|1>` - Ableton Link support (requires Link SDK)

### Additional ModBus Routing [Medium]
- [ ] ModBus → delay time routing
- [ ] ModBus → reverb size routing
- [ ] ModBus → resonator frequency routing

### Tempo-Synced Delay [Low]
- [ ] `DS` parameter - Delay time sync to metro (divisions: 1/4, 1/8, 1/16, etc.)

### Command Delay System (Teletype DEL) [Medium]
Scheduled command execution with delay buffer (inspired by Teletype).

**Basic Delay:**
- [ ] `DEL <ms>: <cmd>` - Execute command after delay (max 16000ms)
- [ ] `DEL.CLR` - Clear all pending delayed commands

**Repeated Delays:**
- [ ] `DEL.X <count> <ms>: <cmd>` - Queue command N times at intervals
  - Example: `DEL.X 4 100: TR` fires at 100ms, 200ms, 300ms, 400ms
- [ ] `DEL.R <count> <ms>: <cmd>` - Execute immediately, then repeat
  - Example: `DEL.R 4 100: TR` fires now, then at 100ms, 200ms, 300ms

**Advanced Delays:**
- [ ] `DEL.B <ms> <bitmask>: <cmd>` - Bitmasked delay (16-bit pattern)
  - LSB = immediate, each bit = one interval of <ms>
  - Example: `DEL.B 100 0b1010: TR` fires at 100ms, 300ms
- [ ] `DEL.G <ms> <exp>: <cmd>` - Exponential delay timing (non-linear)

**Implementation Notes:**
- Delay buffer holds 16-64 pending commands
- Commands execute on metro thread at scheduled time
- Buffer overflow behavior: oldest dropped or error

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
- [ ] Multiple destinations per LFO (optional)
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

### Help Page Styling [Low] - COMPLETE
- [x] Add explicit `#` prefix marker to HELP_LINES section headers
- [x] Replace heuristic-based styling with marker-based detection

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

### Waveform Preview [Medium] (Optional)
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

Feature requests and suggestions welcome. All contributions should maintain the project's terse command syntax and CLI-native philosophy.
