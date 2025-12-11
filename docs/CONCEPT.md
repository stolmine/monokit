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
  - **Beat Repeat:** BRP, BRT, BRG, BRM
  - **Pitch Shift:** PST, PSD, PTW, PSM
  - **Recording:** REC, REC.STOP, REC.PATH <prefix>
  - **System:** RST (reset to defaults), CLEAR (clear output), DEBUG <0-2> (verbosity), PRINT (output to REPL), help, exit, quit
- Envelope amounts: PA (pitch), FA (FM), DA (discontinuity) — added to base parameter via modulation amount
- M commands execute locally; parameter updates sent via OSC `/monokit/param` protocol

**SuperCollider Server** (`sc/monokit_server.scd`)
- Runs headless scsynth with persistent HD2-style voice
- `\monokit` SynthDef: complex oscillator with dual waveform engines, FM, discontinuity, comprehensive DSP effects, and multi-stage processing
- Additive envelope model: output = base parameter + env * amount
- Signal chain: Oscillators → FM → Mix → Discontinuity → Lo-Fi → SVF Filter → Ring Mod → Comb Resonator → Amp → Compressor → Pan → Beat Repeat → Pitch Shift → Stereo Delay → 3-Band EQ → Plate Reverb → Out
- 77 parameters (25 oscillator/envelope + 48 DSP + 4 routing):
  - **Oscillators:** pf (primary freq), pw (primary waveform 0-2), mf (mod freq), mw (mod waveform 0-3)
  - **Feedback FM:** fb (feedback amount 0-16383), fba (feedback env amount 0-16383), fbd (feedback decay ms)
  - **Discontinuity:** dc (amount 0-16383), dm (mode 0-6: fold/tanh/softclip/hard/asym/rectify/crush), dd (discontinuity decay ms)
  - **Tracking/Modulation:** tk (tracking 0-16383), mb (mod bus 0-16383), mp/md/mt/ma (switches 0-1)
  - **FM:** fm (index 0-16383)
  - **Envelopes:** ad (amp decay ms), pd (pitch decay ms), fd (FM decay ms), dd (disc decay ms)
  - **Envelope Amounts:** pa (pitch 0-16), fa (FM 0-16), da (discontinuity 0-16)
  - **Mix Controls:** mx (mix to disc input 0-16383), mm (mix modulation amount 0-16383), me (mix modulation enable 0-1)
  - **Lo-Fi:** lb (bit depth 1-16), ls (sample rate 100-48000 Hz), lm (mix 0-16383)
  - **SVF Filter:** fc (cutoff Hz), fq (resonance 0-16383), ft (type 0-3), fe (env amount), fed (env decay ms), fk (key tracking), mc (modbus→cutoff), mq (modbus→res)
  - **Ring Modulator:** rgf (frequency 20-2000 Hz), rgw (waveform 0-3), rgm (mix 0-16383)
  - **Comb Resonator:** rf (freq Hz), rd (decay ms), rm (mix 0-16383), rk (key tracking)
  - **Compressor:** ct (threshold 0-16383), cr (ratio 1-20), ca (attack ms 1-500), cl (release ms 10-2000), cm (makeup gain 0-16383)
  - **Pan:** pn (position -16383 to +16383)
  - **Beat Repeat:** brp (probability 0-16383), brt (time subdivision 0-5), brg (gate length 0-16383), brm (mix 0-16383)
  - **Pitch Shift:** pst (transpose semitones -24 to +24), psd (dispersion 0-16383), ptw (time dispersion 0-16383), psm (mix 0-16383)
  - **Stereo Delay:** dt (time ms), df (feedback), dlp (lowpass Hz), dw (wet/send), ds (sync 0-1), dmode (routing 0-2), dtail (tail mode 0-2)
  - **3-Band EQ:** el (low shelf dB -24 to +24), em (mid peak dB -24 to +24), ef (mid freq Hz), eq (mid Q), eh (high shelf dB -24 to +24)
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
- ✓ Tier 2 DSP blocks: Lo-Fi, Ring Modulator, Compressor, 3-Band EQ, Pan - COMPLETE
- ✓ Tier 3 DSP blocks: Beat Repeat, Pitch Shift - COMPLETE
- ✓ Effect routing system: BYPASS/INSERT/SEND modes with CUT/RING/FREEZE tail behaviors - COMPLETE
- ✓ ModBus routing to filter cutoff - COMPLETE
- ✓ Extended discontinuity modes (0-6: fold, tanh, softclip, hard, asym, rectify, crush) - COMPLETE
- ✓ Phase 1 language features: MAP operator, TOG generator, N1-N4 counters - COMPLETE
- ✓ All DSP processing complete - COMPLETE
- Pattern/sequencing enhancements
- LFO system for parameter modulation
- Additional modulation routing (ModBus to delay time, reverb size, resonator frequency)
- Tempo-synced delay (DS parameter implementation)
