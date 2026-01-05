# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice.

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## Version History (Latest First)

### v0.4.22 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| MiClouds Granular Effect | Low-Medium | **DONE** |
| EQ High Shelf Frequency Adjustment | Low | **DONE** |

**MiClouds Granular Effect** - Integrated MiClouds UGen with 14 parameters (CL.*) for granular synthesis/processing. Full command set with aliases, pattern triggering via CL.TRIG, 4 processing modes (grain/pitch/loop/spectral), freeze mode for sound design. Placed in signal chain after pan, before beat repeat. Integrated with RND.FX and RST. Complete documentation in MANUAL.md and help system. Output gain boosted to 2.69x for proper mixing presence.

**EQ Adjustments** - High shelf frequency moved from 4000Hz to 2500Hz for better tonal balance.

---

### v0.4.21 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| MiClouds Output Gain Boost | Low | **DONE** |

**MiClouds Output Level** - Increased MiClouds output gain from 1x to 2.69x for proper mixing presence.

---

### v0.4.12 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Beat Repeat/Pitch Shift Short Aliases | Low | **DONE** |
| Envelope Parameter Tier Fixes | Low | **DONE** |
| ExecutionContext Test Updates | Low | **DONE** |
| Dynamic Bundle Size in Release | Low | **DONE** |
| CPU Readout Fixed Width | Low | **DONE** |
| Title Timer Persistence Fix | Low | **DONE** |

---

### v0.4.11 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| MFF/MFQ Filter Modulation Amounts | Low | **DONE** |
| Ctrl+Shift+Z Redo Fix | Low | **DONE** |
| Deprecated Noise Params Removal | Low | **DONE** |
| Plaits RST Coverage | Low | **DONE** |
| Debug Tier Refactor | Low | **DONE** |
| ExecutionContext Refactor | Medium | **DONE** |

---

### v0.4.2 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Script Mutes (hotkeys + commands) | Low | **DONE** |
| Page Navigation Commands | Low | **DONE** |
| Title Timer Persistence Fix | Low | **DONE** |
| Compressor Dry/Wet Mix | Low | **DONE** |
| DC (Discontinuity) Fix | Low | **DONE** |
| MiClouds Commands (14 total) | Low-Medium | **DONE** |
| CLR Alias Conflict Resolution | Low | **DONE** |

**MiClouds Granular Effect** - All 14 Clouds commands (CL.*) added and validated. Fixed DC command modulator routing. Resolved CLR alias conflict (CLR=CLEAR, clouds reverb uses CLRV). Full integration with RND.FX and RST.

---

### v0.4.1 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Filter Modulation Amount Controls | Low | **DONE** |
| SynthDef/RST Default Alignment | Low | **DONE** |
| Envelope Parameter Scaling Fix | Low | **DONE** |

**MFF/MFQ Commands** - Added MODF.CUT/MFF and MODF.RES/MFQ for granular control over filter modulation routing.

---

### v0.4.0 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Plaits Pitch Control (PL.FREQ/PLF) | Low | **DONE** |
| Plaits Parameter Aliases (3-letter) | Low | **DONE** |
| PLTR Trigger Readout | Low | **DONE** |
| Multi-Voice Trigger Indicators (H/P) | Low | **DONE** |
| RND.PL Fixes (validation & scaling) | Low | **DONE** |

---

### v0.3.61 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Validation-Aware Highlighting | Medium | **DONE** |
| State Highlight Mitigation | Medium | **DONE** |
| Stateful Operator Bug Fixes | Medium | **DONE** |

**Highlighting System Overhaul** - Fixed critical bugs in stateful operator (TOG/SEQ/EITH) highlighting. Implemented validation-aware highlighting, state snapshots, and proper rollback coverage.

---

### v0.3.6 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| CPU Monitoring Fix | Low | **DONE** |
| Plaits Integration | High | **DONE** |

**Plaits Integration** - Added Mutable Instruments Plaits as 5th sound source with 16 synthesis engines and 9 control commands.

---

### v0.3.5 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Multi-Synth Architecture | High | **DONE** |
| Parameter Routing System | Medium | **DONE** |
| Build System Reliability | Low | **DONE** |
| Scene Path Consistency | Low | **DONE** |

**Multi-Synth Architecture** - Restructured from monolithic SynthDef into 4 modular synths to fix SuperCollider optimizer bug causing parameter cross-talk.

---

### v0.3.4 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Additional Filter Types (14 total) | Medium | **DONE** |
| Noise Source Integration | Medium | **DONE** |
| ModBus Envelope (MBA/MBD) | Low | **DONE** |
| Script Undo/Redo | Low | **DONE** |
| Frappe Theme | Low | **DONE** |
| MC/MQ Filter Routing | Low | **DONE** |

---

### v0.3.2 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Beat Repeat Fix | Low | **DONE** |
| Config Command Output Fix | Low | **DONE** |

---

### v0.3.0 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| SYNC Command | Low | **DONE** |
| AUTOLOAD Setting | Low | **DONE** |
| Script Validation Overhaul | Medium | **DONE** |
| Pattern Operations as Expressions | Medium | **DONE** |

---

### Older Versions

**v0.2.0** - scsynth-direct integration, bundled binary distribution
**v0.1.1** - Initial Homebrew tap release
**v0.1.0** - Initial release with core voice and pattern system

---

## Future Versions

### v0.5.0 - Sampler & Grid Expansion (IN PROGRESS)

**See `docs/SAMPLER_DESIGN.md` for detailed design**

| Feature | Effort | Status |
|---------|--------|--------|
| **Phase 1: Mixer Mode** | | |
| VOL.*/PAN.*/MUTE.* commands | Low | **DONE** |
| GRID.MODE 3 mixer display | Medium | **DONE** |
| monokit_main per-voice mixing | Medium | **DONE** |
| **Phase 2: Basic Sampler** | | |
| KIT/STR command parsing | Low | **DONE** |
| Folder loading / kit mode | Low | **DONE** |
| S.* playback params (16 cmds) | Low | **DONE** |
| SF.* FX params (9 cmds) | Low | **DONE** |
| OSC param routing | Low | **DONE** |
| Buffer loading (/b_allocRead) | Medium | **DONE** |
| monokit_main bus 21 routing | Low | **DONE** |
| monokit_sampler SynthDef | Medium | **DONE** |
| **Phase 3: Slicing** | | |
| S.SLICE equal division | Low | **DONE** |
| STR sends frame boundaries | Low | **DONE** |
| hound WAV metadata | Low | **DONE** |
| S.ONSET energy-based detection | Medium | **DONE** |
| S.ONSET.MIN spacing param | Low | **DONE** |
| Spectral flux upgrade (rustfft) | Medium | Future |
| GRID.MODE 5 slice visualization | Medium | Pending |
| **Phase 4: Polish** | | |
| Sample params coverage for RST | Low | **DONE** |
| SFX command for FX routing | Low | Pending |
| Filter options (more types) | Medium | Pending |
| Replace glitch effect | Medium | Pending |
| S.PITCH range & note LUT | Low | Pending |
| Gate length control | Low | Pending |
| STR indicator 'S' in header | Low | **DONE** |
| KIT.LEN / KL getter | Low | **DONE** |
| KIT.INFO command | Low | **DONE** |
| KIT getter (no arg lists kits, REPL only) | Low | **DONE** |
| GRID.MODE 4 FX viz | Medium | Placeholder |
| GRID.MODE 5 sampler viz | Medium | Placeholder |
| Help system updates | Low | **DONE** |
| Scene persistence | Medium | Pending |

**Completed this session:**
- S.ONSET transient detection with exponential sensitivity curve
- S.ONSET.MIN minimum spacing parameter
- Pure-Rust energy-based onset detector (no external deps)
- KIT listing now shows file extensions

---

### v0.4.3 - Stability & Polish (COMPLETE)

**See `docs/V0.4.3_PLAN.md` for detailed progress tracking**

| Category | Status |
|----------|--------|
| Critical Bug Fixes | 4/7 DONE |
| Quick Polish Wins | 5/5 DONE |
| FX Visualization | 2/2 DONE |
| Documentation Cleanup | Deferred |
| Stretch Goals | Deferred |

**Critical Fixes:**
- **Audio engine loss on scene load** - Sequencing continues and UI remains responsive, but meters show no activity and audio output is silent; no specific scene combination isolated as cause
- ~~**RST Plaits squeals**~~ ✅ FIXED - Changed resonator/ring mod frequencies from 440Hz to 131Hz
- **RST should run Init script** - RST command should execute Init script (I) after reset; watch for race conditions with parameter sends
- ~~**Debug levels broken**~~ ✅ FIXED - DEBUG now synchronizes OUT.* flags; DEBUG 0 silences all output
- ~~**Scene loading audio crashes**~~ ✅ FIXED - Configurable RST delays prevent OSC flooding
- Stateful highlighting issues
- ~~**Script entry validation refactor**~~ ✅ FIXED - Centralized command registry with 408 commands

**Quick Polish:**
- ~~Scope gain parameter + reset command~~ ✅ DONE - SCOPE.GAIN, SCOPE.RST
- ~~Programmable EQ shelving frequencies (ELF/EHF)~~ ✅ DONE - ELF, EHF commands
- ~~Compressor auto-makeup toggle (COMP.AUTO)~~ ✅ DONE - COMP.AUTO/CAU implemented
- ~~**Confirmation dialogs (persistent settings)**~~ ✅ DONE - CFM.QUIT, CFM.SAVE commands; quit protection when metro active + named scene loaded
- ~~N1-N4 help section relocation~~ ✅ DONE - Moved to VARIABLES & MATH section

**Documentation:**
- CHANGELOG cleanup
- Documentation audit (README, MANUAL, VOICE_ARCHITECTURE)
- Reorganize internal docs to `docs/internal/`

**Stretch:**
- Error message refactor (restore "ERROR:" prefix, red color)
- PRE command same-line usage clarification


## Prioritized Incomplete Items

### P1 - High Value Features
- **MIDI CC and Note Input** [Medium] - External control, performance capability
- ~~**Compressor Visualization**~~ ✅ DONE - IN/OUT meters + GR display in GRID.MODE 2; fixed OSC connection for scsynth-direct mode
- ~~**EQ Visualization**~~ ✅ DONE - Real-time frequency response curve in GRID.MODE 2; braille rendering of 3-band EQ response
- **Layered Scope Visualizations** [Medium] - Architecture for overlaying multiple visualizations (scope, spectrum, EQ, compressor) on single page; toggle layers independently
- **Customizable Grid Array** [Medium] - User-configurable grid parameters via settings file; allow users to choose which parameters appear on grid and their layout
- **Trackpad/Mousewheel Scrolling** [Low-Medium] - Implement scrolling support for help pages, REPL history, and other scrollable views; improve navigation UX

### P2 - Polish & Features
- **Per-Voice FX Routing** [Medium] - Individual routing options per voice (primary, noise, plaits, sampler) to mix/bypass global FX chain flexibly; enable dry/wet/blend modes for each voice against delay/reverb
- **Error Message Refactor** [Low-Medium] - Standardize error output system-wide: restore "ERROR:" prefix for all error messages; implement red color for error messages; ensure 46-char compliance with prefix
- **Multiple Reverb Types** [Medium] - Add alternative reverb algorithms (JVerb, FDN, etc.); allow switching between reverb types; maintain current FreeVerb as default
- **Script Mute Hotkeys (Terminal Research)** [Medium] - Fix Ctrl+Shift+1-8/M/I hotkeys not working; terminal emulators handle Ctrl+Shift differently; research alternatives
- **Global Distortion/Saturation** [Low-Medium] - Natural-sounding saturation/distortion with antialiasing; multiple modes (tube, tape, soft clip, fold)
- **Alias & Command Name Standardization** [Low] - Audit all commands for consistent naming patterns; standardize canonical form patterns
- **Help System Cleanup** [Low] - Comb through help to find and remove deprecated commands
- **Dynamic Grid Layout** [Medium] - Responsive UI spacing
- **Tempo-Synced Delay** [Low] - DS parameter for musical delay times
- **Manual Update + Voice Architecture Diagram** [Low] - Fill documentation gaps, add ASCII voice architecture diagram
- **Gain Staging Audit** [Medium] - Review clipping behavior with modbus/noise routing
- **Slew Coverage Expansion** [Low] - Extend SLEW to all continuous voice parameters (currently 30/88)

### P2 - Bug Fixes & Stability
- ~~**Confirmation Dialog Styling Bug**~~ ✅ FIXED - Added Clear widget before dialog render
- ~~**Scene Loading Audio Crashes**~~ ✅ FIXED - Configurable RST delays prevent OSC flooding
- **Stateful Highlighting Issues** [Medium] - Address unintended consequences in current stateful highlighting system
- **PRE Command Same-Line Usage** [Low-Medium] - Clarify and validate same-line PRE usage and execution order

### P2 - Deferred Features
- **Noise Envelope & Gating** [Medium] - Re-implement envelope and gate control in noise synth after multi-synth architecture split; add NA, ND, NC, NE parameters back to monokit_noise SynthDef; separate noise audio output from FM routing
- **Noise FM Routing Fix** [Low] - Ensure noise FM (NP/NM) works independently of NV volume; users should be able to use noise as FM source without hearing it
- **Oscillator Sync** [Medium] - Hard/soft sync between oscillators for classic analog tones

### P3 - Future / Large Effort
- **Cross-Platform Compatibility** [High] - Linux/Windows/Intel Mac
- **Sample Playback System** [Very High] - Major feature (see SAMPLE_PLAYBACK_DESIGN.md)
- **Song Mode/Arranger** [Very High] - Pattern chaining, arrangement sequencing, section management for complete song construction beyond loop-based performance
- **Additional Voice Types** [Very High] - Architecture change
- **Optional Polyphony** [Very High] - Architecture change

---

## Current Feature Set

For detailed completion records, see `CHANGELOG.md` and `docs/history/`

### Core Voice & DSP
- Complex dual oscillator with FM, discontinuity, and modulation routing
- Plaits macro oscillator (16 synthesis engines)
- Noise source with waveform selection
- Full DSP signal chain with 10+ effect blocks
- MiClouds granular synthesis/processing
- 88+ real-time parameters

### Language & Scripting
- Page-based interface with 8 scripts + Metro + Init
- Pattern system: 6 patterns × 64 steps
- Full control flow: IF/ELIF/ELSE, loops, probability, scheduling
- SEQ inline sequencing with mini notation
- Expression evaluation and stateful operators (TOG, EITH)
- Euclidean and prime rhythm generators (ER, NR)

### Infrastructure
- Direct scsynth integration (bundled binary)
- Automated release pipeline with Homebrew
- Theme system with 30+ themes
- Scene and preset management
- Audio recording via DiskOut

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

