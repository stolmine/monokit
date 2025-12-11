# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## v0.3.4 Progress (December 2025)

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
| Noise Source | Medium | Not started |
| Oscillator Sync | Medium-High | Not started |

---

## Prioritized Incomplete Items

### P1 - High Value Features
- **MIDI CC and Note Input** [Medium] - External control, performance capability

### P2 - Polish & Documentation
- **State Highlight Timing Verification** [Low] - Audit SEQ/TOG timing
- **Dynamic Grid Layout** [Medium] - Responsive UI spacing
- **Tempo-Synced Delay** [Low] - DS parameter for musical delay times

### P3 - Future / Large Effort
- **Noise Source Integration** [Medium] - DSP addition (see v0.3.4_noise_source.md)
- **Oscillator Sync** [Medium] - DSP addition (see v0.3.4_oscillator_sync.md)
- **Additional Filter Types** [Medium] - DSP addition (see v0.3.4_filter_types.md)
- **Cross-Platform Compatibility** [High] - Linux/Windows/Intel Mac
- **Sample Playback System** [Very High] - Major feature
- **Additional Voice Types** [Very High] - Architecture change
- **Optional Polyphony** [Very High] - Architecture change

---

## Recent Updates (December 2025)

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
- Full DSP signal chain with 10+ effect blocks
- 77 real-time parameters

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
