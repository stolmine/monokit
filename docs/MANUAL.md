# Monokit Manual

Updated 2026-01-13

**Version 0.6.0** - Teletype-style scripting for a complex oscillator voice

Runs on macOS (Apple Silicon), Linux (x86_64), and Windows (x86_64).

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
- `Ctrl+Z` - Undo (page-local)
- `Ctrl+Shift+Z` - Redo
- `Ctrl+Left/Right` - Word movement
- `Ctrl+Shift+1-8` - Toggle script 1-8 mute
- `Ctrl+Shift+M` - Toggle metro script mute
- `Ctrl+Shift+I` - Toggle init script mute

**REPL (Live page):**
- `Ctrl+Up/Down` - Scroll output history
- `CLEAR` - Clear all output
- `Ctrl+Q` - Quit application

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
- Voice trigger indicators (H|P):
  - H = HD2/Complex oscillator triggered (TR command)
  - P = Plaits engine triggered (PLTR command)
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
- `SCOPE.GAIN` / `SCG <0-16383>` - Input gain (8192=1x)
- `SCOPE.RST` / `SCR` - Reset all scope settings to defaults

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

**Noise Source:**
- `NOISE.WAV` / `NW <0-2>` - Type (0=White, 1=Pink, 2=Brown)
- `NOISE.PRI` / `NP <0-16383>` - Noise → primary FM
- `NOISE.MOD` / `NM <0-16383>` - Noise → modulator FM
- `NOISE.VOL` / `NV <0-16383>` - Noise volume

**Source Levels:**
- `PRI.VOL` / `PV <0-16383>` - Primary volume
- `MOD.VOL` / `MV <0-16383>` - Modulator volume

### FM Synthesis

**FM Modulation:**
- `MBUS.FM` / `FM <0-16383>` - FM index amount
- `FMEV.AMT` / `FA <0-16383>` - FM envelope (14-bit)
- `FMEV.DEC` / `FD <1-10000>` - FM envelope decay (ms)
- `FMEV.ATK` / `FAA <1-10000>` - FM envelope attack (ms)
- `FMEV.CRV <-8 to 8>` - FM envelope curve

**Feedback FM:**
- `MOSC.FB` / `FB <0-16383>` - Feedback amount
- `FBEV.AMT` / `FBA <0-16383>` - FB envelope (14-bit)
- `FBEV.DEC` / `FBD <1-10000>` - FB envelope decay (ms)
- `FBEV.ATK` / `FBAA <1-10000>` - FB envelope attack (ms)
- `FBEV.CRV` / `FBC <-8 to 8>` - FB envelope curve

### Discontinuity / Waveshaping

**Discontinuity:**
- `DISC.AMT` / `DC <0-16383>` - Discontinuity amount
- `DISC.MODE` / `DM <0-6>` - Mode (0=Fold, 1=Tanh, 2=Softclip, 3=Hard, 4=Asym, 5=Rectify, 6=Crush)
- `DENV.AMT` / `DA <0-16383>` - Disc envelope (14-bit)
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
- `ROUT.MC` / `MC` - Mod → Filter cutoff

**Mix Routing:**
- `MBUS.MIX` / `MX <0-16383>` - Mix amount
- `MBUS.MMX` / `MM <0|1>` - Mod → Mix
- `MBUS.EMX` / `ME <0|1>` - Envelope → Mix

### Envelopes

All envelopes are simple percussive (attack + decay) and trigger on each `TR`.

| Envelope | Decay | Amount | Attack | Curve |
|----------|-------|--------|--------|-------|
| **Amplitude** | AD | - | AENV.ATK (AA) | AENV.CRV (AC) |
| **Pitch** | PD | PA (0-16) | PENV.ATK (PAA) | PENV.CRV (PC) |
| **FM** | FD | FA (0-16383) | FMEV.ATK (FAA) | FMEV.CRV |
| **Discontinuity** | DD | DA (0-16383) | DENV.ATK (DAA) | DENV.CRV |
| **Feedback** | FBD | FBA (0-16383) | FBEV.ATK (FBAA) | FBEV.CRV (FBC) |
| **Filter** | FED | FE (0-16383) | FLEV.ATK (FLAA) | FLEV.CRV (FLC) |

**Envelope Parameters:**
- **Decay**: 1-10000 ms
- **Attack**: 1-10000 ms
- **Curve**: -8 (log) to 8 (exp), 0=linear
- **Amount**: 14-bit (0-16383) except pitch (0-16 octaves)

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
Ring Mod → Amp → EQ → Pan → Pitch Shift → Beat Repeat →
MiClouds Granular → Stereo Delay → Compressor → Plate Reverb
```

### Multi-Mode Filter (14 Types)

- `FILT.CUT` / `FC <20-20000>` - Cutoff (Hz)
- `FILT.RES` / `FQ <0-16383>` - Resonance
- `FILT.TYP` / `FT <0-13>` - Type (14 types)
- `FILT.KEY` / `FK <0-16383>` - Key tracking
- `FLEV.AMT` / `FE <0-16383>` - Filter env amt
- `FLEV.DEC` / `FED <1-10000>` - Filter env decay

**Filter Types (FT):**
- 0: SVF LP (lowpass)
- 1: SVF HP (highpass)
- 2: SVF BP (bandpass)
- 3: SVF Notch
- 4: MoogFF (warm, self-oscillating)
- 5: RLPF (punchy 12dB LP)
- 6: RHPF (tight 12dB HP)
- 7: DFM1 LP (scuzzy diode LP)
- 8: DFM1 HP (diode HP)
- 9: BMoog LP (24dB saturating LP)
- 10: BMoog HP (24dB saturating HP)
- 11: BMoog BP (24dB saturating BP)
- 12: Latch-SC LP (switched-cap LP)
- 13: Latch-SC HP (switched-cap HP)

**ModBus Routing:**
- `MC <0|1>` - ModBus → Filter cutoff (toggle)
- `MQ <0|1>` - ModBus → Filter resonance (toggle)
- `MODF.CUT` / `MFF <0-16383>` - ModBus → Cutoff amount
- `MODF.RES` / `MFQ <0-16383>` - ModBus → Resonance amount

### Ring Modulator

- `RING.FRQ` / `RGF <20-2000>` - Ring mod frequency (Hz)
- `RING.WAV` / `RGW <0-3>` - Waveform (0=Sin, 1=Tri, 2=Saw)
- `RING.MIX` / `RGM <0-16383>` - Ring mod mix

### Compressor

- `COMP.THR` / `CT <0-16383>` - Threshold
- `COMP.RAT` / `CR <1-20>` - Ratio (1=off)
- `COMP.ATK` / `CA <1-500>` - Attack (ms)
- `COMP.REL` / `CL <10-2000>` - Release (ms)
- `COMP.MKP` / `CM <0-16383>` - Makeup gain
- `COMP.AUTO` / `CAU <0|1>` - Auto makeup gain (0=manual, 1=auto)
- `CR.MIX` / `CRMIX <0-16383>` - Dry/wet mix

**Dry/Wet Mix:**
- 0 = 100% dry (bypass compression)
- 16383 = 100% wet (fully compressed)
- Enables parallel compression techniques
- Default: 16383 (backward compatible)

### Pan

- `OUT.PAN` / `PAN <-16383 to 16383>` - Stereo position

### Per-Voice Mixer

Individual volume, pan, and mute for each voice. View in GRID.MODE 3.

**Volume (0-16383):**
- `VOL.OSC` / `VO` - Complex oscillator volume
- `VOL.PLA` / `VP` - Plaits volume
- `VOL.NOS` / `VN` - Noise volume
- `VOL.SMP` / `VS` - Sampler volume

**Pan (-8192 to 8191, 0=center):**
- `PAN.OSC` / `PO` - Complex oscillator pan
- `PAN.PLA` / `PP` - Plaits pan
- `PAN.NOS` / `PNN` - Noise pan
- `PAN.SMP` / `PS` - Sampler pan

**Mute (0|1):**
- `MUTE.OSC` / `MO` - Mute complex oscillator
- `MUTE.PLA` / `MPL` - Mute Plaits
- `MUTE.NOS` / `MN` - Mute noise
- `MUTE.SMP` / `MS` - Mute sampler

Note: These are distinct from script mutes (`MUTE.1-8`, `MUTE.M`, `MUTE.I`).

### MiClouds Granular Effect

MiClouds continuously records incoming audio into its buffer. Use `CL.TRIG` to trigger grain playback from the buffer. Activates when `CL.WET` > 0.

**Core Parameters:**
- `CL.TRIG` / `CLTR` - Trigger grain playback
- `CL.PITCH` / `CLP <0-16383>` - Pitch (8192=center/no transpose)
- `CL.POS` / `CLO <0-16383>` - Buffer read position
- `CL.SIZE` / `CLS <0-16383>` - Grain size
- `CL.DENS` / `CLD <0-16383>` - Grain density
- `CL.TEX` / `CLT <0-16383>` - Texture/character
- `CL.WET` / `CLW <0-16383>` - Wet/dry mix (activates effect)

**Processing:**
- `CL.GAIN` / `CLG <0-16383>` - Input gain (8192=unity)
- `CL.SPREAD` / `CLSP <0-16383>` - Stereo spread
- `CL.RVB` / `CLRV <0-16383>` - Internal reverb
- `CL.FB` / `CLF <0-16383>` - Feedback (>10000 risky!)
- `CL.FREEZE` / `CLFZ <0|1>` - Freeze buffer recording
- `CL.MODE` / `CLM <0-3>` - Mode (0=Granular, 1=Pitch, 2=Looping, 3=Spectral)
- `CL.LOFI` / `CLLO <0-16383>` - Lo-fi sample rate reduction

**Usage Tips:**
- Buffer continuously records unless frozen
- `CL.TRIG` triggers grain playback from buffer
- High feedback values (>10000) can self-oscillate
- Freeze buffer to process static material
- Combine with pattern triggers for rhythmic granulation

### Beat Repeat

Beat repeat activates automatically when `BR.MIX` is greater than 0.

- `BR.LEN` / `BRL <0-7>` - Division (0=1/16, 1=1/8...7=8×)
- `BR.REV` / `BRR <0|1>` - Reverse playback
- `BR.WIN` / `BRW <1-50>` - Window size (ms)
- `BR.MIX` / `BRX <0-16383>` - Beat repeat mix (activates when > 0)

### Pitch Shift

- `PS.MODE` / `PSM <0|1>` - Mode (0=Normal, 1=Granular)
- `PS.SEMI` / `PSS <-24 to 24>` - Pitch shift (semitones)
- `PS.GRAIN` / `PSG <5-100>` - Grain size (ms)
- `PS.MIX` / `PSX <0-16383>` - Pitch shift mix
- `PS.TARG` / `PST <0|1>` - Target (0=Input, 1=Output)

### Stereo Delay

- `DLY.TIME` / `DT <0-16383>` - Delay time (14-bit, exponential curve)
- `DLY.SYN` / `DS <0|1>` - Sync mode (0=Free running, 1=BPM sync)
- `DLY.FB` / `DF <0-16383>` - Feedback
- `DLY.LP` / `DLP <Hz>` - Lowpass filter
- `DLY.WET` / `DW <0-16383>` - Wet mix

**Sync Modes:**
- Free (DS=0): DT maps to ~1ms-2s via exponential curve
- BPM Sync (DS=1): DT quantizes to musical divisions (1/64 to 8 bars)

**Delay Routing:**
- `DLY.MODE` / `D.MODE <0-2>` - 0=Bypass, 1=Insert, 2=Send
- `DLY.TAIL` / `D.TAIL <0-2>` - 0=Cut, 1=Ring, 2=Freeze

### 3-Band EQ

- `EQ.LOW` / `EL <-24 to 24>` - Low shelf (dB)
- `ELF <20-2000>` - Low shelf frequency (Hz, default 200)
- `EQ.MID` / `EM <-24 to 24>` - Mid peak (dB)
- `EQ.FRQ` / `EF <200-8000>` - Mid frequency (Hz)
- `EQ.Q <0.1-10>` - Mid Q factor
- `EQ.HI` / `EH <-24 to 24>` - High shelf (dB)
- `EHF <1000-20000>` - High shelf frequency (Hz, default 4000)

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

### Sampler

The sampler provides two loading modes: **KIT mode** (directory of one-shot samples) and **SLICE mode** (single file auto-sliced). Sampler output mixes with the HD2/Plaits voices.

**Loading:**
- `KIT <path>` - Load kit or file
  - If path is a directory: KIT mode (one sample per slot, 0-127)
  - If path is a file: SLICE mode (auto-slices file across slots)
- `KIT` - List available kits (REPL only)
- `KIT.LEN` / `KL` - Returns number of slots (usable in expressions: `PN.L 0 KL`)
- `KIT.INFO` - Displays current kit information
- `STR <n>` - Trigger slot (accepts expressions: `STR RND 0 15`, `STR + A 1`)
- `STR` - Re-trigger current slot

**Playback Parameters:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `S.RATE` | `SR` | 0-16383 | Playback rate (8192=1x) |
| `S.PITCH` | `SPT` | -24 to 24 | Pitch shift (semitones) |
| `S.FINE` | `SFN` | -100 to 100 | Fine tune (cents) |
| `S.DIR` | `SD` | 0\|1 | Direction (0=forward, 1=reverse) |
| `S.LOOP` | `SL` | 0\|1 | Loop mode on/off |
| `S.START` | `SST` | 0-16383 | Start offset position |
| `S.LEN` | `SLE` | 0-16383 | Loop length |

**Slicing:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `S.SLICE` | `SSL` | 2-128 | Equal-divide sample into N slices |
| `S.ONSET` | `SON` | 1-100 | Detect transients (sensitivity 1-100) |
| `S.ONSET.MIN` | `SONM` | 10-1000 | Minimum slice spacing (ms) |

**Envelope:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `S.ATK` | `SA` | 0-16383 | Attack time (ms) |
| `S.DEC` | `SDC` | 0-16383 | Decay time (ms) |
| `S.REL` | `SRE` | 0-16383 | Release time (ms) |
| `S.SUST` | `SSU` | 0\|1 | Sustain mode (0=one-shot, 1=gate) |

**Modulation:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `S.RATEMOD` | `SRM` | 0-16383 | Rate modulation amount (via modbus) |
| `S.PITCHMOD` | `SPM` | 0-16383 | Pitch modulation amount (via modbus) |

**Sampler Effects - Filter Modulation:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `SF.CUTMOD` | `SFCM` | 0-16383 | Filter cutoff modulation (via modbus) |
| `SF.RESMOD` | `SFQM` | 0-16383 | Filter resonance modulation (via modbus) |

**Sampler Effects - Filter:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `SF.CUT` | `SFC` | 0-16383 | Filter cutoff |
| `SF.RES` | `SFQ` | 0-16383 | Filter resonance |
| `SF.TYPE` | `SFT` | 0-13 | Filter type (see below) |

**Sampler Filter Types:**
- 0: SVF LP, 1: SVF HP, 2: SVF BP, 3: SVF Notch
- 4: MoogFF, 5: Butter LP, 6: Butter HP
- 7: DFM1 LP, 8: DFM1 HP
- 9: BMoog LP, 10: BMoog HP, 11: BMoog BP
- 12: SVF Latched LP, 13: SVF Latched BP

**Sampler Effects - Decimator:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `SF.BITS` | `SFB` | 1-24 | Bit depth reduction |
| `SF.RATE` | `SFR` | 0-16383 | Sample rate reduction amount |
| `SF.DECI` | `SFD` | 0-16383 | Decimator mix |

**Sampler Effects - MiRings Resonator:**

| Command | Alias | Range | Description |
|---------|-------|-------|-------------|
| `SRINGS.PIT` | `SRRP` | 0-127 | Resonator pitch (MIDI note) |
| `SRINGS.STRC` | `SRRS` | 0-16383 | Structure/material |
| `SRINGS.BRIT` | `SRRB` | 0-16383 | Brightness |
| `SRINGS.DAMP` | `SRRD` | 0-16383 | Damping (decay time) |
| `SRINGS.POS` | `SRRO` | 0-16383 | Excitation position |
| `SRINGS.MODE` | `SRRM` | 0-5 | Resonator model |
| `SRINGS.WET` | `SRRW` | 0-16383 | Wet/dry mix |

**MiRings Models:**
- 0: Modal (bells, metal, wood)
- 1: Sympathetic strings
- 2: Inharmonic/modulated string
- 3: 2-OP FM voice
- 4: Sympathetic quantized (chords)
- 5: String + reverb

**Signal Flow:**
```
Sample Playback → Decimator → Filter → MiRings →
VOL.SMP/PAN.SMP → Main Mix (with HD2/Plaits)
```

For spatial effects, use the global delay, reverb, and MiClouds.

**Usage Example:**
```
KIT ~/samples/drums        # Load drum kit
STR 0                      # Trigger kick (slot 0)
S.PITCH -12                # Down one octave
STR 1                      # Trigger snare (slot 1)
SF.CUT 8000                # Filter cutoff
SF.DECI 10000              # Add lo-fi decimation
```

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

### Script Mutes

Individual mute toggles for scripts 1-8, M (metro), and I (init).

**Commands:**
```
MUTE              # Show all mute states
MUTE <1-8|M|I>    # Toggle script mute
MUTE <1-8|M|I> <0|1>  # Set mute (0=unmuted, 1=muted)
MUTE.1-8, MUTE.M, MUTE.I  # Direct mute commands
```

**Hotkeys:**
- `Ctrl+Shift+1-8` - Toggle script 1-8 mute
- `Ctrl+Shift+M` - Toggle metro script mute
- `Ctrl+Shift+I` - Toggle init script mute

**Features:**
- Muted scripts skip execution but preserve content
- Visual [MUTED] indicators in script page titles
- Mute state persists in saved scenes
- Enables workflow where scripts can be prepared without executing

**Examples:**
```
MUTE 1            # Toggle script 1 mute
MUTE M 1          # Mute metro script
MUTE.3            # Toggle script 3 mute
Ctrl+Shift+5      # Toggle script 5 mute (hotkey)
```

### Page Navigation

Programmatic page switching via PAGE command.

**Commands:**
```
PAGE <page>       # Navigate to page
PG <page>         # Short alias
```

**Supported Pages:**
- `PAGE LIVE` / `PAGE L` - Live page (REPL)
- `PAGE 1-8` - Script pages 1-8
- `PAGE M` - Metro script page
- `PAGE I` - Init script page
- `PAGE P` - Pattern page
- `PAGE V` - Variables page
- `PAGE N` - Notes page
- `PAGE S` - Scope page
- `PAGE HELP` / `PAGE H` - Help page
- `PAGE GRID` / `PAGE G` - Grid view (sets Live to grid mode)

**Examples:**
```
PAGE 1            # Switch to script 1 page
PG HELP           # Switch to help page
PAGE V            # Switch to variables page
```

**Use Cases:**
- Script-controlled UI navigation
- Metro scripts that switch pages
- Automated presentations
- Future animated transitions

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
REC.SPINNER <0-6> # Set spinner animation style
```

**Output:**
- Format: 24-bit stereo WAV @ 48kHz
- Location: Current directory
- Filename: `monokit_audio_N.wav` (sequential numbering)

**Spinner Styles:**
- 0: CIRCLED - ⊕⊖⊗⊘⊙⊚⊛⊜⊝ (9 frames, default)
- 1: FILL - ○◔◑◕●◕◑◔ (8 frames)
- 2: BREATHE - ▉▊▋▌▍▎▏... (13 frames)
- 3: BRAILLE - ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ (10 frames)
- 4: DOT - ⠁⠂⠄⡀⢀⠠⠐⠈ (8 frames)
- 5: STAR - ✶✸✹✺✹✸ (6 frames)
- 6: HALF - ◐◓◑◒ (4 frames)

Spinner animation syncs to metro beat. Use REC.SPINNER with no argument to query current style.

**Examples:**
```
REC               # Start recording to monokit_audio_1.wav
TR                # Record some sound
REC.STOP          # Stop and save file
REC.SPINNER 3     # Set to Braille spinner style
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
- Mode 1: Full reset before loading (uses safe 1ms delays)
- LOAD.RST prevents scene loading crashes via parameter throttling

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
GRID.MODE <0-5>   # Grid display (0=Text, 1=Icons, 2=EQ/Comp, 3=Mixer, 4=FX, 5=Sampler)
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
SCRMBL.GRID <0|1> # Grid scramble (independent of title)
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
DEBUG             # Query current level
```

**Tiers:**

| Level | Name | Shows |
|-------|------|-------|
| 0 | SILENT | Nothing (completely quiet) |
| 1 | ERRORS | Validation/parse errors only |
| 2 | ESSENTIAL | + Metro, scene, recording status |
| 3 | QUERIES | + Variable/pattern reads, M? |
| 4 | CONFIRMS | + "SET X TO Y", "SENT TRIGGER" |
| 5 | VERBOSE | Reserved for future diagnostics |

**Examples by tier:**
- **Tier 1** (ERRORS): `FT 99` → "FT: RANGE 0-13"
- **Tier 2** (ESSENTIAL): `M 250` → "METRO: 250 MS"
- **Tier 3** (QUERIES): `A` → "A: 42"
- **Tier 4** (CONFIRMS): `VOL 0.5` → "SET VOLUME TO 0.5"

**Behavior:** Setting DEBUG automatically synchronizes
the OUT.* flags to match. DEBUG 0 disables all output
categories; DEBUG 4 enables all. Use OUT.* commands
for fine-grained control after setting DEBUG level.

**Category Overrides:**
```
OUT.ERR <0|1>     # Toggle: show errors
OUT.ESS <0|1>     # Toggle: show essential
OUT.QRY <0|1>     # Toggle: show queries
OUT.CFM <0|1>     # Toggle: show confirms
```

These allow selective enabling after DEBUG is set.
Example: `DEBUG 0` then `OUT.ERR 1` shows only errors.

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
REPL.DUMP         # Save REPL output to repl_dump.txt
REPL.DUMP <file>  # Save to custom filename
CFM.QUIT <0|1>    # Toggle confirm-on-quit for unsaved scenes
CFM.SAVE <0|1>    # Toggle confirm-on-overwrite for existing scenes
```

**Default Values (RST):**
- PF: 131 Hz (C3)
- MF: 262 Hz (C4)
- PA: 0 (no pitch envelope)
- FC: 10000 Hz (filter open)
- D.MODE: 2 (Send)
- R.MODE: 2 (Send)
- CR: 1 (Compressor off)
- PLV: 0 (Plaits silent, prevents artifacts)

**RST Behavior:**
- Manual RST: Instant (0ms delays)
- LOAD.RST: Safe delays (1ms per param)
- Plaits requires PLV after RST to produce sound

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
| `Ctrl+G` | Cycle GRID.MODE 0-5 (Live page) |
| `Ctrl+F` | Search mode |
| `Ctrl+Up/Down` | Scroll REPL |
| `Ctrl+D` | Duplicate line |
| `Ctrl+K` | Delete line |
| `Ctrl+C/X/V` | Copy/cut/paste line |
| `Ctrl+Z` | Undo (page-local) |
| `Ctrl+Shift+Z` | Redo |
| `Ctrl+Shift+1-8` | Toggle script 1-8 mute |
| `Ctrl+Shift+M` | Toggle metro mute |
| `Ctrl+Shift+I` | Toggle init mute |
| `Ctrl+Q` | Quit application |

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
| `FMEV.AMT <0-16383>` | `FA` | FM envelope (14-bit) |
| `FMEV.DEC <ms>` | `FD` | FM envelope decay |
| `FMEV.ATK <ms>` | `FAA` | FM envelope attack |
| `FMEV.CRV <v>` | - | FM envelope curve |
| `MOSC.FB <amt>` | `FB` | Feedback amount |
| `FBEV.AMT <0-16383>` | `FBA` | FB envelope (14-bit) |
| `FBEV.DEC <ms>` | `FBD` | FB envelope decay |
| `FBEV.ATK <ms>` | `FBAA` | FB envelope attack |
| `FBEV.CRV <v>` | `FBC` | FB envelope curve |

### Discontinuity & Lo-Fi

| Command | Alias | Description |
|---------|-------|-------------|
| `DISC.AMT <amt>` | `DC` | Disc amount |
| `DISC.MODE <0-6>` | `DM` | Disc mode |
| `DENV.AMT <0-16383>` | `DA` | Disc env (14-bit) |
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
| `ROUT.MC <0\|1>` | `MC` | Mod → Filter CUT |
| `ROUT.MQ <0\|1>` | `MQ` | Mod → Filter RES |

**MOD ROUTING (AMOUNTS):**
| `MODF.CUT <0-16383>` | `MFF` | Mod→CUT Amount |
| `MODF.RES <0-16383>` | `MFQ` | Mod→RES Amount |
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
| `FILT.TYP <0-13>` | `FT` | Type (14 filter types) |
| `FILT.KEY <amt>` | `FK` | Key tracking |
| `MODF.CUT <0-16383>` | `MFF` | Mod→Cutoff amount |
| `MODF.RES <0-16383>` | `MFQ` | Mod→Res amount |
| `RING.FRQ <hz>` | `RGF` | Ring mod freq |
| `RING.WAV <0-3>` | `RGW` | Ring mod wave |
| `RING.MIX <amt>` | `RGM` | Ring mod mix |
| `COMP.THR <amt>` | `CT` | Compressor threshold |
| `COMP.RAT <1-20>` | `CR` | Compressor ratio |
| `COMP.ATK <ms>` | `CA` | Compressor attack |
| `COMP.REL <ms>` | `CL` | Compressor release |
| `COMP.MKP <amt>` | `CM` | Compressor makeup |
| `COMP.AUTO <0\|1>` | `CAU` | Auto makeup (0=manual, 1=auto) |
| `CR.MIX <0-16383>` | `CRMIX` | Compressor dry/wet mix |
| `OUT.PAN <amt>` | `PAN` | Stereo pan |

### MiClouds Granular

| Command | Alias | Description |
|---------|-------|-------------|
| `CL.TRIG` | `CLTR` | Trigger grain playback |
| `CL.PITCH <0-16383>` | `CLP` | Pitch (8192=center) |
| `CL.POS <0-16383>` | `CLO` | Buffer position |
| `CL.SIZE <0-16383>` | `CLS` | Grain size |
| `CL.DENS <0-16383>` | `CLD` | Density |
| `CL.TEX <0-16383>` | `CLT` | Texture |
| `CL.WET <0-16383>` | `CLW` | Wet mix (activates) |
| `CL.GAIN <0-16383>` | `CLG` | Input gain (8192=1x) |
| `CL.SPREAD <0-16383>` | `CLSP` | Stereo spread |
| `CL.RVB <0-16383>` | `CLRV` | Internal reverb |
| `CL.FB <0-16383>` | `CLF` | Feedback |
| `CL.FREEZE <0\|1>` | `CLFZ` | Freeze buffer |
| `CL.MODE <0-3>` | `CLM` | Mode (Gran/Pitch/Loop/Spec) |
| `CL.LOFI <0-16383>` | `CLLO` | Lo-fi effect |

### Beat Repeat & Pitch Shift

| Command | Alias | Description |
|---------|-------|-------------|
| `BR.LEN <0-7>` | `BRL` | Division |
| `BR.REV <0\|1>` | `BRR` | Reverse |
| `BR.WIN <1-50>` | `BRW` | Window (ms) |
| `BR.MIX <amt>` | `BRX` | Beat repeat mix (activates >0) |
| `PS.MODE <0\|1>` | `PSM` | Pitch shift mode |
| `PS.SEMI <-24-24>` | `PSS` | Semitones |
| `PS.GRAIN <5-100>` | `PSG` | Grain size (ms) |
| `PS.MIX <amt>` | `PSX` | Pitch shift mix |
| `PS.TARG <0\|1>` | `PST` | Target (In/Out) |

### Delay & Reverb

| Command | Alias | Description |
|---------|-------|-------------|
| `DLY.TIME <0-16383>` | `DT` | Delay time (14-bit exp) |
| `DLY.SYN <0\|1>` | `DS` | Sync (0=Free 1=BPM) |
| `DLY.FB <amt>` | `DF` | Delay feedback |
| `DLY.LP <hz>` | `DLP` | Delay lowpass |
| `DLY.WET <amt>` | `DW` | Delay wet mix |
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
| `ELF <20-2000>` | - | Low shelf freq (Hz, default 200) |
| `EQ.MID <db>` | `EM` | Mid peak (-24 to 24) |
| `EQ.FRQ <hz>` | `EF` | Mid frequency |
| `EQ.Q <q>` | - | Mid Q (0.1-10) |
| `EQ.HI <db>` | `EH` | High shelf (-24 to 24) |
| `EHF <1000-20000>` | - | High shelf freq (Hz, default 4000) |

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
| `MUTE [<1-8\|M\|I>] [<0\|1>]` | Query/toggle/set script mutes |
| `MUTE.1-8, MUTE.M, MUTE.I` | Direct script mute commands |
| `PAGE <page>` / `PG <page>` | Navigate to page |
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
| `REC.SPINNER <0-6>` | Set spinner style (0-6) |

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
| `GRID.MODE <0-5>` | Text/Icons/EQ-Comp/Mixer/FX/Sampler |
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
| `SCRMBL.GRID <0\|1>` / `SG` | Grid scramble toggle |
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
| `SCOPE.GAIN / SCG <0-16383>` | Scope input gain (8192=1x) |
| `SCOPE.RST / SCR` | Reset scope settings |

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
| `CFM.QUIT <0\|1>` | Confirm on quit |
| `CFM.SAVE <0\|1>` | Confirm on overwrite |

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

Configuration files are stored in platform-specific locations:

**macOS:**
- Config: `~/Library/Application Support/monokit/config.toml`
- Scenes: `~/Library/Application Support/monokit/scenes/`
- User presets: `~/Library/Application Support/monokit/presets/`

**Linux:**
- Config: `~/.config/monokit/config.toml`
- Scenes: `~/.config/monokit/scenes/`
- User presets: `~/.config/monokit/presets/`

**Windows:**
- Config: `%APPDATA%\monokit\config.toml`
- Scenes: `%APPDATA%\monokit\scenes\`
- User presets: `%APPDATA%\monokit\presets\`

Note: `%APPDATA%` typically expands to `C:\Users\<username>\AppData\Roaming`

---

## Terminal Requirements

**Minimum:** 50 columns × 18 rows

**Recommended:**
- True color support (iTerm2, Kitty, Alacritty, Windows Terminal)
- 256-color fallback supported (automatic)
- Monospace font (Monaco recommended for block chars)

**macOS Terminal.app limitations:**
- Limited color support (256-color mode auto-enabled)
- Use `METER.ASCII 1` for better meter rendering

**Windows:**
- Windows Terminal (recommended) - full true color support
- PowerShell or Command Prompt work but may have limited color support
- For low-latency audio, install ASIO drivers (ASIO4ALL or your interface's native driver)

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
