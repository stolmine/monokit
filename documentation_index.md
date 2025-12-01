# Monokit Documentation Index

## Recent Updates (November 2025)

### MIDI Clock Sync

**MIDI Clock Input:**
- External tempo sync from DAW or MIDI clock devices
- Transport start/stop follows MIDI messages
- 16th note resolution (each MIDI clock tick = metro tick)
- Auto-detection of available MIDI inputs
- `M.SYNC <0|1>` - Set sync mode (0=internal, 1=MIDI clock)
- `MIDI.IN` - List available MIDI input devices
- `MIDI.IN <name>` - Connect to specific MIDI device

When M.SYNC is enabled (1), the metro follows external MIDI clock. Transport start/stop messages control metro activation.

### REPL Scrolling and UI Enhancements

**REPL Output Scrolling (Live Page):**
- Ctrl+Up/Down scrolls through REPL output history
- Shows scroll indicator [↑N] in title when scrolled up from bottom
- Auto-resets to bottom when new output arrives
- Smooth navigation through command history and output

**Help System Updates:**
- Added SEQ inline sequence documentation
- Added PSET preset system documentation
- Added REPL scrolling keybinding documentation
- Comprehensive coverage of all system features

**Roadmap Updates:**
- "Live Sequence State Highlighting" feature added to Phase 5 (UI enhancements)

### DRY Refactoring Complete (All Phases)

**Total lines removed: ~5,942 lines**

| Phase | Target | Reduction | Status |
|-------|--------|-----------|--------|
| Phase 0 | Reorganization | - | ✓ Complete |
| Phase 1 | Envelopes | 918 lines (81%) | ✓ Complete |
| Phase 2 | Patterns | 1,573 lines (78%) | ✓ Complete |
| Phase 3 | Synth Params | 2,325 lines (80%) | ✓ Complete |
| Phase 4A | Variable/Counter Macros | 489 lines (66%) | ✓ Complete |
| Phase 4B | Expression Helpers | +39 lines (infra) | ✓ Complete |
| Phase 4C | Test Fixtures | 637 lines | ✓ Complete |

**Macro Systems Created:**
- `synth/envelopes/common.rs` - Envelope parameter macros
- `patterns/common.rs` - Pattern operation macros
- `synth/common.rs` - Synth parameter macros (6 types)
- `core/variables.rs` - Variable handler macros
- `core/counters.rs` - Counter handler macros
- `commands/common.rs` - Expression parsing helpers
- `tests/common.rs` - Test setup macro

All 411 tests pass. Codebase reduced by ~28% while maintaining full functionality.

---

### Phase 1 DRY Refactoring Complete
**Changes:** Envelope handler consolidation using `define_int_param!` and `define_float_param!` macros.

**Results:**
- Created `synth/envelopes/common.rs` with shared macro definitions
- Refactored all 6 envelope files to use macros (amp, pitch, fm, disc, feedback, filter)
- Removed dead code: `handle_*_mode` handlers, `global.rs`
- Line reduction: ~1,141 lines → 223 lines (~918 line reduction, 81% decrease)
- All 411 tests pass

**Technical Details:**
- Each envelope file reduced from ~140 lines to ~10-12 lines
- Shared parameter generation via macros eliminates boilerplate
- Maintains exact same functionality and API
- Foundation for Phase 2 (Pattern Operation DRY) and Phase 3 (Synth Parameter DRY)

---

### Envelope System Simplification
**Changes:** Envelope system refactored to use simple `Env.perc` with controllable attack and curve parameters.

**What Changed:**
- Removed gate-based envelope triggering (no gate parameter, no mode switching)
- Simplified to single percussive envelope type per parameter
- Each envelope has: decay time, attack time, curve, and amount
- Fixed pitch envelope parameter routing with `Lag.kr` control capture

**Removed Commands:**
- `ENV.ATK`, `ENV.DEC`, `ENV.CRV`, `ENV.MODE` - Global envelope controls
- `GATE` - Global gate duration
- `*.MODE`, `*.GATE` - Per-envelope mode and gate overrides (AENV.MODE, PENV.GATE, etc.)

**Added Commands:**
- `FBEV.AMT` - Alias for FBA (feedback envelope amount)

**Current Envelope Commands:**

| Envelope | Decay | Amount | Attack | Curve |
|----------|-------|--------|--------|-------|
| Amp | AD | - | AENV.ATK | AENV.CRV |
| Pitch | PD | PA | PENV.ATK | PENV.CRV |
| FM | FD | FA | FMEV.ATK | FMEV.CRV |
| Disc | DD | DA | DENV.ATK | DENV.CRV |
| Feedback | FBD | FBA/FBEV.AMT | FBEV.ATK | FBEV.CRV |
| Filter | FED | FE | FLEV.ATK | FLEV.CRV |

**Technical Details:**
- Exponential pitch envelope scaling: `pow(2, pitchEnv * pa)` for proper octave behavior
- PA parameter represents octaves (PA=4 = 4 octaves of pitch sweep)
- Control capture using `Lag.kr(param, 0)` prevents UGen graph optimization issues
- All envelopes trigger on each `TR` command

**Files Changed:**
- `sc/monokit_server.scd` - Envelope implementation and control captures
- `src/commands/synth/envelopes/*.rs` - Envelope command handlers

---

## Documentation

- **CONCEPT.md** - Project overview, architecture, MVP implementation
- **PLAN.md** - UI refactor plan: Teletype-style interface with page system, script storage, patterns, and control flow
- **ROADMAP.md** - Single source of truth for all future development plans and roadmap
- **DSP_TIER1_IMPLEMENTATION_PLAN.md** - Detailed implementation plan for Filter, Resonator, Delay, and Reverb DSP blocks
- **DSP_TIER3_BUFFER_EFFECTS_PLAN.md** - Implementation plan for Beat Repeat and Pitch Shift buffer effects
- **EFFECT_ROUTING_DESIGN.md** - Design document for flexible effect routing system
- **DRY_REFACTOR_PLAN.md** - Comprehensive refactoring plan for codebase reorganization and DRY consolidation
- **DRY_ANALYSIS_REPORT.md** - Detailed analysis of code duplication and refactoring opportunities (~4,000-5,000 line reduction potential)
- **documentation_index.md** - This file, listing all documentation and key project files

## Key Project Files

### Configuration

- **Cargo.toml** - Rust project manifest with dependencies (rosc, ratatui, crossterm, nom, anyhow, thiserror, serde)
- **Cargo.lock** - Dependency lock file

### Source Code

Modular Rust implementation (~17,300 total lines across 93 files, after Phase 1 DRY refactoring):

- **src/main.rs** (69 lines) - Application entry point, initializes TUI and starts main loop
- **src/metro.rs** (112 lines) - Metro thread implementation with absolute timing
- **src/types.rs** (233 lines) - Core data structures, enums, constants, and type definitions
- **src/eval/** - Expression evaluation module (~720 lines across 4 files)
  - **mod.rs** - Core dispatch, resolve_value, quantize_note, eval_condition
  - **patterns.rs** - Pattern expression handling (PN.*, P.*)
  - **math.rs** - Math operators (ADD, SUB, MUL, DIV, MOD, Q, N)
  - **logic.rs** - Comparisons, RND, TOG, MAP, counters
- **src/ui/** - TUI rendering module (~1,370 lines across 10 files)
  - **mod.rs** - Module coordinator
  - **header.rs** - Header rendering
  - **footer.rs** - Footer rendering
  - **pages/** - Page implementations (7 files: live, script, metro, init, pattern, help, mod)
- **src/scene.rs** (169 lines) - Scene persistence, file I/O
- **src/theme.rs** (67 lines) - Theme struct and built-in themes (dark, light, system)
- **src/config.rs** (170 lines) - Configuration loading, named theme support
- **src/app/** - Application module (~750 lines across 6 files)
  - **mod.rs** - App struct, constructor, navigation
  - **input.rs** - Input handling methods
  - **script_exec/** - Script/command execution module (4 files)
    - **mod.rs** - Core execution, metro updates
    - **control_flow.rs** - If/elif/else, conditionals
    - **loops.rs** - Loop processing
    - **interactive.rs** - User command execution
- **src/commands/** - Command processing module (48 files, ~8,167 lines, -918 from Phase 1)
  - **mod.rs** - Main dispatcher with command routing
  - **validate.rs** - Command validation
  - **aliases.rs** - Alias resolution for PREFIX.SUFFIX → short form mapping (93 aliases)
  - **core/** - Core language primitives (7 files)
    - **mod.rs** - Module coordinator
    - **variables.rs** - Variable handlers (A-D, X-Z, T, I, J, K)
    - **counters.rs** - Auto-increment counters (N1-N4 with MIN/MAX/RST)
    - **math_ops.rs** - Math operations (ADD, SUB, MUL, DIV, MOD, MAP)
    - **random_ops.rs** - Random operations (RND, RRND, TOSS, EITH, TOG)
    - **scale.rs** - Quantization (Q, Q.ROOT, Q.SCALE, Q.BIT)
    - **scheduling.rs** - Delayed execution (DEL, DEL.CLR, DEL.X, DEL.R)
  - **patterns/** - Pattern operations module (9 files)
    - **mod.rs** - Module coordinator
    - **working.rs** - Working pattern state (P.N, P.L, P.I)
    - **working_query.rs** - Working pattern queries (P.HERE, P.NEXT, P.PREV)
    - **working_manip.rs** - Working pattern manipulation (P.PUSH, P.POP, P.INS, P.RM, P.REV, P.ROT, P.SHUF, P.SORT, P.RND)
    - **working_math.rs** - Working pattern math (P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE, P.MIN, P.MAX, P.SUM, P.AVG, P.FND)
    - **explicit.rs** - Explicit pattern state (PN.N, PN.L, PN.I)
    - **explicit_query.rs** - Explicit pattern queries (PN.HERE, PN.NEXT, PN.PREV)
    - **explicit_manip.rs** - Explicit pattern manipulation (PN.PUSH, PN.POP, PN.INS, PN.RM, PN.REV, PN.ROT, PN.SHUF, PN.SORT, PN.RND)
    - **explicit_math.rs** - Explicit pattern math (PN.ADD, PN.SUB, PN.MUL, PN.DIV, PN.MOD, PN.SCALE, PN.MIN, PN.MAX, PN.SUM, PN.AVG, PN.FND)
  - **system/** - System and session commands (4 files)
    - **mod.rs** - Module coordinator
    - **metro.rs** - Metro commands (M, M.BPM, M.ACT, M.SCRIPT)
    - **scene.rs** - Scene commands (SAVE, LOAD, SCENES, DELETE)
    - **misc.rs** - Other system commands (TR, RST, SCRIPT, THEME, DEBUG, PRINT, HELP, CLEAR, REC)
  - **synth/** - Synth parameter handlers (18 files, reorganized from synth_params/)
    - **mod.rs** - Module coordinator
    - **oscillator.rs** - Oscillator parameters (PF, PW, MF, MW, FB)
    - **modulation.rs** - Modulation and tracking (TK, MB, MP/MD/MT/MA, FM, MX, MM, ME)
    - **discontinuity.rs** - Discontinuity/waveshaping (DC, DM)
    - **slew.rs** - Parameter slewing (SLEW.ALL global, SLEW per-parameter)
    - **output.rs** - Output parameters (VOL, PAN)
    - **envelopes/** - Envelope module (8 files, 223 lines total after Phase 1 DRY)
      - **mod.rs** - Module coordinator
      - **common.rs** - Shared macro definitions (define_int_param!, define_float_param!)
      - **amp.rs** - Amplitude envelope (AD, AENV.ATK, AENV.CRV) - 11 lines
      - **pitch.rs** - Pitch envelope (PD, PA, PENV.ATK, PENV.CRV) - 12 lines
      - **fm.rs** - FM envelope (FD, FA, FMEV.ATK, FMEV.CRV) - 12 lines
      - **disc.rs** - Discontinuity envelope (DD, DA, DENV.ATK, DENV.CRV) - 11 lines
      - **feedback.rs** - Feedback envelope (FBD, FBA/FBEV.AMT, FBEV.ATK, FBEV.CRV) - 10 lines
      - **filter.rs** - Filter envelope (FED, FE, FLEV.ATK, FLEV.CRV) - 10 lines
    - **filter.rs** - SVF filter parameters (FC, FQ, FT, FK, MF.F)
    - **resonator.rs** - Comb resonator (RF, RD, RM, RK)
    - **effects/** - Time-based and processing effects (9 files)
      - **mod.rs** - Module coordinator
      - **delay.rs** - Stereo delay (DT, DF, DLP, DW, DS, D.MODE, D.TAIL)
      - **reverb.rs** - Plate reverb (RV, RP, RH, RW, R.MODE, R.TAIL)
      - **eq.rs** - 3-band EQ (EL, EM, EF, EQ, EH)
      - **lofi.rs** - Lo-Fi processor (LB, LS, LM)
      - **ring_mod.rs** - Ring modulator (RGF, RGW, RGM)
      - **compressor.rs** - Compressor (CT, CR, CA, CL, CM)
      - **beat_repeat.rs** - Beat repeat (BR.ACT, BR.LEN, BR.REV, BR.WIN, BR.MIX)
      - **pitch_shift.rs** - Pitch shift (PS.MODE, PS.SEMI, PS.GRAIN, PS.MIX, PS.TARG)
  - **randomization.rs** - Randomization commands (RND.VOICE, RND.OSC, RND.FM, RND.MOD, RND.ENV, etc.)
- **src/tests/** - Test suite module (21 files, ~5,288 lines, 411 tests)
  - Organized by category: envelope, counter, slew, tog, rnd, toss_eith, expr, condition, pattern, pattern_ops, variable, validation, math, map, comparison, scene, debug, buffer_effects

Key features:
- Page-based interface: Live, Script 1-8, Metro (M), Init (I), Pattern (P), Help (paginated with 10 category pages)
- Script storage: 10 scripts × 8 lines (Scripts 1-8, M, I)
- Pattern storage: 6 patterns × 64 steps (i16 values, patterns 0-5)
- Variables: A-D, X-Y-Z-T (global), J-K (per-script local), I (loop counter)
- Control flow: IF/ELIF/ELSE conditions, PROB probabilistic, EV/SKIP every-N-tick
- Comparison operators: EZ, NZ, EQ, NE, GT, LT, GTE, LTE (return 1/0)
- N operator: Semitone to frequency conversion (N 0 = C3 = 131 Hz)
- Expression evaluation in all numeric arguments
- Metro thread sends script execution requests to main thread
- OSC client sending to SuperCollider (127.0.0.1:57120)

- **sc/monokit_server.scd** (626 lines) - SuperCollider sound engine
  - `\monokit` SynthDef: HD2-style dual oscillator with FM, discontinuity, comprehensive DSP effects, and multi-stage processing
  - Additive envelope model: output = base + env * amount
  - Signal chain: Oscillators → FM → Mix → Discontinuity → Lo-Fi → SVF Filter → Ring Mod → Comb Resonator → Amp → Compressor → Pan → Beat Repeat → Pitch Shift → Stereo Delay → 3-Band EQ → Plate Reverb → Out
  - 77 synth parameters (25 oscillator/envelope + 48 DSP + 4 routing)
  - Includes implemented Beat Repeat and Pitch Shift buffer effects
  - Global parameter slew with Lag.kr smoothing for artifact-free transitions
  - OSC responders:
    - `/monokit/trigger` - Gate trigger (no args)
    - `/monokit/param` - Generic parameter setter (string name, float/int value)
    - `/monokit/slew` - Set global slew time (ms)
    - `/monokit/rec` - Start recording
    - `/monokit/rec/stop` - Stop recording
    - `/monokit/rec/path` - Set recording path prefix
  - Stateless sound engine (no metro logic)

### Build Artifacts

- **target/** - Rust build output (ignored by git)

---

## DRY Refactoring Plan

### Overview
The codebase contains ~6,500-7,000 lines of systematic duplication (30-33% of total code). A comprehensive refactoring plan targets **~4,000-5,000 line reduction** through reorganization and macro consolidation.

### Four-Phase Approach

**Phase 0: Codebase Reorganization [COMPLETE]**
- Reorganized command structure by logical domain (core, patterns, system, synth)
- Consolidated scattered envelope parameters into proper locations
- Moved envelope decay/amount params to respective envelope files:
  - DD from `discontinuity.rs` → `envelopes/disc.rs`
  - FBA, FBD from `oscillator.rs` → `envelopes/feedback.rs`
  - FE, FED from `filter.rs` → `envelopes/filter.rs`
- Created `synth/output.rs` to consolidate VOL and PAN
- Deleted dead code (MODE handlers, deprecated GATE commands, unused global.rs)
- Renamed `delay.rs` → `core/scheduling.rs` (handles DEL commands, not synth delay)
- Split `effects.rs` into modular effect files (lofi.rs, ring_mod.rs, compressor.rs, etc.)
- Renamed `synth_params/` → `synth/` for clarity
- All 411 tests pass

**Phase 1: Envelope Handler DRY [COMPLETE - 918 line reduction]**
- Created `synth/envelopes/common.rs` with `define_int_param!` and `define_float_param!` macros
- Generated DEC, AMT, ATK, CRV handlers for all 6 envelopes from macro definitions
- Each envelope file reduced from ~140 lines → ~10-12 lines
- Total reduction: ~1,141 lines → 223 lines (81% decrease)
- All 411 tests pass

**Phase 2: Pattern Operation DRY [COMPLETE - 1,573 line reduction in wrappers]**
- Created `patterns/common.rs` (902 lines) with `PatternRef` enum, shared implementations, and macro system
- Aggressive macro approach: 10 macros generate both P.* and PN.* handlers from single definitions
- Unified P.* (working) and PN.* (explicit) operations via PatternRef::Working/Explicit
- Wrapper code reduced from 2023 → 450 lines (78% reduction)
- Explicit files now just re-export from working files (~10 lines each)
- All 411 tests pass

**Phase 3: Synth Parameter DRY [PLANNED - ~2,000 line reduction]**
- Create `synth/param_macro.rs` with `define_param!` macro
- Generate parameter handlers from declarative specifications
- Consolidate 70+ nearly-identical parameter handlers into config-driven system

### Results So Far
- Clear, logical file organization by domain (Phase 0)
- 918 line reduction from envelope consolidation (Phase 1)
- 1,573 line reduction in pattern wrappers via macro system (Phase 2)
- Total reduction: ~2,491 lines
- Easier to add new commands (single macro invocation for both P.* and PN.*)
- All 411 tests continue to pass
- Maintains backward compatibility

### Expected Final Results
- ~4,500+ line reduction with Phase 3 (synth parameter macros)
- Further simplified maintenance through macro-driven parameter handlers

See **DRY_REFACTOR_PLAN.md** for complete implementation details and **DRY_ANALYSIS_REPORT.md** for duplication analysis.

---

## Architecture Overview

```
Rust TUI (src/main.rs)
    |
    +-- Main Thread
    |    - TUI rendering (ratatui)
    |    - Command processing
    |    - Script execution (with App context)
    |    - Pattern/variable access
    |    - Recording management (WAV int24)
    |    |
    |    v OSC messages
    |    127.0.0.1:57120
    |
    +-- Metro Thread (absolute timing)
         - Sends ExecuteScript events to main thread
         - No direct script execution (thread safety)

SuperCollider Sound Engine (sc/monokit_server.scd)
    |
    v
Audio output → Recording (optional)
```

## Command Reference

### Naming Convention

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
- `ROUT` - Routing Matrix (ROUT.MP → MP, ROUT.MD → MD, ROUT.MF → MF.F)
- `OUT` - Output (OUT.VOL → VOL, OUT.PAN → PAN)
- `AENV` - Amplitude Envelope (AENV.ATK, AENV.CRV, decay → AD)
- `PENV` - Pitch Envelope (PENV.ATK, PENV.CRV, decay → PD, amount → PA)
- `FMEV` - FM Envelope (FMEV.ATK, FMEV.CRV, decay → FD, amount → FA)
- `DENV` - Discontinuity Envelope (DENV.ATK, DENV.CRV, decay → DD, amount → DA)
- `FBEV` - Feedback Envelope (FBEV.ATK, FBEV.CRV, FBEV.AMT → FBA, decay → FBD)
- `FLEV` - Filter Envelope (FLEV.ATK, FLEV.CRV, decay → FED, amount → FE)

**Alias System:**
- Current short forms (PF, FC, AD, etc.) remain as **aliases** to canonical forms
- You can use either form: `PF 440` or `POSC.FREQ 440` (both work identically)
- Legacy commands in existing scenes continue to work
- Use whichever form you prefer (terse aliases or explicit canonical names)

### Current Commands (v0.1.0)

#### Trigger & Volume
- `TR` - Trigger voice (sends gate pulse)
- `VOL <0.0-1.0>` - Set master volume

#### Metro/Timing
- `M` - Show current metro interval (milliseconds)
- `M <ms>` - Set metro interval in milliseconds
- `M.BPM <bpm>` - Set metro tempo as BPM
- `M.ACT <0|1>` - Activate (1) or deactivate (0) metro
- `M.SCRIPT <1-8>` - Set which script metro calls on each tick (default: M script)
- `M.SYNC <0|1>` - Set sync mode (0=internal, 1=MIDI clock)
- `MIDI.IN` - List available MIDI input devices
- `MIDI.IN <name>` - Connect to MIDI device for clock sync

#### Delayed Execution
- `DEL <ms>: <cmd>` - Execute command after delay (max 16000ms)
- `DEL.CLR` - Clear all pending delayed commands
- `DEL.X <count> <ms>: <cmd>` - Queue command N times at intervals
  - Example: `DEL.X 4 100: TR` fires at 100ms, 200ms, 300ms, 400ms
- `DEL.R <count> <ms>: <cmd>` - Execute immediately, then repeat
  - Example: `DEL.R 4 100: TR` fires now, then at 100ms, 200ms, 300ms
- Supports expressions in delay time and counts
- Commands execute on metro thread at scheduled time

#### Scripts
- `SCRIPT <1-8>` - Execute stored script (can be called from other scripts, supports expressions)
  - `SCRIPT 1` - Direct script number
  - `SCRIPT A` - Variable reference
  - `SCRIPT ADD 1 1` - Math expression
  - `SCRIPT PN.NEXT 0` - Pattern operation
- Scripts 1-8: User scripts
- M script (index 8): Called on each metro tick
- I script (index 9): Called on startup

#### Scenes
- `SAVE <name>` - Save current scene (scripts + patterns) to ~/.monokit/scenes/
- `LOAD <name>` - Load scene from file, resets variables
- `SCENES` - List available saved scenes
- `DELETE <name>` - Delete a saved scene

#### Variables
- `A`, `B`, `C`, `D` - General accumulators (get/set: `A` or `A 100`)
- `X`, `Y`, `Z`, `T` - General accumulators
- `J`, `K` - Per-script local variables (each script has its own J and K)
- `I` - Loop counter (scoped to L loops, read-only)
- Variables accept expressions for their value: `A ADD 1 1`, `J RND 100`, `X PN.NEXT 0`, `B MUL A 2`
- Variables can be used in expressions: `PF A`, `DC X`

#### Counters (N1-N4)
- `N1`, `N2`, `N3`, `N4` - Read current value and auto-increment (works in expressions)
- `N1.MIN <n>` - Set minimum value (default 0, accepts expressions)
- `N1.MAX <n>` - Set maximum value (wraps to MIN when exceeded, 0=disabled/no wrap, accepts expressions)
- `N1.RST` - Reset counter to MIN value
- Same operations available for N2, N3, N4
- Example usage:
  ```
  N1.MIN 10; N1.MAX 14    # Counter cycles 10,11,12,13,14,10...
  PF N N1                 # Use counter for pitch (semitones)
  N1.RST                  # Reset to 10
  ```

#### Patterns (Working Pattern - P)
**State & Query:**
- `P.N` / `P.N <0-5>` - Get/set working pattern
- `P.L` / `P.L <1-64>` - Get/set pattern length
- `P.I` / `P.I <0-63>` - Get/set playhead index
- `P <idx>` / `P <idx> <val>` - Get/set value at index
- `P.HERE` - Get value at playhead
- `P.NEXT` - Advance playhead, return value
- `P.PREV` - Reverse playhead, return value

**Manipulation:**
- `P.PUSH <val>` - Push value to end, shift all values left
- `P.POP` - Return value at end
- `P.INS <idx> <val>` - Insert value at index, shift right
- `P.RM <idx>` - Remove value at index, shift left
- `P.REV` - Reverse pattern order
- `P.ROT <n>` - Rotate pattern by n positions
- `P.SHUF` - Shuffle pattern randomly
- `P.SORT` - Sort pattern ascending

**Math Operations:**
- `P.ADD <val>` - Add value to all steps
- `P.SUB <val>` - Subtract value from all steps
- `P.MUL <val>` - Multiply all steps by value
- `P.DIV <val>` - Divide all steps by value
- `P.MOD <val>` - Modulo all steps by value
- `P.SCALE <min> <max>` - Scale pattern to new range
- `P.RND [min] [max]` - Randomize all steps (default: 0-127)

**Query Operations:**
- `P.MIN` - Return minimum value in pattern
- `P.MAX` - Return maximum value in pattern
- `P.SUM` - Return sum of all values
- `P.AVG` - Return average of all values
- `P.FND <val>` - Find value, return index (-1 if not found)

#### Patterns (Explicit Pattern - PN)
**State & Query:**
- `PN <pat> <idx>` / `PN <pat> <idx> <val>` - Get/set value (pat: 0-5)
- `PN.L <pat>` / `PN.L <pat> <len>` - Get/set length
- `PN.I <pat>` / `PN.I <pat> <idx>` - Get/set playhead
- `PN.HERE <pat>` - Get value at playhead
- `PN.NEXT <pat>` - Advance playhead, return value
- `PN.PREV <pat>` - Reverse playhead, return value

**Manipulation:**
- `PN.PUSH <pat> <val>` - Push value to end, shift all values left
- `PN.POP <pat>` - Return value at end
- `PN.INS <pat> <idx> <val>` - Insert value at index, shift right
- `PN.RM <pat> <idx>` - Remove value at index, shift left
- `PN.REV <pat>` - Reverse pattern order
- `PN.ROT <pat> <n>` - Rotate pattern by n positions
- `PN.SHUF <pat>` - Shuffle pattern randomly
- `PN.SORT <pat>` - Sort pattern ascending

**Math Operations:**
- `PN.ADD <pat> <val>` - Add value to all steps
- `PN.SUB <pat> <val>` - Subtract value from all steps
- `PN.MUL <pat> <val>` - Multiply all steps by value
- `PN.DIV <pat> <val>` - Divide all steps by value
- `PN.MOD <pat> <val>` - Modulo all steps by value
- `PN.SCALE <pat> <min> <max>` - Scale pattern to new range
- `PN.RND <pat> [min] [max]` - Randomize all steps (default: 0-127)

**Query Operations:**
- `PN.MIN <pat>` - Return minimum value in pattern
- `PN.MAX <pat>` - Return maximum value in pattern
- `PN.SUM <pat>` - Return sum of all values
- `PN.AVG <pat>` - Return average of all values
- `PN.FND <pat> <val>` - Find value, return index (-1 if not found)

Note: All PN and P operations accept variables/expressions as arguments (e.g., `DC PN.NEXT 0`, `P I`, `PN A B`)

#### Expression Support
All numeric arguments accept nested expressions, including:
- Math operations: `PF ADD A 100`, `DC MUL X 2`
- Pattern operations: `PF PN.NEXT 0`, `DC SUB PN.HERE 0 PN.HERE 1`
- Random operations: `PF RND 1000`, `A RRND 0 127`
- Sequence operations: `PF N SEQ "C3 E3 G3"`, `IF SEQ "x _": TR`
- Variables: `PF A`, `DC X`, `MF J`
- Nested combinations: `PF ADD PN.NEXT 0 RND 100`

#### Inline Sequences (SEQ) - Phase 2 Complete
- `SEQ "<pattern>"` - Cycle through values on each evaluation
  - **Phase 1 Tokens:**
    - `x` = trigger (returns 1)
    - `_` or `.` = rest (returns 0)
    - Numbers: `100`, `-12`, `0`
    - Note names: `C3`, `E3`, `F#4`, `Bb2` (returns semitones relative to C3)
  - **Phase 2 Features:**
    - `*n` = repeat token n times (e.g., `C3*4` expands to `C3 C3 C3 C3`)
    - `?` = random trigger (50% chance of 1, 50% chance of 0)
    - `<a b>` = toggle/cycle (deterministic, like TOG operator - returns A, then B, then A, then B...)
    - `{a b}` = random choice (unpredictable, like EITH - randomly picks A or B each time)
    - Combinable modifiers (e.g., `<C3 E3>*2` toggles twice, `{C3 E3}*2` picks randomly twice)

**Distinction Between Toggle and Random:**
- `<a b>` = **Toggle/Cycle** - deterministic state machine (like TOG)
  - `SEQ "<C3 E3>"` → C3, E3, C3, E3, C3, E3...
  - State persists across calls (remembers last value)

- `{a b}` = **Random Choice** - unpredictable selection (like EITH)
  - `SEQ "{C3 E3}"` → randomly C3 or E3, then randomly again
  - No state (picks fresh each time)

- State persists per-script and per-pattern (independent counters)
- Use with N operator for Hz: `PF N SEQ "C3 E3 G3"`
- Use with Q for quantization: `PF N Q SEQ "0 3 5 7"`

Examples:
```
IF SEQ "x _ x _": TR           # Trigger on beats 1 and 3
PF N SEQ "C3 E3 G3 C4"         # Arpeggiate C major
A SEQ "0 1 2 3"                # Store in variable
SEQ "C3*4 E3*2"                # Repeated notes (C3 C3 C3 C3 E3 E3)
SEQ "<C3 E3> G3"               # Toggle C3/E3, then G3
SEQ "{C3 E3} G3"               # Random C3 or E3, then G3
SEQ "x ? x ?"                  # Random triggers (50% chance each)
SEQ "<C3 E3>*2"                # Two toggles
SEQ "{C3 E3}*2"                # Two random choices
SEQ "<C3 E3> {G3 B3}"          # First toggles, second random
```

#### Preset System (PSET)
- `PSET <script> <name>` - Load preset into script 1-8
- `PSET.SAVE <script> <name>` - Save script as user preset
- `PSET.DEL <name>` - Delete user preset
- `PSETS` - List all presets ([F] factory, [U] user)

Factory Presets (22 total):
- **Drums (10):** 808-kick, punch-kick, sub-kick, basic-snare, snap-snare, hat-closed, hat-open, fm-hat, clap, rim
- **Bass (3):** sub-bass, saw-bass, fm-bass
- **Lead (3):** saw-lead, fm-lead, pluck-lead
- **Percussion (3):** metal-hit, conga, tom
- **FX (3):** noise, zap, rise

User presets stored in `~/.monokit/presets/`

Examples:
```
PSET 1 808-kick              # Load kick preset into script 1
PSET.SAVE 2 my-bass          # Save script 2 as user preset
PSET.DEL old-sound           # Delete user preset
PSETS                        # List all available presets
```

#### Control Flow (PRE separator)
- `IF <expr>: <cmd>` - Execute cmd if expr != 0 (truthy). Example: `IF PN.HERE 0: TR`
- `IF <cond>: <cmd>` - With comparison: `IF A > 5: TR`, `IF GT A 5: TR`
- `ELIF <cond>: <cmd>` - Else-if, executes if previous IF/ELIF was false and condition is true
- `ELSE: <cmd>` - Else branch, executes if all previous IF/ELIF were false
- `PROB <0-100>: <cmd>` - Execute cmd with probability
- `EV <n>: <cmd>` - Execute cmd every Nth tick (applies to whole line including semicolons)
- `SKIP <n>: <cmd>` - Skip every Nth tick (inverse of EV, executes on all other ticks)
- `L <start> <end>: <commands>` - Loop from start to end (inclusive), supports forward/backward iteration, I is loop counter
- Sub-commands: `cmd1; cmd2; cmd3` - Multiple commands on one line

#### Comparison Operators (return 1 for true, 0 for false)
- `EZ <x>` - Equals zero (x == 0)
- `NZ <x>` - Not zero (x != 0)
- `EQ <a> <b>` - Equals (a == b)
- `NE <a> <b>` - Not equals (a != b)
- `GT <a> <b>` - Greater than (a > b)
- `LT <a> <b>` - Less than (a < b)
- `GTE <a> <b>` - Greater than or equal (a >= b)
- `LTE <a> <b>` - Less than or equal (a <= b)
- Infix comparisons also supported in conditions: `>`, `<`, `>=`, `<=`, `==`, `!=`

Examples:
- `IF PN.HERE 0: TR` - Trigger if pattern value is non-zero
- `IF EZ A: TR` - Trigger if A equals zero
- `IF GT A 5: TR` - Trigger if A > 5
- `IF A > 5: TR` - Same as above (infix syntax)

#### Synth Parameters (77 total)

**Primary Oscillator**
- `PF <hz>` - Primary frequency (20-20000)
- `PW <0-2>` - Primary waveform (0=sine, 1=triangle, 2=sawtooth)

**Modulator Oscillator**
- `MF <hz>` - Modulator frequency (20-20000)
- `MW <0-3>` - Modulator waveform (0=sine, 1=triangle, 2=sawtooth, 3=feedback)

**Feedback FM (Noise Generation)**
- `FB <0-16383>` - Feedback amount (0=clean, >8191=chaotic/noise)
- `FBA <0-16383>` - Feedback envelope amount
- `FBD <ms>` - Feedback envelope decay time (1-10000 ms)

**Discontinuity (Waveshaping)**
- `DC <0-16383>` - Discontinuity amount (mix of modulator into shaper)
- `DM <0-6>` - Discontinuity mode (0=fold, 1=tanh, 2=softclip, 3=hard, 4=asym, 5=rectify, 6=crush)
- `DD <ms>` - Discontinuity envelope decay time (milliseconds, 1-10000)

**Lo-Fi Processor**
- `LB <1-16>` - Bit depth (1=crushed, 16=clean)
- `LS <100-48000>` - Sample rate reduction (Hz)
- `LM <0-16383>` - Lo-fi mix (dry/wet)

**Tracking & Modulation Bus**
- `TK <0-16383>` - Tracking amount (modulator frequency follows pitch envelope)
- `MB <0-16383>` - Modulation bus amount (general modulation depth)
- `MP <0|1>` - Enable modulation -> primary frequency (FM-independent)
- `MD <0|1>` - Enable modulation -> discontinuity amount
- `MT <0|1>` - Enable modulation -> tracking
- `MA <0|1>` - Enable modulation -> amplitude

**FM Synthesis**
- `FM <0-16383>` - FM index (modulator phase modulates primary frequency, additive with mod bus routing)

**Mix Controls (Additive Routing)**
- `MX <0-16383>` - Mix amount (modulator output to discontinuity input)
- `MM <0-16383>` - Mix modulation amount (depth of mod bus modulation on mix)
- `ME <0|1>` - Mix modulation enable (route mod bus to mix amount)

**Envelope Controls**
All envelopes use consistent *ENV prefix naming:
- `*.DEC` - Decay time (1-10000 ms) - short forms: AD, PD, FD, DD, FBD, FED
- `*.ATK` - Attack time (1-10000 ms)
- `*.CRV` - Curve shape (-8.0 to 8.0, -8=log, 0=linear, 8=exp)
- `*.AMT` - Envelope amount (where applicable) - short forms: PA, FA, DA, FBA, FE

Available envelope prefixes:
- `AENV` - Amplitude envelope (AENV.DEC→AD, AENV.ATK, AENV.CRV)
- `PENV` - Pitch envelope (PENV.DEC→PD, PENV.AMT→PA, PENV.ATK, PENV.CRV)
- `FMEV` - FM envelope (FMEV.DEC→FD, FMEV.AMT→FA, FMEV.ATK, FMEV.CRV)
- `DENV` - Discontinuity envelope (DENV.DEC→DD, DENV.AMT→DA, DENV.ATK, DENV.CRV)
- `FBEV` - Feedback envelope (FBEV.DEC→FBD, FBEV.AMT, FBEV.ATK, FBEV.CRV)
- `FLEV` - Filter envelope (FLEV.DEC→FED, FLEV.AMT→FE, FLEV.ATK, FLEV.CRV)

Examples:
- `AENV.ATK 50` - Set amp envelope attack to 50ms
- `PENV.CRV -4` - Set pitch envelope to logarithmic curve
- `AD 500` or `AENV.DEC 500` - Set amplitude decay to 500ms
- `PA 4` or `PENV.AMT 4` - Set pitch envelope to 4 octaves
- `FE 8000` or `FLEV.AMT 8000` - Set filter envelope amount

**Decay Time Short Forms**
- `AD <ms>` - Amplitude decay (AENV.DEC)
- `PD <ms>` - Pitch decay (PENV.DEC)
- `FD <ms>` - FM decay (FMEV.DEC)
- `DD <ms>` - Discontinuity decay (DENV.DEC)
- `FBD <ms>` - Feedback decay (FBEV.DEC)
- `FED <ms>` - Filter decay (FLEV.DEC)

**Envelope Amounts (Additive Model: output = base + env*amount)**
- `PA <0-16>` - Pitch envelope amount in octaves (PENV.AMT)
- `FA <0-16>` - FM envelope amount (FMEV.AMT)
- `DA <0-16>` - Discontinuity envelope amount (DENV.AMT)
- `FBA <0-16383>` - Feedback envelope amount (FBEV.AMT)
- `FE <0-16383>` - Filter envelope amount (FLEV.AMT)

**SVF Multi-Mode Filter**
- `FC <hz>` - Filter cutoff frequency (20-20000)
- `FQ <0-16383>` - Filter resonance (0-16383)
- `FT <0-3>` - Filter type (0=LP, 1=HP, 2=BP, 3=Notch)
- `FE <0-16383>` - Filter envelope amount
- `FED <ms>` - Filter envelope decay (1-10000 ms)
- `FK <0-16383>` - Filter key tracking amount
- `MF.F <0|1>` - ModBus -> Filter cutoff routing

**Ring Modulator**
- `RGF <20-2000>` - Ring mod frequency (Hz)
- `RGW <0-3>` - Ring mod waveform (0=sine, 1=triangle, 2=sawtooth, 3=square)
- `RGM <0-16383>` - Ring mod mix (0=dry, 16383=100% modulated)

**Comb Resonator (Karplus-Strong)**
- `RF <hz>` - Resonator frequency (20-5000)
- `RD <ms>` - Resonator decay time (10-5000 ms)
- `RM <0-16383>` - Resonator mix (dry/wet)
- `RK <0-16383>` - Resonator key tracking

**Compressor**
- `CT <0-16383>` - Threshold
- `CR <1-20>` - Ratio (1=off, 20=limiting)
- `CA <1-500>` - Attack (ms)
- `CL <10-2000>` - Release (ms)
- `CM <0-16383>` - Makeup gain

**Pan**
- `PAN <-16383 to +16383>` - Stereo position (-L, 0=center, +R)

**Beat Repeat**
- `BR.ACT <0|1>` - Enable/disable beat repeat
- `BR.LEN <0-7>` - Loop division (0=1/16, 1=1/8, 2=1/4, 3=1/2, 4=1, 5=2, 6=4, 7=8 beats)
- `BR.REV <0|1>` - Reverse playback
- `BR.WIN <1-50>` - Window/capture size (ms)
- `BR.MIX <0-16383>` - Dry/wet mix

**Pitch Shift**
- `PS.MODE <0|1>` - Mode (0=normal, 1=granular)
- `PS.SEMI <-24 to 24>` - Pitch shift (semitones)
- `PS.GRAIN <5-100>` - Grain size (ms)
- `PS.MIX <0-16383>` - Dry/wet mix
- `PS.TARG <0|1>` - Target (0=input, 1=output)

**Stereo Delay**
- `DT <ms>` - Delay time (1-2000 ms)
- `DF <0-16383>` - Delay feedback amount
- `DLP <hz>` - Delay lowpass filter cutoff (100-20000)
- `DW <0-16383>` - Delay wet mix (INSERT mode) or send level (SEND mode)
- `DS <0-1>` - Delay sync (0=free, 1=tempo - not implemented)

**3-Band EQ (Post-Delay)**
- `EL <-24 to +24>` - Low shelf gain (dB at 200Hz)
- `EM <-24 to +24>` - Mid peak gain (dB)
- `EF <200-8000>` - Mid center frequency (Hz)
- `EQ <0.1-10>` - Mid Q/bandwidth
- `EH <-24 to +24>` - High shelf gain (dB at 4000Hz)

**Plate Reverb**
- `RV <0-16383>` - Reverb size/decay time
- `RP <ms>` - Reverb pre-delay (0-100 ms)
- `RH <0-16383>` - Reverb high damping
- `RW <0-16383>` - Reverb wet mix (INSERT mode) or send level (SEND mode)

**Effect Routing**
- `D.MODE <0-2>` - Delay routing mode (0=BYPASS, 1=INSERT, 2=SEND)
- `D.TAIL <0-2>` - Delay tail behavior (0=CUT, 1=RING, 2=FREEZE)
- `R.MODE <0-2>` - Reverb routing mode (0=BYPASS, 1=INSERT, 2=SEND)
- `R.TAIL <0-2>` - Reverb tail behavior (0=CUT, 1=RING, 2=FREEZE)

Effect routing modes:
- **BYPASS (0)**: Effect disabled, signal passes through unchanged
- **INSERT (1)**: Traditional series processing with wet/dry mix
- **SEND (2)**: Parallel processing where wet parameter controls send level

Tail behaviors:
- **CUT (0)**: Tails cut immediately when wet=0 (gated output)
- **RING (1)**: Tails decay naturally (default behavior)
- **FREEZE (2)**: Stop new input, sustain current tail indefinitely

Note: In SEND mode with RING or FREEZE tail modes, the effect output remains at full level when wet=0, allowing tails to continue naturally. In CUT mode, output is gated by the wet parameter.

#### Math Operations
- `ADD <a> <b>` or `+ <a> <b>` - Add two values (works as command and in expressions)
- `SUB <a> <b>` or `- <a> <b>` - Subtract b from a (works as command and in expressions)
- `MUL <a> <b>` or `* <a> <b>` - Multiply two values (works as command and in expressions)
- `DIV <a> <b>` or `/ <a> <b>` - Divide a by b (works as command and in expressions)
- `MOD <a> <b>` or `% <a> <b>` - Modulo a by b (works as command and in expressions)
- `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Range mapping with clamping (works as command and in expressions)
  - Maps input value from input range to output range
  - Automatically clamps result to output range
  - Example: `MAP 50 0 100 0 1000` maps 50 from 0-100 range to 500 in 0-1000 range
  - Works with reversed ranges: `MAP 25 0 100 1000 0` maps and inverts

#### Random Number Generation
- `RND <max>` - Random integer from 0 to max inclusive (works as command and in expressions)
- `RRND <min> <max>` - Random integer from min to max inclusive (works as command and in expressions)
- `TOSS` - Random 0 or 1 (coin flip, works as command and in expressions)
- `EITH <a> <b>` - Random choice between a and b (works as command and in expressions)
- `TOG <a> <b>` - Alternates between a and b on each call (works as command and in expressions)
  - State is per-script and per-line
  - First call returns a, second returns b, third returns a, etc.
  - Example: `PF TOG N 0 N 7` alternates between C3 and G3

#### Randomization Commands
**Voice Randomization:**
- `RND.VOICE` - Randomize all oscillator/FM parameters within musical ranges
- `RND.OSC` - Randomize oscillator params only (PF, PW, MF, MW)
- `RND.FM` - Randomize FM-related params (FM, FB, FBA, FBD)

**Modulation Randomization:**
- `RND.MOD` - Randomize modulation routing (MB, TK, MP, MD, MT, MA)
- `RND.ENV` - Randomize envelope times and amounts (ATK, DEC, CRV, PA, FA, DA)

**FX Randomization:**
- `RND.FX` - Randomize all effect parameters (filter + delay + reverb)
- `RND.FILT` - Randomize filter (FC, FQ, FT, FE)
- `RND.DLY` - Randomize delay (DT, DF, DLP, DW)
- `RND.VERB` - Randomize reverb (RV, RP, RH, RW)

**Pattern Randomization:**
- `RND.P [min] [max]` - Randomize working pattern values (default: 0-127)
- `RND.PN <n> [min] [max]` - Randomize specific pattern n
- `RND.PALL [min] [max]` - Randomize all patterns

#### Note/Pitch Conversion
- `N <semitones>` - Convert semitones to frequency in Hz (12-TET, works in expressions)
  - N 0 = C3 (131 Hz) - matches Teletype reference pitch
  - N 12 = C4 (262 Hz)
  - N 21 = A4 (440 Hz)
  - N -12 = C2 (65 Hz)
  - Usage: `PF N 0` (set primary freq to C3), `PF N ADD A 7` (C3 + A semitones + perfect 5th)

#### Scale Quantization
- `Q <note>` - Quantize note (semitone value) to current scale (works in expressions)
  - Returns nearest note in scale as semitone value
  - Use with N operator to convert to Hz: `PF N Q A`
  - Works with pattern data: `PF N Q P.NEXT`
- `Q.ROOT <0-11>` - Set scale root note (C=0, C#=1, D=2, ..., B=11)
- `Q.SCALE <0-11>` - Set scale type
  - 0 = Chromatic (all notes)
  - 1 = Major
  - 2 = Minor (natural)
  - 3 = Dorian
  - 4 = Phrygian
  - 5 = Lydian
  - 6 = Mixolydian
  - 7 = Pentatonic Major
  - 8 = Pentatonic Minor
  - 9 = Blues
  - 10 = Whole Tone
  - 11 = Diminished
- `Q.BIT <binary>` - Set custom scale mask as binary string
  - Each bit represents a semitone (1=in scale, 0=not in scale)
  - Examples:
    - `Q.BIT 101010110101` - Major scale (12-TET)
    - `Q.BIT 10101` - Pentatonic (5-EDO)
    - `Q.BIT <24 bits>` - Quarter-tone systems
  - Allows arbitrary microtonal scales and EDO systems

Example usage:
```
Q.ROOT 0          // Set root to C
Q.SCALE 1         // Set to Major scale
PF N Q A          // Quantize variable A to C Major, convert to Hz
PF N Q P.NEXT     // Quantize pattern value to scale
Q.BIT 10101       // Custom 5-note scale
```

#### Recording
- `REC` - Start recording to current working directory (timestamped WAV file)
- `REC.STOP` - Stop recording (automatically called on quit)
- `REC.PATH <prefix>` - Set custom recording path prefix
- Files saved as WAV (int24) format
- UI shows red "● REC MM:SS" indicator when recording
- Recording auto-stops on quit to prevent file corruption

#### Beat Repeat
- `BR.ACT <0|1>` - Enable/disable beat repeat (0=off, 1=on)
  - Buffer freezes on activation (captures audio at moment of activation)
  - Automatically sets BR.MIX to 100% (16383) when activated
  - Uses separate L/R buffers for proper stereo operation
- `BR.LEN <0-7>` - Loop division/length setting
  - 0 = 1/16 beat (shortest loop)
  - 1 = 1/8 beat
  - 2 = 1/4 beat
  - 3 = 1/2 beat
  - 4 = 1 beat
  - 5 = 2 beats
  - 6 = 4 beats
  - 7 = 8 beats (longest loop)
- `BR.REV <0|1>` - Reverse playback (0=normal, 1=reversed)
- `BR.WIN <1-50>` - Window/capture size in milliseconds (1-50ms)
- `BR.MIX <0-16383>` - Dry/wet mix (0=dry, 16383=100% wet)
  - Mix is adjustable after activation even though it auto-sets to 100%

Example usage:
```
BR.ACT 1          // Enable beat repeat (freezes buffer, sets mix to 100%)
BR.LEN 2          // Set to 1/4 beat loop
BR.WIN 10         // 10ms window
BR.MIX 8192       // Adjust mix to 50% (can change after activation)
BR.REV 1          // Reverse playback
```

#### Pitch Shift
- `PS.MODE <0|1>` - Pitch shift mode (0=normal, 1=granular)
- `PS.SEMI <-24 to 24>` - Pitch shift amount in semitones
  - Negative values shift down
  - Positive values shift up
  - Range: -24 (2 octaves down) to +24 (2 octaves up)
- `PS.GRAIN <5-100>` - Grain size in milliseconds (5-100ms)
  - Smaller grains = more artifacts but tighter timing
  - Larger grains = smoother but more latency
- `PS.MIX <0-16383>` - Dry/wet mix (0=dry, 16383=100% wet)
- `PS.TARG <0|1>` - Processing target (0=input signal, 1=output signal)

Example usage:
```
PS.MODE 1         // Granular mode
PS.SEMI 12        // Shift up one octave
PS.GRAIN 20       // 20ms grain size
PS.MIX 8192       // 50% mix
PS.TARG 0         // Process input signal
```

#### Parameter Slew
- `SLEW.ALL <0-10000>` - Set global slew time in milliseconds
  - 0 = instant changes (default, backward compatible)
  - Higher values = smoother transitions
  - Applies to all smoothable parameters via SC-side Lag.kr
  - Example: `SLEW.ALL 200` makes all param changes glide over 200ms
- `SLEW <param> <0-10000>` - Set slew time for specific parameter
  - Per-parameter slew time overrides SLEW.ALL
  - Only affects the specified parameter
  - Example: `SLEW PF 500` sets only PF to 500ms slew time
  - Example: `SLEW.ALL 100; SLEW FC 1000` sets global 100ms, but FC gets 1000ms
- Smoothed parameters: PF, MF, FC, FM, MX, DC, FB, FQ, FK, FE, RF, RM, DT, DF, DW, RV, RW, volume, pan, LB, LS, LM, RGF, RGM, CT, CM, EL, EM, EH, EF
- NOT smoothed (discrete): PW, MW, FT, DM, mode switches, triggers

#### Output
- `PRINT "<text>"` or `PRINT '<text>'` - Print literal string to REPL output
- `PRINT <expr>` - Evaluate expression and print result to REPL
  - Examples: `PRINT A`, `PRINT ADD 1 2`, `PRINT PN.NEXT 0`
  - Works with variables, math operations, pattern operations, etc.

#### System
- `RST` - Reset all parameters to defaults
- `CLEAR` - Clear REPL output history
- `DEBUG <level>` - Set debug verbosity level:
  - `DEBUG 0` - Silent mode (no REPL output except errors and PRINT commands)
  - `DEBUG 1` - Important messages (metro status, PRINT commands) - minimal verbosity
  - `DEBUG 2` - Verbose mode (all parameter changes) - default level
- `q`, `quit`, or `exit` - Quit application (typed in REPL)

#### Themes
- `THEME` - Show current theme and list all available themes
- `THEME <name>` - Switch to theme by name (case-insensitive)
- Built-in themes: `dark`, `light`, `system`
- Custom themes defined in `~/.monokit/config.toml` under `[themes.name]` sections
- Example themes included: dracula, solarized, coral, copper, neo_peachio_dark, nougat_light, and many more

### Navigation (Keybindings)

#### Page Cycling
- `[` / `]` - Cycle through pages (Live → 1-8 → M → I → P → wrap)

#### Direct Page Access (Function Keys)
- `F1` through `F8` - Script pages 1-8
- `F9` - Live page
- `F10` - Metro page
- `F11` - Init page
- `F12` - Pattern page
- `ESC` - Toggle Help (overlay, scrollable with arrow keys)

#### Alternative: Alt+key (requires iTerm2 configuration)
- `Alt+L` - Live page
- `Alt+1` through `Alt+8` - Script pages 1-8
- `Alt+M` - Metro page
- `Alt+I` - Init page
- `Alt+P` - Pattern page
- `Alt+H` - Toggle Help

**iTerm2 Note:** Alt+key combinations require setting "Left Option key = Esc+" in iTerm2 Preferences > Profiles > Keys > General. Function keys (F1-F12) work in all terminals without configuration.

#### Input
- `Enter` - Execute command
- `Up/Down` - Command history (on non-Help pages) / REPL scroll (Live page with Ctrl modifier)
- `Ctrl+Up/Down` - Scroll REPL output (Live page only)
  - Scrolls through command output history
  - Shows [↑N] indicator in title when scrolled
  - Auto-resets to bottom on new output
- `Left/Right` - Cursor movement
- `Ctrl+Left/Right` - Word-by-word cursor movement
- `Ctrl+D` - Duplicate line (script pages)
- `Ctrl+K` - Delete entire line (script pages)
- `Ctrl+C` - Copy line (script pages)
- `Ctrl+X` - Cut line (script pages)
- `Ctrl+V` - Paste line (script pages)
- Script pages show validation errors that auto-clear after 3 seconds or on successful save

### UI Style (Teletype)
- **Uppercase text:** All UI text (commands, labels, output) displays in uppercase for a Teletype-style aesthetic
- **User input conversion:** User input is automatically converted to uppercase on entry
- **Script page highlighting:** Selected line shows white background with black text for brightness-based distinction
- **Line numbers:** Removed from script pages for a cleaner display

### Theme System
The UI uses a comprehensive theme system with RGB colors for terminal compatibility.

**Named Theme Support:** Config file supports multiple named themes:
```toml
[display]
theme = "dracula"    # Active theme name

[themes.dracula]
background = "#282a36"
foreground = "#f8f8f2"
# ... other colors

[themes.coral]
background = "#2d3748"
foreground = "#fa8072"
# ... other colors
```

**Theme Colors:**
- `background` - Main background color
- `foreground` - Primary text color
- `secondary` - Secondary/dimmed text (footer hints, etc.)
- `highlight_bg` / `highlight_fg` - Selection highlighting (cursor, selected line)
- `border` - Border elements
- `error` - Error messages
- `accent` - Selected items and active indicators
- `success` - Positive states (metro active, etc.)
- `label` - Section headers and labels

**Rendering:** Uses buffer-based background rendering for proper theme support across different terminal emulators. Themes use RGB color values (Color::Rgb) for consistent cross-platform display.

**Config Location:** `~/.monokit/config.toml`
**Example Config:** See `config.toml.example` in repo for 30+ pre-defined themes

## OSC Protocol

All communication from Rust CLI to SuperCollider server uses UDP over localhost (127.0.0.1:57120).

**Message Format**
- **Trigger:** `/monokit/trigger` (no arguments)
- **Master Volume:** `/monokit/volume` with float value (0.0-1.0)
- **Parameter Control:** `/monokit/param <name> <value>` where:
  - `<name>` = parameter name (string):
    - Oscillator/FM: pf, pw, mf, mw, fb, fba, fbd, dc, dm, dd, tk, mb, mp, md, mt, ma, fm, mx, mm, me
    - Envelopes: ad, pd, fd, dd, pa, fa, da
    - Lo-Fi: lb, ls, lm
    - Filter: fc, fq, ft, fe, fed, fk, mf_f
    - Ring Mod: rgf, rgw, rgm
    - Resonator: rf, rd, rm, rk
    - Compressor: ct, cr, ca, cl, cm
    - Pan: pan
    - Beat Repeat: br_act, br_len, br_rev, br_win, br_mix
    - Pitch Shift: ps_mode, ps_semi, ps_grain, ps_mix, ps_targ
    - Delay: dt, df, dlp, dw, ds, dmode, dtail
    - EQ: el, em, ef, eq, eh
    - Reverb: rv, rp, rh, rw, rmode, rtail
  - `<value>` = float or int depending on parameter type
- **Reset:** `/monokit/reset` (no arguments, resets all parameters to defaults)
- **Recording:**
  - `/monokit/rec` - Start recording (with optional directory path)
  - `/monokit/rec/stop` - Stop recording
  - `/monokit/rec/path` - Set custom recording path prefix

All parameter updates are validated in Rust CLI before sending and applied immediately on SuperCollider voice.

## Dependencies

### Rust
- rosc 0.10 - OSC protocol
- ratatui 0.29 - Terminal UI framework
- crossterm 0.28 - Terminal backend
- rand 0.8 - Random number generation (for PROB, RND, RRND)
- anyhow 1 - Error handling
- thiserror 1 - Error types

### SuperCollider
- SuperCollider 3.x with scsynth
