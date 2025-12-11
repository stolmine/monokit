# Monokit Future Plans

Detailed plans for Phase 7 (Advanced DSP) and Phase 8 (Distribution).

---

## Phase 7: Advanced DSP

**Focus:** Major architectural changes requiring deep SuperCollider work

### Noise Source Integration [Medium]
Add multi-colored noise generator before filter and amp stages.

- [ ] Add noise oscillator to voice (white, pink, brown/red, blue, violet)
- [ ] `NS` / `NOISE` - Noise level (mix amount into signal path)
- [ ] `NS.CLR` / `NOISE.CLR` - Noise color (0=white, 1=pink, 2=brown, 3=blue, 4=violet)
- [ ] Insert point: after oscillator mix, before filter
- [ ] Envelope option: noise follows amp envelope or constant level
- [ ] Update SynthDef with noise UGens (WhiteNoise, PinkNoise, BrownNoise, etc.)

See: `docs/v0.3.4_noise_source.md` for detailed implementation plan.

### Oscillator Sync [Medium]
Add hard sync between primary and modulator oscillators.

- [ ] `SYNC <0|1>` - Enable/disable oscillator sync
- [ ] Primary oscillator resets phase on modulator zero-crossing
- [ ] Classic sync sweep sound when modulator frequency changes
- [ ] Update SynthDef with sync logic (Sync or manual phase reset)

See: `docs/v0.3.4_oscillator_sync.md` for detailed implementation plan.

### Additional Filter Types [Medium]
Expand filter options beyond SVF.

- [ ] `FT` / `FILT.TYPE` extended modes:
  - Current: 0=LP, 1=HP, 2=BP, 3=Notch
  - Add: 4=Ladder (Moog-style 24dB/oct)
  - Add: 5=Formant (vowel filter)
  - Add: 6=Comb (as filter, not resonator)
- [ ] Consider separate filter UGens or multi-mode SynthDef
- [ ] Maintain filter envelope compatibility across types

See: `docs/v0.3.4_filter_types.md` for detailed implementation plan.

### Additional Voice Types [Very High]
- [ ] `VOICE <0-N>` - Voice type selector
- [ ] Voice 0: Current HD2-style complex oscillator (default)
- [ ] Voice 1: FM (DX-style 4-operator)
- [ ] Voice 2: Subtractive (classic analog)
- [ ] Voice 3: Wavetable
- [ ] Voice 4: Physical modeling (Karplus-Strong)
- [ ] Voice 5: Noise/percussion focused
- [ ] Full SynthDef implementation per voice type
- [ ] Parameter mapping and compatibility layer

### Plaits Macro Voice [Very High]
- [ ] Import Mutable Instruments Plaits algorithms
- [ ] `PLAITS.MODEL <0-15>` - Select Plaits model
- [ ] `PLAITS.TIMBRE`, `PLAITS.MORPH`, `PLAITS.HARM` controls
- [ ] Dependencies: SC Plaits UGen or full port of algorithms

### Optional Polyphony [Very High]
- [ ] `POLY <1-8>` - Number of voices
- [ ] Voice allocation: round-robin or lowest
- [ ] Per-voice detuning
- [ ] Unison mode with spread
- [ ] State management per voice
- [ ] Complex routing and mixing requirements

### Sample Playback System [Very High]
- [ ] `S.LOAD <path>` - Load sample from file (WAV/AIFF)
- [ ] `S.BANK <0-N>` - Select sample bank/folder
- [ ] `S.SEL <0-N>` - Select sample within bank
- [ ] `S.SLICE <n>` - Set number of slices (auto-divide sample)
- [ ] `S.IDX <0-N>` - Select slice index for playback
- [ ] `S.START <0-1>`, `S.END <0-1>` - Manual start/end points
- [ ] `S.DIR <-1|0|1>` - Playback direction
- [ ] `S.RATE <0.1-4>` - Playback rate/pitch
- [ ] `S.PITCH <semitones>` - Pitch shift independent of rate
- [ ] `S.LOOP <0|1>` - Loop mode
- [ ] `S.TRIG` - Trigger sample playback
- [ ] Buffer management in SuperCollider
- [ ] Sample browser/indexing system
- [ ] Integration with pattern system: `S.IDX PN.NEXT 0`

---

## Phase 8: Distribution

**Focus:** Packaging and deployment infrastructure (post-release expansion)

### Cross-Platform Compatibility [High]
Expand beyond Apple Silicon macOS.

**macOS Intel (x86_64):**
- [ ] Add x86_64-apple-darwin target to release workflow
- [ ] Bundle Intel scsynth binary
- [ ] Test on Intel Mac hardware
- [ ] Universal binary option (fat binary)

**Linux x86_64:**
- [ ] Linux build target (x86_64-unknown-linux-gnu)
- [ ] Bundle or document scsynth installation
- [ ] Handle audio backend differences (JACK, PulseAudio, PipeWire)
- [ ] AppImage or Flatpak packaging
- [ ] Test on common distros (Ubuntu, Fedora, Arch)

**Linux ARM64 (uConsole/Raspberry Pi):**
- [ ] Linux build target (aarch64-unknown-linux-gnu)
- [ ] Build or bundle scsynth for ARM64 Linux
- [ ] Build or bundle sc3-plugins for ARM64 (or remove dependency)
- [ ] Implement ALSA/PipeWire audio backend selection
- [ ] Test on ClockworkPi uConsole (CM4 variant)
- [ ] Test on Raspberry Pi 4/5
- [ ] Optimize buffer sizes for ARM CPU constraints
- [ ] Document ClockworkOS / Raspberry Pi OS setup

**Windows:**
- [ ] Windows build target (x86_64-pc-windows-msvc)
- [ ] Bundle scsynth for Windows
- [ ] Handle Windows audio APIs (WASAPI, ASIO)
- [ ] Portable .exe or MSI installer
- [ ] Terminal emulator recommendations (Windows Terminal)

### Unified Installer [High]
- [ ] Single installer package bundling:
  - Rust CLI binary
  - SuperCollider runtime (scsynth)
  - SC SynthDefs
  - Default config/themes
- [ ] Platform-specific installers:
  - macOS: .pkg or Homebrew formula (COMPLETE for ARM)
  - Linux: .deb, .rpm, AppImage
  - Windows: .msi or portable .exe
- [ ] Auto-start SC server on launch
- [ ] No manual SuperCollider installation required
- [ ] Dependency management and version checking

---

## Implementation Notes

### Dependencies Between Phases
- **Phase 7** advanced DSP requires stable release before major voice architecture changes
- **Phase 8** unified installer builds on Phase 6 release infrastructure

### Complexity Legend
- **[Low]** - 1-3 days, minimal dependencies, straightforward implementation
- **[Medium]** - 1-2 weeks, moderate complexity, some new infrastructure
- **[High]** - 2-4 weeks, significant new systems, external dependencies
- **[Very High]** - 4+ weeks, major architectural changes, deep domain expertise required

**Note:** Modulation System (LFO, Aux Envelopes) moved to `ON_HOLD.md` due to SuperCollider UGen complexity limits.
