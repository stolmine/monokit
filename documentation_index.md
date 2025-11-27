# Monokit Documentation Index

## Documentation

- **CONCEPT.md** - Project overview, architecture, MVP implementation, and roadmap
- **PLAN.md** - UI refactor plan: Teletype-style interface with page system, script storage, patterns, and control flow
- **documentation_index.md** - This file, listing all documentation and key project files

## Key Project Files

### Configuration

- **Cargo.toml** - Rust project manifest with dependencies (rosc, ratatui, crossterm, nom, anyhow, thiserror, serde)
- **Cargo.lock** - Dependency lock file

### Source Code

- **src/main.rs** - Rust TUI application
  - Page-based interface with ratatui/crossterm
  - 13 pages: Live, Script 1-8, Metro (M), Init (I), Pattern (P), Help
  - Script storage: 10 scripts × 8 lines (Scripts 1-8, M, I)
  - Pattern storage: 4 patterns × 64 steps (i16 values)
  - Variables: A-D, X-Y-Z-T (global), J-K (per-script)
  - Control flow: IF conditions, PROB probabilistic execution, EV every-N-tick execution
  - Expression evaluation in arguments (P.NEXT, variables, etc.)
  - Metro thread sends script execution requests to main thread
  - OSC client sending to SuperCollider (127.0.0.1:57120)

- **sc/monokit_server.scd** - SuperCollider sound engine
  - `\monokit` SynthDef: HD2-style dual oscillator with FM, discontinuity, envelopes
  - Additive envelope model: output = base + env * amount
  - OSC responders:
    - `/monokit/trigger` - Gate trigger (no args)
    - `/monokit/param` - Generic parameter setter (string name, float/int value)
  - Stateless sound engine (no metro logic)

### Build Artifacts

- **target/** - Rust build output (ignored by git)

## Architecture Overview

```
Rust TUI (src/main.rs)
    |
    +-- Main Thread
    |    - TUI rendering (ratatui)
    |    - Command processing
    |    - Script execution (with App context)
    |    - Pattern/variable access
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
Audio output
```

## Command Reference

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

#### Scripts
- `SCRIPT <1-8>` - Execute stored script (can be called from other scripts)
- Scripts 1-8: User scripts
- M script (index 8): Called on each metro tick
- I script (index 9): Called on startup

#### Variables
- `A`, `B`, `C`, `D` - General accumulators (get/set: `A` or `A 100`)
- `X`, `Y`, `Z`, `T` - General accumulators
- Variables can be used in expressions: `PF A`, `DC X`

#### Patterns (Working Pattern - P.N)
- `P.N` / `P.N <0-3>` - Get/set working pattern
- `P.L` / `P.L <1-64>` - Get/set pattern length
- `P.I` / `P.I <0-63>` - Get/set playhead index
- `P.HERE` - Get value at playhead
- `P.NEXT` - Advance playhead, return value
- `P.PREV` - Reverse playhead, return value
- `P <idx>` / `P <idx> <val>` - Get/set value at index

#### Patterns (Explicit Pattern - PN)
- `PN <pat> <idx>` / `PN <pat> <idx> <val>` - Get/set value
- `PN.L <pat>` / `PN.L <pat> <len>` - Get/set length
- `PN.I <pat>` / `PN.I <pat> <idx>` - Get/set playhead
- `PN.HERE <pat>` - Get value at playhead
- `PN.NEXT <pat>` - Advance playhead, return value
- `PN.PREV <pat>` - Reverse playhead, return value

Note: All PN operations work in expression context (e.g., `DC PN.NEXT 0`)

#### Control Flow (PRE separator)
- `IF <cond>: <cmd>` - Execute cmd if condition true
- `PROB <0-100>: <cmd>` - Execute cmd with probability
- `EV <n>: <cmd>` - Execute cmd every Nth tick (applies to whole line including semicolons)
- Comparisons: `>`, `<`, `>=`, `<=`, `==`, `!=`
- Sub-commands: `cmd1; cmd2; cmd3` - Multiple commands on one line

#### HD2 Voice Parameters (22 total)

**Primary Oscillator**
- `PF <hz>` - Primary frequency (20-20000)
- `PW <0-2>` - Primary waveform (0=sine, 1=triangle, 2=sawtooth)

**Modulator Oscillator**
- `MF <hz>` - Modulator frequency (20-20000)
- `MW <0-2>` - Modulator waveform (0=sine, 1=triangle, 2=sawtooth)

**Discontinuity (Waveshaping)**
- `DC <0-16383>` - Discontinuity amount (mix of modulator into shaper)
- `DM <0-2>` - Discontinuity mode (0=fold, 1=tanh, 2=softclip)
- `DD <ms>` - Discontinuity envelope decay time (milliseconds, 1-10000)

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

**Envelopes (all in milliseconds, 1-10000 range)**
- `AD <ms>` - Amplitude decay time
- `PD <ms>` - Pitch decay time
- `FD <ms>` - FM decay time
- `DD <ms>` - Discontinuity decay time

**Envelope Amounts (Additive Model: output = base + env*amount)**
- `PA <0-16>` - Pitch envelope amount
- `FA <0-16>` - FM envelope amount
- `DA <0-16>` - Discontinuity envelope amount

#### Random Number Generation
- `RND <max>` - Random integer from 0 to max-1 (works as command and in expressions)
- `RRND <min> <max>` - Random integer from min to max inclusive (works as command and in expressions)

#### System
- `RST` - Reset all parameters to defaults
- `q` - Quit application

### Navigation (Keybindings)

#### Page Cycling
- `[` / `]` - Cycle through pages (Live → 1-8 → M → I → P → wrap)

#### Direct Page Access (Alt+key)
- `Alt+L` - Live page
- `Alt+1` through `Alt+8` - Script pages 1-8
- `Alt+M` - Metro page
- `Alt+I` - Init page
- `Alt+P` - Pattern page
- `Alt+H` - Toggle Help (overlay, scrollable with arrow keys)

#### Input
- `Enter` - Execute command
- `Up/Down` - Command history (on non-Help pages)
- `Left/Right` - Cursor movement
- `Ctrl+C` - Quit

## OSC Protocol

All communication from Rust CLI to SuperCollider server uses UDP over localhost (127.0.0.1:57120).

**Message Format**
- **Trigger:** `/monokit/trigger` (no arguments)
- **Master Volume:** `/monokit/volume` with float value (0.0-1.0)
- **Parameter Control:** `/monokit/param <name> <value>` where:
  - `<name>` = parameter name (string): pf, pw, mf, mw, dc, dm, dd, tk, mb, mp, md, mt, ma, fm, mx, mm, me, ad, pd, fd, dd, pa
  - `<value>` = float or int depending on parameter type
- **Reset:** `/monokit/reset` (no arguments, resets all parameters to defaults)

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
