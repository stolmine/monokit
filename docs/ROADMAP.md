# Monokit Development Roadmap

## Overview

Monokit is a text-based scripting language for a monophonic drum synthesizer built on a complex oscillator. It bridges the gap between sequencer-first tools (TidalCycles, Strudel) and synth-first engines (Plaits), offering tight integration between a Teletype-inspired scripting interface and a dedicated complex oscillator voice. In addition to its complex voice, it hosts a sample playback engine capable of loading loops for slicing and folders for one shot switching, and an implementation of plaits. 

**Architecture:** Rust CLI + SuperCollider sound engine
**Philosophy:** CLI-native, headless-capable, Teletype-inspired terse command syntax

---

## Current Development

*No active development cycle. See Future Priorities for upcoming work.*

**v0.6.0 Windows Support** - Completed January 2026

---

## Future Priorities

### P1 - High Value Features
- **MIDI CC and Note Input** [Medium] - External control, performance capability
- ~~**Compressor Visualization**~~ ✅ DONE - IN/OUT meters + GR display in GRID.MODE 2; fixed OSC connection for scsynth-direct mode
- ~~**EQ Visualization**~~ ✅ DONE - Real-time frequency response curve in GRID.MODE 2; braille rendering of 3-band EQ response
- **Layered Scope Visualizations** [Medium] - Architecture for overlaying multiple visualizations (scope, spectrum, EQ, compressor) on single page; toggle layers independently
- **Customizable Grid Array** [Medium] - User-configurable grid parameters via settings file; allow users to choose which parameters appear on grid and their layout
- **Trackpad/Mousewheel Scrolling** [Low-Medium] - Implement scrolling support for help pages, REPL history, and other scrollable views; improve navigation UX

### P2 - Polish & Features
- ~~**Windows Theme Defaults**~~ ✅ DONE - Bundled themes now populate config.toml on first run (all platforms)
- **Update command_reference.md** [Low] - Sync with current command set (MiClouds, sampler commands, delay sync, etc.)
- ~~**Verify Windows SC Bundle Pathing**~~ ✅ DONE - Fixed scsynth.exe path discovery (v0.6.1); bundled path check now uses correct .exe extension on Windows
- **cpal Audio Device Enumeration** [Medium] - Replace platform-specific audio device code (coreaudio-rs) with cpal for cross-platform device enumeration and runtime selection
- **Keyboard Internationalization** [Medium] - Force/detect international keyboard layouts; Spanish keyboard user reported [ ] page nav broken; **Workaround (v0.6.1):** Ctrl+[/] now works as alternative page navigation
- **F11 Windows Terminal Conflict** [Low] - F11 (Pattern page) conflicts with Windows Terminal fullscreen shortcut; need alternative binding or documentation
- **Per-Voice FX Routing** [Medium] - Individual routing options per voice (primary, noise, plaits, sampler) to mix/bypass global FX chain flexibly; enable dry/wet/blend modes for each voice against delay/reverb
- **Error Message Refactor** [Low-Medium] - Standardize error output system-wide: restore "ERROR:" prefix for all error messages; implement red color for error messages; ensure 46-char compliance with prefix
- **Multiple Reverb Types** [Medium] - Add alternative reverb algorithms (JVerb, FDN, etc.); allow switching between reverb types; maintain current FreeVerb as default
- **Script Mute Hotkeys (Terminal Research)** [Medium] - Fix Ctrl+Shift+1-8/M/I hotkeys not working; terminal emulators handle Ctrl+Shift differently; research alternatives
- **Global Distortion/Saturation** [Low-Medium] - Natural-sounding saturation/distortion with antialiasing; multiple modes (tube, tape, soft clip, fold)
- **Alias & Command Name Standardization** [Low] - Audit all commands for consistent naming patterns; standardize canonical form patterns; add short aliases for verbose commands (e.g., GRID.MODE needs GM or similar)
- ~~**Grid Mode Toggle Hotkey**~~ ✅ DONE - Ctrl+G cycles GRID.MODE 0-5 on live page; SCRMBL.GRID/SG for independent grid scramble toggle
- **Help System Cleanup** [Low] - Comb through help to find and remove deprecated commands - done
- **Dynamic Grid Layout** [Medium] - Responsive UI spacing
- **Tempo-Synced Delay** [Low] - DS parameter for musical delay times - done
- **Manual Update + Voice Architecture Diagram** [Low] - Fill documentation gaps, add ASCII voice architecture diagram
- **Gain Staging Audit** [Medium] - Review clipping behavior with modbus/noise routing
- **Slew Coverage Expansion** [Low] - Extend SLEW to all continuous voice parameters (currently 30/88)
- **Envelope Parameter Consistency** [Low] - Sampler uses 0-16383 mapped to ms, HD2 uses raw ms directly; standardize approach across all voices for consistent UX
- **P.CLR Pattern Clear Command** [Low] - Clear all values in working pattern to 0; reset pattern to empty state
- ~~**Sampler Grid Filter Type Display**~~ ✅ DONE - TYP field in GRID.MODE 5 now shows numerical designation (0-13)
- ~~**Unicode Spinner Animation**~~ ✅ DONE - REC.SPINNER command with 7 spinner styles synced to metro beat; configurable via REC.SPINNER 0-6

### P2 - Bug Fixes & Stability
- ~~**Confirmation Dialog Styling Bug**~~ ✅ FIXED - Added Clear widget before dialog render
- ~~**Scene Loading Audio Crashes**~~ ✅ FIXED - Configurable RST delays prevent OSC flooding
- ~~**Compressor Auto Makeup Hunting**~~ ✅ FIXED - Replaced feedback-loop algorithm with static formula; metering now uses compressor attack/release times
- ~~**M Script Validation Logic Audit**~~ ✅ FIXED - $ command dispatch was discarding returned script indices
- **Stateful Highlighting Issues** [Medium] - Address unintended consequences in current stateful highlighting system
- **PRE Command Same-Line Usage** [Low-Medium] - Clarify and validate same-line PRE usage and execution order

### P2 - Deferred Features
- ~~**Noise Envelope & Gating**~~ ✅ FIXED - Noise now routes through HD2's shared VCA/EG; respects VCA.MODE (0=drone, 1=enveloped)
- **Noise FM Routing Fix** [Low] - Ensure noise FM (NP/NM) works independently of NV volume; users should be able to use noise as FM source without hearing it
- **Oscillator Sync** [Medium] - Hard/soft sync between oscillators for classic analog tones

### P3 - Future / Large Effort
- ~~**Cross-Platform Compatibility**~~ ✅ DONE - Linux (v0.5.2), Windows (v0.6.0); Intel Mac remaining - see `docs/CROSS_PLATFORM_PORT.md` for implementation guide
- **Sample Playback System** [Very High] - Major feature (see SAMPLE_PLAYBACK_DESIGN.md) - done v0.5.0
- **Song Mode/Arranger** [Very High] - Pattern chaining, arrangement sequencing, section management for complete song construction beyond loop-based performance
- **Custom Keybinds** [High] - User-configurable keyboard shortcuts via config file; addresses terminal conflicts (F11, Ctrl+Shift) and international keyboard layouts
- **Command Naming & Param Organization Overhaul** [Medium] - Comprehensive consistency pass for all command names, aliases, and parameter organization; establish and document naming conventions; improve intuitiveness across voice types (osc, noise, plaits, sampler); unify modbus/modulation patterns
- **Additional Voice Types** [Very High] - Architecture change
- **Optional Polyphony** [Very High] - Architecture change

---

## Version History (Latest First)

### v0.6.0 (January 2026) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Windows Platform Support | High | **DONE** |
| Windows CI/CD Pipeline | Medium | **DONE** |
| SC 3.14+ Compatibility Fixes | Low | **DONE** |
| Platform-Aware Sample Paths | Low | **DONE** |
| Documentation Updates | Low | **DONE** |

**Windows Platform Support** - Full Windows x86_64 support with self-contained ZIP bundle. Config stored in `%APPDATA%\monokit\`. SuperCollider 3.14.1, sc3-plugins, and mi-UGens bundled.

**Windows CI/CD Pipeline** - Automated Windows builds in GitHub Actions release workflow. Builds alongside macOS and Linux. SHA256 checksums for all release artifacts.

**SC 3.14+ Compatibility Fixes** - Select.ar rate matching (K2A.ar wrapper for control-rate selectors). Limiter multichannel pattern fix. Windows SC3plugins path discovery (.scx at root vs subdirs).

**Platform-Aware Sample Paths** - Fixed hardcoded `~/.config/monokit/samples` to use `monokit_config_dir()` for cross-platform support.

**Documentation Updates** - README, MANUAL, CHANGELOG updated with Windows installation instructions, config paths, and terminal recommendations.

---

### v0.5.32 (January 2026) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| AppImage PipeWire Compatibility | Low | **DONE** |
| CI AppImage FUSE Workaround | Low | **DONE** |
| Release Artifact Glob Fix | Low | **DONE** |

**AppImage PipeWire Compatibility** - Excluded libjack from Linux bundle so AppImage uses system's pipewire-jack library. Bundled libjack from Ubuntu build environment was incompatible with PipeWire systems.

**CI AppImage FUSE Workaround** - Extract appimagetool before running to bypass FUSE requirement on GitHub Actions runners.

**Release Artifact Glob Fix** - Use recursive globs (`dist/**/*.tar.gz`) in release workflow to find Linux artifacts in subdirectories.

---

### v0.5.2 (January 2026) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Linux Audio Support (PipeWire/JACK) | Medium | **DONE** |
| GitHub Actions Linux Build | Medium | **DONE** |
| AppImage Distribution | Low | **DONE** |
| AUR PKGBUILD | Low | **DONE** |
| SC 3.14+ Rate Matching Fix | Low | **DONE** |

**Linux Audio Support** - Added platform-aware audio routing with automatic PipeWire/JACK connection via pw-link. SuperCollider JACK ports auto-connect to system playback on startup. Platform-conditional plugin paths for Linux system and user extensions.

**GitHub Actions Linux Build** - Complete CI/CD pipeline for Linux x86_64. Builds tarball and AppImage on Ubuntu runner with headless sclang (QT_QPA_PLATFORM=offscreen). Publishes to GitHub Releases alongside macOS builds.

**AppImage Distribution** - Portable single-file distribution for Linux. Self-contained bundle with scsynth, plugins, synthdefs, and shared libraries. No installation required.

**AUR PKGBUILD** - Arch User Repository package definition prepared (packaging/aur/PKGBUILD). Downloads tarball from GitHub releases, installs to /usr/lib/monokit/ with wrapper script for LD_LIBRARY_PATH.

**SC 3.14+ Rate Matching Fix** - Fixed Limiter Select.ar requiring audio-rate selector in SuperCollider 3.14+. Wrapped selector with K2A.ar() for control-to-audio rate conversion.

---

### v0.5.0 (January 2026) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Sampler + MiRings Resonator | High | **DONE** |
| FX Chain Reorder | Medium | **DONE** |
| Delay Clock Sync | Medium | **DONE** |
| GRID.MODE 4/5 Visualizations | Medium | **DONE** |
| 14 Sampler Filter Types | Low | **DONE** |
| DRY Refactoring Pass | Low | **DONE** |

**Sampler System** - Complete sample playback with kit loading (KIT), slice triggering (STR), onset detection (S.ONSET), and MiRings physical modeling resonator. 25+ new commands for playback, FX, and modbus routing.

**FX Chain Reorder** - Signal chain restructured: Mix → EQ → Pan → Pitch Shift → Beat Repeat → Clouds → Delay → Comp → Reverb. Removed broken comb resonator.

**Delay Clock Sync** - DS/DT now 14-bit with BPM sync mode. Fixed CombC delay bug.

**Grid Visualizations** - GRID.MODE 4 (FX dry/wet levels), GRID.MODE 5 (sampler state). Ctrl+G hotkey for cycling.

**Code Quality** - DRY pass consolidating EQ handlers (82% reduction), FX mix handlers, shared utilities. File size audit complete.

---

### v0.4.4 (January 2026) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| GRID.MODE 2 FX Visualization | Low | **DONE** |
| Compressor Auto-Makeup Fix | Low | **DONE** |
| Compressor NaN Guard | Low | **DONE** |

**GRID.MODE 2 FX Visualization** - Improved FX visualization layout with compressor and EQ displays.

**Compressor Auto-Makeup** - Fixed auto-makeup gain to be proportional to gain reduction for more natural behavior.

**Compressor NaN Guard** - Added NaN protection, meter hysteresis, and EQ layout fixes for stability.

---

### v0.4.3 (January 2026) - COMPLETE

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

---

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
| Beat Repeat/Pitch Shift Alias Registration | Low | **DONE** |
| Envelope Parameter Tier Fixes | Low | **DONE** |

**Alias Registration** - Fixed beat repeat and pitch shift short aliases not being registered in command validation (aliases themselves were added in v0.4.1).

**Envelope Parameter Tier Fixes** - Fixed envelope parameter macros to respect tier system properly.

---

### v0.4.11 (December 2025) - COMPLETE

| Feature | Effort | Status |
|---------|--------|--------|
| Plaits RST Coverage | Low | **DONE** |
| Debug Tier Refactor | Low | **DONE** |
| ExecutionContext Refactor | Medium | **DONE** |
| ExecutionContext Test Updates | Low | **DONE** |
| Dynamic Bundle Size in Release | Low | **DONE** |
| CPU Readout Fixed Width | Low | **DONE** |

**Plaits RST Coverage** - Added Plaits oscillator parameters to RST command.

**Debug Tier Refactor** - Fixed debug tier filtering on output commands; DEBUG now properly controls output levels.

**ExecutionContext Refactor** - Centralized command execution context for cleaner code organization.

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
| Pitch Shift/Beat Repeat Short Aliases | Low | **DONE** |
| Ctrl+Shift+Z Redo Fix | Low | **DONE** |

**MFF/MFQ Commands** - Added MODF.CUT/MFF and MODF.RES/MFQ for granular control over filter modulation routing.

**PS/BR Aliases** - Added short-form aliases for pitch shift and beat repeat commands.

**Redo Keybinding** - Fixed Ctrl+Shift+Z redo keybinding handling.

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
- Cross-platform: macOS (Homebrew), Linux (AppImage/tarball), Windows (ZIP)
- Automated release pipeline (GitHub Actions)
- Theme system with 48 themes
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
