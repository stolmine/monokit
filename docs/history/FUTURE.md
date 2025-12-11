# Monokit Future Plans

Detailed plans for Phase 7 (Advanced DSP) and Phase 8 (Distribution).

---

## Phase 7: Advanced DSP

**Focus:** Major architectural changes requiring deep SuperCollider work

### Noise Source Integration [Medium] ✅ COMPLETE
Added in v0.3.4. See `docs/v0.3.4_noise_source.md`.

- [x] NW (noise type: white/pink/brown)
- [x] Dedicated envelope (NA/ND/NC/NE)
- [x] Oscillator FM routing (NP/NM)
- [x] Source level controls (PV/MV/NV)
- [x] Gate mode (NG: drone/gated)

### Oscillator Sync [Medium]
Add hard sync between primary and modulator oscillators.

- [ ] `SYNC <0|1>` - Enable/disable oscillator sync
- [ ] Primary oscillator resets phase on modulator zero-crossing
- [ ] Classic sync sweep sound when modulator frequency changes
- [ ] Update SynthDef with sync logic (Sync or manual phase reset)

See: `docs/v0.3.4_oscillator_sync.md` for detailed implementation plan.

### Additional Filter Types [Medium] ✅ COMPLETE
Added in v0.3.4. See `docs/v0.3.4_filter_types.md`.

- [x] FT 0-13: 14 filter algorithms
- [x] SVF, MoogFF, RLPF, RHPF, DFM1, BMoog, Latch-SC
- [x] MC/MQ modbus routing to filter

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

---

## Future Refactors

### Command Naming Convention Overhaul [Medium]
Redesign command naming for consistency, memorability, and inferability.

**Current Issues:**
- Inconsistent prefix patterns (POSC vs PRI, MOSC vs MOD, AENV vs AEG)
- Long aliases don't always map intuitively to short forms
- New users struggle to guess command names
- Some abbreviations conflict or overlap

**Goals:**
- Establish clear, consistent prefix conventions
- Make short forms predictable from long forms
- Group related commands with shared prefixes
- Reduce cognitive load for learning commands

**Considerations:**
- Backward compatibility with existing scenes
- Teletype-inspired terseness vs readability
- Logical groupings by function vs signal flow
- Alias system can provide migration path
