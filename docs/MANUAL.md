# Monokit Manual

**Version 0.1.0** - Teletype-style scripting for a complex oscillator voice

---

## Quick Start

### First Launch

Run `monokit` in your terminal. The app is designed for a 50x18 terminal window but will scale to your window size.

On launch, monokit:
1. Spawns the SuperCollider audio engine (scsynth)
2. Loads the voice synthesizer
3. Opens with a silent voice (VCA mode = GATED)

### Making Your First Sound

```
TR              # Trigger the voice
PF 200          # Set primary oscillator to 200 Hz
TR              # Trigger again to hear the change
AD 500          # Set amplitude decay to 500ms
TR              # Trigger - longer tail
```

The voice starts silent (VCA=1, gated mode). Each `TR` command triggers all envelopes.

### Basic Workflow

1. **Trigger**: `TR` fires all envelopes
2. **Set parameters**: Commands like `PF`, `AD`, `FC` set voice params
3. **Use scripts**: Navigate to Script 1-8 (F1-F8) to build sequences
4. **Metro**: F10 opens Metro page - set `M.ACT 1` to start clock
5. **Save**: `SAVE mysound` saves everything (scripts + patterns + synth state)

---

## Interface

### Pages Overview

| Page | Key | Description |
|------|-----|-------------|
| **Live** | F9, Alt+L | REPL and parameter activity grid |
| **Script 1-8** | F1-F8, Alt+1-8 | 8 lines each, local J/K vars |
| **Metro** | F10, Alt+M | Runs on clock (internal or MIDI) |
| **Init** | F11, Alt+I | Runs on scene load |
| **Patterns** | F12, Alt+P | 6 patterns × 64 steps |
| **Variables** | Alt+V | Monitor all variable state |
| **Notes** | Alt+N | 8 lines of text notes |
| **Scope** | Alt+S | Real-time oscilloscope |
| **Help** | ESC, Alt+H | Searchable help system |

### Navigation

**Page Navigation:**
- `[ ]` - Cycle through pages
- `ESC` - Toggle help on/off
- `Alt+<key>` - Direct page access (requires iTerm2 config)
- `Tab` - On Live page: toggle REPL/Grid view

**Script Editing (Script pages):**
- `Up/Down` - Select line
- `Enter` - Save edited line
- `Ctrl+D` - Duplicate line
- `Ctrl+K` - Delete line
- `Ctrl+C/X/V` - Copy/cut/paste line
- `Ctrl+Left/Right` - Word movement

**REPL (Live page):**
- `Ctrl+Up/Down` - Scroll output history
- `CLEAR` - Clear all output

**Search:**
- `Ctrl+F` - Enter search mode
- `Enter` - Next match
- `Shift+Enter` - Previous match
- `ESC` - Exit search
- Help page searches help only
- Other pages search scripts

### Meters and Indicators

**Header Display:**
- **Left**: Current page name
- **Center**: "MONOKIT" or scene name (see `TITLE`)
- **Right**: CPU%, BPM, REC indicator
- **Meters**: `L▅▆ R▅▅` bargraph (peak/RMS, clip detection)

**Activity Indicators:**
- Script indicators (1-8, M, I) pulse when executing
- TR indicator pulses on each trigger
- Smooth decay animation (200ms hold, adjust with `FLASH`)

**Grid View (Tab on Live page):**
- 48 parameter icons in 8×6 grid
- Icons light up when parameters change
- Vertical stereo meters (left side, 8 rows tall)
- 15-band spectrum analyzer (bottom, 2 rows tall)

### Oscilloscope (Scope Page)

Real-time waveform visualization at 20Hz update rate.

**Commands:**
- `SCOPE.TIME <5-500>` - Waveform timespan (ms)
- `SCOPE.MODE <0-4>` - 0=Braille, 1=Block, 2=Line, 3=Dot, 4=Quad
- `SCOPE.CLR <name|0-8>` - Color (see color names below)
- `SCOPE.UNI <0|1>` - 0=Bipolar, 1=Unipolar display

**Color Names:**
- FOREGROUND, SECONDARY, HIGHLIGHT_BG, HIGHLIGHT_FG
- BORDER, ERROR, ACCENT, SUCCESS, LABEL
- Or numeric: 0-8

---

## Sound Engine

### Oscillators

**Primary Oscillator:**
- `POSC.FREQ` / `PF <20-20000>` - Frequency (Hz)
- `POSC.WAVE` / `PW <0-2>` - Waveform (0=Sin, 1=Tri, 2=Saw)

**Modulation Oscillator:**
- `MOSC.FREQ` / `MF <20-20000>` - Frequency (Hz)
- `MOSC.WAVE` / `MW <0-2>` - Waveform (0=Sin, 1=Tri, 2=Saw)

### FM Synthesis

**FM Modulation:**
- `MBUS.FM` / `FM <0-16383>` - FM index amount
- `FMEV.AMT` / `FA <0-16383>` - FM envelope amount
- `FMEV.DEC` / `FD <1-10000>` - FM envelope decay (ms)
- `FMEV.ATK` / `FAA <1-10000>` - FM envelope attack (ms)
- `FMEV.CRV <-8 to 8>` - FM envelope curve

**Feedback FM:**
- `MOSC.FB` / `FB <0-16383>` - Feedback amount
- `FBEV.AMT` / `FBA <0-16383>` - Feedback envelope amount
- `FBEV.DEC` / `FBD <1-10000>` - Feedback envelope decay (ms)
- `FBEV.ATK` / `FBAA <1-10000>` - Feedback envelope attack (ms)
- `FBEV.CRV` / `FBC <-8 to 8>` - Feedback envelope curve

### Discontinuity / Waveshaping

**Discontinuity:**
- `DISC.AMT` / `DC <0-16383>` - Discontinuity amount
- `DISC.MODE` / `DM <0-6>` - Mode (0=Fold, 1=Tanh, 2=Softclip, 3=Hard, 4=Asym, 5=Rectify, 6=Crush)
- `DENV.AMT` / `DA <0-16383>` - Disc envelope amount
- `DENV.DEC` / `DD <1-10000>` - Disc envelope decay (ms)
- `DENV.ATK` / `DAA <1-10000>` - Disc envelope attack (ms)
- `DENV.CRV <-8 to 8>` - Disc envelope curve

### Lo-Fi

- `LOFI.BIT` / `LB <1-16>` - Bit depth (16=clean)
- `LOFI.SMP` / `LS <100-48000>` - Sample rate (Hz)
- `LOFI.MIX` / `LM <0-16383>` - Lo-fi mix amount

### Mod Bus Routing

**Modulation Amount:**
- `MBUS.AMT` / `MB <0-16383>` - Modulation bus amount
- `MBUS.TRK` / `TK <0-16383>` - Key tracking amount

**Routing Switches (0=Off, 1=On):**
- `ROUT.MP` / `MP` - Mod → Primary frequency
- `ROUT.MD` / `MD` - Mod → Discontinuity
- `ROUT.MT` / `MT` - Mod → Tracking
- `ROUT.MA` / `MA` - Mod → Amplitude
- `ROUT.MF` / `MF.F` - Mod → Filter cutoff

**Mix Routing:**
- `MBUS.MIX` / `MX <0-16383>` - Mix amount
- `MBUS.MMX` / `MM <0|1>` - Mod → Mix
- `MBUS.EMX` / `ME <0|1>` - Envelope → Mix

### Envelopes

All envelopes are simple percussive (attack + decay) and trigger on each `TR`.

| Envelope | Decay | Amount | Attack | Curve |
|----------|-------|--------|--------|-------|
| **Amplitude** | AD | - | AENV.ATK (AA) | AENV.CRV (AC) |
| **Pitch** | PD | PA | PENV.ATK (PAA) | PENV.CRV (PC) |
| **FM** | FD | FA | FMEV.ATK (FAA) | FMEV.CRV |
| **Discontinuity** | DD | DA | DENV.ATK (DAA) | DENV.CRV |
| **Feedback** | FBD | FBA | FBEV.ATK (FBAA) | FBEV.CRV (FBC) |
| **Filter** | FED | FE | FLEV.ATK (FLAA) | FLEV.CRV (FLC) |

**Envelope Parameters:**
- **Decay**: 1-10000 ms
- **Attack**: 1-10000 ms
- **Curve**: -8 (log) to 8 (exp), 0=linear
- **Amount**: Varies by envelope type

**Pitch Envelope:**
- `PA` - Amount in octaves (0-16)
- Exponential scaling: 2^(envelope × PA)
- Example: PA=4 sweeps 4 octaves down from primary freq

**Amplitude Envelope:**
- No separate amount parameter (always 0-1)
- Controlled by VCA mode (see below)

### VCA Mode

- `VCA <0|1>` - 0=Drone (open), 1=Gated (default)
- VCA 0: Amplitude envelope ignored, continuous tone
- VCA 1: Amplitude envelope gates the voice

### Effects Chain Order

Signal flow (left to right):
```
Oscillators → FM → Mix → Discontinuity → Lo-Fi → SVF Filter →
Ring Mod → Comb Resonator → Amp → Compressor → Pan →
Beat Repeat → Pitch Shift → Stereo Delay → 3-Band EQ → Plate Reverb
```

### SVF Filter

- `FILT.CUT` / `FC <20-20000>` - Cutoff frequency (Hz)
- `FILT.RES` / `FQ <0-16383>` - Resonance
- `FILT.TYP` / `FT <0-3>` - Type (0=LP, 1=HP, 2=BP, 3=Notch)
- `FILT.KEY` / `FK <0-16383>` - Key tracking
- `FLEV.AMT` / `FE <0-16383>` - Filter envelope amount
- `FLEV.DEC` / `FED <1-10000>` - Filter envelope decay (ms)

### Ring Modulator

- `RING.FRQ` / `RGF <20-2000>` - Ring mod frequency (Hz)
- `RING.WAV` / `RGW <0-3>` - Waveform (0=Sin, 1=Tri, 2=Saw)
- `RING.MIX` / `RGM <0-16383>` - Ring mod mix

### Comb Resonator

- `RESO.FRQ` / `RF <Hz>` - Resonator frequency
- `RESO.DEC` / `RD <ms>` - Decay time
- `RESO.MIX` / `RM <0-16383>` - Resonator mix
- `RESO.KEY` / `RK <0-16383>` - Key tracking

### Compressor

- `COMP.THR` / `CT <0-16383>` - Threshold
- `COMP.RAT` / `CR <1-20>` - Ratio (1=off)
- `COMP.ATK` / `CA <1-500>` - Attack (ms)
- `COMP.REL` / `CL <10-2000>` - Release (ms)
- `COMP.MKP` / `CM <0-16383>` - Makeup gain

### Pan

- `OUT.PAN` / `PAN <-16383 to 16383>` - Stereo position

### Beat Repeat

- `BR.ACT <0|1>` - Enable beat repeat
- `BR.LEN <0-7>` - Division (0=1/16, 1=1/8...7=8×)
- `BR.REV <0|1>` - Reverse playback
- `BR.WIN <1-50>` - Window size (ms)
- `BR.MIX <0-16383>` - Beat repeat mix

### Pitch Shift

- `PS.MODE <0|1>` - Mode (0=Normal, 1=Granular)
- `PS.SEMI <-24 to 24>` - Pitch shift (semitones)
- `PS.GRAIN <5-100>` - Grain size (ms)
- `PS.MIX <0-16383>` - Pitch shift mix
- `PS.TARG <0|1>` - Target (0=Input, 1=Output)

### Stereo Delay

- `DLY.TIME` / `DT <1-2000>` - Delay time (ms)
- `DLY.FB` / `DF <0-16383>` - Feedback
- `DLY.LP` / `DLP <Hz>` - Lowpass filter
- `DLY.WET` / `DW <0-16383>` - Wet mix
- `DLY.SYN` / `DS <0-16383>` - Stereo width

**Delay Routing:**
- `DLY.MODE` / `D.MODE <0-2>` - 0=Bypass, 1=Insert, 2=Send
- `DLY.TAIL` / `D.TAIL <0-2>` - 0=Cut, 1=Ring, 2=Freeze

### 3-Band EQ

- `EQ.LOW` / `EL <-24 to 24>` - Low shelf (dB)
- `EQ.MID` / `EM <-24 to 24>` - Mid peak (dB)
- `EQ.FRQ` / `EF <200-8000>` - Mid frequency (Hz)
- `EQ.Q <0.1-10>` - Mid Q factor
- `EQ.HI` / `EH <-24 to 24>` - High shelf (dB)

### Plate Reverb

- `REV.DEC` / `RV <0-16383>` - Decay time
- `REV.PRE` / `RP <ms>` - Pre-delay
- `REV.DMP` / `RH <0-16383>` - High damping
- `REV.WET` / `RW <0-16383>` - Wet mix

**Reverb Routing:**
- `REV.MODE` / `R.MODE <0-2>` - 0=Bypass, 1=Insert, 2=Send
- `REV.TAIL` / `R.TAIL <0-2>` - 0=Cut, 1=Ring, 2=Freeze

### Volume and Trigger

- `OUT.VOL` / `VOL <0-1>` - Master volume
- `TR` - Trigger voice (fires all envelopes)
- `SLEW.ALL <0-10000>` - Global slew time (ms)
- `SLEW <param> <ms>` - Per-parameter slew

---

## Scripting

### Command Syntax Basics

Commands follow a terse, PREFIX.SUFFIX naming convention:
```
POSC.FREQ 440     # Canonical form
PF 440            # Short alias (preferred)
```

**Examples:**
```
PF 200            # Set primary freq to 200 Hz
AD 500            # Set amp decay to 500ms
TR                # Trigger the voice
A 100             # Store 100 in variable A
PF A              # Set primary freq to A (100 Hz)
```

### Variables

**Global Accumulators:**
- `A B C D` - General-purpose globals
- `X Y Z T` - Additional globals

**Local Variables:**
- `J K` - Per-script locals (separate for each script 1-8, M, I)

**Loop Counter:**
- `I` - Auto-increments in `L` loops

**Auto-Increment Counters:**
- `N1 N2 N3 N4` - Read value, auto-increment, wrap at MAX
- `N1.MIN <n>` - Set minimum (default 0)
- `N1.MAX <n>` - Set maximum (wraps to MIN)
- `N1.RST` - Reset to MIN

**Usage:**
```
A 100             # Set A to 100
PF A              # Use A as frequency
J 10              # Set local J to 10
K SEQ "1 2 3"     # Store sequence value in K
N1.MIN 10         # Counter starts at 10
N1.MAX 14         # Counter wraps at 14
PF N1             # Use counter (reads 10, increments to 11)
```

### Expressions and Operators

All numeric arguments accept expressions:

**Math:**
- `ADD` / `+` - Addition
- `SUB` / `-` - Subtraction
- `MUL` / `*` - Multiplication
- `DIV` / `/` - Division
- `MOD` / `%` - Modulo

**Range Mapping:**
- `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Map with clamping

**Random:**
- `RND <max>` - Random 0 to max
- `RRND <min> <max>` - Random min to max
- `TOSS` - Coin flip (0 or 1)
- `EITH <a> <b>` - Random choice between a and b
- `TOG <a> <b>` - Toggle/alternate between a and b (stateful)

**Note Conversion:**
- `N <semitones>` - Convert semitones to Hz
- `N 0` = C3 (131 Hz), `N 12` = C4, `N 21` = A4 (440 Hz)

**Examples:**
```
PF ADD 100 50              # 150 Hz
PF MUL A 2                 # A × 2
PF MAP A 0 127 200 2000    # Map MIDI range
PF N 12                    # C4 (261.6 Hz)
PF N ADD A 7               # A semitones + 7
A RND 127                  # Random MIDI value
PF EITH 200 400            # Randomly 200 or 400
PF TOG 200 400             # Alternates each call
```

### Control Flow

All control flow uses the **PRE separator** (`:`) syntax.

**Conditionals:**
```
IF <expr>: <cmd>           # Execute if expr != 0
IF <cond>: <cmd>           # With comparison
ELIF <cond>: <cmd>         # Else-if
ELSE: <cmd>                # Else
```

**Probability and Timing:**
```
PROB <0-100>: <cmd>        # Execute with N% probability
EV <n>: <cmd>              # Every Nth execution
SKIP <n>: <cmd>            # Skip every Nth execution
```

**Loops:**
```
L <start> <end>: <cmds>    # Loop from start to end
                           # I variable holds current index
BRK                        # Break out of script
```

**Sub-Commands:**
```
CMD1; CMD2; CMD3           # Multiple commands on one line
                           # Use ; separator
```

**Comparison Operators:**

In conditions:
```
EZ <x>           # x == 0
NZ <x>           # x != 0
EQ <a> <b>       # a == b
NE <a> <b>       # a != b
GT <a> <b>       # a > b
LT <a> <b>       # a < b
GTE <a> <b>      # a >= b
LTE <a> <b>      # a <= b
```

Can also use infix in conditionals:
```
IF A > 100: TR
IF A == B: PF 440
```

**Examples:**
```
IF A: TR                   # Trigger if A is nonzero
IF GT A 100: PF 200        # If A > 100, set freq
PROB 50: TR                # 50% chance to trigger
EV 4: TR                   # Every 4th execution
L 0 7: PF N I              # Loop I from 0-7, set freq
A 0; B 0                   # Reset two variables
```

### Scenes and Presets

**Scenes** save complete state (scripts, patterns, synth params):
```
SAVE <name>       # Save current state
LOAD <name>       # Load saved state
SCENES            # List all scenes
DELETE <name>     # Delete scene
```

**Presets** load parameter configurations into script slots:
```
PSET <1-8> <name>         # Load preset into script
PSET.SAVE <1-8> <name>    # Save script as user preset
PSET.DEL <name>           # Delete user preset
PSETS                     # List all presets
                          # [F] = Factory, [U] = User
```

**Factory Presets:**

Drums: `808-kick`, `punch-kick`, `sub-kick`, `basic-snare`, `snap-snare`, `hat-closed`, `hat-open`, `fm-hat`, `clap`, `rim`

Bass: `sub-bass`, `saw-bass`, `fm-bass`

Lead: `saw-lead`, `fm-lead`, `pluck-lead`

Percussion: `metal-hit`, `conga`, `tom`

FX: `noise`, `zap`, `rise`

**Example:**
```
PSET 1 808-kick           # Load kick into script 1
PSET 2 hat-closed         # Load hat into script 2
M.SCRIPT 1                # Set metro to run script 1
M.ACT 1                   # Start metro
SAVE my-beat              # Save everything
```

---

## Patterns

### Working Pattern (P) vs Explicit (PN)

**Working Pattern (P):**
- One pattern is "active" at a time (default: pattern 0)
- `P.*` commands operate on the working pattern
- `P.N` to get/set which pattern is active

**Explicit Pattern (PN):**
- `PN.*` commands take a pattern number (0-5) as first argument
- Work with any pattern directly

### Pattern Operations

**Working Pattern:**
```
P.N               # Get working pattern number
P.N <0-5>         # Set working pattern
P.L               # Get length
P.L <1-64>        # Set length
P.I               # Get index
P.I <0-63>        # Set index
P.HERE            # Get value at current index
P.NEXT            # Advance index, return value
P.PREV            # Reverse index, return value
P <idx>           # Get value at idx
P <idx> <val>     # Set value at idx
```

**Explicit Pattern:**
```
PN.L <pat>        # Get pattern length
PN.L <pat> <n>    # Set pattern length
PN.I <pat>        # Get pattern index
PN.I <pat> <n>    # Set pattern index
PN.HERE <pat>     # Get value at index
PN.NEXT <pat>     # Advance, return value
PN.PREV <pat>     # Reverse, return value
PN <pat> <idx>    # Get value
PN <pat> <idx> <v> # Set value
```

### Pattern Manipulation

```
P.PUSH <val>      # Push value (shift left)
P.POP             # Pop last value
P.INS <idx> <val> # Insert at index
P.RM <idx>        # Remove at index
P.REV             # Reverse pattern
P.ROT <n>         # Rotate by n steps
P.SHUF            # Shuffle randomly
P.SORT            # Sort ascending
```

All have explicit versions: `PN.PUSH <pat> <val>`, etc.

### Pattern Math

```
P.ADD <val>       # Add to all values (saturating)
P.SUB <val>       # Subtract from all
P.MUL <val>       # Multiply all
P.DIV <val>       # Divide all (zero-safe)
P.MOD <val>       # Modulo all
P.SCALE <min> <max> # Scale to range
```

### Pattern Queries

```
P.MIN             # Return minimum value
P.MAX             # Return maximum value
P.SUM             # Return sum of all
P.AVG             # Return average (integer)
P.FND <val>       # Find index of value (-1 if not found)
```

**Examples:**
```
P.N 1             # Switch to pattern 1
P.L 8             # Set length to 8
L 0 7: P I MUL I 10   # Fill: 0,10,20...70
PF N P.NEXT       # Use next pattern value as freq
PN.NEXT 2         # Advance pattern 2
A PN.MIN 3        # Get min value from pattern 3
```

---

## SEQ Notation

SEQ provides inline sequence notation that cycles through values on each call. State is per-script and per-pattern.

### Basic Syntax

```
SEQ "<pattern>"   # Cycle through values
```

**Tokens:**
- `X` - Trigger (returns 1)
- `_` or `.` - Rest (returns 0)
- `?` - Random (50% chance of 1 or 0)
- Numbers - Literal values (100, -12, 0)
- Note names - C3, E3, F#4, Bb2 (returns semitones)

### Note Names and Accidentals

- Format: `<Note><Accidental><Octave>`
- Notes: C, D, E, F, G, A, B
- Accidentals: `#` (sharp), `b` (flat)
- Octave: Number (C3 = 0 semitones)

**Examples:**
```
C3                # 0 semitones
E3                # 4 semitones
F#4               # 17 semitones
Bb2               # -2 semitones
```

### Repeats

Use `*n` to repeat a token n times:

```
SEQ "C3*4"        # C3 C3 C3 C3
SEQ "X*3 _"       # X X X _
SEQ "100*2 200"   # 100 100 200
```

### Toggle vs Random

**Toggle `<a b>`** - Deterministic alternation (like `TOG`):
```
SEQ "<C3 E3>"     # C3, E3, C3, E3, C3, E3...
SEQ "<C3 E3> G3"  # C3, G3, E3, G3, C3, G3...
SEQ "<100 200>*2" # 100, 200, 100, 200
```

State persists - remembers last value between calls.

**Random `{a b}`** - Unpredictable choice (like `EITH`):
```
SEQ "{C3 E3}"     # Randomly C3 or E3 each time
SEQ "{C3 E3} G3"  # Random C3/E3, then G3
SEQ "{100 200}*2" # Two random picks
```

No state - picks fresh each evaluation.

### Using with Expressions

SEQ works in all expression contexts:

```
PF N SEQ "C3 E3 G3"           # Arpeggiate C major
A SEQ "0 1 2 3"               # Store sequence in var
IF SEQ "X _ X _": TR          # Trigger pattern
PF N Q SEQ "0 3 5 7"          # Quantized sequence
DC SEQ "<1000 5000>"          # Toggle disc amount
```

**Examples:**
```
SEQ "X _ X _"                 # Trigger pattern
SEQ "C3*4 E3*2 G3*2"          # Repeated notes
SEQ "<C3 E3> G3 <F3 A3>"      # Multiple toggles
SEQ "{C3 E3} {G3 B3}"         # Multiple randoms
SEQ "X ? X ?"                 # Random triggers
SEQ "100 200 ? 400"           # Mix triggers and values
```

---

## Scale Quantization

The `Q` operator quantizes note values to the current scale.

### Q Operator

```
Q <note>          # Quantize note (in expressions)
```

**Examples:**
```
PF N Q A          # Quantize A to scale, convert to Hz
PF N Q P.NEXT     # Quantize pattern value
PF N Q SEQ "0 3 5 7 12"  # Quantize sequence
```

### Scale Settings

```
Q.ROOT <0-11>     # Set root note (C=0, C#=1...B=11)
Q.SCALE <0-11>    # Set scale type (see below)
Q.BIT <binary>    # Custom scale bitmask
```

### Scale Types

| Number | Scale | Degrees |
|--------|-------|---------|
| 0 | Chromatic | All 12 notes |
| 1 | Major | Ionian (W-W-H-W-W-W-H) |
| 2 | Minor | Natural minor |
| 3 | Dorian | Minor with raised 6th |
| 4 | Phrygian | Minor with lowered 2nd |
| 5 | Lydian | Major with raised 4th |
| 6 | Mixolydian | Major with lowered 7th |
| 7 | Pentatonic Major | 5-note major |
| 8 | Pentatonic Minor | 5-note minor |
| 9 | Blues | Minor pent + b5 |
| 10 | Whole Tone | 6-note equal steps |
| 11 | Diminished | Half-whole octatonic |

### Custom Scales with Q.BIT

Q.BIT accepts binary bitmasks for arbitrary scales:

```
Q.BIT 101010110101   # Major (12-TET)
Q.BIT 10101          # Pentatonic (5-EDO)
Q.BIT <24 bits>      # Quarter-tones (24-EDO)
```

Bits set to 1 represent allowed scale degrees.

**Examples:**
```
Q.ROOT 0          # Root = C
Q.SCALE 1         # Major scale
PF N Q 5          # Quantize 5 semitones → E (part of C maj)

Q.SCALE 8         # Pentatonic minor
Q.ROOT 2          # Root = D
PF N Q P.NEXT     # Quantize pattern to D pent minor

Q.BIT 101010110101  # Custom major scale
```

---

## Metro and Timing

### Metro Commands

```
M                 # Show current interval (ms)
M <ms>            # Set interval in milliseconds
M.BPM <bpm>       # Set tempo in beats per minute
M.ACT <0|1>       # Start/stop metro (0=stop, 1=start)
M.SCRIPT <1-8>    # Set which script runs on metro
```

**Examples:**
```
M 500             # 500ms interval (120 BPM)
M.BPM 140         # 140 BPM
M.SCRIPT 1        # Run script 1 on metro
M.ACT 1           # Start metro
```

### MIDI Clock Sync

```
M.SYNC            # Show sync mode
M.SYNC <0|1>      # 0=Internal, 1=MIDI clock
MIDI.IN           # List MIDI input devices
MIDI.IN <name>    # Connect to MIDI device
```

**Setup:**
1. Connect MIDI device: `MIDI.IN "Your Device"`
2. Enable sync: `M.SYNC 1`
3. Start/stop follows MIDI transport

MIDI clock runs at 16th note resolution (24 PPQN standard).

### MIDI Clock Diagnostics

```
MIDI.DIAG <0|1>   # Enable/disable timing diagnostics
MIDI.DIAG REPORT  # Write report to midi_timing_report.txt
SC.DIAG <0|1>     # Enable/disable SC timing diagnostics
SC.DIAG REPORT    # Write report to sc_timing_report.txt
```

### Delayed Execution

Commands can be scheduled to execute after a delay:

**Basic Delay:**
```
DEL <ms>: <cmd>   # Execute cmd after ms (max 16000)
DEL.CLR           # Clear all pending delays
```

**Repeated Delays:**
```
DEL.X <n> <ms>: <cmd>   # Execute n times at intervals
                        # Example: DEL.X 4 100: TR
                        # Fires at 100ms, 200ms, 300ms, 400ms

DEL.R <n> <ms>: <cmd>   # Execute now, then repeat n-1 times
                        # Example: DEL.R 4 100: TR
                        # Fires now, 100ms, 200ms, 300ms
```

**Examples:**
```
DEL 1000: TR              # Trigger after 1 second
DEL 500: PF 440           # Change freq after 500ms
DEL.X 8 125: TR           # 8 triggers at 125ms intervals
DEL.R 4 250: A ADD A 10   # Increment A 4 times
```

---

## Recording

### Recording Commands

```
REC               # Start recording (24-bit stereo WAV)
REC.STOP          # Stop recording
REC.PATH <path>   # Set output path prefix
```

**Output:**
- Format: 24-bit stereo WAV @ 48kHz
- Location: Current directory
- Filename: `monokit_audio_N.wav` (sequential numbering)

**Examples:**
```
REC               # Start recording to monokit_audio_1.wav
TR                # Record some sound
REC.STOP          # Stop and save file
```

---

## Scenes and Presets

### Scenes

Scenes save complete state: scripts, patterns, synth parameters, notes.

```
SAVE <name>       # Save current state
LOAD <name>       # Load scene
SCENES            # List all scenes
DELETE <name>     # Delete scene
```

**Load Mode:**
```
LOAD.RST          # Query reset mode
LOAD.RST <0|1>    # 0=Keep synth (default), 1=Reset all
```

- Mode 0: Scripts/patterns replace current, synth params persist
- Mode 1: Full reset before loading (like `RST` then load)

**Examples:**
```
SAVE my-beat      # Save current state
LOAD my-beat      # Load saved state
LOAD.RST 1        # Enable reset-on-load
LOAD my-beat      # Will reset all first
```

### Presets

Presets load parameter scripts into script slots 1-8.

```
PSET <1-8> <name>         # Load preset into script
PSET.SAVE <1-8> <name>    # Save script as user preset
PSET.DEL <name>           # Delete user preset
PSETS                     # List all ([F]=Factory, [U]=User)
```

**Factory Presets:**

**Drums:**
- `808-kick` - Classic 808 kick
- `punch-kick` - Punchy kick with disc
- `sub-kick` - Deep sub kick
- `basic-snare` - Basic snare
- `snap-snare` - Snappy snare
- `hat-closed` - Closed hi-hat
- `hat-open` - Open hi-hat
- `fm-hat` - FM hi-hat
- `clap` - Hand clap
- `rim` - Rim shot

**Bass:**
- `sub-bass` - Deep sub bass
- `saw-bass` - Sawtooth bass
- `fm-bass` - FM bass

**Lead:**
- `saw-lead` - Sawtooth lead
- `fm-lead` - FM lead
- `pluck-lead` - Plucked lead

**Percussion:**
- `metal-hit` - Metallic hit
- `conga` - Conga drum
- `tom` - Tom drum

**FX:**
- `noise` - Noise burst
- `zap` - Zap effect
- `rise` - Rising sweep

---

## Configuration

### Theme System

```
THEMES            # List all available themes
THEME <name>      # Switch to theme
```

48 themes included. Config stored in `~/.config/monokit/themes/themes.toml`.

### UI Toggles

**Header/Grid Visibility:**
```
METER.HDR <0|1>   # Audio meters in header
METER.GRID <0|1>  # Audio meters in grid view
METER.ASCII <0|1> # ASCII-only meter chars (.:-=+#)
SPECTRUM <0|1>    # Spectrum analyzer
ACTIVITY <0|1>    # Script activity indicators
GRID <0|1>        # Parameter activity grid
GRID.DEF <0|1>    # Default view (0=REPL, 1=Grid)
GRID.MODE <0|1>   # Grid labels (0=Text, 1=Icons)
CPU <0|1>         # CPU meter in header
BPM <0|1>         # BPM display in header
```

**Highlighting:**
```
HL.SEQ <0|1>      # Highlight SEQ/TOG current step
HL.COND <0|1>     # Highlight conditionals when true
```

**Title Display:**
```
TITLE <0|1>       # 0="MONOKIT", 1=Scene name
TITLE.TIMER       # Query timer state
TITLE.TIMER 0     # Disable auto-cycle
TITLE.TIMER 1 <s> # Cycle every <s> seconds (1-1800)
```

**Scramble Animation:**
```
SCRMBL <0|1>      # Enable scramble animation
SCRMBL.MODE <0-3> # 0=Regular, 1=Smash, 2=Roll, 3=Over
SCRMBL.SPD <1-10> # Speed (1=slow, 10=fast)
SCRMBL.CRV <0|1>  # Curve (0=linear, 1=settle)
```

### Header Verbosity

```
HEADER            # Query current level
HEADER <0-4>      # Set verbosity
```

**Levels:**
- 0: Navigation only
- 1: Nav + Meters + BPM
- 2: Nav + Meters + Trigger + BPM
- 3: Full nav + Meters + Trigger
- 4: Full + CPU (default)

### DEBUG Levels

```
DEBUG <0-5>       # Set verbosity tier
```

**Tiers:**
- 0: SILENT (use category overrides only)
- 1: ERRORS (error messages)
- 2: ESSENTIAL (state changes)
- 3: QUERIES (value reads)
- 4: CONFIRMS (set confirmations)
- 5: VERBOSE (all output)

**Category Overrides:**
```
OUT.ERR <0|1>     # Override: show errors
OUT.ESS <0|1>     # Override: show essential
OUT.QRY <0|1>     # Override: show queries
OUT.CFM <0|1>     # Override: show confirms
```

### Terminal Compatibility

```
COMPAT            # Show terminal capabilities
COMPAT.MODE <0|1> # 0=Full color, 1=256-color
```

Monokit auto-detects terminal capabilities. Use `COMPAT.MODE 1` to force 256-color mode on limited terminals.

**Note:** Terminal.app has limited color support. For best results, use iTerm2 or similar.

### System Commands

```
RST               # Reset to defaults
Q / QUIT / EXIT   # Quit application
CLEAR             # Clear REPL output
PRINT "text"      # Print literal string
PRINT <expr>      # Evaluate and print
FLASH <ms>        # Set activity hold time (default 200ms)
```

**Default Values (RST):**
- PF: 131 Hz (C3)
- MF: 262 Hz (C4)
- PA: 0 (no pitch envelope)
- FC: 10000 Hz (filter open)
- D.MODE: 2 (Send)
- R.MODE: 2 (Send)
- CR: 1 (Compressor off)

### Audio Output

```
AUDIO.OUT         # List output devices
AUDIO.OUT <n>     # Set output device by number
```

Device changes restart the audio engine. Selection is saved to config for next launch.

### Notes Page

```
NOTE "text"       # Append text to Notes page
NOTE.CLR          # Clear all notes
```

Notes are saved with scenes. 8 lines maximum.

---

## Command Reference

### Navigation & Editing

| Command | Description |
|---------|-------------|
| `[ ]` | Cycle pages |
| `ESC` | Toggle help |
| `F1-F8` | Script 1-8 |
| `F9` | Live page |
| `F10` | Metro page |
| `F11` | Init page |
| `F12` | Pattern page |
| `Alt+L` | Live page |
| `Alt+1-8` | Script 1-8 |
| `Alt+M` | Metro page |
| `Alt+I` | Init page |
| `Alt+P` | Pattern page |
| `Alt+S` | Scope page |
| `Alt+V` | Variables page |
| `Alt+H` | Toggle help |
| `Tab` | Live page: REPL/Grid toggle |
| `Ctrl+F` | Search mode |
| `Ctrl+Up/Down` | Scroll REPL |
| `Ctrl+D` | Duplicate line |
| `Ctrl+K` | Delete line |
| `Ctrl+C/X/V` | Copy/cut/paste line |

### Variables & Math

| Command | Description |
|---------|-------------|
| `A B C D X Y Z T` | Global variables |
| `J K` | Per-script local variables |
| `I` | Loop counter |
| `N1-N4` | Auto-increment counters |
| `N1.MIN <n>` | Set counter minimum |
| `N1.MAX <n>` | Set counter maximum |
| `N1.RST` | Reset counter to min |
| `ADD / +` | Addition |
| `SUB / -` | Subtraction |
| `MUL / *` | Multiplication |
| `DIV / /` | Division |
| `MOD / %` | Modulo |
| `MAP <v> <i1> <i2> <o1> <o2>` | Range mapping |
| `RND <max>` | Random 0 to max |
| `RRND <min> <max>` | Random min to max |
| `TOSS` | Coin flip (0/1) |
| `EITH <a> <b>` | Random choice |
| `TOG <a> <b>` | Toggle/alternate |
| `N <semi>` | Semitones to Hz |

### Control Flow

| Command | Description |
|---------|-------------|
| `IF <x>: <cmd>` | Execute if x != 0 |
| `IF <cond>: <cmd>` | Execute if condition true |
| `ELIF <cond>: <cmd>` | Else-if |
| `ELSE: <cmd>` | Else |
| `PROB <0-100>: <cmd>` | Probability % |
| `EV <n>: <cmd>` | Every Nth execution |
| `SKIP <n>: <cmd>` | Skip every Nth |
| `L <s> <e>: <cmds>` | Loop start to end |
| `BRK` | Break script |
| `CMD1; CMD2` | Sub-commands |
| `EZ <x>` | x == 0 |
| `NZ <x>` | x != 0 |
| `EQ <a> <b>` | a == b |
| `NE <a> <b>` | a != b |
| `GT <a> <b>` | a > b |
| `LT <a> <b>` | a < b |
| `GTE <a> <b>` | a >= b |
| `LTE <a> <b>` | a <= b |

### Sequences & Patterns

| Command | Description |
|---------|-------------|
| `SEQ "<pattern>"` | Inline sequence |
| `P.N [<0-5>]` | Get/set working pattern |
| `P.L [<n>]` | Get/set pattern length |
| `P.I [<n>]` | Get/set pattern index |
| `P.HERE` | Value at index |
| `P.NEXT` | Advance, get value |
| `P.PREV` | Reverse, get value |
| `P <i> [<v>]` | Get/set at index |
| `P.PUSH <val>` | Push value |
| `P.POP` | Pop last value |
| `P.INS <i> <v>` | Insert at index |
| `P.RM <i>` | Remove at index |
| `P.REV` | Reverse pattern |
| `P.ROT <n>` | Rotate by n |
| `P.SHUF` | Shuffle |
| `P.SORT` | Sort ascending |
| `P.ADD <v>` | Add to all |
| `P.SUB <v>` | Subtract from all |
| `P.MUL <v>` | Multiply all |
| `P.DIV <v>` | Divide all |
| `P.MOD <v>` | Modulo all |
| `P.SCALE <min> <max>` | Scale to range |
| `P.MIN` | Minimum value |
| `P.MAX` | Maximum value |
| `P.SUM` | Sum of all |
| `P.AVG` | Average (int) |
| `P.FND <val>` | Find index |
| `PN.*` | Explicit pattern (add pattern # as 1st arg) |

### Scale Quantization

| Command | Description |
|---------|-------------|
| `Q <note>` | Quantize to scale |
| `Q.ROOT <0-11>` | Set root note |
| `Q.SCALE <0-11>` | Set scale type |
| `Q.BIT <binary>` | Custom scale mask |

### Oscillators & FM

| Command | Alias | Description |
|---------|-------|-------------|
| `POSC.FREQ <hz>` | `PF` | Primary frequency |
| `POSC.WAVE <0-2>` | `PW` | Primary waveform |
| `MOSC.FREQ <hz>` | `MF` | Mod frequency |
| `MOSC.WAVE <0-2>` | `MW` | Mod waveform |
| `MBUS.FM <amt>` | `FM` | FM index |
| `FMEV.AMT <amt>` | `FA` | FM envelope amount |
| `FMEV.DEC <ms>` | `FD` | FM envelope decay |
| `FMEV.ATK <ms>` | `FAA` | FM envelope attack |
| `FMEV.CRV <v>` | - | FM envelope curve |
| `MOSC.FB <amt>` | `FB` | Feedback amount |
| `FBEV.AMT <amt>` | `FBA` | FB envelope amount |
| `FBEV.DEC <ms>` | `FBD` | FB envelope decay |
| `FBEV.ATK <ms>` | `FBAA` | FB envelope attack |
| `FBEV.CRV <v>` | `FBC` | FB envelope curve |

### Discontinuity & Lo-Fi

| Command | Alias | Description |
|---------|-------|-------------|
| `DISC.AMT <amt>` | `DC` | Disc amount |
| `DISC.MODE <0-6>` | `DM` | Disc mode |
| `DENV.AMT <amt>` | `DA` | Disc env amount |
| `DENV.DEC <ms>` | `DD` | Disc env decay |
| `DENV.ATK <ms>` | `DAA` | Disc env attack |
| `DENV.CRV <v>` | - | Disc env curve |
| `LOFI.BIT <1-16>` | `LB` | Bit depth |
| `LOFI.SMP <hz>` | `LS` | Sample rate |
| `LOFI.MIX <amt>` | `LM` | Lo-fi mix |

### Mod Bus & Mix

| Command | Alias | Description |
|---------|-------|-------------|
| `MBUS.AMT <amt>` | `MB` | Mod bus amount |
| `MBUS.TRK <amt>` | `TK` | Track amount |
| `ROUT.MP <0\|1>` | `MP` | Mod → Pri freq |
| `ROUT.MD <0\|1>` | `MD` | Mod → Disc |
| `ROUT.MT <0\|1>` | `MT` | Mod → Track |
| `ROUT.MA <0\|1>` | `MA` | Mod → Amp |
| `ROUT.MF <0\|1>` | `MF.F` | Mod → Filter |
| `MBUS.MIX <amt>` | `MX` | Mix amount |
| `MBUS.MMX <0\|1>` | `MM` | Mod → Mix |
| `MBUS.EMX <0\|1>` | `ME` | Env → Mix |

### Envelopes

| Command | Alias | Description |
|---------|-------|-------------|
| `AENV.DEC <ms>` | `AD` | Amp decay |
| `AENV.ATK <ms>` | `AA` | Amp attack |
| `AENV.CRV <v>` | `AC` | Amp curve |
| `PENV.AMT <oct>` | `PA` | Pitch amount (octaves) |
| `PENV.DEC <ms>` | `PD` | Pitch decay |
| `PENV.ATK <ms>` | `PAA` | Pitch attack |
| `PENV.CRV <v>` | `PC` | Pitch curve |
| `FLEV.AMT <amt>` | `FE` | Filter env amount |
| `FLEV.DEC <ms>` | `FED` | Filter env decay |
| `FLEV.ATK <ms>` | `FLAA` | Filter env attack |
| `FLEV.CRV <v>` | `FLC` | Filter env curve |

(FM, Disc, FB envelopes: see above)

### Filter & Effects

| Command | Alias | Description |
|---------|-------|-------------|
| `FILT.CUT <hz>` | `FC` | Filter cutoff |
| `FILT.RES <amt>` | `FQ` | Resonance |
| `FILT.TYP <0-3>` | `FT` | Type (LP/HP/BP/N) |
| `FILT.KEY <amt>` | `FK` | Key tracking |
| `RING.FRQ <hz>` | `RGF` | Ring mod freq |
| `RING.WAV <0-3>` | `RGW` | Ring mod wave |
| `RING.MIX <amt>` | `RGM` | Ring mod mix |
| `RESO.FRQ <hz>` | `RF` | Resonator freq |
| `RESO.DEC <ms>` | `RD` | Resonator decay |
| `RESO.MIX <amt>` | `RM` | Resonator mix |
| `RESO.KEY <amt>` | `RK` | Resonator key track |
| `COMP.THR <amt>` | `CT` | Compressor threshold |
| `COMP.RAT <1-20>` | `CR` | Compressor ratio |
| `COMP.ATK <ms>` | `CA` | Compressor attack |
| `COMP.REL <ms>` | `CL` | Compressor release |
| `COMP.MKP <amt>` | `CM` | Compressor makeup |
| `OUT.PAN <amt>` | `PAN` | Stereo pan |

### Beat Repeat & Pitch Shift

| Command | Description |
|---------|-------------|
| `BR.ACT <0\|1>` | Beat repeat enable |
| `BR.LEN <0-7>` | Division |
| `BR.REV <0\|1>` | Reverse |
| `BR.WIN <1-50>` | Window (ms) |
| `BR.MIX <amt>` | Beat repeat mix |
| `PS.MODE <0\|1>` | Pitch shift mode |
| `PS.SEMI <-24-24>` | Semitones |
| `PS.GRAIN <5-100>` | Grain size (ms) |
| `PS.MIX <amt>` | Pitch shift mix |
| `PS.TARG <0\|1>` | Target (In/Out) |

### Delay & Reverb

| Command | Alias | Description |
|---------|-------|-------------|
| `DLY.TIME <ms>` | `DT` | Delay time |
| `DLY.FB <amt>` | `DF` | Delay feedback |
| `DLY.LP <hz>` | `DLP` | Delay lowpass |
| `DLY.WET <amt>` | `DW` | Delay wet mix |
| `DLY.SYN <amt>` | `DS` | Delay stereo width |
| `DLY.MODE <0-2>` | `D.MODE` | Delay routing |
| `DLY.TAIL <0-2>` | `D.TAIL` | Delay tail mode |
| `REV.DEC <amt>` | `RV` | Reverb decay |
| `REV.PRE <ms>` | `RP` | Reverb pre-delay |
| `REV.DMP <amt>` | `RH` | Reverb damping |
| `REV.WET <amt>` | `RW` | Reverb wet mix |
| `REV.MODE <0-2>` | `R.MODE` | Reverb routing |
| `REV.TAIL <0-2>` | `R.TAIL` | Reverb tail mode |

### EQ

| Command | Alias | Description |
|---------|-------|-------------|
| `EQ.LOW <db>` | `EL` | Low shelf (-24 to 24) |
| `EQ.MID <db>` | `EM` | Mid peak (-24 to 24) |
| `EQ.FRQ <hz>` | `EF` | Mid frequency |
| `EQ.Q <q>` | - | Mid Q (0.1-10) |
| `EQ.HI <db>` | `EH` | High shelf (-24 to 24) |

### Metro & Timing

| Command | Description |
|---------|-------------|
| `M [<ms>]` | Get/set interval |
| `M.BPM <bpm>` | Set BPM |
| `M.ACT <0\|1>` | Start/stop metro |
| `M.SCRIPT <1-8>` | Set metro script |
| `M.SYNC [<0\|1>]` | Get/set sync mode |
| `MIDI.IN [<name>]` | List/connect MIDI |
| `MIDI.DIAG <0\|1>` | MIDI diagnostics |
| `MIDI.DIAG REPORT` | Write MIDI report |
| `SC.DIAG <0\|1>` | SC diagnostics |
| `SC.DIAG REPORT` | Write SC report |
| `SCRIPT <1-8>` | Execute script |
| `DEL <ms>: <cmd>` | Delayed execution |
| `DEL.CLR` | Clear pending |
| `DEL.X <n> <ms>: <cmd>` | Repeat n times |
| `DEL.R <n> <ms>: <cmd>` | Now + repeat |

### Scenes & Presets

| Command | Description |
|---------|-------------|
| `SAVE <name>` | Save scene |
| `LOAD <name>` | Load scene |
| `SCENES` | List scenes |
| `DELETE <name>` | Delete scene |
| `LOAD.RST [<0\|1>]` | Get/set reset mode |
| `PSET <1-8> <name>` | Load preset |
| `PSET.SAVE <1-8> <name>` | Save preset |
| `PSET.DEL <name>` | Delete preset |
| `PSETS` | List presets |

### Recording

| Command | Description |
|---------|-------------|
| `REC` | Start recording |
| `REC.STOP` | Stop recording |
| `REC.PATH <path>` | Set output path |

### Randomization

| Command | Description |
|---------|-------------|
| `RND.VOICE` | Randomize voice |
| `RND.OSC` | Randomize oscillators |
| `RND.FM` | Randomize FM |
| `RND.MOD` | Randomize mod routing |
| `RND.ENV` | Randomize envelopes |
| `RND.FX` | Randomize all FX |
| `RND.FILT` | Randomize filter |
| `RND.DLY` | Randomize delay |
| `RND.VERB` | Randomize reverb |
| `RND.P [min] [max]` | Randomize working pattern |
| `RND.PN <n> [min] [max]` | Randomize pattern n |
| `RND.PALL [min] [max]` | Randomize all patterns |

### UI & Display

| Command | Description |
|---------|-------------|
| `METER.HDR <0\|1>` | Header meters |
| `METER.GRID <0\|1>` | Grid meters |
| `METER.ASCII <0\|1>` | ASCII meters |
| `SPECTRUM <0\|1>` | Spectrum analyzer |
| `ACTIVITY <0\|1>` | Activity indicators |
| `GRID <0\|1>` | Param grid |
| `GRID.DEF <0\|1>` | Default view |
| `GRID.MODE <0\|1>` | Grid labels/icons |
| `HL.SEQ <0\|1>` | SEQ highlighting |
| `HL.COND <0\|1>` | Conditional highlighting |
| `CPU <0\|1>` | CPU meter |
| `BPM <0\|1>` | BPM display |
| `TITLE <0\|1>` | Title mode |
| `TITLE.TIMER [<0\|1> <s>]` | Auto-cycle title |
| `SCRMBL [<0\|1>]` | Scramble animation |
| `SCRMBL.MODE <0-3>` | Scramble style |
| `SCRMBL.SPD <1-10>` | Scramble speed |
| `SCRMBL.CRV <0\|1>` | Scramble curve |
| `HEADER [<0-4>]` | Header verbosity |
| `DEBUG <0-5>` | Debug verbosity |
| `OUT.ERR <0\|1>` | Override: errors |
| `OUT.ESS <0\|1>` | Override: essential |
| `OUT.QRY <0\|1>` | Override: queries |
| `OUT.CFM <0\|1>` | Override: confirms |
| `SCOPE.TIME <5-500>` | Scope timespan (ms) |
| `SCOPE.CLR <name\|0-8>` | Scope color |
| `SCOPE.MODE <0-4>` | Scope render mode |
| `SCOPE.UNI <0\|1>` | Scope unipolar |

### System

| Command | Description |
|---------|-------------|
| `RST` | Reset to defaults |
| `Q / QUIT / EXIT` | Quit application |
| `CLEAR` | Clear REPL output |
| `PRINT "text"` | Print string |
| `PRINT <expr>` | Print expression |
| `NOTE "text"` | Append to notes |
| `NOTE.CLR` | Clear notes |
| `FLASH <ms>` | Activity hold time |
| `VCA <0\|1>` | VCA mode |
| `SLEW.ALL <ms>` | Global slew |
| `SLEW <p> <ms>` | Per-param slew |
| `TR` | Trigger voice |
| `OUT.VOL <0-1>` | Master volume |
| `AUDIO.OUT [<n>]` | List/set audio device |
| `COMPAT` | Show terminal caps |
| `COMPAT.MODE <0\|1>` | Force compat mode |
| `THEMES` | List themes |
| `THEME <name>` | Switch theme |

---

## Tips and Tricks

### Making a Drum Kit

Use presets to load drum sounds into different scripts:

```
PSET 1 808-kick
PSET 2 basic-snare
PSET 3 hat-closed
PSET 4 hat-open
M.SCRIPT 1          # Set metro to script 1
```

Edit Script 1 to sequence:
```
PROB 100: SCRIPT 1; TR     # Kick on every beat
PROB 50: SCRIPT 2; TR      # Snare 50% of the time
EV 2: SCRIPT 3; TR         # Closed hat every other
EV 4: SCRIPT 4; TR         # Open hat every 4th
```

### Building Basslines

Use patterns and scale quantization:

```
Q.ROOT 0            # Root = C
Q.SCALE 8           # Pentatonic minor
P.N 0               # Use pattern 0
P.L 8               # 8 steps
L 0 7: P I RND 24   # Fill with random notes (0-24 semitones)
M.SCRIPT 1          # Metro runs script 1
```

Script 1:
```
PF N Q P.NEXT       # Next pattern value, quantize, to Hz
TR                  # Trigger
```

### Euclidean-Style Rhythms

Use SEQ with probabilities:

```
SEQ "X ? ? X ? ? X ?"    # Probabilistic euclidean
```

Or build with patterns:
```
P.L 16
L 0 15: IF MOD I 5: P I 1; ELSE: P I 0
# Creates: X____X____X____X___
```

### Parameter Modulation

Use metro to animate parameters:

```
A 0
M.SCRIPT 1
```

Script 1:
```
A ADD A 10                    # Increment A
IF GT A 127: A 0              # Wrap at 127
FC MAP A 0 127 200 2000       # Map to filter cutoff
```

### Live Performance

- Use F1-F8 to switch between different grooves
- Tab to grid view to see which params are active
- Alt+V to monitor variables during execution
- Save snapshots with `SAVE performance-1`, etc.

### Debugging Scripts

Set DEBUG level for feedback:

```
DEBUG 4             # Show all set confirmations
OUT.CFM 1           # Force confirm output
```

Use PRINT to inspect values:

```
PRINT A             # Print variable A
PRINT P.NEXT        # Print next pattern value
PRINT SEQ "C3 E3 G3"  # Print sequence step
```

---

## Configuration File

User config: `~/.config/monokit/config.toml`

Scenes: `~/.config/monokit/scenes/`

User presets: `~/.config/monokit/presets/`

Themes: `~/.config/monokit/themes/themes.toml`

---

## Terminal Requirements

**Minimum:** 50 columns × 18 rows

**Recommended:**
- True color support (iTerm2, Kitty, Alacritty)
- 256-color fallback supported (automatic)
- Monospace font (Monaco recommended for block chars)

**Terminal.app limitations:**
- Limited color support (256-color mode auto-enabled)
- Use `METER.ASCII 1` for better meter rendering

---

## Command Naming Convention

Monokit uses a consistent PREFIX.SUFFIX naming pattern:

**Canonical form:**
```
POSC.FREQ 440     # Primary oscillator frequency
FILT.CUT 2000     # Filter cutoff
AENV.ATK 10       # Amplitude envelope attack
```

**Short aliases:**
```
PF 440            # Same as POSC.FREQ
FC 2000           # Same as FILT.CUT
AA 10             # Same as AENV.ATK
```

Use whichever form you prefer - they're identical. Short forms are faster for live coding; canonical forms are clearer for saved scripts.

---

**End of Manual**

For the latest updates and community discussion, visit the [monokit repository](https://github.com/stolmine/monokit).
