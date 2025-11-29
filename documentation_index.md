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

Modular Rust implementation (11 files, 4,824 total lines):

- **src/main.rs** (62 lines) - Application entry point, initializes TUI and starts main loop
- **src/metro.rs** (112 lines) - Metro thread implementation with absolute timing
- **src/types.rs** (230 lines) - Core data structures, enums, constants, and type definitions
- **src/eval.rs** (482 lines) - Expression evaluation engine for nested operations, pattern access, and comparison operators
- **src/ui.rs** (637 lines) - TUI rendering with ratatui, page-based interface
- **src/tests.rs** (2,421 lines) - Comprehensive unit test suite (138 tests)
- **src/scene.rs** (169 lines) - Scene persistence, file I/O
- **src/app/mod.rs** (125 lines) - App struct, constructor, navigation
- **src/app/input.rs** (131 lines) - Input handling methods
- **src/app/script_exec.rs** (473 lines) - Script/command execution with 10 extracted helper methods
- **src/commands.rs** (1,335 lines) - Command parsing and processing logic

Key features:
- Page-based interface: Live, Script 1-8, Metro (M), Init (I), Pattern (P), Help
- Script storage: 10 scripts × 8 lines (Scripts 1-8, M, I)
- Pattern storage: 4 patterns × 64 steps (i16 values)
- Variables: A-D, X-Y-Z-T (global), J-K (per-script local), I (loop counter)
- Control flow: IF/ELIF/ELSE conditions, PROB probabilistic, EV/SKIP every-N-tick
- Comparison operators: EZ, NZ, EQ, NE, GT, LT, GTE, LTE (return 1/0)
- N operator: Semitone to frequency conversion (N 0 = C3 = 131 Hz)
- Expression evaluation in all numeric arguments
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

Note: All PN and P operations accept variables/expressions as arguments (e.g., `DC PN.NEXT 0`, `P I`, `PN A B`)

#### Expression Support
All numeric arguments accept nested expressions, including:
- Math operations: `PF ADD A 100`, `DC MUL X 2`
- Pattern operations: `PF PN.NEXT 0`, `DC SUB PN.HERE 0 PN.HERE 1`
- Random operations: `PF RND 1000`, `A RRND 0 127`
- Variables: `PF A`, `DC X`, `MF J`
- Nested combinations: `PF ADD PN.NEXT 0 RND 100`

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

#### Math Operations
- `ADD <a> <b>` or `+ <a> <b>` - Add two values (works as command and in expressions)
- `SUB <a> <b>` or `- <a> <b>` - Subtract b from a (works as command and in expressions)
- `MUL <a> <b>` or `* <a> <b>` - Multiply two values (works as command and in expressions)
- `DIV <a> <b>` or `/ <a> <b>` - Divide a by b (works as command and in expressions)
- `MOD <a> <b>` or `% <a> <b>` - Modulo a by b (works as command and in expressions)

#### Random Number Generation
- `RND <max>` - Random integer from 0 to max-1 (works as command and in expressions)
- `RRND <min> <max>` - Random integer from min to max inclusive (works as command and in expressions)
- `TOSS` - Random 0 or 1 (coin flip, works as command and in expressions)
- `EITH <a> <b>` - Random choice between a and b (works as command and in expressions)

#### Note/Pitch Conversion
- `N <semitones>` - Convert semitones to frequency in Hz (12-TET, works in expressions)
  - N 0 = C3 (131 Hz) - matches Teletype reference pitch
  - N 12 = C4 (262 Hz)
  - N 21 = A4 (440 Hz)
  - N -12 = C2 (65 Hz)
  - Usage: `PF N 0` (set primary freq to C3), `PF N ADD A 7` (C3 + A semitones + perfect 5th)

#### System
- `RST` - Reset all parameters to defaults
- `q`, `quit`, or `exit` - Quit application (typed in REPL)

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
- `Ctrl+Left/Right` - Word-by-word cursor movement
- `Ctrl+D` - Duplicate line (script pages)
- `Ctrl+K` - Delete entire line (script pages)
- `Ctrl+C` - Copy line (script pages)
- `Ctrl+X` - Cut line (script pages)
- `Ctrl+V` - Paste line (script pages)
- Script pages show validation errors that auto-clear after 3 seconds or on successful save

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
