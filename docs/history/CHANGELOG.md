# Monokit Changelog

Detailed record of completed features organized by date and category.

---

## December 2025

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

### VCA Default Mode Changed [COMPLETE]
Changed VCA default from 0 (drone) to 1 (gate) for more intuitive behavior.

### AUDIO.OUT Direct Mode Support [COMPLETE]
AUDIO.OUT command now works in scsynth-direct mode with graceful device switching.

### TUI/Startup Timing Fixed [COMPLETE]
Fixed timing so startup prints complete before TUI renders.

### t_gate Trigger Implementation [COMPLETE]
Reliable triggering in gate mode using TrigControl.

### Error Display Consistency [COMPLETE]
Unified error formatting across REPL and script views.

### Negative Number Handling [COMPLETE]
Prevent crashes from negative values in expressions.

### SEQ Highlight Fix [COMPLETE]
Fixed highlighting for commands with semicolons after quoted strings.

### Command Validation & Error Display Fixes [COMPLETE]
Comprehensive command validation audit (29 missing commands fixed).

### Global Search Feature [COMPLETE]
Unified search functionality with Ctrl+F, isolated scopes for help/scripts.

### VCA Mode Command [COMPLETE]
VCA envelope mode control with DRONE and GATED modes.

### TITLE.TIMER Command [COMPLETE]
Auto-cycle header title between "MONOKIT" and scene name.

### Terminal Compatibility System [COMPLETE]
Automatic detection and fallback for limited terminal capabilities.

### SCOPE.CLR Color Labels [COMPLETE]
Enhanced scope color selection with named colors.

### REPL Output Consistency [COMPLETE]
UI/settings commands always display regardless of DEBUG level.

### Scramble Animation System [COMPLETE]
Rolling text scramble animation for header with comprehensive controls.

---

## November 2025

### Envelope System Simplification
Envelope system refactored to use simple `Env.perc` with controllable attack and curve parameters.

**Removed Commands:**
- `ENV.ATK`, `ENV.DEC`, `ENV.CRV`, `ENV.MODE` - Global envelope controls
- `GATE` - Global gate duration
- `*.MODE`, `*.GATE` - Per-envelope mode and gate overrides

**Current Envelope Commands:**

| Envelope | Decay | Amount | Attack | Curve |
|----------|-------|--------|--------|-------|
| Amp | AD | - | AENV.ATK | AENV.CRV |
| Pitch | PD | PA | PENV.ATK | PENV.CRV |
| FM | FD | FA | FMEV.ATK | FMEV.CRV |
| Disc | DD | DA | DENV.ATK | DENV.CRV |
| Feedback | FBD | FBA/FBEV.AMT | FBEV.ATK | FBEV.CRV |
| Filter | FED | FE | FLEV.ATK | FLEV.CRV |

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
- [x] Pattern system: 6 patterns × 64 steps with comprehensive operations
- [x] Variables: A, B, C, D, X, Y, Z, T (global), I (loop), J, K (per-script)
- [x] Control flow: IF/ELIF/ELSE, L (loop), BRK (break), PROB, EVERY, SKIP with PRE separator (`:`)
- [x] Comparison operators: EQ, NE, GT, LT, GTE, LTE, EZ, NZ (both prefix and infix)
- [x] Sub-command separator: `;` for multiple commands per line
- [x] Scene persistence: SAVE/LOAD system for scripts + patterns
- [x] MAP operator: Range mapping with clamping
- [x] TOG generator: Toggle between two values on each trigger
- [x] N1-N4 counters: Auto-increment variables with MIN/MAX/RST control

### Modulation & Routing
- [x] ModBus routing to filter cutoff (MC parameter)
- [x] ModBus routing to filter resonance (MQ parameter)
- [x] Envelope system with PREFIX.SUFFIX naming (AENV, PENV, FMEV, DENV, FBEV, FLEV)
- [x] Per-envelope control: ATK, CRV for each envelope type
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

## DRY Refactoring Summary (November 2025)

**Total Reduction: ~6,742 lines (31% of original codebase)**

### Phase 0: Codebase Reorganization
- Created `core/`, `system/`, `synth/` directory structure
- Moved command handlers to logical domains

### Phase 1: Envelope Handler DRY
- Line reduction: ~1,141 lines → 223 lines (81% decrease)

### Phase 2: Pattern Operation DRY
- Wrapper code reduced from 2023 → 450 lines (78% decrease)

### Phase 3: Synth Parameter DRY
- Line reduction: ~2,325 lines

### Phase 4: Variables, Counters, Test Fixtures
- Total: ~1,126 lines removed

### Phase 5: Additional DRY Refactors
- Boolean toggle macro, Integer enum macro, F-key handler loop
- Total: ~800+ lines
