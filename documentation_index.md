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
  - OSC client sending to SuperCollider
  - Command processor (TR, VOL, help, exit/quit)
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

- `TR` - Trigger voice
- `VOL <0.0-1.0>` - Set volume
- `help` - Show help
- `exit`, `quit` - Exit REPL

## Dependencies

### Rust
- rosc 0.10 - OSC protocol
- rustyline 13 - REPL/readline
- nom 7 - Parser (future use)
- tokio 1 - Async runtime (future use)
- anyhow 1 - Error handling
- thiserror 1 - Error types
- serde 1, serde_json 1 - Serialization (future use)

### SuperCollider
- SuperCollider 3.x with scsynth
