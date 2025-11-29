# Monokit Future Ideas

## STATUS: BRAINSTORM / WISHLIST

This document captures future feature ideas for consideration. Not prioritized or committed.

---

## Randomization System

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

## LFO System

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

## Slewing / Interpolation

### Parameter Slew
- `SLEW <param> <ms>` - Set slew time for parameter changes
- `SLEW.ALL <ms>` - Global slew time
- Applied to any parameter change (instant → smooth)

### Variable Lerp
- `LERP <var> <target> <steps>` - Interpolate variable over N ticks
- `A.LERP <target> <ms>` - Interpolate A to target over time
- Runs in background, updates each metro tick

---

## Envelope Shaping

### Envelope Curve Control
- `AD.CURVE <-8 to 8>` - Amp envelope curve (-8=log, 0=lin, 8=exp)
- `PD.CURVE` - Pitch envelope curve
- `FD.CURVE` - FM envelope curve
- `DD.CURVE` - Discontinuity envelope curve

### Attack Times
- `AA <ms>` - Amp attack time
- `PAT <ms>` - Pitch attack time
- Convert from decay-only to full ADSR?

### Envelope Modes
- `ENV.MODE <0-2>` - 0=perc (AD), 1=ASR (gate), 2=ADSR

---

## TOG Generator

Toggle between values on each trigger:
- `TOG <a> <b>` - Returns a, then b, then a, then b...
- `TOG <a> <b> <c>` - Cycle through 3+ values
- `TOG.RST` - Reset toggle state
- Usage: `PF TOG 200 400` - Alternate between frequencies

---

## Mini Notation / Inline Sequencing

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

## Remaining Pattern Ops (from Teletype)

### Pattern Manipulation
- `P.PUSH <val>` - Push value, shift pattern
- `P.POP` - Pop and return last value
- `P.INS <idx> <val>` - Insert at index
- `P.RM <idx>` - Remove at index
- `P.REV` - Reverse pattern
- `P.ROT <n>` - Rotate pattern by n
- `P.SHUF` - Shuffle pattern randomly
- `P.SORT` - Sort pattern ascending
- `P.RND` - Randomize all values

### Pattern Math
- `P.ADD <val>` - Add to all values
- `P.SUB <val>` - Subtract from all
- `P.MUL <val>` - Multiply all
- `P.DIV <val>` - Divide all
- `P.MOD <val>` - Modulo all
- `P.SCALE <min> <max>` - Scale to range

### Pattern Queries
- `P.MIN` - Get minimum value
- `P.MAX` - Get maximum value
- `P.SUM` - Sum of all values
- `P.AVG` - Average of all values
- `P.FND <val>` - Find index of value

---

## Scale Quantization

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

## CPU / Performance Metering

### CPU Meter
- `CPU` - Display current SC server CPU usage
- Show in UI status bar
- Warning when >80%

### Audio Metering
- `METER` - Show current output level
- Peak hold display
- Clip indicator

---

## Live Parameter Visualization

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

## Auto-Incrementing Variables

### Dedicated Counter Variables
- `N1`, `N2`, `N3`, `N4` - Auto-increment on each read
- `N1.RST` - Reset to 0
- `N1.MAX <n>` - Set wrap-around point
- `N1.STEP <n>` - Set increment amount

### Accumulator Mode
- `A.INC <n>` - Increment A by n each tick
- `A.DEC <n>` - Decrement A by n each tick
- `A.WRAP <min> <max>` - Wrap within range

---

## Expanded Pattern Storage

### More Pattern Slots
- Increase from 4 to 8 or 16 patterns
- `PN <0-15>` addressing
- Bank system: `P.BANK <0-3>` for 4 banks of 4

### Pattern Length
- Increase max from 64 to 128 or 256
- Or variable per-pattern length limits

---

## Visual Feedback (KO II Style)

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

## DAW / MIDI Clock Sync

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

### Additional Voice Types
- Voice type selector: `VOICE <0-N>`
- 0 = Current HD2-style complex oscillator
- 1 = FM (DX-style 4-op)
- 2 = Subtractive (classic analog)
- 3 = Wavetable
- 4 = Physical modeling (Karplus-Strong)
- 5 = Noise/percussion focused

### Plaits Macro Voice
- Import Mutable Instruments Plaits algorithms
- `PLAITS.MODEL <0-15>` - Select model
- `PLAITS.TIMBRE` - Timbre control
- `PLAITS.MORPH` - Morph control
- `PLAITS.HARM` - Harmonics
- Requires: SC Plaits UGen or port

### Optional Polyphony
- `POLY <1-8>` - Number of voices
- Voice allocation: round-robin or lowest
- Per-voice detuning
- Unison mode with spread

### Unified Installation
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

## Implementation Priority Tiers

### Tier 4 (Next)
- LFO system
- Slewing
- TOG generator
- More pattern ops
- Variable display in UI

### Tier 5 (Medium Term)
- Scale quantization
- Mini notation
- Pattern randomization
- Activity visualization
- DAW/MIDI clock sync

### Tier 6 (Long Term)
- CPU/metering
- Envelope shaping
- Auto-increment variables
- More pattern slots

### Tier 7 (Aspirational)
- Additional voice types
- Plaits integration
- Polyphony
- Unified installer

---

## Notes

- Features should maintain CLI-first philosophy
- All new params should support expression evaluation
- Keep commands terse (Teletype-inspired)
- Consider CPU impact for real-time features
- UI features should be optional/toggleable
