## Command Reference

### Basic Usage:

Use `TR` in the `LIVE` page REPL to play the synth with its default settings.
Then get scripting!

```
SCRIPT 1:
PF 75; AD 250      # set primary osc freq to 100hz; set amp decay envelope to 250ms
PD 60; PA 4        # set pitch envelope decay to 60ms; set pitch envelope to sweep up 4 octaves
TR                 # trigger the voice after applying your settings!

METRO SCRIPT:
EV 4: SCRIPT 1     # call script 1 every 4 metro ticks

On the LIVE page enter the following:
M.ACT 1; M 125     # turn on the metro (clock), and make it tick every 125ms (120 BPM)

To stop ticking the metro (and effectively turn off the sound) enter the folllowing on the LIVE page:
M.ACT 0
```
Congrats, you have made a 4 on the floor kick pattern!
Go experiment with more complicated sequencing setups to trigger the voice with different parameter and fx settings.

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
| `BR.LEN <0-7>` | Division |
| `BR.REV <0\|1>` | Reverse |
| `BR.WIN <1-50>` | Window (ms) |
| `BR.MIX <amt>` | Beat repeat mix (activates >0) |
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
| `LOAD.RST [<0\|1>]` | Reset params on load |
| `LOAD.CLR [<0\|1>]` | Clear REPL on load |
| `AUTOLOAD [<0\|1>]` | Auto-load last scene |
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
| `VERSION / VER` | Show version |
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
| `LIMIT [<0\|1>]` | Limiter on/off |
| `SYNC` | Reset all stateful elements |
| `SYNC.SEQ` | Reset SEQ sequences |
| `SYNC.TOG` | Reset TOG toggles |
| `SYNC.PAT` | Reset pattern indices |
