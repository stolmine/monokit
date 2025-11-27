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
  - `\monokit` SynthDef with sine oscillator
  - OSC responders for `/monokit/trigger` and `/monokit/volume`
  - Persistent voice architecture with gate triggering

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

#### Basic Commands
- `TR` - Trigger voice
- `VOL <0.0-1.0>` - Set volume
- `help` - Show help
- `exit`, `quit` - Exit REPL

#### Metro/Timing Commands
- `M` - Show current metro interval
- `M <ms>` - Set metro interval in milliseconds
- `M.BPM <bpm>` - Set metro interval as BPM
- `M.ACT <0|1>` - Activate/deactivate metro (0=off, 1=on)
- `M: <script>` - Set M script (validated before setting, semicolon-separated commands)

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
