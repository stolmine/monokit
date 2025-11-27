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

The MVP implements a full HD2-style dual oscillator voice with FM, discontinuity, and complex modulation.

### Components

**SuperCollider Server** (`sc/monokit_server.scd`)
- Runs headless scsynth with persistent HD2-style voice
- `\monokit` SynthDef: complex oscillator with dual waveform engines, FM, discontinuity, and modulation
- Full parameter set (20 parameters):
  - **Oscillators:** pf (primary freq), pw (primary waveform 0-2), mf (mod freq), mw (mod waveform 0-2)
  - **Discontinuity:** dc (amount 0-16383), dm (mode 0-2: fold/tanh/softclip), dd (discontinuity decay 0.001-10s)
  - **Tracking/Modulation:** tk (tracking 0-16383), mb (mod bus 0-16383), mp/md/mt/ma (switches 0-1)
  - **FM:** fm (index 0-16383)
  - **Envelopes:** ad (amp decay ms), pd (pitch decay ms), fd (FM decay ms), dd (disc decay ms), pa (pitch env amount 0-16)
  - **Mix Controls:** mx (mix to disc input 0-16383), mm (mix modulation amount 0-16383), me (mix modulation enable 0-1)
  - **Volume:** volume (0.0-1.0)
- Sample-accurate Routine-based metro for scheduled script execution
- OSC responders:
  - `/monokit/trigger` - triggers gate for note playback
  - `/monokit/param <name> <value>` - sets any parameter by name
  - `/monokit/metro <tempo_ms>` - sets metro interval (sample-accurate)
  - `/monokit/metro/act <0|1>` - activates/deactivates metro
  - `/monokit/metro/script <commands>` - sets script to execute on each metro tick

**Rust CLI** (`src/main.rs`)
- REPL interface with rustyline (synchronous, no async runtime)
- OSC client sending to 127.0.0.1:57120
- Commands (Teletype-inspired terse style):
  - **Trigger/Volume:** TR, VOL <0.0-1.0>
  - **Metro:** M, M <ms>, M.BPM <bpm>, M.ACT <0|1>, M: <script>
  - **HD2 Parameters:** PF/MF, PW/MW, DC/DM, DD, TK/MB, MP/MD/MT/MA, FM, AD/PD/FD/DD, PA
  - **Mix Controls:** MX, MM, ME
  - **System:** RST (reset to defaults), help, exit, quit
- M commands send script/timing to SC server as OSC
- SC server handles metro execution with sample-accurate Routine-based timing
- All parameters sent via OSC `/monokit/param` protocol

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
   monokit> M 500
   monokit> M: TR
   monokit> M.ACT 1
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
