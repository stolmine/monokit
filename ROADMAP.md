# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## Recent Updates (December 2025)

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
- [x] All 411 tests pass

**Phase 2: Pattern Operation DRY** - COMPLETE (November 2025)
- [x] Created `patterns/common.rs` (902 lines) with `PatternRef` enum, shared implementations, and macro system
- [x] Aggressive macro approach: 10 macros generate both P.* and PN.* handlers from single definitions
- [x] Unified P.* (working) and PN.* (explicit) operations via PatternRef::Working/Explicit
- [x] Wrapper code reduced from 2023 → 450 lines (**1,573 line reduction, 78% decrease**)
- [x] Explicit files now just re-export from working files (~10 lines each)
- [x] All 411 tests pass

**Phase 3: Synth Parameter DRY** - COMPLETE (November 2025)
- [x] Created `synth/param_macro.rs` with generic parameter macros
- [x] Consolidated 70+ similar parameter handlers
- [x] Line reduction: **~2,325 lines**
- [x] All 411 tests pass

**Phase 4: Variables, Counters, and Test Fixtures** - COMPLETE (November 2025)
- [x] Phase 4A: Variable/Counter macros - **489 lines removed**
- [x] Phase 4B: Expression helpers - Infrastructure added
- [x] Phase 4C: Test fixture optimization - **637 lines removed**
- [x] Total Phase 4: **~1,126 lines removed**
- [x] All 411 tests pass

**Program Completion Summary:**
- **Total DRY Reduction (Phases 1-4): ~5,942 lines (28% of original codebase)**
- Clear, logical file organization by domain
- Easier to add new commands (single macro invocation)
- All 411 tests continue to pass throughout
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
- [x] User presets stored in `~/.monokit/presets/`

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

**MIDI Clock Output:** - ON HOLD INDEFINITELY
- [ ] `M.SEND <0|1>` - Send MIDI clock out
- [ ] Send start/stop/continue messages

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

## Phase 4: Modulation System - ON HOLD

**Status:** ON HOLD INDEFINITELY - SuperCollider UGen complexity limits

**Issue:** SuperCollider SynthDef complexity limits prevent implementation of freely routable LFO destinations. Extensive testing (December 2025) showed:
- 2 LFOs × 5 destinations worked initially
- 3-4 LFOs × 7+ destinations hit UGen limits
- Architectural attempts (control bus separation, split routers) all failed
- The InRange.kr routing matrix creates too many UGens regardless of synth splitting

**Attempted Solutions:**
- Control bus architecture (LFO generation separate from main synth)
- Pre-computed destination routing in separate synth
- Split lfo_bank into lfo_gen + lfo_router
- Split router into two 8-destination synths (lfo_router_a + lfo_router_b)
- All approaches hit SC's internal optimization/complexity limits

**Conclusion:** Freely routable multi-LFO modulation is not feasible within SuperCollider's SynthDef architecture for a synth of monokit's complexity. Would require either:
- Fixed LFO→destination assignments (not user-routable)
- Significant reduction in main synth features
- Alternative audio engine (not SC)

---

### Aux Envelope System [High] - ON HOLD
Flexible auxiliary envelope that can be routed to any synth parameter.
Same routing complexity issues as LFO system.

- [ ] `XENV.DEC <ms>` - Aux envelope decay time
- [ ] `XENV.ATK <ms>` - Aux envelope attack time
- [ ] `XENV.CRV <-8 to 8>` - Aux envelope curve
- [ ] `XENV.AMT <0-16383>` - Aux envelope amount
- [ ] `XENV.DEST <param>` - Set destination parameter (e.g., `XENV.DEST FC`)
- [ ] Multiple destinations support (optional)
- [ ] SC implementation: New envelope with routing matrix

### Extended Envelope Coverage [Medium] - ON HOLD
Dedicated envelopes for synth/FX parameters currently lacking envelope control.
Blocked by same SC complexity constraints.

**Lo-Fi Effect:**
- [ ] `LOEV.DEC <ms>` - Lo-Fi envelope decay
- [ ] `LOEV.ATK <ms>` - Lo-Fi envelope attack
- [ ] `LOEV.CRV <-8 to 8>` - Lo-Fi envelope curve
- [ ] `LOEV.AMT <0-16383>` - Lo-Fi envelope amount (modulates LM mix)

**Other candidates (to be evaluated):**
- [ ] Ring mod envelope (RGM mix)
- [ ] Resonator envelope (RM mix)
- [ ] Delay envelope (DW wet)
- [ ] Reverb envelope (RW wet)

### LFO System [High] - ON HOLD
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

**Reference:** See `UI_REFINEMENT_PLAN.md` for detailed implementation guide.

### Phase 5.1: Activity Indicators [Medium] - COMPLETE (December 2025)
Script and metro execution feedback with decay animations (KO II style).

- [x] Add activity tracking to App struct (activity_last_active, activity_hold_ms)
- [x] Smooth color decay using cubic ease-out interpolation
- [x] Header shows script indicators (1-8, M, I) with color decay
- [x] Metro tick pulses M indicator
- [x] TR command pulses trigger indicator
- [x] FLASH command to adjust hold time (default 200ms)
- [x] Works for nested SCRIPT calls from metro
- [x] Theme-aware activity_color() in theme.rs

### Phase 5.2: SEQ/TOG State Highlighting [Medium] - COMPLETE (December 2025)
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

### Phase 5.3: Variables Page [Medium] - COMPLETE
Dedicated page showing all variable state (Teletype-style monitor).

- [x] Add `Page::Variables` to page enum
- [x] Create `src/ui/pages/variables.rs`
- [x] Display global vars: A, B, C, D, X, Y, Z, T
- [x] Display counters: N1-N4 with min/max bounds
- [x] Display per-script locals: J, K for all 10 scripts
- [ ] Optional: highlight recently-changed values

### Phase 5.4: Parameter Activity Grid [Medium] - COMPLETE
Alternate grid view on Live page showing parameter activity with unicode icons.

- [x] Add `ParamActivity` struct with per-parameter timestamps
- [x] Mark activity when parameters change in script execution and REPL
- [x] Tab keybinding to toggle between REPL and Grid view on Live page
- [x] 8x6 grid of 48 unicode icons representing synth parameters
- [x] Icons light up and decay when parameters change (reuses activity_color())
- [x] Grid center-justified with 3-space gaps between icons
- [x] Same decay timing as script indicators

### Phase 5.5: Audio Metering [High] - COMPLETE
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

### Phase 5.6: Notes Page [Medium] - COMPLETE (December 2025)
Dedicated notes page with command-based text entry and scene integration.

- [x] Redesigned to use 8 fixed lines like script pages
- [x] Line navigation with Up/Down arrows
- [x] Same editing commands: Ctrl+D/K/C/X/V
- [x] `NOTE "text"` - Append quoted text to Notes page (error if all 8 lines full)
- [x] `NOTE.CLR` - Clear all notes
- [x] Notes saved and loaded with scenes
- [x] Consistent with script page UX
- [x] Added to validator for script execution

### Phase 5.7: Conditional Execution Highlighting [Medium] - COMPLETE (December 2025)
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

### Phase 5.8: Scope Page [Medium] - COMPLETE (December 2025)
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

## Phase 6: Polish & Refinements

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

### Initial Window Sizing [Low] - ON HOLD INDEFINITELY (UNENFORCEABLE)
Provide terminal window sizing defaults/recommendations.

- [ ] Document recommended terminal size (50x18 minimum)
- [ ] Detect current terminal size on startup
- [ ] Warn if terminal too small (< 50x18)
- [ ] Optional: Set terminal size via ANSI escape sequences (if supported)
- [ ] Add to README/docs as setup requirement

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
- [x] All 527 tests pass

### Audio Device Selection [High]
Add runtime command to choose SuperCollider audio output device.

- [ ] `AUDIO.OUT` - List available audio output devices
- [ ] `AUDIO.OUT <device>` - Select audio output device
- [ ] `AUDIO.IN` - List available audio input devices (if needed)
- [ ] `AUDIO.IN <device>` - Select audio input device (if needed)
- [ ] Query SC for available devices via OSC
- [ ] Restart audio subsystem on device change
- [ ] Persist selection to config.toml
- [ ] Handle device not available gracefully

### Global Error Handling Audit [Medium]
Review codebase for silent failures, add proper error reporting.

**DRY Approach:**
- [ ] Create `report_error(ctx, msg)` helper that respects OutputCategory::Error
- [ ] Macro for common error patterns: `try_or_error!(expr, "MSG")`
- [ ] Centralized error message constants (avoid string duplication)

**Implementation:**
- [ ] Audit all `.unwrap()` calls
- [ ] Replace panics with proper error messages
- [ ] Add error reporting to REPL (partially done via MetroEvent::Error)
- [ ] File I/O error handling (SAVE/LOAD)
- [ ] OSC communication error handling
- [ ] MIDI connection error handling
- [ ] Pattern operation bounds and args checking
- [ ] Expression evaluation error messages
- [ ] Uniformity between live and script error feedback
- [ ] Illegality enforcement and validation

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
- [x] All 527 unit tests pass

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

---

## Phase 7: Advanced DSP

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
- **Phase 6** polish items should be implemented after core features complete (helps identify what needs polish)
- **Phase 7** requires all features complete and stable before voice architecture changes
- **Phase 8** requires all features complete and tested before packaging

### Complexity Legend
- **[Low]** - 1-3 days, minimal dependencies, straightforward implementation
- **[Medium]** - 1-2 weeks, moderate complexity, some new infrastructure
- **[High]** - 2-4 weeks, significant new systems, external dependencies
- **[Very High]** - 4+ weeks, major architectural changes, deep domain expertise required

---

## Contributing

Feature requests and suggestions welcome. All contributions should maintain the project's terse command syntax and CLI-native philosophy.
