# Monokit Debug Tier Classification

## Overview

Monokit uses a 5-tier debug output system to control verbosity. Each command's output is classified according to these tiers, allowing users to control what feedback they see.

## Tier Definitions

| Tier | Constant | Name | Purpose |
|------|----------|------|---------|
| **0** | `TIER_SILENT` | Silent | No output at all |
| **1** | `TIER_ERRORS` | Errors | Only error messages |
| **2** | `TIER_ESSENTIAL` | Essential | Errors + essential information |
| **3** | `TIER_QUERIES` | Queries | Errors + essential + query results |
| **4** | `TIER_CONFIRMS` | Confirms | All output including confirmations |

### Output Categories

Commands use the `OutputCategory` enum to specify output type:
- `Error` - Maps to TIER_ERRORS (1)
- `Essential` - Maps to TIER_ESSENTIAL (2)
- `Query` - Maps to TIER_QUERIES (3)
- `Confirm` - Maps to TIER_CONFIRMS (4)

### Control Mechanism

Output is shown when:
- `debug_level >= tier_threshold` OR
- The category-specific override flag is enabled (`out_err`, `out_ess`, `out_qry`, `out_cfm`)

---

## Commands by Tier

### TIER 0 (TIER_SILENT) - No Output

**Config Commands** (46 commands)
- DEBUG, HEADER, CPU, BPM
- AUTOLOAD, LOAD.RST, LOAD.CLR
- METER.HDR, METER.GRID, METER.ASCII
- SPECTRUM, ACTIVITY, GRID, GRID.DEF, GRID.MODE
- HL.SEQ
- OUT.ERR, OUT.ESS, OUT.QRY, OUT.CFM
- SCRMBL, SCRMBL.MODE, SCRMBL.SPD, SCRMBL.CRV
- (All config toggle/enum select commands - see system/misc.rs)

**Randomization Commands** (13 commands)
- RND.VOICE, RND.OSC, RND.FM, RND.MOD, RND.ENV
- RND.P, RND.PN, RND.PALL
- RND.FX, RND.FILT, RND.DLY, RND.VERB
- RND.PL

---

### TIER 1 (TIER_ERRORS) - Errors Only

**All commands produce error output at this tier**, but only a few produce ONLY errors:

**Commands with potential misconfiguration:**
- SLEW, SLEW.ALL (currently show confirmations at Tier 1 - should be higher)

---

### TIER 2 (TIER_ESSENTIAL) - Essential Information

**System Commands** (11 commands)
- M, M.BPM, M.ACT, M.SCRIPT - Metro configuration
- RST - Reset to defaults
- SAVE, LOAD, DELETE - Scene management
- PSET, PSET.SAVE, PSET.DEL - Preset management
- REC, REC.STOP - Recording control
- LIMIT - Limiter toggle
- THEME (when setting) - Theme change
- SCOPE.TIME, SCOPE.CLR, SCOPE.MODE, SCOPE.UNI - Scope config
- TITLE, TITLE.TIMER - Title configuration
- MIDI.IN (when connecting) - MIDI connection
- AUDIO.OUT (when setting) - Audio device
- PRINT - Print expressions

**Core Commands** (4 commands)
- DEL.CLR - Clear all delayed commands
- Q.ROOT, Q.SCALE, Q.BIT - Scale configuration (hardcoded `>= 2`)

**Gate Commands** (7 commands)
- GATE, AENV.GATE, PENV.GATE, FMEV.GATE, DENV.GATE, FBEV.GATE, FLEV.GATE

**Synth Commands** (20 commands)
- VOL, PAN - Output control
- DC - Discontinuity amount
- **Envelope parameters** (17 commands):
  - AD, AENV.ATK, AENV.CRV - Amp envelope
  - FLEV.ATK, FLEV.CRV - Filter envelope
  - PD, PA, PENV.ATK, PENV.CRV - Pitch envelope
  - FD, FA, FMEV.ATK, FMEV.CRV - FM envelope
  - FBEV.ATK, FBEV.CRV - Feedback envelope
  - DA, DENV.ATK, DENV.CRV - Discontinuity envelope

---

### TIER 3 (TIER_QUERIES) - Query Results

**System Queries** (9 commands)
- M (no args) - Query metro interval
- M.SYNC (no args) - Query sync mode
- SCENES - List all scenes
- PSETS - List all presets
- MIDI.IN (no args) - List MIDI inputs
- MIDI.DIAG (no args) - MIDI diagnostics help
- MIDI.DIAG REPORT - MIDI timing report
- SC.DIAG (no args) - SC diagnostics help
- SC.DIAG REPORT - SC timing report
- SC.DIAG TRIGGERS - Show trigger count
- AUDIO.OUT (no args) - Query audio devices
- THEME (no args) - Show current theme

**Pattern Queries** (26 commands)

*Explicit Pattern (PN.*)* - 13 commands:
- PN.L (no args), PN.I (no args), PN (2 args) - State queries
- PN.MIN, PN.MAX, PN.SUM, PN.AVG, PN.FND - Math queries
- PN.HERE, PN.NEXT, PN.PREV, PN.POP - Navigation/value queries

*Working Pattern (P.*)* - 13 commands:
- P.N (no args), P.L (no args), P.I (no args), P (1 arg) - State queries
- P.MIN, P.MAX, P.SUM, P.AVG, P.FND - Math queries
- P.HERE, P.NEXT, P.PREV, P.POP - Navigation/value queries

**Core Math Operations** (6 commands)
- ADD, SUB, MUL, DIV, MOD, MAP

**Core Counter Queries** (4 commands)
- N1, N2, N3, N4 (when querying without args)

**Core Random Operations** (5 commands)
- RND, RRND, TOSS, EITH, TOG

**Core Variables** (9 commands when querying)
- A, B, C, D, X, Y, Z, T, I, J, K (when called without args)

**Synth Queries** (1 command)
- VCA (no args) - Query VCA mode

---

### TIER 4 (TIER_CONFIRMS) - Confirmations

**System Commands** (10 commands)
- TR, PLTR - Trigger confirmations
- VOL - Volume setting
- REC.PATH - Recording path
- NOTE, NOTE.CLR - Note management
- MIDI.DIAG 1/0 - Enable/disable MIDI diagnostics
- SC.DIAG 1/0, SC.DIAG RST - SC diagnostics control

**Pattern Manipulation** (36 commands)

*Explicit Pattern (PN.*)* - 18 commands:
- PN.L (with value), PN.I (with value), PN (3 args) - State setters
- PN.PUSH, PN.INS, PN.RM - Element manipulation
- PN.REV, PN.ROT, PN.SHUF, PN.SORT, PN.RND - Order manipulation
- PN.ADD, PN.SUB, PN.MUL, PN.DIV, PN.MOD, PN.SCALE - Math operations

*Working Pattern (P.*)* - 18 commands:
- P.N (with value), P.L (with value), P.I (with value), P (2 args) - State setters
- P.PUSH, P.INS, P.RM - Element manipulation
- P.REV, P.ROT, P.SHUF, P.SORT, P.RND - Order manipulation
- P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE - Math operations

**Core Commands** (16 commands)
- N1.RST, N2.RST, N3.RST, N4.RST - Counter resets
- N1.MAX, N2.MAX, N3.MAX, N4.MAX - Counter max setters
- N1.MIN, N2.MIN, N3.MIN, N4.MIN - Counter min setters
- DEL, DEL.X, DEL.R - Scheduling commands
- SYNC, SYNC.SEQ, SYNC.TOG, SYNC.PAT - Sync commands
- A, B, C, D, X, Y, Z, T, I, J, K (when setting with args)

**Synth Parameter Commands** (89 commands)

*Oscillator* (7):
- PF, PW, MF, MW, FB, FBA, FBD

*Filter* (10):
- FC, FQ, FT, FE, FED, FK, MFF, MFQ, MC, MQ

*Resonator* (4):
- RF, RD, RM, RK

*Discontinuity* (2):
- DM, DD

*Noise* (2):
- NW, NV

*Modulation* (12):
- TK, MB, MBA, MBD, MP, MD, MT, MA, FM, MX, MM, ME

*Source Levels* (2):
- PV, MV

*Effects - Compressor* (5):
- CT, CR, CA, CL, CM

*Effects - Delay* (7):
- D.MODE, D.TAIL, DT, DF, DLP, DW, DS

*Effects - EQ* (5):
- EL, EM, EF, EQ, EH

*Effects - Lo-Fi* (3):
- LB, LS, LM

*Effects - Reverb* (6):
- R.MODE, R.TAIL, RV, RP, RH, RW

*Effects - Ring Mod* (3):
- RGF, RGW, RGM

*Effects - Pitch Shift* (5):
- PS.MODE, PS.SEMI, PS.GRAIN, PS.MIX, PS.TARG

*Effects - Beat Repeat* (4):
- BR.LEN, BR.REV, BR.WIN, BR.MIX

*Plaits* (8):
- PL.ENG, PL.FREQ, PL.HARM, PL.TIMB, PL.MORPH, PL.DEC, PL.LPG, PLV, PAV

---

## Special Cases & Notes

### Dual-Mode Commands
Several commands act as queries or setters depending on arguments:
- **PN.L, PN.I, PN** (explicit patterns) - Tier 3 query / Tier 4 setter
- **P.N, P.L, P.I, P** (working patterns) - Tier 3 query / Tier 4 setter
- **Counter commands (N1-N4)** - Tier 3 query / Tier 4 setter
- **Variable commands (A-Z, T, I, J, K)** - Tier 3 query / Tier 4 setter
- **VCA** - Tier 3 query / Tier 2 setter
- **System queries (M, M.SYNC, etc)** - Tier 3 query / Tier 2 setter

### Commands with Hardcoded Tier Checks
These commands use `debug_level >= 2` instead of tier constants (pre-refactor code):
- Q.ROOT, Q.SCALE, Q.BIT - Scale commands
- All gate commands (GATE, AENV.GATE, etc)
- All envelope commands (AD, PD, FD, DA, PA, FA, etc)
- VOL, PAN, DC

### Potential Misconfiguration
- **SLEW, SLEW.ALL** - Currently use `debug_level >= 1` (TIER_ERRORS) but show confirmation messages. Should likely be TIER_ESSENTIAL (2) or TIER_CONFIRMS (4).

### Always Shown
- **HELP** - Displayed regardless of debug level
- **VERSION** - Displayed regardless of debug level
- **COMPAT** - Terminal compatibility info (always shown)

---

## Command Count Summary

| Tier | Count | Percentage |
|------|-------|------------|
| Tier 0 (Silent) | 59 | 17% |
| Tier 1 (Errors) | All (on error) | 100% |
| Tier 2 (Essential) | 42 | 12% |
| Tier 3 (Queries) | 60 | 17% |
| Tier 4 (Confirms) | 151 | 43% |

**Total Commands**: ~350 (including aliases and dual-mode variants)

---

## Implementation

**Tier constants** are defined in `/Users/why/repos/monokit/src/types.rs`:
```rust
pub const TIER_SILENT: u8 = 0;
pub const TIER_ERRORS: u8 = 1;
pub const TIER_ESSENTIAL: u8 = 2;
pub const TIER_QUERIES: u8 = 3;
pub const TIER_CONFIRMS: u8 = 4;
```

**Output control** is centralized in `ExecutionContext` at `/Users/why/repos/monokit/src/commands/context.rs` via:
- `should_output(category: OutputCategory)` method
- Category-specific flags: `out_err`, `out_ess`, `out_qry`, `out_cfm`

---

## Usage

Set debug level with:
```
DEBUG 0  # Silent - no output
DEBUG 1  # Errors only
DEBUG 2  # Errors + essential info
DEBUG 3  # Errors + essential + queries
DEBUG 4  # All output including confirmations
DEBUG 5  # Verbose (reserved for future diagnostics)
```

**Important:** Setting DEBUG automatically synchronizes the OUT.* flags:
- DEBUG 0 → out_err=false, out_ess=false, out_qry=false, out_cfm=false
- DEBUG 1 → out_err=true, out_ess=false, out_qry=false, out_cfm=false
- DEBUG 2 → out_err=true, out_ess=true, out_qry=false, out_cfm=false
- DEBUG 3 → out_err=true, out_ess=true, out_qry=true, out_cfm=false
- DEBUG 4+ → out_err=true, out_ess=true, out_qry=true, out_cfm=true

Override specific categories after setting DEBUG:
```
OUT.ERR 1  # Show errors (even if DEBUG 0)
OUT.ESS 1  # Show essential info
OUT.QRY 1  # Show query results
OUT.CFM 1  # Show confirmations
```

Example: For errors-only mode with confirmations:
```
DEBUG 1    # Start with errors only
OUT.CFM 1  # Also show confirmations
```
