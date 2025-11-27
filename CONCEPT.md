# Monokit

A text-based scripting language for a monophonic drum synthesizer built on a complex oscillator.

## Core Concept

- **Monophonic drum kit** driven by a single complex oscillator voice
- **Scripting language** controls the voice — not just triggering, but defining behavior per-step
- **CLI-native** — runs headless, REPL or pipe-friendly

## Gap This Fills

Existing tools are either:
- **Sequencer-first** (TidalCycles, Strudel, Orca) — synth-agnostic, pattern-focused
- **Synth-first** (Plaits, Basimilus Iteritas) — no integrated scripting

Nothing tightly couples a text scripting language with a dedicated complex oscillator drum voice as a single integrated instrument.

## Current MVP Implementation

**Architecture: SC Backend + Rust CLI Frontend**

The MVP implements a minimal working system with OSC communication between components.

### Components

**SuperCollider Server** (`sc/monokit_server.scd`)
- Runs headless scsynth with persistent voice
- `\monokit` SynthDef: simple sine oscillator with pitch/amp envelopes
- Parameters: freq, pitchDecay, pitchAmt, ampDecay, volume, gate
- OSC responders:
  - `/monokit/trigger` - triggers gate for note playback
  - `/monokit/volume` - sets voice volume (0.0-1.0)

**Rust CLI** (`src/main.rs`)
- REPL interface with rustyline
- OSC client sending to 127.0.0.1:57120
- Commands:
  - `TR` - send trigger
  - `VOL <0.0-1.0>` - set volume
  - `help`, `exit`, `quit`
- Teletype-inspired terse command style

### Running the MVP

1. Start SuperCollider server:
   ```
   sclang sc/monokit_server.scd
   ```

2. Start Rust CLI:
   ```
   cargo run
   ```

3. Type commands in REPL:
   ```
   monokit> TR
   monokit> VOL 0.5
   monokit> TR
   ```

## Reference Tools

| Tool | Type | Notes |
|------|------|-------|
| TidalCycles | Text sequencer | Mini-notation, SC backend |
| Strudel | Text sequencer | Tidal in JS/browser, Web Audio |
| Orca | Grid sequencer | Esoteric, MIDI/OSC output |
| Sonic Pi | Live coding | Ruby-based, educational |
| ChucK | Audio language | CLI-native, real-time |
| Teletype | Hardware scripting | Closest paradigm to this concept |

## Next Steps

- Expand command set (pitch, envelope parameters)
- Add pattern/sequencing capabilities
- Enhance oscillator complexity (FM, waveshaping)
- Implement script file execution
- Add metronome/clock system
