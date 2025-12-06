# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## Recent Updates (December 2025)

### SC Process Management [COMPLETE]
Automatic SuperCollider lifecycle management - monokit spawns sclang on startup.

**Completed (Phases 1-4):**
- [x] SC process manager (`src/sc_process.rs`) - spawn/stop/restart
- [x] SC script mods - env var for audio device, ready signal
- [x] Ready detection in meter thread - `/monokit/ready` OSC
- [x] Main.rs integration - spawn SC, wait for ready, graceful shutdown
- [x] Graceful shutdown - `Server.quitAll; 0.exit;` + pkill scsynth

**Completed (Phases 5-8):**
- [x] AUDIO.OUT command for device selection
- [x] SC restart on device change
- [x] Config persistence for audio device
- [x] Help system update

**Implementation Notes:**
- Numbered device selection (AUDIO.OUT 0-N) implemented to solve case-sensitivity issue with device name matching

**Documentation:**
- `AUDIO_DEVICE_PLAN.md` - Implementation plan
- `CROSS_PLATFORM_AUDIO.md` - Future cross-platform design

### VCA Default Mode Changed [COMPLETE]
Changed VCA default from 0 (drone) to 1 (gate) for more intuitive behavior on startup/restart.

**Changes:**
- [x] VCA default changed from 0 to 1 (synth silent until triggered)
- [x] Prevents unintended audio on startup/restart
- [x] More predictable user experience for new users
- [x] Matches traditional drum synthesizer behavior

### AUDIO.OUT Direct Mode Support [COMPLETE]
AUDIO.OUT command now works in scsynth-direct mode with graceful device switching.

**Changes:**
- [x] AUDIO.OUT queries devices directly via CoreAudio
- [x] Silent restart without terminal output corruption
- [x] Meters continue working during device switching
- [x] No audio glitches during device changes

### TUI/Startup Timing Fixed [COMPLETE]
Fixed timing so startup prints complete before TUI renders.

**Changes:**
- [x] Ready sender thread spawned after meter thread
- [x] Waits for ready completion before enabling raw mode
- [x] Prevents TUI rendering during startup messages
- [x] Clean startup sequence with proper console output

### t_gate Trigger Implementation [COMPLETE]
Reliable triggering in gate mode using TrigControl.

**Changes:**
- [x] Replaced `gate` parameter with `t_gate` (TrigControl)
- [x] t_gate automatically resets after each control block
- [x] More reliable trigger behavior in scsynth-direct mode
- [x] Backward compatibility for sclang-based triggering

### Error Display Consistency [COMPLETE]
Unified error formatting across REPL and script views.
- [x] All errors use "ERROR:" prefix in uppercase
- [x] REPL view shows errors in red with consistent formatting
- [x] Removed duplicate "ERROR: ERROR:" issue
- [x] Standardized error prefixes across all source files

### Negative Number Handling [COMPLETE]
Prevent crashes from negative values in expressions.
- [x] TOG/EITH bounds checking in eval/logic.rs
- [x] Pattern expression validation (PN.*, P.*)
- [x] Clear error messages instead of panics

### SEQ Highlight Fix [COMPLETE]
Fixed highlighting for commands with semicolons after quoted strings.
- [x] Created shared `split_respecting_quotes()` utility
- [x] State highlight uses same parsing as execution

### Command Validation & Error Display Fixes [COMPLETE]
Comprehensive command validation audit and error display improvements.

**Command Validation Audit (29 missing commands):**
- [x] CPU, BPM, HEADER - System query commands now validated
- [x] METER.HDR, METER.GRID, SPECTRUM, ACTIVITY, GRID, GRID.MODE - UI toggles now validated
- [x] HL.SEQ, HL.COND - Highlighting commands now validated
- [x] N1, N2, N3, N4 counter queries now validated
- [x] N1.MIN, N1.MAX (and N2-N4 equivalents) now validated
- [x] TITLE, FLASH, CLEAR - Utility commands now validated
- [x] REPL.DUMP - REPL export command now validated
- [x] OUT.ERR, OUT.ESS, OUT.QRY, OUT.CFM - Output control commands now validated
- [x] MIDI.IN, LOAD.RST - Additional missing commands now validated
- [x] GATE.ATK, GATE.DEC, GATE.SUS, GATE.REL, GATE.CRV, GATE.MODE, GATE - 7 GATE commands now validated

**PN Expression Validation:**
- [x] PN.* commands now require pattern argument even when used as expressions
- [x] Prevents cryptic "not enough arguments" errors during evaluation
- [x] Examples: PN.NEXT, PN.MIN, PN.MAX, PN.SUM, PN.AVG properly validated

**Error Display Improvements:**
- [x] Errors now display in bottom border of script blocks (fits 50x18 terminal)
- [x] Error message prefix fix: "ERROR: ERROR:" duplicate removed
- [x] Validation errors show immediately on script page line edit

**UI Bug Fixes:**
- [x] Ctrl+U now works on Notes page (was missing from input handler)

### Global Search Feature [COMPLETE]
Unified search functionality with isolated scopes for help pages and scripts.

**Keybindings:**
- `Ctrl+F` - Enter search mode (context-aware scope based on current page)
- `Enter` - Jump to next match
- `Shift+Enter` - Jump to previous match
- `ESC` - Exit search mode (preserves user position)

**Behavior:**
- Help search scope: searches help pages only
- Script search scope: searches scripts 1-8, M, I (no patterns)
- Case-insensitive queries
- Current match highlighted with highlight_bg/highlight_fg
- Other matches highlighted with accent color
- Match count indicator: [N/M] format
- Navigation hotkeys (Alt+key, F1-F12) exit search and navigate

**Implementation:**
- Search state tracked in App struct
- Two isolated search result buffers (help_search, script_search)
- Dynamic scope switching on page navigation
- Position restoration on ESC

### VCA Mode Command [COMPLETE]
VCA envelope mode control with DRONE and GATED modes.

- [x] `VCA <0|1>` - Set VCA mode (0=DRONE, 1=GATED)
- [x] VCA 0 (DRONE): Open VCA, no envelope modulation
- [x] VCA 1 (GATED): Amplitude envelope controls VCA (default)
- [x] Expression support in command arguments
- [x] Config persistence across sessions

### TITLE.TIMER Command [COMPLETE]
Auto-cycle header title between "MONOKIT" and scene name.

- [x] `TITLE.TIMER <0|1> <secs>` - Enable/disable title timer (1-1800 seconds)
- [x] TITLE.TIMER 0: Disable auto-cycling
- [x] TITLE.TIMER 1 <secs>: Enable with specified interval
- [x] Expression support in timer duration
- [x] Config persistence across sessions
- [x] Automatically starts cycling between "MONOKIT" and scene name

### Terminal Window Title [COMPLETE]
Custom terminal window title for better taskbar/window list identification.

- [x] Terminal window title shows "monokit" instead of "scsynth"
- [x] Improves task switching and window identification
- [x] Set via ANSI escape sequences on startup

### Terminal Compatibility System [COMPLETE]
Automatic detection and fallback for limited terminal capabilities.

- [x] Terminal capability detection at startup (true color vs 256-color)
- [x] `COMPAT` - Display terminal capabilities
- [x] `COMPAT.MODE <0|1>` - Toggle compatibility mode (0=standard, 1=256-color)
- [x] `METER.ASCII <0|1>` - Toggle ASCII meter characters (.:-=+#)
- [x] 256-color theme fallback when true color unavailable
- [x] High-contrast cursor (white/black) in 256-color mode
- [x] Startup warning for Terminal.app users

### SCOPE.CLR Color Labels [COMPLETE]
Enhanced scope color selection with named colors.

- [x] `SCOPE.CLR <name|0-8>` - Set waveform color by name or number
- [x] Color names: FOREGROUND, SECONDARY, HIGHLIGHT_BG, HIGHLIGHT_FG, BORDER, ERROR, ACCENT, SUCCESS, LABEL
- [x] Aliases: FG, SEC, HL_BG, HL_FG, ERR, ACC, SUC, LBL
- [x] All 9 theme colors now available (was only 4)
- [x] Backwards compatible with numeric 0-8

### REPL Output Consistency [COMPLETE]
UI/settings commands always display regardless of DEBUG level.

- [x] Fixed: LIMIT, SCOPE.TIME, SCOPE.CLR, SCOPE.MODE, SCOPE.UNI
- [x] Direct queries and setting confirmations always visible
- [x] DEBUG level only gates background/automated output

### Scramble Animation System [COMPLETE]
Rolling text scramble animation for header with comprehensive controls.

- [x] `SCRMBL <0|1>` - Enable/disable scramble animation
- [x] `SCRMBL.MODE <0|1>` - Animation mode (0=random, 1=sequential)
- [x] `SCRMBL.SPD <0-5>` - Animation speed (0=slowest, 5=fastest)
- [x] `SCRMBL.CRV <0-4>` - Animation curve (linear, ease-in, ease-out, ease-in-out, exponential)
- [x] Title scramble on scene load/title change
- [x] Grid icons no longer scramble on tab switch (disabled)
- [x] Config persistence for all scramble settings

---

## Recent Updates (November 2025)

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
- Simple percussive envelopes (Env.perc) for predictable behavior

**Code Changes (sc/monokit_server.scd):**
```supercollider
// Control captures prevent optimizer issues
var paCtl, faCtl, daCtl, fbaCtl;

paCtl = Lag.kr(pa, 0);
faCtl = Lag.kr(fa, 0);
daCtl = Lag.kr(da, 0);
fbaCtl = Lag.kr(fba, 0);

// Envelope generation using Env.perc
ampEnv = EnvGen.kr(Env.perc(aenvAtk/1000, ad/1000, 1, aenvCrv), gate);
pitchEnv = EnvGen.kr(Env.perc(penvAtk/1000, pd/1000, 1, penvCrv), gate);
// ... etc for all envelopes

// Additive application with exponential pitch scaling
primaryFreq = pfSmooth * pow(2, pitchEnv * paCtl);
```

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
- [x] Control flow: IF/ELIF/ELSE, L (loop), BRK (break), PROB, EVERY, SKIP with PRE separator (`:`)
- [x] Comparison operators: EQ, NE, GT, LT, GTE, LTE, EZ, NZ (both prefix and infix)
- [x] Sub-command separator: `;` for multiple commands per line
- [x] Scene persistence: SAVE/LOAD system for scripts + patterns
- [x] MAP operator: Range mapping with clamping
- [x] TOG generator: Toggle between two values on each trigger
- [x] N1-N4 counters: Auto-increment variables with MIN/MAX/RST control

### Modulation & Routing
- [x] ModBus routing to filter cutoff (MF_F parameter)
- [x] Envelope system with PREFIX.SUFFIX naming (AENV, PENV, FMEV, DENV, FBEV, FLEV)
- [x] Per-envelope control: ATK, CRV for each envelope type
- [x] Envelope decay times: AD, PD, FD, DD, FBD, FED
- [x] Envelope amounts: PA (pitch, octaves), FA (FM), DA (discontinuity), FBA (feedback), FE (filter)
- [x] Simple percussive envelopes (Env.perc) triggered on TR command
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

## Priority: Infrastructure Refactoring

### Command System DRY Refactor [High] - COMPLETE (November 2025)
Consolidated command definitions into a single source of truth to eliminate synchronization bugs.

**Phase 0: Codebase Reorganization** - COMPLETE
- [x] Created `core/`, `system/`, `synth/` directory structure
- [x] Moved command handlers to logical domains
- [x] Renamed `delay.rs` → `scheduling.rs`
- [x] Renamed `synth_params/` → `synth/`
- [x] Split effects into modular files (`synth/effects/`)

**Phase 1: Envelope Handler DRY** - COMPLETE (November 2025)
- [x] Created `synth/envelopes/common.rs` with `define_int_param!` and `define_float_param!` macros
- [x] Refactored all 6 envelope files (amp, pitch, fm, disc, feedback, filter)
- [x] Removed dead code: `handle_*_mode` handlers, `global.rs`
- [x] Line reduction: ~1,141 lines → 223 lines (**918 line reduction, 81% decrease**)
- [x] All 558 tests pass

**Phase 2: Pattern Operation DRY** - COMPLETE (November 2025)
- [x] Created `patterns/common.rs` (902 lines) with `PatternRef` enum, shared implementations, and macro system
- [x] Aggressive macro approach: 10 macros generate both P.* and PN.* handlers from single definitions
- [x] Unified P.* (working) and PN.* (explicit) operations via PatternRef::Working/Explicit
- [x] Wrapper code reduced from 2023 → 450 lines (**1,573 line reduction, 78% decrease**)
- [x] Explicit files now just re-export from working files (~10 lines each)
- [x] All 558 tests pass

**Phase 3: Synth Parameter DRY** - COMPLETE (November 2025)
- [x] Created `synth/param_macro.rs` with generic parameter macros
- [x] Consolidated 70+ similar parameter handlers
- [x] Line reduction: **~2,325 lines**
- [x] All 558 tests pass

**Phase 4: Variables, Counters, and Test Fixtures** - COMPLETE (November 2025)
- [x] Phase 4A: Variable/Counter macros - **489 lines removed**
- [x] Phase 4B: Expression helpers - Infrastructure added
- [x] Phase 4C: Test fixture optimization - **637 lines removed**
- [x] Total Phase 4: **~1,126 lines removed**
- [x] All 558 tests pass

**Phase 5: Additional DRY Refactors** - COMPLETE (December 2025)
- [x] Boolean toggle macro (define_bool_toggle!) - **337 lines saved**
- [x] Integer enum macro (define_enum_select!) - **416 lines saved**
- [x] F-key handler loop refactor - **36 lines saved**
- [x] Pattern macros extraction to patterns/macros.rs
- [x] ScopeSettings struct bundling
- [x] Context structs defined (OutputSettings, ScrambleSettings, UIToggles)
- [x] Total Phase 5 reduction: **~800+ lines**
- [x] All tests pass

**Program Completion Summary:**
- **Total DRY Reduction (Phases 1-5): ~6,742 lines (31% of original codebase)**
- Clear, logical file organization by domain
- Easier to add new commands (single macro invocation)
- All tests continue to pass throughout
- Significantly reduced maintenance burden

**Reference:** See `DRY_REFACTOR_PLAN.md` and `DRY_PHASE4_PLAN.md` for comprehensive implementation details.

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
- [x] Per-envelope attack time (*.ATK) for all 6 envelope types
- [x] Per-envelope curve control (*.CRV, -8 to 8, log/linear/exp)
- [x] Decay time controls: AD, PD, FD, DD, FBD, FED
- [x] Envelope amounts: PA, FA, DA, FBA (FBEV.AMT), FE
- [x] Simple percussive envelopes (Env.perc) with controllable attack and curve
- [x] Exponential pitch envelope scaling for proper octave behavior
- [x] Removed: Global envelope controls, gate parameters, mode switching

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
- [x] `RND.VOICE` - Randomize all oscillator/FM parameters within musical ranges
- [x] `RND.OSC` - Randomize oscillator params only (PF, PW, MF, MW)
- [x] `RND.FM` - Randomize FM-related params (FM, FB, FBA, FBD)

**Modulation Randomization:**
- [x] `RND.MOD` - Randomize modulation routing (MB, TK, MP, MD, MT, MA)
- [x] `RND.ENV` - Randomize envelope times and amounts

**FX Randomization:**
- [x] `RND.FX` - Randomize all effect parameters
- [x] `RND.FILT` - Randomize filter (FC, FQ, FT, FE)
- [x] `RND.DLY` - Randomize delay (DT, DF, DLP, DW)
- [x] `RND.VERB` - Randomize reverb (RV, RP, RH, RW)

**Pattern Randomization:**
- [x] `RND.P` - Randomize working pattern values
- [x] `RND.P <min> <max>` - Randomize within range
- [x] `RND.PN <n>` - Randomize specific pattern
- [x] `RND.PALL` - Randomize all patterns

---

## Phase 3: Musical Features

**Focus:** Musical utilities and external sync capabilities

### Scale Quantization [Medium] - COMPLETE
- [x] `Q <note>` - Quantize note to current scale (expression operator)
- [x] `Q.SCALE <0-11>` - Set scale type (12 presets)
- [x] `Q.ROOT <0-11>` - Set root note (C=0, C#=1, etc.)
- [x] `Q.BIT <binary>` - Custom scale mask (arbitrary length for microtonal)
- [x] `PF N Q A` - Quantize variable to scale, convert to frequency
- [x] N operator adapts to scale.divisions (supports non-12-TET)

**Scale Types:**
- 0=Chromatic, 1=Major, 2=Minor, 3=Dorian, 4=Phrygian, 5=Lydian
- 6=Mixolydian, 7=Pentatonic Major, 8=Pentatonic Minor
- 9=Blues, 10=Whole Tone, 11=Diminished

**Q.BIT Examples:**
- `Q.BIT 101010110101` - Major scale (12-TET)
- `Q.BIT 10101` - Pentatonic (5-EDO)
- `Q.BIT <24 bits>` - Quarter-tones (24-EDO)

### Mini Notation / Inline Sequencing [High] - PHASE 2 COMPLETE
SEQ provides inline sequence notation that cycles through values on each evaluation.

**Phase 1 (Complete):**
- [x] `SEQ "x _ x _"` - Trigger pattern notation (x=1, _=0)
- [x] `SEQ "200 400 300 _"` - Numeric value sequences
- [x] `SEQ "C3 E3 G3 C4"` - Note names (returns semitones, use with N)
- [x] Note accidentals: sharps (#) and flats (b) - `C#3`, `Bb4`, `F#2`
- [x] Per-script, per-pattern independent state
- [x] Works in all expression contexts: `PF N SEQ "C3 E3"`, `IF SEQ "x _": TR`

**Phase 2 (Complete):**
- [x] `*n` - Repeat token n times (e.g., `C3*4` expands to `C3 C3 C3 C3`)
- [x] `?` - Random trigger (50% chance of 1, 50% chance of 0)
- [x] `<a b>` - Toggle/Cycle (deterministic, like TOG - cycles A, B, A, B...)
- [x] `{a b}` - Random Choice (unpredictable, like EITH - randomly picks A or B each time)
- [x] Combinable modifiers (e.g., `<C3 E3>*2` toggles twice, `{C3 E3}*2` picks randomly twice)

**Distinction Between Toggle and Random:**
- `<a b>` = **Toggle/Cycle** - deterministic state machine
  - `SEQ "<C3 E3>"` → C3, E3, C3, E3, C3, E3...
  - State persists across calls (remembers last value)
- `{a b}` = **Random Choice** - unpredictable selection
  - `SEQ "{C3 E3}"` → randomly C3 or E3, then randomly again
  - No state (picks fresh each time)

**Phase 3 (Future):**
- [ ] `[a b]` - Subdivision brackets
- [ ] Probability modifiers `@p` for per-step probabilities
- [ ] Euclidean rhythm syntax

**Phase 2 Usage Examples:**
```
SEQ "C3*4 E3*2"                # Repeated notes (C3 C3 C3 C3 E3 E3)
SEQ "<C3 E3> G3"               # Toggle C3/E3, then G3
SEQ "{C3 E3} G3"               # Random C3 or E3, then G3
SEQ "x ? x ?"                  # Random triggers (50% chance each)
SEQ "<C3 E3>*2"                # Two toggles
SEQ "{C3 E3}*2"                # Two random choices
SEQ "<C3 E3> {G3 B3}"          # First toggles, second random

IF SEQ "x _ x _": TR           # Trigger on beats 1 and 3
PF N SEQ "C3 E3 G3 C4"         # Arpeggiate C major (semitones → Hz)
PF N Q SEQ "0 3 5 7"           # Quantized to current scale
A SEQ "0 1 2 3"                # Store sequence value in variable
```

### Preset System [Medium] - COMPLETE
Save and load parameter configurations into script slots.

- [x] `PSET <script> <name>` - Load preset into script 1-8
- [x] `PSET.SAVE <script> <name>` - Save script as user preset
- [x] `PSET.DEL <name>` - Delete user preset
- [x] `PSETS` - List all presets ([F] factory, [U] user)
- [x] 22 factory presets (drums, bass, lead, percussion, FX)
- [x] User presets stored in `~/.config/monokit/presets/`

**Factory Presets:**
- Drums: 808-kick, punch-kick, sub-kick, basic-snare, snap-snare, hat-closed, hat-open, fm-hat, clap, rim
- Bass: sub-bass, saw-bass, fm-bass
- Lead: saw-lead, fm-lead, pluck-lead
- Percussion: metal-hit, conga, tom
- FX: noise, zap, rise

**Usage:**
```
PSET 1 808-kick              # Load kick preset into script 1
PSET.SAVE 2 my-bass          # Save script 2 as user preset
PSETS                        # List all presets
```

### DAW / MIDI Clock Sync [High] - PARTIAL

**MIDI Clock Input:** - COMPLETE
- [x] `M.SYNC <0|1>` - Sync mode (0=internal, 1=MIDI clock)
- [x] `MIDI.IN` - List available MIDI input devices
- [x] `MIDI.IN <name>` - Connect to MIDI device for clock sync
- [x] Auto-detect MIDI input devices
- [x] Follow external tempo (24 PPQN standard, 16th note resolution)
- [x] Start/stop follows MIDI transport commands
- [x] Rock-solid timing (0.02% jitter, no swing artifacts)

**Timing Diagnostics:** - COMPLETE (December 2025)
- [x] `MIDI.DIAG 1/0` - Enable/disable Rust-side timing diagnostics
- [x] `MIDI.DIAG REPORT` - Write timing report to `midi_timing_report.txt`
- [x] `SC.DIAG 1/0` - Enable/disable SuperCollider-side timing diagnostics
- [x] `SC.DIAG REPORT` - Write timing report to `sc_timing_report.txt`
- [x] Trigger counting on both sides (detect packet loss)
- [x] Swing detection via odd/even interval analysis
- [x] 4MB UDP socket buffer (eliminates packet loss)
- [x] 1ms UI event poll (eliminates timing jitter)

**Technical Details:** See `docs/MIDI_CLOCK_TIMING_LESSONS.md` for comprehensive debugging notes.

**Clock Division/Multiplication:** - NOT IMPLEMENTED
- [ ] `M.DIV <1-16>` - Divide incoming clock
- [ ] `M.MUL <1-4>` - Multiply incoming clock

**Transport Control:** - NOT IMPLEMENTED
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

### Command Delay System (Teletype DEL) [Medium] - COMPLETE
Scheduled command execution with delay buffer (inspired by Teletype).

**Basic Delay:**
- [x] `DEL <ms>: <cmd>` - Execute command after delay (max 16000ms)
- [x] `DEL.CLR` - Clear all pending delayed commands

**Repeated Delays:**
- [x] `DEL.X <count> <ms>: <cmd>` - Queue command N times at intervals
  - Example: `DEL.X 4 100: TR` fires at 100ms, 200ms, 300ms, 400ms
- [x] `DEL.R <count> <ms>: <cmd>` - Execute immediately, then repeat
  - Example: `DEL.R 4 100: TR` fires now, then at 100ms, 200ms, 300ms

**Advanced Delays (Future):**
- [ ] `DEL.B <ms> <bitmask>: <cmd>` - Bitmasked delay (16-bit pattern)
  - LSB = immediate, each bit = one interval of <ms>
  - Example: `DEL.B 100 0b1010: TR` fires at 100ms, 300ms
- [ ] `DEL.G <ms> <exp>: <cmd>` - Exponential delay timing (non-linear)

**Implementation Notes:**
- Delay buffer holds pending commands in metro thread
- Commands execute at scheduled time with absolute timing
- Supports expressions in delay time and counts

---

## Phase 4: UI/Feedback

**Focus:** Visual enhancements and real-time parameter monitoring

**Reference:** See `UI_REFINEMENT_PLAN.md` for detailed implementation guide.

### Phase 4.1: Activity Indicators [Medium] - COMPLETE (December 2025)
Script and metro execution feedback with decay animations (KO II style).

- [x] Add activity tracking to App struct (activity_last_active, activity_hold_ms)
- [x] Smooth color decay using cubic ease-out interpolation
- [x] Header shows script indicators (1-8, M, I) with color decay
- [x] Metro tick pulses M indicator
- [x] TR command pulses trigger indicator
- [x] FLASH command to adjust hold time (default 200ms)
- [x] Works for nested SCRIPT calls from metro
- [x] Theme-aware activity_color() in theme.rs

### Phase 4.2: SEQ/TOG State Highlighting [Medium] - COMPLETE (December 2025)
Show current position in stateful operators within script display.

- [x] Color-only highlighting (no bracket markers) for cleaner display
- [x] Current SEQ step highlighted in foreground/success color
- [x] TOG active option highlighted in foreground/success color
- [x] Nested alternation `<a b>` shows active option based on stored state
- [x] Nested random choice `{a b}` shows last selected option (state now tracked)
- [x] Color strategy: non-selected lines use foreground/secondary, selected lines use success/highlight_fg
- [x] State lookup from existing `toggle_state` HashMap
- [x] Handle multiple operators per line
- [x] Created `src/ui/state_highlight.rs` module with unit tests
- [x] Integrated into script.rs, metro.rs, init.rs pages
- [x] SEQ validation: reject invalid syntax (`SEQ"..."` and `SEQ "...`)
- [x] Random choice state tracking with `seq_rnd_` keys

### Phase 4.3: Variables Page [Medium] - COMPLETE
Dedicated page showing all variable state (Teletype-style monitor).

- [x] Add `Page::Variables` to page enum
- [x] Create `src/ui/pages/variables.rs`
- [x] Display global vars: A, B, C, D, X, Y, Z, T
- [x] Display counters: N1-N4 with min/max bounds
- [x] Display per-script locals: J, K for all 10 scripts
- [ ] Optional: highlight recently-changed values

### Phase 4.4: Parameter Activity Grid [Medium] - COMPLETE
Alternate grid view on Live page showing parameter activity with unicode icons.

- [x] Add `ParamActivity` struct with per-parameter timestamps
- [x] Mark activity when parameters change in script execution and REPL
- [x] Tab keybinding to toggle between REPL and Grid view on Live page
- [x] 8x6 grid of 48 unicode icons representing synth parameters
- [x] Icons light up and decay when parameters change (reuses activity_color())
- [x] Grid center-justified with 3-space gaps between icons
- [x] Same decay timing as script indicators

### Phase 4.5: Audio Metering [High] - COMPLETE
Real-time amplitude display via bidirectional OSC.

- [x] Add `SendPeakRMS` to SuperCollider SynthDef (20Hz updates)
- [x] Create `src/meter.rs` receiver thread (port 57121)
- [x] Add `MeterData` type with peak/RMS/clip fields
- [x] Spawn meter thread in main.rs
- [x] Unicode bargraph display: `L▅▆ R▅▅` in header
- [x] Clip indicator (meter turns error color on clip)
- [x] REC indicator moved to right-aligned border title
- [x] Vertical 8-row meters on grid view (Tab on Live page)
- [x] 64 levels of resolution per meter
- [x] 15-band spectrum analyzer on grid view, 2 rows tall
- [x] Logarithmically-spaced frequency bands (25Hz to 16kHz)
- [x] Square root scaling for better visual response
- [x] Smooth decay (0.85 rate) for gentle falloff
- [x] SuperCollider BPF filters + Amplitude followers at 20Hz
- [x] OSC messages sent to port 57121 via /monokit/spectrum
- [x] CPU percentage display in header border (right-aligned, toggleable with CPU 1/0 command)
- [x] CPU percentage text on grid view (bottom-right, aligned with spectrum bottom)
- [x] CPU label on grid view (same row as SPECTRUM label, right side)
- [x] CPU command (CPU 0/1) to toggle header border display
- [x] SuperCollider CPU sender routine at 2Hz via /monokit/cpu OSC message
- [x] Color scheme: secondary normally, error when >= 80%
- [ ] Peak hold decay visualization (future)

### Phase 4.6: Notes Page [Medium] - COMPLETE (December 2025)
Dedicated notes page with command-based text entry and scene integration.

- [x] Redesigned to use 8 fixed lines like script pages
- [x] Line navigation with Up/Down arrows
- [x] Same editing commands: Ctrl+D/K/C/X/V
- [x] `NOTE "text"` - Append quoted text to Notes page (error if all 8 lines full)
- [x] `NOTE.CLR` - Clear all notes
- [x] Notes saved and loaded with scenes
- [x] Consistent with script page UX
- [x] Added to validator for script execution

### Phase 4.7: Conditional Execution Highlighting [Medium] - COMPLETE (December 2025)
Show visual feedback when conditionals and control flow commands execute their bodies.

**PRE Commands (use `:` separator):**
- [x] `IF <expr>:` - Highlight when condition is truthy (expr != 0)
- [x] `ELIF <expr>:` - Highlight when previous IF/ELIF false AND condition truthy
- [x] `ELSE:` - Highlight when all previous IF/ELIF were false
- [x] `PROB <n>:` - Highlight when probability check passes (n% chance)
- [x] `EV <n>:` - Highlight on every Nth tick (when executing)
- [x] `SKIP <n>:` - Highlight when NOT skipping (inverse of EV)
- [x] NOT L loops - they would flash constantly

**Implementation Details:**
- [x] Segment-based highlighting: only the PRE portion highlights, not the entire line
- [x] Example: `$ 2; IF PN.NEXT 2: TR` - only `IF PN.NEXT 2:` highlights when condition passes
- [x] Multiple PREs on same line highlight independently
- [x] Nested PREs (like `EV 4: IF A: TR`) each highlight their own segment
- [x] Reuses activity_color() decay system from Phase 5.1
- [x] Color strategy: unselected lines use foreground->secondary decay, selected lines use success->highlight_fg decay
- [x] `HL.COND <0|1>` command to toggle conditional highlighting on/off
- [x] State tracked per script line in App struct

### Global Search [Medium] - COMPLETE (December 2025)
Search functionality with isolated scopes for help and scripts.

- [x] `Ctrl+F` enters search mode
- [x] Search bar shows `/` prefix to indicate search mode
- [x] `Enter` jumps to next match, `Shift+Enter` to previous match
- [x] `ESC` exits search mode
- [x] Two isolated scopes: Help search (help pages only), Script search (scripts only)
- [x] Match highlighting on current page
- [x] Show match count indicator (e.g., "2/5")
- [x] User position preserved when exiting search mode

### Phase 4.8: Scope Page [Medium] - COMPLETE (December 2025)
Dedicated oscilloscope page with real-time waveform visualization.

- [x] New Scope page showing 128 samples at 20Hz from SuperCollider
- [x] Multiple character rendering modes (BRAILLE, BLOCK, LINE, DOT, QUADRANT)
- [x] `SCOPE.TIME <5-500>` - Set waveform timespan in milliseconds (accepts expressions)
- [x] `SCOPE.CLR <0-3>` - Set waveform color: 0=success, 1=error, 2=foreground, 3=accent (accepts expressions)
- [x] `SCOPE.MODE <0-4>` - Set display mode (accepts expressions)
- [x] `SCOPE.UNI <0|1>` - Toggle unipolar/bipolar display (accepts expressions)
- [x] All SCOPE commands now support expressions for dynamic/visualizer control
- [x] `Alt+S` navigation hotkey
- [x] Info display on bottom border (TIME, SAMPLES)
- [x] DC blocking via LeakDC in SuperCollider
- [x] Created `src/ui/pages/scope.rs` and `src/ui/braille.rs`

### Help System [Low] - PARTIAL
- [x] Add explicit `#` prefix marker to section headers
- [x] Marker-based styling (replaces fragile heuristic)
- [x] Paginated help with category pages (10 pages)
- [x] `[` / `]` navigate help pages when help is active

---

## Phase 5: Polish & Refinements

**Focus:** Cherry-on-top polish items for production readiness and optimal user experience

### Config Persistence Audit [Medium] - COMPLETE
Ensure all user preferences persist across sessions for consistent experience.

- [x] DEBUG level persistence to config.toml
- [x] HEADER verbosity level persistence (0/1/2)
- [x] CPU indicator toggle state persistence
- [x] Activity grid label/icon mode persistence
- [x] Per-element UI toggle states persistence
- [x] Load saved preferences on startup
- [x] Save preferences on change (not just on exit)

### Alias Coverage Audit [Low] - COMPLETE
Complete alias to canonical name coverage - many parameters have no short forms in CLI even though they are generally addressed with short forms in SuperCollider.

- [x] Audit all parameters for missing short-form aliases
- [x] Add aliases for envelope attacks (AENV.ATK → AA, PENV.ATK → PAA, etc.)
- [x] Add aliases for envelope curves (AENV.CRV → AC, PENV.CRV → PC, etc.)
- [x] Review SC parameter names vs CLI command names for consistency
- [x] Update aliases.rs with new mappings
- [x] Update help system with new aliases

### Per-Element UI Toggles [Low] - COMPLETE
Individual override commands to toggle visual elements independently.

- [x] `METER.HDR <0|1>` - Toggle audio meters on/off in header
- [x] `METER.GRID <0|1>` - Toggle audio meters on/off in grid view
- [x] `SPECTRUM <0|1>` - Toggle spectrum analyzer on/off
- [x] `ACTIVITY <0|1>` - Toggle script activity indicators on/off
- [x] `GRID <0|1>` - Toggle parameter activity grid on/off
- [x] `GRID.MODE <0|1>` - Toggle between text labels and icons on grid
- [x] `HL.SEQ <0|1>` - Toggle SEQ/TOG state highlighting on/off
- [x] `HL.COND <0|1>` - Toggle conditional execution highlighting on/off (already existed)
- [x] Each toggle state persists to config.toml
- [x] Works independently and on top of global UI modes

### Dynamic Grid Layout [Medium]
Intelligent UI response for Live tab page - dynamically adjust spacing and layout based on visible elements.

- [ ] Calculate available width based on meter visibility (METER.GRID)
- [ ] Calculate available height based on spectrum visibility (SPECTRUM)
- [ ] Adjust grid icon/label spacing dynamically to fill available space
- [ ] Maintain proper alignment and centering as elements toggle
- [ ] Consider font-width-aware spacing for different terminal emulators
- [ ] Optimize visual density without crowding

### Scene Name Header Display [Low] - COMPLETE (December 2025)
Toggleable display of current scene name in header, replacing "MONOKIT" title.

- [x] `TITLE <0|1>` - Toggle between "MONOKIT" (0) and scene name (1)
- [x] Display current scene name when loaded via LOAD command
- [x] Show "MONOKIT" or "[UNSAVED]" when no scene loaded
- [x] Truncate long scene names to fit header width (15 chars max, 12 chars + "...")
- [x] State persists to config.toml
- [x] Update display immediately on SAVE/LOAD
- [ ] Optional "Matrix" style character replacement lerp animation on save/load/change

### BPM Header Display [Medium] - COMPLETE (December 2025)
Add BPM readout to header border alongside CPU and REC indicators.

- [x] Add BPM display to header upper border
- [x] Format: "BPM 120" (right-aligned)
- [x] Integrate with HEADER command verbosity levels:
  - HEADER 0: no BPM display
  - HEADER 1-4: BPM display
- [x] Update BPM display when metro tempo changes (automatically reads from metro_state)
- [x] Calculate BPM from metro interval_ms (15000 / interval_ms, rounded)

### Global Text Audit [High] - COMPLETE (December 2025)
Ensure all UI text fits within 50x18 terminal window constraints (46-char safe content width).

**All UI text elements verified to fit within 46-character safe content width:**

- [x] Audit all pages for text overflow beyond 50 columns
- [x] Check Help pages for line wrapping
- [x] Test all error messages fit within bounds
- [x] Validate grid layouts don't exceed boundaries
- [x] Check header/footer borders stay within limits
- [x] All identified violations fixed (completed December 2025)

### Command Arg Scaling Audit [Medium]
Review parameter ranges for consistency and user-friendliness.

**⚠️ PREREQUISITE: Complete "File Size and DRY Audit" first!**
Changing param ranges will touch every synth handler - same sprawl risk as REPL audit.

**DRY Approach:**
- [ ] Define param metadata (name, range, scale) in single source
- [ ] Macro generates: validation, OSC scaling, help text
- [ ] Range change = 1-line edit vs 100+ file changes

**Implementation:**
- [ ] Review all 0-16383 parameters (14-bit)
- [ ] Consider 0-100 scaling for mix/level params
- [ ] Evaluate 0-127 (MIDI-style) for appropriate params
- [ ] Document rationale for each range choice
- [ ] Update help system with new ranges
- [ ] Add MAP operators if needed (MAP7, MAP14)
- [ ] Ensure backwards compatibility or migration path

### REPL/DEBUG Level Audit [Medium] - COMPLETE (December 2025)
Comprehensive tiered verbosity system with category overrides.

**Tier System (DEBUG 0-5):**
- [x] Tier 0: SILENT - Nothing (use category overrides)
- [x] Tier 1: ERRORS - All error messages
- [x] Tier 2: ESSENTIAL - State changes (scene, metro, recording)
- [x] Tier 3: QUERIES - Value read responses
- [x] Tier 4: CONFIRMS - Set confirmations
- [x] Tier 5: VERBOSE - All output (randomization, slew, etc.)

**Category Override Commands:**
- [x] `OUT.ERR <0|1>` - Override: show errors
- [x] `OUT.ESS <0|1>` - Override: show essential
- [x] `OUT.QRY <0|1>` - Override: show queries
- [x] `OUT.CFM <0|1>` - Override: show confirms

**REPL Management:**
- [x] `CLEAR` - Clear all REPL output
- [x] `REPL.DUMP [filename]` - Save REPL output to file (default: repl_dump.txt)

**Additional Fixes:**
- [x] Fixed 21 messages exceeding 46-char limit
- [x] Standardized message formats (ERROR:, SET...TO, etc.)
- [x] Routed meter/MIDI errors to REPL (was stderr)
- [x] All output calls tagged with categories
- [x] All 558 tests pass

### Global Error Handling Audit [Medium] - COMPLETE (December 2025)
Comprehensive error handling improvements implemented as part of REPL audit and test suite work.

**Infrastructure:**
- [x] Error reporting routed to REPL via MetroEvent::Error (meter/MIDI errors)
- [x] All output calls tagged with OutputCategory::Error
- [x] Consistent error message formatting (46-char limit compliance)

**Validation & Bounds Checking:**
- [x] Pattern operation bounds checking (patterns 0-5, indices 0-63)
- [x] Expression evaluation error messages (descriptive failures)
- [x] Negative value validation for all synth parameters
- [x] Bool parameter validation (0/1 only for BR.REV, PS.MODE, etc.)
- [x] DEL bounds enforcement (max 16000ms, count >= 1)
- [x] Extra argument rejection for zero-arg commands (TR, TOSS, RST, etc.)
- [x] SEQ syntax error messages (unclosed quote, empty pattern, etc.)

**Error Feedback:**
- [x] Uniformity between live REPL and script error feedback
- [x] File I/O error handling (SAVE/LOAD with proper messages)
- [x] MIDI connection error handling (routed to REPL)
- [x] Quote-aware semicolon parsing (`PRINT "hello;world"` works)

**Testing:**
- [x] Error test suite: 10 scenes covering ~80 error conditions
- [x] ~95% error test pass rate
- [x] All 558 unit tests pass

See `repl_tests/ERROR_TEST_REPORT.md` for comprehensive error handling documentation.

### Help Coverage Audit [Low] - COMPLETE (December 2025)
Update help system with all new commands and features.

- [x] Review all Phase 1-6 completed features
- [x] Add missing commands to help pages
- [x] Document all UI toggles (METER.HDR, METER.GRID, SPECTRUM, ACTIVITY, GRID, GRID.MODE, HL.SEQ, HL.COND, CPU)
- [x] Document HEADER levels 0-4 with CPU display option
- [x] Document DEBUG levels 0-2
- [x] Add NOTE and NOTE.CLR commands
- [x] Add FLASH command
- [x] Add CLEAR command
- [x] Document Alt+key navigation shortcuts (Alt+L, Alt+1-8, Alt+M, Alt+I, Alt+P, Alt+S, Alt+V, Alt+H)
- [x] Document Tab key for grid view toggle
- [x] Reorganize navigation documentation for clarity (function keys vs alt keys)
- [x] Document all SCOPE commands (SCOPE.TIME, SCOPE.CLR, SCOPE.MODE, SCOPE.UNI)
- [x] All SEQ notation examples already documented
- [x] All MIDI sync commands already documented
- [x] Help page navigation functional

**Files Modified:**
- `src/ui/pages/help_content.rs` - Added missing documentation for all Phase 6 features

### REPL Test Suite Issues [Medium] - COMPLETE (December 2025)
Address issues discovered during comprehensive REPL command testing (December 2025).

**Test Suites Created:**
- 12 test scenes covering all command categories
- 10 error test scenes covering ~80 error conditions
- Added `--run` batch mode for automated testing

**Findings Documents:**
- `repl_tests/TEST_FINDINGS.md` - Command behavior testing
- `repl_tests/ERROR_TEST_REPORT.md` - Validation testing

**Critical Validation Gaps (P0) - FIXED:**
- [x] Semicolon parsing breaks quoted strings: `PRINT "hello;world"` - Fixed with quote-aware splitting
- [x] Negative values not validated: `PF -1` - Fixed with range validation in synth macros
- [x] Fix: `src/app/script_exec/interactive.rs` and `src/app/script_exec/mod.rs`
- [x] Fix: All param handlers in `src/commands/synth/common.rs`

**High Priority Validation Gaps (P1) - FIXED:**
- [x] SEQ syntax errors not descriptive - Now shows specific messages (unclosed quote, empty pattern, etc.)
- [x] DEL bounds not enforced - Fixed: max 16000ms with proper error messages
- [x] DEL.X count validation - Fixed: count >= 1 required
- [x] DEL commands parsed as conditionals - Fixed: special handling before colon splitting
- [x] Bool params accept values > 1 - Fixed: BR.REV, PS.MODE, PS.TARG now validate 0/1
- [x] Pattern bounds bug in eval - Fixed: `pat <= 5` in `src/eval/patterns.rs`

**High Priority - Pattern Query Expressions - FIXED:**
- [x] P.N, P.L, P.I now valid in expressions (`A P.N`, `PRINT P.L`)
- [x] P.MIN, P.MAX, P.SUM, P.AVG, P.FND now expression-compatible
- [x] PN.MIN, PN.MAX, PN.SUM, PN.AVG, PN.FND for explicit patterns
- [x] Added to `src/eval/patterns.rs`

**Medium Priority - Query Commands - FIXED:**
- [x] `M.SCRIPT M` now parses ("M" recognized as script 8 alias)
- [x] Extra arguments now rejected: `TR 1`, `TOSS 1`, `P.HERE 1`, etc. show errors

**Low Priority - Validation Edge Cases:**
- [ ] Standardize REPL output formatting (some envelope params lack "SET...TO" prefix)
- [ ] Error message prefix standardization (some use "Error:", some use "ERROR:")

**Infrastructure Complete:**
- [x] REPL.DUMP now works in script context (was interactive-only)
- [x] Scene file validation (structural issues in test scenes)
- [x] Error test suite with 10 test scenes (now ~95% pass rate)
- [x] Batch mode `--run` for automated testing
- [x] All 558 unit tests pass

### File Size and DRY Audit [Medium]
Comprehensive audit to reduce parameter sprawl and improve maintainability.

**File Size Limits:**
- [ ] Ensure all files are within agent readable line limits (~500 lines ideal)
- [ ] Split large files into logical modules
- [ ] Audit src/commands/mod.rs (currently large dispatch file)

**Command Context Refactor:**
- [ ] Create `CommandContext` struct to bundle shared parameters:
  ```rust
  pub struct CommandContext {
      pub debug_level: u8,
      pub out_err: bool,
      pub out_ess: bool,
      pub out_qry: bool,
      pub out_cfm: bool,
      // future: osc_client ref, metro_state ref, etc.
  }
  ```
- [ ] Replace individual parameters in handler signatures with `&CommandContext`
- [ ] Update all call sites in process_command (one-time cost)
- [ ] Future additions become 1-line struct changes vs 100+ signature changes

**Smart Output Closure:**
- [ ] Create category-aware output helper:
  ```rust
  let output = |cat: OutputCategory, msg: String| {
      if ctx.should_output(cat) { raw_output(msg); }
  };
  ```
- [ ] Replace tier checks throughout codebase with `output(Confirm, msg)`
- [ ] Eliminates repeated `if debug_level >= TIER_X || out_x` pattern

**Macro Consolidation:**
- [ ] Audit synth param macros for shared patterns
- [ ] Consider trait-based approach for common handler patterns
- [ ] Reduce macro duplication in patterns/common.rs

**Estimated Impact:**
- Reduces future feature additions from hours to minutes
- Eliminates parameter sprawl across 100+ function signatures
- Single point of change for output control logic

### Mod Bus Parameter Cleanup [Low]
Clarify and deduplicate mod bus parameters for consistency.

- [ ] Audit MB, MF_F, MP, MD, MT, MA parameter naming
- [ ] Document mod bus routing clearly (source → destination)
- [ ] Remove any redundant or overlapping parameters
- [ ] Update help system with clearer mod bus explanation

### Script and Trigger Prioritization [Medium]
Define clear execution order to eliminate race conditions.

- [ ] Document current execution order (metro tick vs script calls vs TR)
- [ ] Define priority weighting for concurrent operations
- [ ] Ensure parameter changes apply before trigger fires
- [ ] Handle nested SCRIPT calls with predictable ordering
- [ ] Consider adding explicit priority control commands

### EITH Selection Highlighting [Low]
Fix visual feedback for EITH random choice operator.

- [ ] Track last selected value in EITH expressions
- [ ] Highlight selected option in script display (like TOG does)
- [ ] Ensure state persists correctly across evaluations

### Beat Repeat Stickiness Bug [Medium]
BR (beat repeat) does not turn off reliably when triggered conditionally.

- [ ] Diagnose why BR stays on after conditional trigger
- [ ] Check if BR.ACT 0 is being sent correctly
- [ ] Verify OSC message ordering (BR on/off vs other params)
- [ ] Test with explicit BR.ACT 0 in else branch
- [ ] May need latch/unlatch behavior clarification

### VCA Reset Coverage [Low]
Ensure VCA mode is included in reset commands.

- [ ] Add VCA to RST command (reset to default VCA=1 gated mode)
- [ ] Add VCA to LOAD.RST behavior
- [ ] Verify VCA state is restored correctly on scene load

### Script Undo/Redo [Medium]
Add undo/redo support for script editing.

- [ ] Track edit history per script page
- [ ] `Ctrl+Z` - Undo last edit
- [ ] `Ctrl+Shift+Z` or `Ctrl+Y` - Redo
- [ ] Reasonable history depth (10-20 edits)
- [ ] Clear history on page change or save

### List Output Formatting [Low]
Ensure list queries display vertically, not in overflowing single lines.

- [ ] SCENES - Display one scene per line
- [ ] PSETS / PRESETS - Display one preset per line
- [ ] THEMES - Display one theme per line (or columnar)
- [ ] MIDI.IN (device list) - One device per line
- [ ] AUDIO.OUT (device list) - One device per line
- [ ] Respect 46-char width constraint

### NR and ER Operators [Medium]
Add Teletype-style numeric repetitor and Euclidean rhythm operators.

- [ ] `NR <val> <count>` - Numeric repetitor (repeat val count times, then advance)
- [ ] `ER <fills> <length> [offset]` - Euclidean rhythm generator
- [ ] ER returns 1 or 0 for current step in pattern
- [ ] Per-script/per-line state tracking (like TOG/SEQ)
- [ ] Document Euclidean algorithm implementation

---

## Phase 6: Release Preparation

**Focus:** Pre-release tasks for first public release

### Terminal Compatibility [Medium] - PHASE 1 COMPLETE
Ensure usability across different terminal emulators.

**Phase 1 (Complete):**
- [x] Terminal capability detection at startup
- [x] 256-color theme fallback when true color unavailable
- [x] Startup warning for Terminal.app users
- [x] COMPAT command to display terminal capabilities
- [x] COMPAT.MODE command to force compatibility mode
- [x] METER.ASCII command for ASCII-only meter characters
- [x] High-contrast cursor fallback for 256-color mode

**Phase 2 (v0.2.0):**
- [ ] Add ANSI 16-color fallback themes
- [ ] Character set fallback for scope (ASCII mode)
- [ ] Config option: `compatibility_mode = "auto"|"full"|"basic"`
- [ ] Terminal.app user guide

**Phase 3 (v0.3.0):**
- [ ] Font recommendations at startup
- [ ] Auto theme selection based on terminal
- [ ] VT100 alternate charset support

### Release Build & Tag [High] - COMPLETE
Automated versioned release process.

- [x] Ensure all tests pass (`cargo test`)
- [x] Build release binary (`cargo build --release`)
- [x] Create version tag (`git tag -a v0.1.0`)
- [x] Push tag to origin
- [x] Build platform-specific tarballs with bundled resources
- [x] Generate sha256 checksums for each tarball
- [x] Automated via `.github/workflows/release.yml`

### GitHub Release [Medium] - COMPLETE
Automated release publishing on GitHub.

- [x] Create GitHub release for tagged version
- [x] Upload platform binaries (aarch64-apple-darwin)
- [x] Write release notes documenting features
- [x] Link to documentation
- [x] Automated via `.github/workflows/release.yml`

### Homebrew Tap [High] - COMPLETE
Pre-built bundle distribution for macOS users.

- [x] Create `homebrew-monokit` repository
- [x] Write Formula/monokit.rb with:
  - Platform-specific URLs and sha256 hashes
  - No external dependencies (bundled scsynth)
  - Proper libexec installation with symlink
  - User config path documentation
- [x] Test installation via `brew tap` and `brew install`
- [x] Automated formula updates via `.github/workflows/release.yml`
- [x] Symlink resolution in `src/scsynth_direct.rs` via `get_exe_dir()`

### Future Release Workflow
Document repeatable release process.

- [ ] Write RELEASING.md with step-by-step instructions
- [ ] Version bump in Cargo.toml
- [ ] Changelog maintenance process
- [ ] Binary build automation (optional GitHub Actions)

### Remove sc3-plugins Dependency [Medium]
Rewrite SC server to use only built-in UGens for easier installation.

- [ ] Replace SVF filter with built-in alternatives (RLPF, RHPF, BPF, etc.)
- [ ] Audit server for other sc3-plugins dependencies
- [ ] Test sound parity with original
- [ ] Update documentation

### Direct scsynth Integration [High] - COMPLETE (December 2025)
Bundle scsynth binary directly, eliminating sclang and full SuperCollider dependency.

**Benefits:**
- Single binary distribution (~13 MB bundle vs ~200 MB SC install)
- No user-facing SuperCollider/sc3-plugins installation required
- Faster startup (no sclang interpretation layer)
- Follows Sonic Pi's proven approach

**Completed Implementation Phases:**

Phase 1 - SynthDef Pre-compilation: COMPLETE
- [x] Create build script to compile SynthDefs to .scsyndef files
- [x] Add to build system (build.rs via `sc_compile` feature)
- [x] Store compiled .scsyndef files in sc/synthdefs/

Phase 2 - Direct scsynth Spawning: COMPLETE
- [x] Modify sc_process.rs to spawn scsynth instead of sclang
- [x] Implement OSC boot sequence (/notify → /d_load → /s_new)
- [x] Handle scsynth command-line args (-u port, -U plugins, -D device)
- [x] Add scsynth path discovery (bundled, homebrew, system)
- [x] Renamed gate parameter to t_gate (TrigControl) for reliable triggering

Phase 3 - OSC Message Routing: COMPLETE
- [x] Rework meter/spectrum/scope data flow via /reply messages
- [x] Handle SendReply routing from scsynth to notify client
- [x] All visualization data flows correctly to Rust

Phase 4 - Recording Without sclang: COMPLETE
- [x] Implement recording via DiskOut UGen
- [x] Handle buffer allocation for recording
- [x] Maintain feature parity with sclang-based recording
- [x] Bundle DiskIO_UGens.scx plugin (required for DiskOut)
- [x] Upgrade to scsynth 3.14.1 (fixes CoreAudio input query bug)
- [x] Bundle libsndfile, libfftw3f, libreadline dylibs
- [x] Use /b_write with leaveOpen=1 for streaming writes
- [x] Output: 24-bit stereo WAV @ 48kHz to current directory
- [x] Sequential file naming: monokit_audio_N.wav

Phase 5 - Audio Device Handling: COMPLETE
- [x] Map device names to scsynth device indices
- [x] Query available devices via scsynth or system APIs
- [x] Handle device switching gracefully

Phase 6 - Bundling & Distribution: COMPLETE
- [x] Bundle scsynth binary from SuperCollider.app
- [x] Bundle required .scx plugins only
- [x] Created bundle.sh script with plugin search
- [x] Documented Homebrew formula changes
- [x] Updated path resolution to prioritize bundled resources
- [x] Bundle structure tested with Resources/ subdirectory
- [x] Handle macOS code signing / Gatekeeper (documented)
- [x] Test bundle on clean system (requires sc3-plugins installation)

**Implementation Summary:**
- Feature gate: `cargo build --features scsynth-direct`
- Conditional compilation in src/sc_process.rs
- New module: src/scsynth_direct.rs with boot sequence
- Audio devices: src/audio_devices.rs for device enumeration
- Bundle shell script: scripts/bundle.sh for packaging
- Complete OSC message flow from Rust to scsynth and back

**Effort estimate completed:** 6-8 weeks
**Risk assessment:** Low - proven approach, solid implementation
**Status:** Fully functional, ready for distribution

See: `docs/scsynth_direct_integration.md` for detailed implementation guide

---

## Phase 7: Advanced DSP

**Focus:** Major architectural changes requiring deep SuperCollider work

### Noise Source Integration [Medium]
Add multi-colored noise generator before filter and amp stages.

- [ ] Add noise oscillator to voice (white, pink, brown/red, blue, violet)
- [ ] `NS` / `NOISE` - Noise level (mix amount into signal path)
- [ ] `NS.CLR` / `NOISE.CLR` - Noise color (0=white, 1=pink, 2=brown, 3=blue, 4=violet)
- [ ] Insert point: after oscillator mix, before filter
- [ ] Envelope option: noise follows amp envelope or constant level
- [ ] Update SynthDef with noise UGens (WhiteNoise, PinkNoise, BrownNoise, etc.)

### Oscillator Sync [Medium]
Add hard sync between primary and modulator oscillators.

- [ ] `SYNC <0|1>` - Enable/disable oscillator sync
- [ ] Primary oscillator resets phase on modulator zero-crossing
- [ ] Classic sync sweep sound when modulator frequency changes
- [ ] Update SynthDef with sync logic (Sync or manual phase reset)

### Additional Filter Types [Medium]
Expand filter options beyond SVF.

- [ ] `FT` / `FILT.TYPE` extended modes:
  - Current: 0=LP, 1=HP, 2=BP, 3=Notch
  - Add: 4=Ladder (Moog-style 24dB/oct)
  - Add: 5=Formant (vowel filter)
  - Add: 6=Comb (as filter, not resonator)
- [ ] Consider separate filter UGens or multi-mode SynthDef
- [ ] Maintain filter envelope compatibility across types

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

### Sample Playback System [Very High]
- [ ] `S.LOAD <path>` - Load sample from file (WAV/AIFF)
- [ ] `S.BANK <0-N>` - Select sample bank/folder
- [ ] `S.SEL <0-N>` - Select sample within bank
- [ ] `S.SLICE <n>` - Set number of slices (auto-divide sample)
- [ ] `S.IDX <0-N>` - Select slice index for playback
- [ ] `S.START <0-1>` - Manual start point (normalized)
- [ ] `S.END <0-1>` - Manual end point (normalized)
- [ ] `S.DIR <-1|0|1>` - Playback direction (reverse/pause/forward)
- [ ] `S.RATE <0.1-4>` - Playback rate/pitch
- [ ] `S.PITCH <semitones>` - Pitch shift independent of rate
- [ ] `S.LOOP <0|1>` - Loop mode
- [ ] `S.TRIG` - Trigger sample playback
- [ ] Buffer management in SuperCollider
- [ ] Sample browser/indexing system
- [ ] Integration with pattern system: `S.IDX PN.NEXT 0`

---

## Phase 8: Distribution

**Focus:** Packaging and deployment infrastructure (post-release expansion)

### Cross-Platform Compatibility [High]
Expand beyond Apple Silicon macOS.

**macOS Intel (x86_64):**
- [ ] Add x86_64-apple-darwin target to release workflow
- [ ] Bundle Intel scsynth binary
- [ ] Test on Intel Mac hardware
- [ ] Universal binary option (fat binary)

**Linux:**
- [ ] Linux build target (x86_64-unknown-linux-gnu)
- [ ] Bundle or document scsynth installation
- [ ] Handle audio backend differences (JACK, PulseAudio, PipeWire)
- [ ] AppImage or Flatpak packaging
- [ ] Test on common distros (Ubuntu, Fedora, Arch)

**Windows:**
- [ ] Windows build target (x86_64-pc-windows-msvc)
- [ ] Bundle scsynth for Windows
- [ ] Handle Windows audio APIs (WASAPI, ASIO)
- [ ] Portable .exe or MSI installer
- [ ] Terminal emulator recommendations (Windows Terminal)

### Unified Installer [High]
- [ ] Single installer package bundling:
  - Rust CLI binary
  - SuperCollider runtime (scsynth)
  - SC SynthDefs
  - Default config/themes
- [ ] Platform-specific installers:
  - macOS: .pkg or Homebrew formula (COMPLETE for ARM)
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
- **Phase 4** UI features can be implemented incrementally, no blocking dependencies
- **Phase 5** polish items should be implemented after core features complete (helps identify what needs polish)
- **Phase 6** release preparation should happen after core features stable
- **Phase 7** advanced DSP requires stable release before major voice architecture changes
- **Phase 8** unified installer builds on Phase 6 release infrastructure

**Note:** Modulation System (LFO, Aux Envelopes) moved to `ON_HOLD.md` due to SuperCollider UGen complexity limits.

### Complexity Legend
- **[Low]** - 1-3 days, minimal dependencies, straightforward implementation
- **[Medium]** - 1-2 weeks, moderate complexity, some new infrastructure
- **[High]** - 2-4 weeks, significant new systems, external dependencies
- **[Very High]** - 4+ weeks, major architectural changes, deep domain expertise required

---

## Contributing

Feature requests and suggestions welcome. All contributions should maintain the project's terse command syntax and CLI-native philosophy.
