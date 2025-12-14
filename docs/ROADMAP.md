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
| Plaits Integration | High | **DONE** |

---

## v0.4.0 Progress (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Plaits Pitch Control (PL.FREQ/PLF) | Low | **DONE** |
| Plaits Parameter Aliases (3-letter) | Low | **DONE** |
| PLTR Trigger Readout | Low | **DONE** |
| Multi-Voice Trigger Indicators (P/C) | Low | **DONE** |
| RND.PL Fixes (validation & scaling) | Low | **DONE** |

## v0.4.1 Progress (December 2025) - IN PROGRESS

| Feature | Effort | Status |
|---------|--------|--------|
| MFF/MFQ Filter Modulation Amounts | Low | **DONE** |
| Ctrl+Shift+Z Redo Fix | Low | **DONE** |
| Deprecated Noise Params Removal | Low | **DONE** |
| Plaits RST Coverage | Low | **DONE** |
| Debug Tier Refactor | Low | **DONE** |
| Noise Envelope & Gating | Medium | Deferred |
| Noise FM Routing | Low | Deferred |
| Oscillator Sync | Medium | Not started |

---

## Recent Updates (v0.4.1)

### MFF/MFQ Filter Modulation Amounts [COMPLETE]
Added granular control over filter modulation routing.

**Changes:**
- [x] Added MODF.CUT / MFF command (0-16383) - ModBus → Filter cutoff amount
- [x] Added MODF.RES / MFQ command (0-16383) - ModBus → Filter resonance amount
- [x] MC/MQ toggles now work in conjunction with MFF/MFQ amounts
- [x] Provides fine-grained control over modulation depth per filter parameter
- [x] Files: src/commands/synth/filter.rs, src/commands/aliases.rs

### Ctrl+Shift+Z Redo Fix [COMPLETE]
Fixed redo keybinding to properly handle uppercase Z.

**Changes:**
- [x] Redo now works correctly with Ctrl+Shift+Z
- [x] Fixed uppercase Z handling in keybinding system
- [x] Files: src/ui/input.rs

### Deprecated Noise Params Removal [COMPLETE]
Removed noise envelope parameters that were deprecated in multi-synth architecture split.

**Changes:**
- [x] Removed NA, ND, NC, NE, NG parameters from command system
- [x] Noise now uses simple volume control (NV) only
- [x] Envelope control removed as part of multi-synth architecture
- [x] Deferred: Full envelope/gating restoration (see "Noise Envelope & Gating" below)
- [x] Files: src/commands/validate.rs, docs/MANUAL.md

### Plaits RST Coverage [COMPLETE]
Added core Plaits oscillator parameters to RST command for complete reset coverage.

**Changes:**
- [x] Added pitch (131 Hz C3 default)
- [x] Added detune (0 cents default)
- [x] Added engine (0 default)
- [x] Added harmonics (8192/0.5 default)
- [x] Added timbre (8192/0.5 default)
- [x] Added morph (8192/0.5 default)
- [x] Added decay (8192/0.5 default)
- [x] Added lpg (8192/0.5 default)
- [x] Added plv main volume (8192/0.5 default)
- [x] Added pav aux volume (0 default)
- [x] RST now fully resets Plaits oscillator to defaults
- [x] Files: src/commands/system/misc.rs

### Debug Tier Refactor [COMPLETE]
Centralized command execution through ExecutionContext and fixed tier violations.

**Architecture Changes:**
- [x] Created ExecutionContext struct grouping 47+ parameters
- [x] Reduced process_command signature from 109 → 3 parameters
- [x] Eliminated 165 duplicate tier checks across codebase
- [x] Centralized output control via ExecutionContext.output()
- [x] Files: src/commands/context.rs, src/commands/mod.rs

**Tier Violation Fixes:**
- [x] Fixed SLEW/SLEW.ALL tier violation (tier 1 → tier 4)
- [x] Fixed PRINT tier violation (tier 1 → tier 2)
- [x] Added missing tier gates to SAVE, LOAD, SCENES, DELETE
- [x] Added missing tier gates to PSET, PSET.SAVE, PSET.DEL, PSETS
- [x] Added missing tier gates to REC, REC.STOP, REC.PATH
- [x] All commands now respect tier 0 (TIER_SILENT) properly
- [x] Files: src/commands/slew.rs, src/commands/system/misc.rs
- [x] Files: src/commands/system/scene.rs, src/commands/system/preset.rs

**Documentation:**
- [x] Created DEBUG_TIERS.md - Complete tier classification
- [x] Created TIER_0_COMMANDS.md - Tier 0 analysis
- [x] Created TIER_FIXES_SUMMARY.md - Fix summary
- [x] Updated ARCHITECTURE.md - Added ExecutionContext section
- [x] Updated CHANGELOG.md - Added tier fixes to v0.4.1

---

## Prioritized Incomplete Items

### P1 - High Value Features
- **MIDI CC and Note Input** [Medium] - External control, performance capability

### P2 - Polish & Documentation
- **CPU Readout Fixed Width** [Low] - Ensure CPU percentage always uses 2-digit space formatting in header to prevent BPM readout from shifting horizontally
- **Alias & Command Name Standardization** [Low] - Audit all commands for consistent naming patterns; ensure all parameters have appropriate short-form aliases; standardize canonical form patterns (e.g., CATEGORY.PARAM format); document naming conventions for future development
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

### Plaits Enhancements [v0.3.7]

**PL.FREQ - Pitch/Frequency Control**
- Native-style command taking raw +/- 14-bit values
- Note name (N) support with scale-aware transposition
- Routes to monokit_plaits pitch parameter
- Independent from complex oscillator pitch (PF)

**3-Letter Parameter Aliases**
- PLH → PL.HARM (harmonics)
- PLT → PL.TIMB (timbre)
- PLE → PL.ENG (engine)
- PLM → PL.MORPH (morph)
- PLD → PL.DEC (decay)
- PLL → PL.LPG (lowpass gate)
- Improves legibility and distinguishes from complex oscillator commands

**PLTR Trigger Readout**
- Add REPL confirmation when PLTR is triggered
- Similar to TR readout for complex oscillators
- Shows engine number and current parameter state

**Multi-Voice Trigger Indicators** ✓
- [x] Header indicators for trigger state:
  - P = Plaits trigger active
  - C = Complex oscillators trigger active (renamed from HD2)
- [x] Single-character display for space efficiency
- [x] Replaces current "TR" indicator with multi-voice awareness

**Terminology Update: HD2 → Complex** (In Progress)
- [x] Rename "TR" to "C/P" in UI header indicators
- [ ] Update remaining HD2 references in docs to "Complex"
- C = Complex (dual FM oscillator architecture)
- P = Plaits (Mutable Instruments macro oscillator)
- Note: Core docs already use appropriate terminology

