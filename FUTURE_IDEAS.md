# Monokit Future Ideas

## STATUS: BRAINSTORM / WISHLIST

This document captures future feature ideas for consideration. Not prioritized or committed.

---

## Randomization System [Medium Complexity]

### Voice Randomization
- `RND.VOICE` - Randomize all oscillator/FM parameters within musical ranges
- `RND.OSC` - Randomize just oscillator params (PF, PW, MF, MW)
- `RND.FM` - Randomize FM-related params (FM, FB, FBA, FBD)

### Modulation Randomization
- `RND.MOD` - Randomize modulation routing (MB, TK, MP, MD, MT, MA)
- `RND.ENV` - Randomize envelope times and amounts

### FX Randomization
- `RND.FX` - Randomize all effect parameters
- `RND.FILT` - Randomize filter (FC, FQ, FT, FE)
- `RND.DLY` - Randomize delay (DT, DF, DLP, DW)
- `RND.VERB` - Randomize reverb (RV, RP, RH, RW)

### Pattern Randomization
- `RND.P` - Randomize working pattern values
- `RND.P <min> <max>` - Randomize within range
- `RND.PN <n>` - Randomize specific pattern
- `RND.PALL` - Randomize all patterns

---

## LFO System [High Complexity]

### Core LFO Parameters
- 2-4 LFOs available (L1, L2, L3, L4)
- `L1.RATE <hz>` - LFO frequency (0.01-100 Hz)
- `L1.WAVE <0-4>` - Waveform (sin, tri, saw, square, random)
- `L1.AMP <0-16383>` - Amplitude/depth
- `L1.PHASE <0-360>` - Phase offset
- `L1.SYNC <0|1>` - Sync to metro

### LFO Routing
- `L1 -> PF` - Route LFO1 to primary frequency
- `L1.DEST <param>` - Set destination parameter
- `L1.AMT <0-16383>` - Modulation amount
- Multiple destinations per LFO?

### LFO Modifiers
- `L1.SLEW <ms>` - Slew/lag on LFO output
- `L1.QUANT <steps>` - Quantize LFO to N steps

---

## ✅ DONE: Slewing / Interpolation [Medium Complexity]

### Parameter Slew
- ✅ **IMPLEMENTED:** `SLEW <param> <ms>` - Set slew time for parameter changes
- ✅ **IMPLEMENTED:** `SLEW.ALL <ms>` - Global slew time
- ✅ **IMPLEMENTED:** Applied via SC-side Lag.kr smoothing for 30+ parameters

### Variable Lerp (Not Implemented)
- `LERP <var> <target> <steps>` - Interpolate variable over N ticks
- `A.LERP <target> <ms>` - Interpolate A to target over time
- Runs in background, updates each metro tick

---

## ✅ DONE: Map / Scale Operators [Low Complexity]

### Range Mapping
- ✅ **IMPLEMENTED:** `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Map value from input range to output range
- Example: `PF MAP A 0 127 200 2000` - Map A (0-127) to frequency (200-2000 Hz)
- Example: `DC MAP PN.NEXT 0 0 100 0 16383` - Map pattern value to full DC range

### Future Extensions (Not Implemented)
- `MAPU` - Unclamped, allows extrapolation beyond output range
- `MAP01 <val> <out_min> <out_max>` - Map from 0-1 range (for normalized inputs)
- `MAP7 <val> <out_min> <out_max>` - Map from 0-127 range (MIDI values)
- `MAP14 <val> <out_min> <out_max>` - Map from 0-16383 range (14-bit)
- `MAPC <val> <in_min> <in_max> <out_min> <out_max> <curve>` - Map with curve (curve: -8 to 8, -8=log, 0=linear, 8=exp)

---

## ✅ DONE: Envelope Shaping [Medium Complexity]

### Envelope Curve Control
- ✅ **IMPLEMENTED:** `ENV.CRV <-8 to 8>` - Global envelope curve (-8=log, 0=lin, 8=exp)
- ✅ **IMPLEMENTED:** Per-envelope override: `AENV.CRV`, `PENV.CRV`, `FMEV.CRV`, `DENV.CRV`, `FBEV.CRV`, `FLEV.CRV`

### Attack Times
- ✅ **IMPLEMENTED:** `ENV.ATK <ms>` - Global attack time (1-10000 ms)
- ✅ **IMPLEMENTED:** Per-envelope override: `AENV.ATK`, `PENV.ATK`, `FMEV.ATK`, `DENV.ATK`, `FBEV.ATK`, `FLEV.ATK`
- ✅ **IMPLEMENTED:** Converted from decay-only to full ADSR support

### Envelope Modes
- ✅ **IMPLEMENTED:** `ENV.MODE <0-2>` - 0=AD, 1=ASR (gate), 2=ADSR
- ✅ **IMPLEMENTED:** Per-envelope override via `AENV.MODE`, `PENV.MODE`, etc.

### Gate Duration
- ✅ **IMPLEMENTED:** `GATE <ms>` - Global gate duration (0-10000 ms, 0=instant trigger)
- ✅ **IMPLEMENTED:** Per-envelope override: `AENV.GATE`, `PENV.GATE`, `FMEV.GATE`, `DENV.GATE`, `FBEV.GATE`, `FLEV.GATE`

---

## ✅ DONE: TOG Generator [Low Complexity]

Toggle between values on each trigger:
- ✅ **IMPLEMENTED:** `TOG <a> <b>` - Returns a, then b, then a, then b...
- State is per-script and per-line
- Usage: `PF TOG N 0 N 7` - Alternate between C3 and G3

### Future Extensions (Not Implemented)
- `TOG <a> <b> <c>` - Cycle through 3+ values (currently only 2 values supported)
- `TOG.RST` - Reset toggle state

---

## Mini Notation / Inline Sequencing [High Complexity]

String-based pattern notation (inspired by Tidal):
- `SEQ "x _ x _"` - Simple trigger pattern
- `SEQ "200 400 300 _"` - Value sequence
- `PF SEQ "C3 E3 G3 C4"` - Note names
- `TR SEQ "x _ [x x] _"` - Subdivision brackets

### Pattern Syntax
- `x` = trigger/value
- `_` or `.` = rest
- `[a b]` = subdivision
- `<a b>` = alternation
- `?` = random inclusion
- `*n` = repeat n times

---

## Remaining Pattern Ops (from Teletype) [Low-Medium Complexity]

### Pattern Manipulation [Medium]
- `P.PUSH <val>` - Push value, shift pattern
- `P.POP` - Pop and return last value
- `P.INS <idx> <val>` - Insert at index
- `P.RM <idx>` - Remove at index
- `P.REV` - Reverse pattern
- `P.ROT <n>` - Rotate pattern by n
- `P.SHUF` - Shuffle pattern randomly
- `P.SORT` - Sort pattern ascending
- `P.RND` - Randomize all values

### Pattern Math [Low]
- `P.ADD <val>` - Add to all values
- `P.SUB <val>` - Subtract from all
- `P.MUL <val>` - Multiply all
- `P.DIV <val>` - Divide all
- `P.MOD <val>` - Modulo all
- `P.SCALE <min> <max>` - Scale to range

### Pattern Queries [Low]
- `P.MIN` - Get minimum value
- `P.MAX` - Get maximum value
- `P.SUM` - Sum of all values
- `P.AVG` - Average of all values
- `P.FND <val>` - Find index of value

---

## Scale Quantization [Medium Complexity]

### Quantizer
- `Q <note>` - Quantize note to current scale
- `Q.SCALE <0-11>` - Set scale (major, minor, dorian, etc.)
- `Q.ROOT <0-11>` - Set root note (C=0, C#=1, etc.)
- `PF N Q A` - Quantize variable A to scale, convert to freq

### Scale Types
- 0 = Chromatic
- 1 = Major
- 2 = Minor
- 3 = Dorian
- 4 = Phrygian
- 5 = Lydian
- 6 = Mixolydian
- 7 = Pentatonic Major
- 8 = Pentatonic Minor
- 9 = Blues
- 10 = Whole Tone
- 11 = Diminished

---

## CPU / Performance Metering [Medium Complexity]

### CPU Meter
- `CPU` - Display current SC server CPU usage
- Show in UI status bar
- Warning when >80%

### Audio Metering
- `METER` - Show current output level
- Peak hold display
- Clip indicator

---

## Live Parameter Visualization [Medium Complexity]

### Variable Display
- Dedicated UI area showing A, B, C, D, X, Y, Z, T values
- Update in real-time as they change
- Color coding for recently changed

### Parameter Feedback
- Show last-changed parameter and value
- Brief flash/highlight on change
- Optional: parameter history log

### Nav Icon Activity
- Highlight page icons when that script executes
- M icon pulses on metro tick
- Script 1-8 icons flash when called
- Visual feedback for SCRIPT command execution

---

## ✅ DONE: Auto-Incrementing Variables [Low Complexity]

### Dedicated Counter Variables
- ✅ **IMPLEMENTED:** `N1`, `N2`, `N3`, `N4` - Auto-increment on each read
- ✅ **IMPLEMENTED:** `N1.RST`, `N2.RST`, `N3.RST`, `N4.RST` - Reset to MIN value
- ✅ **IMPLEMENTED:** `N1.MAX <n>` - Set maximum value (wraps to MIN when exceeded, 0=disabled/no wrap)
- ✅ **IMPLEMENTED:** `N1.MIN <n>` - Set minimum value (default 0, counter starts here)
- Example: `N1.MIN 10; N1.MAX 14` - Counter cycles 10,11,12,13,14,10...

### Future Extensions (Not Implemented)
- `N1.STEP <n>` - Set increment amount (currently always increments by 1)
- `A.INC <n>` - Increment A by n each tick
- `A.DEC <n>` - Decrement A by n each tick
- `A.WRAP <min> <max>` - Wrap within range

---

## Expanded Pattern Storage [Low Complexity]

### More Pattern Slots
- ✅ **PARTIALLY DONE:** Increased from 4 to 6 patterns (PN accepts 0-5)
- Future: Increase to 8 or 16 patterns
- Future: `PN <0-15>` addressing
- Future: Bank system: `P.BANK <0-3>` for 4 banks of 4

### Pattern Length
- Future: Increase max from 64 to 128 or 256
- Future: Variable per-pattern length limits

---

## Visual Feedback (KO II Style) [Medium Complexity]

### Live Screen Visualization
- Icon grid on Live page
- Icons light up on events:
  - TR = trigger icon
  - Metro tick = pulse icon
  - Pattern advance = step icon
  - Script execution = script number icon
- Decay/fade animation

### Activity Indicators
- Per-parameter activity dots
- Brightness = recent change
- Color = parameter category

### Waveform Preview
- Mini oscilloscope on Live page
- Show output waveform in real-time
- Optional: spectrum analyzer

---

## DAW / MIDI Clock Sync [High Complexity]

### MIDI Clock Input
- `M.SYNC <0-2>` - Sync mode (0=internal, 1=MIDI clock, 2=MIDI clock + transport)
- Auto-detect MIDI clock from connected devices
- Follow external tempo (24 PPQN standard)
- Start/stop follows MIDI transport commands

### MIDI Clock Output
- `M.SEND <0|1>` - Send MIDI clock out
- Other apps can sync to monokit as master
- Send start/stop/continue messages

### Ableton Link
- `LINK <0|1>` - Enable Ableton Link
- Sync with DAWs and apps on same network
- Shared tempo and phase
- Requires: Link SDK integration

### Clock Division/Multiplication
- `M.DIV <1-16>` - Divide incoming clock
- `M.MUL <1-4>` - Multiply incoming clock
- Allows different relationships to master clock

### Transport Control
- `PLAY` / `STOP` - Start/stop playback
- `PAUSE` - Pause without reset
- `RST.POS` - Reset to beginning
- Respond to MIDI transport (MMC or MIDI realtime)

---

## Final Boss Features

### Additional Voice Types [Very High Complexity]
- Voice type selector: `VOICE <0-N>`
- 0 = Current HD2-style complex oscillator
- 1 = FM (DX-style 4-op)
- 2 = Subtractive (classic analog)
- 3 = Wavetable
- 4 = Physical modeling (Karplus-Strong)
- 5 = Noise/percussion focused

### Plaits Macro Voice [Very High Complexity]
- Import Mutable Instruments Plaits algorithms
- `PLAITS.MODEL <0-15>` - Select model
- `PLAITS.TIMBRE` - Timbre control
- `PLAITS.MORPH` - Morph control
- `PLAITS.HARM` - Harmonics
- Requires: SC Plaits UGen or port

### Optional Polyphony [Very High Complexity]
- `POLY <1-8>` - Number of voices
- Voice allocation: round-robin or lowest
- Per-voice detuning
- Unison mode with spread

### Unified Installation [High Complexity]
- Single installer package
- Bundles:
  - Rust CLI binary
  - SuperCollider runtime (scsynth)
  - SC SynthDefs
  - Default config/themes
- Platform installers:
  - macOS: .pkg or Homebrew
  - Linux: .deb, .rpm, AppImage
  - Windows: .msi or portable
- Auto-start SC server on launch
- No manual SC installation required

---

## Implementation Phases

### ✅ Phase 1: Core Utilities (Low-Medium Complexity) - COMPLETE
Quick wins that add immediate value with minimal dependencies:
- ✅ **MAP operator** [Low] - Simple math transformations, no state management required
- ✅ **TOG generator** [Low] - Basic state toggle with simple cycling logic
- ✅ **Auto-increment variables** [Low] - Simple counter logic with wrap-around
- ✅ **More pattern slots** [Low] - Just increase array size in existing pattern system
- ✅ **Slewing/LERP** [Medium] - Needs background interpolation thread, parameter smoothing
- ✅ **Envelope shaping** [Medium] - Extend existing envelope system with curve control and attack times

### Phase 2: Pattern Expansion (Low-Medium Complexity)
Building on existing pattern infrastructure:
- **Remaining TT pattern ops** [Medium] - Port existing Teletype logic (P.PUSH, P.REV, P.ROT, P.SHUF, P.SORT, etc.)
- **Pattern math operations** [Low] - Simple arithmetic on pattern values (P.ADD, P.MUL, P.SCALE)
- **Pattern queries** [Low] - Read-only stats (P.MIN, P.MAX, P.SUM, P.AVG)
- **Randomization system** [Medium] - Random within ranges for all subsystems (RND.VOICE, RND.FX, RND.P, etc.)

### Phase 3: Musical Features (Medium-High Complexity)
Requires new lookup tables and parsing logic:
- **Scale quantization** [Medium] - Scale lookup tables, note mapping logic
- **Mini notation/sequencing** [High] - String parser needed, syntax handling, pattern compilation
- **DAW/MIDI clock sync** [High] - External protocol handling (MIDI clock 24 PPQN, MMC, Link SDK)

### Phase 4: Modulation System (High Complexity)
New synthesis infrastructure with routing requirements:
- **LFO system** [High] - New SC UGens, waveform generators, modulation routing matrix
- Dependencies: Needs routing system for LFO → parameter mapping, phase sync to metro

### Phase 5: UI/Feedback (Medium Complexity)
Visual enhancements requiring UI layout changes:
- **Variable display** [Medium] - Real-time UI panel for A, B, C, D, X, Y, Z, T values
- **Parameter visualization** [Medium] - Show last-changed params, flash/highlight on change
- **Activity indicators** [Medium] - Event tracking, icon highlighting, decay animations (KO II style)
- **CPU/metering** [Medium] - SC server queries for CPU usage, audio level monitoring, peak hold

### Phase 6: Advanced DSP (Very High Complexity)
Major architectural changes requiring deep SC knowledge:
- **Additional voice types** [Very High] - Multiple new SynthDefs (FM, wavetable, physical modeling, etc.)
- **Plaits integration** [Very High] - External UGen dependency or full port of Plaits algorithms
- **Polyphony** [Very High] - Voice allocation system, state management per voice, unison modes
- Dependencies: Each voice type needs full DSP implementation, testing, and parameter mapping

### Phase 7: Distribution (High Complexity)
Packaging and deployment infrastructure:
- **Unified installer** [High] - Bundle Rust binary + SC runtime + SynthDefs + config
- Platform-specific packaging (macOS .pkg, Linux .deb/.rpm, Windows .msi)
- Auto-start SC server, dependency management
- Dependencies: All features must be complete and stable before packaging

---

## Notes

- Features should maintain CLI-first philosophy
- All new params should support expression evaluation
- Keep commands terse (Teletype-inspired)
- Consider CPU impact for real-time features
- UI features should be optional/toggleable
