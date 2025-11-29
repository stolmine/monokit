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

**Architecture: Rust CLI + SC Sound Engine**

The MVP implements a full HD2-style dual oscillator voice with FM, discontinuity, and complex modulation. Metro and OSC routing managed by CLI; SC server is a stateless sound engine.

### Components

**Rust CLI** (`src/main.rs`)
- REPL interface with rustyline (synchronous)
- OSC client sending to 127.0.0.1:57120
- Dedicated metro thread with absolute timing (no cumulative drift)
- All OSC routed through single metro thread (serialized sending)
- Recording system: WAV (int24) output with timestamped files
- Commands (Teletype-inspired terse style):
  - **Trigger/Volume:** TR, VOL <0.0-1.0>
  - **Metro:** M, M <ms>, M.BPM <bpm>, M.ACT <0|1>, M: <script>
  - **HD2 Parameters:** PF/MF, PW/MW, DC/DM, DD, TK/MB, MP/MD/MT/MA, FM, FB/FBA/FBD, AD/PD/FD/DD, PA, FA, DA
  - **Mix Controls:** MX, MM, ME
  - **Recording:** REC, REC.STOP, REC.PATH <prefix>
  - **System:** RST (reset to defaults), CLEAR (clear output), DEBUG <0-2> (verbosity), PRINT (output to REPL), help, exit, quit
- Envelope amounts: PA (pitch), FA (FM), DA (discontinuity) — added to base parameter via modulation amount
- M commands execute locally; parameter updates sent via OSC `/monokit/param` protocol

**SuperCollider Server** (`sc/monokit_server.scd`)
- Runs headless scsynth with persistent HD2-style voice
- `\monokit` SynthDef: complex oscillator with dual waveform engines, FM, discontinuity, modulation, and DSP effects
- Additive envelope model: output = base parameter + env * amount
- Signal chain: Oscillators → FM → Mix → Discontinuity → SVF Filter → Comb Resonator → Amp → Stereo Delay → Plate Reverb → Out
- 49 parameters (25 oscillator/envelope + 20 DSP + 4 routing):
  - **Oscillators:** pf (primary freq), pw (primary waveform 0-2), mf (mod freq), mw (mod waveform 0-3)
  - **Feedback FM:** fb (feedback amount 0-16383), fba (feedback env amount 0-16383), fbd (feedback decay ms)
  - **Discontinuity:** dc (amount 0-16383), dm (mode 0-2: fold/tanh/softclip), dd (discontinuity decay 0.001-10s)
  - **Tracking/Modulation:** tk (tracking 0-16383), mb (mod bus 0-16383), mp/md/mt/ma (switches 0-1)
  - **FM:** fm (index 0-16383)
  - **Envelopes:** ad (amp decay ms), pd (pitch decay ms), fd (FM decay ms), dd (disc decay ms)
  - **Envelope Amounts:** pa (pitch 0-16), fa (FM 0-16), da (discontinuity 0-16)
  - **Mix Controls:** mx (mix to disc input 0-16383), mm (mix modulation amount 0-16383), me (mix modulation enable 0-1)
  - **SVF Filter:** fc (cutoff Hz), fq (resonance 0-16383), ft (type 0-3), fe (env amount), fed (env decay ms), fk (key tracking), mf_f (modbus routing)
  - **Comb Resonator:** rf (freq Hz), rd (decay ms), rm (mix 0-16383), rk (key tracking)
  - **Stereo Delay:** dt (time ms), df (feedback), dlp (lowpass Hz), dw (wet/send), ds (sync 0-1), dmode (routing 0-2), dtail (tail mode 0-2)
  - **Plate Reverb:** rv (decay), rp (pre-delay ms), rh (damping), rw (wet/send), rmode (routing 0-2), rtail (tail mode 0-2)
  - **Volume:** volume (0.0-1.0)
- OSC responders:
  - `/monokit/trigger` - triggers gate for note playback
  - `/monokit/param <name> <value>` - sets any parameter by name
  - `/monokit/rec`, `/monokit/rec/stop`, `/monokit/rec/path` - recording control
  - Sound engine only; no metro logic

### Running the MVP

1. Start SuperCollider server:
   ```
   sclang sc/monokit_server.scd
   ```

2. Start Rust CLI:
   ```
   cargo run
   ```

3. Type commands in REPL (metro runs in dedicated CLI thread):
   ```
   monokit> M 500
   monokit> M: TR
   monokit> M.ACT 1
   monokit> FA 8
   monokit> DA 4
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

## Recording System

The CLI includes a built-in WAV recording system for capturing audio output:

- **Format:** WAV int24 (24-bit integer)
- **Timestamped files:** `monokit_YYYYMMDD_HHMMSS.wav` by default
- **Custom paths:** Use `REC.PATH <prefix>` to set custom filename prefix
- **UI indicator:** Red "● REC MM:SS" shows recording status and duration
- **Auto-stop:** Recording automatically stops on quit to prevent file corruption
- **Working directory:** Files saved to current working directory unless custom path specified

Recording captures the SuperCollider audio output directly.

## Next Steps

- ✓ Tier 1 DSP blocks: Filter (SVF), Resonator (Comb), Delay (stereo), Reverb (plate) - COMPLETE
- ✓ Effect routing system: BYPASS/INSERT/SEND modes with CUT/RING/FREEZE tail behaviors - COMPLETE
- ✓ ModBus routing to filter cutoff - COMPLETE
- Pattern/sequencing enhancements
- LFO system for parameter modulation
- Additional modulation routing (ModBus to delay time, reverb size, etc.)
- Tempo-synced delay (DS parameter implementation)
