# Monokit Documentation Index

## Documentation

- **CONCEPT.md** - Project overview, architecture, MVP implementation, and roadmap
- **documentation_index.md** - This file, listing all documentation and key project files

## Key Project Files

### Configuration

- **Cargo.toml** - Rust project manifest with dependencies (rosc, rustyline, nom, anyhow, thiserror, serde)
- **Cargo.lock** - Dependency lock file

### Source Code

- **src/main.rs** - Rust CLI application
  - REPL interface with rustyline (synchronous)
  - Dedicated metro thread with absolute timing (no drift)
  - All OSC routed through metro thread (serialized)
  - OSC client sending to SuperCollider (127.0.0.1:57120)
  - Command processor (TR, VOL, M, M.BPM, M.ACT, M:, help, exit/quit)
  - M commands execute locally; parameter updates sent via OSC
  - M script validation before execution
  - UDP socket communication

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
Rust CLI (src/main.rs)
    |
    +-- Metro Thread (absolute timing)
    |    |
    |    v OSC messages (serialized)
    |    127.0.0.1:57120
    |
    +-- REPL Thread (user input)
    |    |
    |    v OSC messages
    |    127.0.0.1:57120
    |
    v
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
- `M: <script>` - Set M script (semicolon-separated commands, validated before setting)

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

#### System
- `RST` - Reset all parameters to defaults
- `help` - Show command help
- `exit`, `quit` - Exit REPL

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
- rustyline 13 - REPL/readline
- nom 7 - Parser (future use)
- anyhow 1 - Error handling
- thiserror 1 - Error types
- serde 1, serde_json 1 - Serialization (future use)

### SuperCollider
- SuperCollider 3.x with scsynth
