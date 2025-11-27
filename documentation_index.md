# Monokit Documentation Index

## Documentation

- **CONCEPT.md** - Project overview, architecture, MVP implementation, and roadmap
- **documentation_index.md** - This file, listing all documentation and key project files

## Key Project Files

### Configuration

- **Cargo.toml** - Rust project manifest with dependencies (rosc, rustyline, nom, tokio, anyhow, thiserror, serde)
- **Cargo.lock** - Dependency lock file

### Source Code

- **src/main.rs** - Rust CLI application
  - REPL interface with rustyline
  - Async tokio runtime with parking_lot::Mutex
  - OSC client sending to SuperCollider
  - Background metro task for scheduled script execution
  - Command processor (TR, VOL, M, M.BPM, M.ACT, M:, help, exit/quit)
  - M script validation before setting
  - UDP socket communication on port 57120

- **sc/monokit_server.scd** - SuperCollider server
  - `\monokit` SynthDef: HD2-style dual oscillator with FM, discontinuity, envelopes
  - OSC responders:
    - `/monokit/trigger` - Gate trigger (no args)
    - `/monokit/volume` - Volume control (float arg)
    - `/monokit/param` - Generic parameter setter (string name, float/int value)
  - Persistent voice architecture with per-trigger gate pulses

### Build Artifacts

- **target/** - Rust build output (ignored by git)

## Architecture Overview

```
Rust CLI (src/main.rs)
    |
    | OSC messages via UDP
    | 127.0.0.1:57120
    |
    v
SuperCollider (sc/monokit_server.scd)
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

#### HD2 Voice Parameters (17 total)

**Primary Oscillator**
- `PF <hz>` - Primary frequency (20-20000)
- `PW <0-2>` - Primary waveform (0=sine, 1=triangle, 2=sawtooth)

**Modulator Oscillator**
- `MF <hz>` - Modulator frequency (20-20000)
- `MW <0-2>` - Modulator waveform (0=sine, 1=triangle, 2=sawtooth)

**Discontinuity (Waveshaping)**
- `DC <0-16383>` - Discontinuity amount (mix of modulator into shaper)
- `DM <0-2>` - Discontinuity mode (0=fold, 1=tanh, 2=softclip)

**Tracking & Modulation Bus**
- `TK <0-16383>` - Tracking amount (modulator frequency follows pitch envelope)
- `MB <0-16383>` - Modulation bus amount (general modulation depth)
- `MP <0|1>` - Enable modulation -> primary frequency
- `MD <0|1>` - Enable modulation -> discontinuity amount
- `MT <0|1>` - Enable modulation -> tracking
- `MA <0|1>` - Enable modulation -> amplitude

**FM Synthesis**
- `FM <0-16383>` - FM index (modulator phase modulates primary frequency)

**Envelopes (all in seconds, 0.001-10 range)**
- `AD <seconds>` - Amplitude decay time
- `PD <seconds>` - Pitch decay time
- `FD <seconds>` - FM decay time
- `PA <0-16>` - Pitch envelope amount (pitch contour depth)

#### System
- `help` - Show command help
- `exit`, `quit` - Exit REPL

## OSC Protocol

All communication from Rust CLI to SuperCollider server uses UDP over localhost (127.0.0.1:57120).

**Message Format**
- **Trigger:** `/monokit/trigger` (no arguments)
- **Master Volume:** `/monokit/volume` with float value (0.0-1.0)
- **Parameter Control:** `/monokit/param <name> <value>` where:
  - `<name>` = parameter name (string): pf, pw, mf, mw, dc, dm, tk, mb, mp, md, mt, ma, fm, ad, pd, fd, pa
  - `<value>` = float or int depending on parameter type

All parameter updates are validated in Rust CLI before sending and applied immediately on SuperCollider voice.

## Dependencies

### Rust
- rosc 0.10 - OSC protocol
- rustyline 13 - REPL/readline
- tokio 1 - Async runtime for metro task
- parking_lot 0.12 - Mutex for metro state
- nom 7 - Parser (future use)
- anyhow 1 - Error handling
- thiserror 1 - Error types
- serde 1, serde_json 1 - Serialization (future use)

### SuperCollider
- SuperCollider 3.x with scsynth
