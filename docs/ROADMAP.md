# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## v0.3.4 Progress (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Config Flag Standardization | Low | **DONE** |
| SCRIPT Expression Support | Low | **DONE** |
| MC/MQ Filter Routing | Low | **DONE** |
| ModBus Envelope (MBA/MBD) | Low | **DONE** |
| Script Undo/Redo | Low | **DONE** |
| ER/NR Rhythm Operators | Low | **DONE** |
| Frappe Theme | Low | **DONE** |
| Additional Filter Types | Medium | **DONE** |
| Noise Source | Medium | **DONE** |

---

## v0.3.5 Progress (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Multi-Synth Architecture | High | **DONE** |
| Parameter Routing System | Medium | **DONE** |
| Build System Reliability | Low | **DONE** |
| Scene Path Consistency | Low | **DONE** |

---

## v0.3.6 Progress (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| CPU Monitoring Fix | Low | **DONE** |

---

## v0.3.7 Plan

| Feature | Effort | Status |
|---------|--------|--------|
| Debug Tier Refactor | Low | Not started |
| Noise Envelope & Gating | Medium | Not started |
| Noise FM Routing | Low | Not started |
| Oscillator Sync | Medium | Not started |

---

## Prioritized Incomplete Items

### P1 - High Value Features
- **MIDI CC and Note Input** [Medium] - External control, performance capability

### P2 - Polish & Documentation
- **Dynamic Grid Layout** [Medium] - Responsive UI spacing
- **Tempo-Synced Delay** [Low] - DS parameter for musical delay times
- **Manual Update + Voice Architecture Diagram** [Low] - Fill documentation gaps, add ASCII voice architecture diagram
- **Gain Staging Audit** [Medium] - Review clipping behavior with modbus/noise routing; consider automatic output level detection via existing meter OSC for testing; balance preventing unwanted distortion vs preserving intentional clipping
- **Slew Coverage Expansion** [Low] - Extend SLEW to all continuous voice parameters (currently 30/88)

### P3 - Future / Large Effort
- **Cross-Platform Compatibility** [High] - Linux/Windows/Intel Mac
- **Sample Playback System** [Very High] - Major feature
- **Additional Voice Types** [Very High] - Architecture change
- **Optional Polyphony** [Very High] - Architecture change

---

## Recent Updates (December 2025)

### Noise Envelope & Gating [PLANNED]
Re-implement envelope and gate control in the noise synth after multi-synth architecture split.

**Requirements:**
- Add envelope (NA, ND, NC, NE) back to monokit_noise SynthDef
- Add gate control (NG) for triggerable vs drone noise
- Output multiple buses from noise synth:
  - Bus 18: Envelope-shaped noise for main output (controlled by NV)
  - Bus 19: Raw noise for FM modulation (controlled by NP/NM)
- This allows independent control of noise as audio source vs modulation source
- Envelope affects main output but not FM routing
- NP/NM act as VCA amounts controlling noise→prim and noise→mod FM

**Architecture:**
```
monokit_noise:
  noise → [envelope + NV] → Bus 18 (to main synth audio mix)
  noise → [NP scaling] → Bus 19 (to monokit_primary FM input)
  noise → [NM scaling] → Bus 20 (to monokit_mod FM input)
```

**Files to modify:**
- build_scripts/compile_synthdefs.scd (monokit_noise SynthDef)
- build_scripts/compile_synthdefs.scd (monokit_primary - read Bus 19 for FM)
- build_scripts/compile_synthdefs.scd (monokit_mod - read Bus 20 for FM)
- sc/monokit_server.scd (same changes for runtime server)

### Noise FM Routing Fix [PLANNED]
Ensure noise FM modulation works even when NV is at zero.

**Problem:**
- Currently noise FM (NP/NM) may be tied to NV volume control
- NP and NM should work independently as FM amounts
- Users should be able to use noise as FM source without hearing it

**Solution:**
- Separate noise FM routing from noise audio volume
- NP controls noise→primary FM amount (independent of NV)
- NM controls noise→mod FM amount (independent of NV)
- This is achieved by the bus architecture above

**Testing:**
- Set NV=0, NP=16383 - should hear FM modulation on primary, no noise audio
- Set NV=0, NM=16383 - should hear FM modulation on mod, no noise audio
- Set NV=16383, NP=0, NM=0 - should hear noise audio, no FM

### Multi-Synth Architecture [COMPLETE]
Fixed SuperCollider optimizer bug causing volume parameter cross-talk (NV/PV/MV).

**Problem:**
- SC optimizer incorrectly conflated volume parameters when in same SynthDef
- NV (noise volume), PV (primary volume), MV (mod volume) shared parameter graph
- Changing one parameter affected others due to optimizer collapse

**Solution:**
- Split monolithic SynthDef into 4 separate SynthDefs with audio bus routing
- monokit_noise: Noise source → Bus 16
- monokit_mod: Modulator → Bus 17
- monokit_primary: Primary oscillator → Bus 18
- monokit_main: Reads buses 16/17/18, processes effects, outputs final audio
- Each source has isolated volume parameter graph preventing cross-talk

**Implementation:**
- [x] Created 4-synth architecture with bus routing
- [x] Updated build process to compile 7 synthdefs (4 sources + 3 utilities)
- [x] Verified volume parameters now operate independently
- [x] Maintained single-trigger interface (Rust sends to all 4 synths)
- [x] Files: build_scripts/compile_synthdefs.scd, sc/monokit_server.scd

### Envelope Parameter Scaling Fix [COMPLETE]
Fixed FM and Disc envelope amount parameter ranges.

**Changes:**
- [x] FA (FM envelope): 0-16 float → 0-16383 int (14-bit)
- [x] DA (Disc envelope): 0-16 float → 0-16383 int (14-bit)
- [x] Root cause: SynthDef divided by 16383 but Rust limited to 0-16
- [x] Restored full resolution for envelope modulation
- [x] Files: src/commands/synth/envelopes/fm.rs, disc.rs

### SynthDef Default Parameter Alignment [COMPLETE]
RST command now matches SynthDef default values.

**Changes:**
- [x] pf: 200 → 131 (C3, ~131 Hz)
- [x] mf: 50 → 262 (C4, ~262 Hz)
- [x] pa: 4 → 0 (no pitch envelope by default)
- [x] fc: 1000 → 10000 (filter wide open)
- [x] cr: 4 → 1 (compressor off by default)
- [x] Files: sc/monokit_server.scd, build_scripts/compile_synthdefs.scd

### Release Pipeline Documentation [COMPLETE]
Automated release infrastructure documented for future reference.

**Documentation Added:**
- [x] docs/RELEASE_PIPELINE.md - Complete release workflow
- [x] docs/MANUAL.md - User manual with comprehensive command reference
- [x] README.md - Updated with feature rundown and installation instructions
- [x] Automated GitHub Actions release workflow
- [x] Homebrew tap auto-updates via release pipeline

**Release Process:**
1. Local: `./scripts/release.sh X.Y.Z` creates signed bundle and git tag
2. GitHub Actions: Builds release bundle on macOS-14 (Apple Silicon)
3. Homebrew: Formula auto-updates with new SHA256 from artifacts

### Recent Feature Completions [COMPLETE]
**List Output Formatting:**
- [x] THEMES command now displays vertically (one per line)
- [x] Improved readability and terminal compatibility
- [x] Consistent with other list commands

**VCA Reset Coverage:**
- [x] RST and LOAD.RST now reset VCA to gated mode (default)
- [x] Ensures predictable behavior after reset operations
- [x] VCA state properly restored on scene load

**EITH Selection Highlighting:**
- [x] Random choice now highlights selected option in script display
- [x] State persists correctly across evaluations like TOG
- [x] Nested random choice `{a b}` in SEQ shows last selection

---

## Completed Features Summary

For detailed completion records, see:
- `docs/history/CHANGELOG.md` - Version history and completed features
- `docs/history/PHASES.md` - Development phases 1-6 completion details

### Core Voice & DSP
- HD2-style dual oscillator with FM, discontinuity, and complex modulation
- Noise source with dedicated envelope and oscillator FM routing
- Full DSP signal chain with 10+ effect blocks
- 88 real-time parameters

### Language & Scripting
- Page-based interface with 8 scripts + Metro + Init
- Pattern system: 6 patterns × 64 steps
- Full control flow: IF/ELIF/ELSE, loops, probability, scheduling
- SEQ inline sequencing with mini notation

### Infrastructure
- Direct scsynth integration (bundled binary)
- Automated release pipeline with Homebrew
- Theme system with 30+ themes

---

## Future Phases

### Phase 7: Advanced DSP
- Noise source integration
- Oscillator sync
- Additional filter types
- Additional voice types
- Sample playback system

### Phase 8: Distribution
- Cross-platform compatibility (Linux, Windows, Intel Mac)
- Unified installer packages

See `docs/history/FUTURE.md` for detailed plans.

---

## Implementation Notes

### Design Principles
- Maintain CLI-first philosophy throughout
- All new parameters must support expression evaluation
- Keep commands terse (Teletype-inspired)
- Consider CPU impact for real-time features
- UI features should be optional/toggleable

### Complexity Legend
- **[Low]** - 1-3 days, minimal dependencies
- **[Medium]** - 1-2 weeks, moderate complexity
- **[High]** - 2-4 weeks, significant new systems
- **[Very High]** - 4+ weeks, major architectural changes

---

## Contributing

Feature requests and suggestions welcome. All contributions should maintain the project's terse command syntax and CLI-native philosophy.
