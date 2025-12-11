# Monokit Command Reference

Complete command reference for all Monokit commands.

---

## Naming Convention

Monokit uses a **PREFIX.SUFFIX** naming convention for canonical command forms:

**Category Prefixes:**
- `POSC` - Primary Oscillator (POSC.FREQ → PF, POSC.WAVE → PW)
- `MOSC` - Modulator Oscillator (MOSC.FREQ → MF, MOSC.WAVE → MW, MOSC.FB → FB)
- `DISC` - Discontinuity/Waveshaping (DISC.AMT → DC, DISC.MODE → DM)
- `FILT` - SVF Filter (FILT.CUT → FC, FILT.RES → FQ, FILT.TYP → FT)
- `RESO` - Comb Resonator (RESO.FRQ → RF, RESO.DEC → RD, RESO.MIX → RM)
- `DLY` - Stereo Delay (DLY.TIME → DT, DLY.FB → DF, DLY.WET → DW)
- `REV` - Plate Reverb (REV.DEC → RV, REV.PRE → RP, REV.WET → RW)
- `LOFI` - Lo-Fi Processor (LOFI.BIT → LB, LOFI.SMP → LS, LOFI.MIX → LM)
- `RING` - Ring Modulator (RING.FRQ → RGF, RING.WAV → RGW, RING.MIX → RGM)
- `COMP` - Compressor (COMP.THR → CT, COMP.RAT → CR, COMP.ATK → CA)
- `EQ` - 3-Band EQ (EQ.LOW → EL, EQ.MID → EM, EQ.HI → EH)
- `MBUS` - Modulation Bus (MBUS.AMT → MB, MBUS.TRK → TK, MBUS.FM → FM)
- `ROUT` - Routing Matrix (ROUT.MP → MP, ROUT.MD → MD, ROUT.MC → MC, ROUT.MQ → MQ)
- `OUT` - Output (OUT.VOL → VOL, OUT.PAN → PAN)
- Envelope prefixes: `AENV`, `PENV`, `FMEV`, `DENV`, `FBEV`, `FLEV`, `MBEV`

**Alias System:**
- Short forms (PF, FC, AD, etc.) are aliases to canonical forms
- Both work identically: `PF 440` or `POSC.FREQ 440`

---

## Trigger & Volume
- `TR` - Trigger voice (sends gate pulse)
- `VOL <0.0-1.0>` - Set master volume

## Metro/Timing
- `M` / `M <ms>` - Get/set metro interval (milliseconds)
- `M.BPM <bpm>` - Set tempo as BPM
- `M.ACT <0|1>` - Activate/deactivate metro
- `M.SCRIPT <1-8>` - Set which script metro calls
- `M.SYNC <0|1>` - Sync mode (0=internal, 1=MIDI clock)
- `MIDI.IN` / `MIDI.IN <name>` - List/connect MIDI input devices

## Delayed Execution
- `DEL <ms>: <cmd>` - Execute command after delay (max 16000ms)
- `DEL.CLR` - Clear all pending delayed commands
- `DEL.X <count> <ms>: <cmd>` - Queue command N times at intervals
- `DEL.R <count> <ms>: <cmd>` - Execute immediately, then repeat

## Scripts
- `SCRIPT <1-8>` - Execute stored script (supports expressions)
- Scripts 1-8: User scripts
- M script (index 8): Called on each metro tick
- I script (index 9): Called on startup

## Scenes
- `SAVE <name>` - Save current scene
- `LOAD <name>` - Load scene from file
- `SCENES` - List available saved scenes
- `DELETE <name>` - Delete a saved scene

## Variables
- `A`, `B`, `C`, `D`, `X`, `Y`, `Z`, `T` - Global accumulators
- `J`, `K` - Per-script local variables
- `I` - Loop counter (read-only, scoped to L loops)

## Counters (N1-N4)
- `N1`, `N2`, `N3`, `N4` - Read and auto-increment
- `N1.MIN <n>`, `N1.MAX <n>` - Set bounds
- `N1.RST` - Reset to MIN value

## Patterns (Working Pattern - P)

**State & Query:**
- `P.N` / `P.N <0-5>` - Get/set working pattern
- `P.L` / `P.L <1-64>` - Get/set pattern length
- `P.I` / `P.I <0-63>` - Get/set playhead index
- `P <idx>` / `P <idx> <val>` - Get/set value at index
- `P.HERE`, `P.NEXT`, `P.PREV` - Playhead operations

**Manipulation:**
- `P.PUSH <val>`, `P.POP`, `P.INS <idx> <val>`, `P.RM <idx>`
- `P.REV`, `P.ROT <n>`, `P.SHUF`, `P.SORT`

**Math Operations:**
- `P.ADD <val>`, `P.SUB <val>`, `P.MUL <val>`, `P.DIV <val>`, `P.MOD <val>`
- `P.SCALE <min> <max>`, `P.RND [min] [max]`

**Query Operations:**
- `P.MIN`, `P.MAX`, `P.SUM`, `P.AVG`, `P.FND <val>`

## Patterns (Explicit Pattern - PN)
Same operations as P.* but with explicit pattern number:
- `PN <pat> <idx>`, `PN.L <pat>`, `PN.I <pat>`, etc.

## Inline Sequences (SEQ)
- `SEQ "<pattern>"` - Cycle through values on each evaluation

**Phase 1 Tokens:** `x` (trigger), `_`/`.` (rest), numbers, note names (C3, F#4, Bb2)

**Phase 2 Features:**
- `*n` - Repeat token n times
- `?` - Random trigger (50% chance)
- `<a b>` - Toggle/cycle (deterministic)
- `{a b}` - Random choice (unpredictable)

## Preset System (PSET)
- `PSET <script> <name>` - Load preset into script
- `PSET.SAVE <script> <name>` - Save script as preset
- `PSET.DEL <name>` - Delete user preset
- `PSETS` - List all presets

## Control Flow
- `IF <expr>: <cmd>` - Execute if expr != 0
- `ELIF <cond>: <cmd>` - Else-if branch
- `ELSE: <cmd>` - Else branch
- `PROB <0-100>: <cmd>` - Execute with probability
- `EV <n>: <cmd>` - Execute every Nth tick
- `SKIP <n>: <cmd>` - Skip every Nth tick
- `L <start> <end>: <commands>` - Loop
- `BRK` - Stop current script execution
- `cmd1; cmd2; cmd3` - Multiple commands on one line

## Comparison Operators
- `EZ <x>`, `NZ <x>` - Equals/not equals zero
- `EQ <a> <b>`, `NE <a> <b>` - Equals/not equals
- `GT <a> <b>`, `LT <a> <b>` - Greater/less than
- `GTE <a> <b>`, `LTE <a> <b>` - Greater/less than or equal
- Infix: `>`, `<`, `>=`, `<=`, `==`, `!=`

## Synth Parameters (77 total)

### Primary Oscillator
- `PF <hz>` - Primary frequency (20-20000)
- `PW <0-2>` - Primary waveform (0=sine, 1=triangle, 2=sawtooth)

### Modulator Oscillator
- `MF <hz>` - Modulator frequency (20-20000)
- `MW <0-3>` - Modulator waveform (0=sine, 1=triangle, 2=sawtooth, 3=feedback)
- `FB <0-16383>` - Feedback amount

### Discontinuity (Waveshaping)
- `DC <0-16383>` - Discontinuity amount
- `DM <0-6>` - Mode (0=fold, 1=tanh, 2=softclip, 3=hard, 4=asym, 5=rectify, 6=crush)

### Lo-Fi Processor
- `LB <1-16>` - Bit depth
- `LS <100-48000>` - Sample rate reduction (Hz)
- `LM <0-16383>` - Lo-fi mix

### Tracking & Modulation Bus
- `TK <0-16383>` - Tracking amount
- `MB <0-16383>` - Modulation bus amount
- `MBA <0-16383>` - ModBus envelope amount
- `MBD <1-10000>` - ModBus envelope decay (ms)
- `MP`, `MD`, `MT`, `MA` <0|1> - Routing enables

### FM Synthesis
- `FM <0-16383>` - FM index

### Mix Controls
- `MX <0-16383>` - Mix amount
- `MM <0-16383>` - Mix modulation amount
- `ME <0|1>` - Mix modulation enable

### Envelope Controls
All envelopes: `*.DEC` (decay), `*.ATK` (attack), `*.CRV` (curve), `*.AMT` (amount)

Prefixes: `AENV`, `PENV`, `FMEV`, `DENV`, `FBEV`, `FLEV`, `MBEV`

Short forms: AD, PD, FD, DD, FBD, FED (decay); PA, FA, DA, FBA, FE (amount)

### SVF Multi-Mode Filter
- `FC <hz>` - Cutoff frequency (20-20000)
- `FQ <0-16383>` - Resonance
- `FT <0-3>` - Type (0=LP, 1=HP, 2=BP, 3=Notch)
- `FE <0-16383>` - Filter envelope amount
- `FED <ms>` - Filter envelope decay
- `FK <0-16383>` - Key tracking amount
- `MC <0|1>` - ModBus → Filter cutoff routing
- `MQ <0|1>` - ModBus → Filter resonance routing

### Ring Modulator
- `RGF <20-2000>` - Frequency (Hz)
- `RGW <0-3>` - Waveform
- `RGM <0-16383>` - Mix

### Comb Resonator
- `RF <hz>` - Frequency (20-5000)
- `RD <ms>` - Decay time (10-5000)
- `RM <0-16383>` - Mix
- `RK <0-16383>` - Key tracking

### Compressor
- `CT <0-16383>` - Threshold
- `CR <1-20>` - Ratio
- `CA <1-500>` - Attack (ms)
- `CL <10-2000>` - Release (ms)
- `CM <0-16383>` - Makeup gain

### Pan
- `PAN <-16383 to +16383>` - Stereo position

### Beat Repeat
- `BR.LEN <0-7>` - Loop division
- `BR.REV <0|1>` - Reverse playback
- `BR.WIN <1-50>` - Window size (ms)
- `BR.MIX <0-16383>` - Mix (activates when > 0)

### Pitch Shift
- `PS.MODE <0|1>` - Mode (0=normal, 1=granular)
- `PS.SEMI <-24 to 24>` - Semitones
- `PS.GRAIN <5-100>` - Grain size (ms)
- `PS.MIX <0-16383>` - Mix
- `PS.TARG <0|1>` - Target (0=input, 1=output)

### Stereo Delay
- `DT <ms>` - Delay time (1-2000)
- `DF <0-16383>` - Feedback
- `DLP <hz>` - Lowpass cutoff (100-20000)
- `DW <0-16383>` - Wet mix

### 3-Band EQ
- `EL <-24 to +24>` - Low shelf gain (dB)
- `EM <-24 to +24>` - Mid peak gain (dB)
- `EF <200-8000>` - Mid center frequency (Hz)
- `EQ <0.1-10>` - Mid Q
- `EH <-24 to +24>` - High shelf gain (dB)

### Plate Reverb
- `RV <0-16383>` - Size/decay
- `RP <ms>` - Pre-delay (0-100)
- `RH <0-16383>` - High damping
- `RW <0-16383>` - Wet mix

### Effect Routing
- `D.MODE <0-2>` - Delay mode (0=BYPASS, 1=INSERT, 2=SEND)
- `D.TAIL <0-2>` - Delay tail (0=CUT, 1=RING, 2=FREEZE)
- `R.MODE <0-2>` - Reverb mode
- `R.TAIL <0-2>` - Reverb tail

## Math Operations
- `ADD <a> <b>`, `SUB <a> <b>`, `MUL <a> <b>`, `DIV <a> <b>`, `MOD <a> <b>`
- `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Range mapping

## Random Number Generation
- `RND <max>` - Random 0 to max
- `RRND <min> <max>` - Random min to max
- `TOSS` - Random 0 or 1
- `EITH <a> <b>` - Random choice
- `TOG <a> <b>` - Alternates between values
- `ER <fill> <length> <step>` - Euclidean rhythm
- `NR <prime> <mask> <factor> <step>` - Numeric repetitor

## Note/Pitch Conversion
- `N <semitones>` - Semitones to Hz (N 0 = C3 = 131 Hz)

## Scale Quantization
- `Q <note>` - Quantize to current scale
- `Q.ROOT <0-11>` - Set root note
- `Q.SCALE <0-11>` - Set scale type
- `Q.BIT <binary>` - Custom scale mask

## Recording
- `REC` - Start recording
- `REC.STOP` - Stop recording
- `REC.PATH <prefix>` - Set recording path

## Parameter Slew
- `SLEW.ALL <0-10000>` - Global slew time (ms)
- `SLEW <param> <0-10000>` - Per-parameter slew

## Oscilloscope
- `SCOPE.TIME <5-500>` - Waveform timespan (ms)
- `SCOPE.CLR <0-8>` - Waveform color
- `SCOPE.MODE <0-4>` - Display mode
- `SCOPE.UNI <0|1>` - Unipolar mode

## Notes
- `NOTE "text"` - Append to Notes page
- `NOTE.CLR` - Clear all notes

## System
- `RST` - Reset all parameters to defaults
- `LOAD.RST <0|1>` - Reset mode on scene load
- `CLEAR` - Clear REPL output
- `DEBUG <0-2>` - Set verbosity level
- `CPU <0|1>` - Toggle CPU meter
- `HEADER <0-4>` - Set header verbosity
- `PRINT "<text>"` / `PRINT <expr>` - Print to REPL
- `VERSION` / `VER` - Show version

## Themes
- `THEME` - Show/list themes
- `THEME <name>` - Switch theme

---

## Navigation (Keybindings)

### Page Cycling
- `[` / `]` - Cycle through pages

### Function Keys
- `F1`-`F8` - Script pages 1-8
- `F9` - Live, `F10` - Metro, `F11` - Init, `F12` - Pattern
- `ESC` - Toggle Help

### Alt+key (iTerm2)
- `Alt+L` - Live, `Alt+1`-`Alt+8` - Scripts
- `Alt+M` - Metro, `Alt+I` - Init, `Alt+P` - Pattern
- `Alt+S` - Scope, `Alt+V` - Variables, `Alt+H` - Help

### Editing
- `Ctrl+D` - Duplicate line
- `Ctrl+K` - Delete line
- `Ctrl+C` / `Ctrl+X` / `Ctrl+V` - Copy/Cut/Paste
- `Ctrl+Z` / `Ctrl+Shift+Z` - Undo/Redo

### Search
- `Ctrl+F` - Enter search mode
- `Enter` / `Shift+Enter` - Next/previous match
- `ESC` - Exit search
