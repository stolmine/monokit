# Monokit Architecture

Project structure, key files, and technical documentation.

---

## Documentation Index

### User Documentation
- **MANUAL.md** - Comprehensive user manual with command reference and tutorials
- **README.md** - Project overview, installation instructions, feature rundown

### Development Documentation
- **ROADMAP.md** - Current priorities and v0.3.4 progress
- **CHANGELOG.md** - Version history and recent updates
- **CONCEPT.md** - Project overview, architecture, MVP implementation
- **PLAN.md** - UI refactor plan: Teletype-style interface

### Release & Distribution
- **RELEASE_PIPELINE.md** - Automated release workflow
- **HOMEBREW_BUNDLE_FORMULA.md** - Homebrew formula documentation
- **BUNDLE_QUICK_START.md** - Quick start guide for bundle distribution

### Technical Specifications
- **scsynth_direct_integration.md** - Direct scsynth integration design
- **DSP_TIER1_IMPLEMENTATION_PLAN.md** - Filter, Resonator, Delay, Reverb
- **DSP_TIER3_BUFFER_EFFECTS_PLAN.md** - Beat Repeat and Pitch Shift
- **EFFECT_ROUTING_DESIGN.md** - Effect routing system
- **DRY_REFACTOR_PLAN.md** - Codebase refactoring plan
- **UI_REFINEMENT_PLAN.md** - UI enhancements
- **MIDI_CLOCK_TIMING_LESSONS.md** - MIDI clock sync diagnostics

### History
- **history/CHANGELOG.md** - Detailed completion records
- **history/PHASES.md** - Development phases 1-6
- **history/FUTURE.md** - Phase 7-8 plans

### Deferred Features
- **ON_HOLD.md** - Features deferred due to technical constraints

---

## Key Project Files

### Configuration
- **Cargo.toml** - Rust project manifest
- **Cargo.lock** - Dependency lock file

### Source Code Structure

Modular Rust implementation (~17,300 lines across 93 files):

```
src/
├── main.rs (69 lines) - Entry point
├── metro.rs (112 lines) - Metro thread with absolute timing
├── types.rs (233 lines) - Core data structures
├── eval/ (~720 lines) - Expression evaluation
│   ├── mod.rs - Core dispatch
│   ├── patterns.rs - Pattern expressions
│   ├── math.rs - Math operators
│   └── logic.rs - Comparisons, RND, TOG, MAP
├── ui/ (~1,370 lines) - TUI rendering
│   ├── mod.rs - Module coordinator
│   ├── header.rs, footer.rs
│   └── pages/ (7 files)
├── app/ (~750 lines) - Application logic
│   ├── mod.rs - App struct, navigation
│   ├── input.rs - Input handling
│   └── script_exec/ - Script execution
├── commands/ (~8,167 lines) - Command processing
│   ├── mod.rs - Main dispatcher
│   ├── validate.rs - Command validation
│   ├── aliases.rs - 93 aliases
│   ├── core/ - Language primitives
│   ├── patterns/ - Pattern operations
│   ├── system/ - System commands
│   └── synth/ - Synth parameters
│       └── envelopes/ - Envelope handlers
└── tests/ (~5,288 lines, 558 tests)
```

### SuperCollider
- **sc/monokit_server.scd** - Multi-synth sound engine
  - 5-synth architecture with audio bus routing
  - 97 total parameters across all voices
  - See Voice Architecture diagram below for signal flow

---

## Voice Architecture

Monokit uses a multi-synth architecture with separate SynthDefs for each voice, communicating via audio buses:

```
┌─────────────────────────────────────────────────────────────┐
│                   RUST CLI (Command Layer)                  │
│  Commands: TR, PLTR, PF, PLF, PLH, etc.                    │
└────────────────────┬────────────────────────────────────────┘
                     │ OSC Messages (/n_set)
                     │ Node Routing by Parameter
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              SUPERCOLLIDER (Audio Engine)                    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ monokit_noise│  │ monokit_mod  │  │monokit_primary│     │
│  │  (Node 1002) │  │  (Node 1003) │  │  (Node 1004)  │     │
│  ├──────────────┤  ├──────────────┤  ├──────────────┤     │
│  │ Params:      │  │ Params:      │  │ Params:       │     │
│  │  NW, NV      │  │  MF, MW, MV  │  │  PF, PW, PV   │     │
│  │              │  │  FB, FBA,FBD │  │  FM, FA, FD   │     │
│  │ [Noise Gen]  │  │              │  │  PA, PD       │     │
│  │      │       │  │ [FM Osc]     │  │  DC, DM, DD   │     │
│  │      ▼       │  │      │       │  │  TK           │     │
│  │  Out→Bus 18  │  │      ▼       │  │              │     │
│  └──────────────┘  │  Out→Bus 17  │  │ [Complex Osc]│     │
│                    └──────────────┘  │      │       │     │
│                                      │      ▼       │     │
│  ┌──────────────┐                    │  Out→Bus 16  │     │
│  │monokit_plaits│                    └──────────────┘     │
│  │  (Node 1005) │                                          │
│  ├──────────────┤                                          │
│  │ Params:      │                                          │
│  │  PLF, PLE    │                                          │
│  │  PLH, PLT    │                                          │
│  │  PLM, PLD    │                                          │
│  │  PLL, PLV    │                                          │
│  │              │                                          │
│  │ [MiPlaits]   │                                          │
│  │  Main │ AUX  │                                          │
│  │    ▼     ▼   │                                          │
│  │ Out→19  Out→20                                          │
│  └──────────────┘                                          │
│          │   │                                             │
│  ┌───────┴───┴────────────────────────────────────┐       │
│  │           monokit_main (Node 1006)             │       │
│  ├────────────────────────────────────────────────┤       │
│  │ In: Bus 16 (Primary) + Bus 17 (Mod) +         │       │
│  │     Bus 18 (Noise) + Bus 19/20 (Plaits)       │       │
│  │                                                 │       │
│  │ Signal Chain:                                  │       │
│  │  Mix Sources → Discontinuity → Lo-Fi →        │       │
│  │  Filter (14 types) → Ring Mod → Resonator →   │       │
│  │  VCA/Envelope → Compressor → Pan →            │       │
│  │  Beat Repeat → Pitch Shift → Stereo Delay →  │       │
│  │  3-Band EQ → Plate Reverb                     │       │
│  │                                                 │       │
│  │ Params: FC, FQ, FT, DT, DF, RV, etc. (60+)   │       │
│  │                      │                         │       │
│  │                      ▼                         │       │
│  │                 Out→ Audio                     │       │
│  └────────────────────────────────────────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘

Trigger Commands:
  TR   → Triggers monokit_noise, monokit_mod, monokit_primary, monokit_main
  PLTR → Triggers monokit_plaits only

UI Indicators:
  C → Complex oscillators active (TR triggered)
  P → Plaits voice active (PLTR triggered)
```

### Voice Parameter Routing

Parameters are automatically routed to the correct synth node based on name:

| Parameter Pattern | Target Node | Voice |
|------------------|-------------|-------|
| `NW`, `NV` | Node 1002 | Noise generator |
| `MF`, `MW`, `MV`, `FB*`, `MB*` | Node 1003 | Modulator oscillator |
| `PF`, `PW`, `PV`, `FM`, `FA`, `PA`, `DC`, `DM`, `TK` | Node 1004 | Primary oscillator |
| `pitch`, `detune`, `PL*`, `PLV`, `PAV` | Node 1005 | Plaits voice |
| All others (filters, effects, etc.) | Node 1006 | Main signal path |

This architecture allows:
- Independent voice control and triggering
- Isolated parameter spaces (no cross-talk)
- Parallel voice development
- Multiple voices active simultaneously
- Efficient CPU usage (voices only process when needed)

---

## Architecture Overview

```
Rust TUI (src/main.rs)
    |
    +-- Main Thread
    |    - TUI rendering (ratatui)
    |    - Command processing
    |    - Script execution
    |    - OSC → 127.0.0.1:57120
    |
    +-- Metro Thread (absolute timing)
         - ExecuteScript events to main thread

SuperCollider Sound Engine (sc/monokit_server.scd)
    |
    v
Audio output → Recording (optional)
```

---

## DRY Refactoring Summary

**Total Reduction: ~6,742 lines (31% of original codebase)**

### Completed Phases

**Phase 0: Codebase Reorganization**
- Created `core/`, `system/`, `synth/` directory structure
- Moved command handlers to logical domains

**Phase 1: Envelope Handler DRY**
- ~1,141 lines → 223 lines (81% decrease)
- Created `define_int_param!` and `define_float_param!` macros

**Phase 2: Pattern Operation DRY**
- Wrapper code: 2023 → 450 lines (78% decrease)
- 10 macros generate both P.* and PN.* handlers

**Phase 3: Synth Parameter DRY**
- ~2,325 lines removed

**Phase 4: Variables, Counters, Test Fixtures**
- ~1,126 lines removed

**Phase 5: Additional DRY Refactors**
- Boolean toggle macro, Integer enum macro
- ~800+ lines removed

---

## Script Validation System

**Status: COMPLETE (December 2025)**

Comprehensive validation prevents invalid scripts from being saved:

1. **Bypass Paths Closed** - All entry paths run validation
2. **Pattern Operations** - 32 operations (16 P.*, 16 PN.*)
3. **Expression Validation** - Syntax validation without evaluation
4. **Control Flow** - Loop, conditional, DEL command validation
5. **SEQ Pattern Content** - Bracket balancing, token verification
6. **Reference Ranges** - Pattern 0-5, Script 1-8/M/I validation

### Test Coverage
- 9 comprehensive test scenes
- All tests pass with validation enabled

---

## REPL Test Scenes

**Location:** `repl_tests/`

- 12 test scene files (.json)
- 12 dump files (.txt) for verification
- TEST_FINDINGS.md - Detailed analysis
- ERROR_TEST_REPORT.md - Validation testing

---

## Key Features

- **Page-based interface:** Live, Script 1-8, Metro (M), Init (I), Pattern (P), Help
- **Script storage:** 10 scripts × 8 lines
- **Pattern storage:** 6 patterns × 64 steps (i16 values)
- **Variables:** A-D, X-Y-Z-T (global), J-K (per-script), I (loop counter)
- **Control flow:** IF/ELIF/ELSE, PROB, EV/SKIP, L loops, BRK
- **Comparison operators:** EZ, NZ, EQ, NE, GT, LT, GTE, LTE
- **N operator:** Semitone to frequency (N 0 = C3 = 131 Hz)
- **Expression evaluation** in all numeric arguments
- **OSC client:** 127.0.0.1:57120 to SuperCollider
