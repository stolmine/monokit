# Monokit Changelog

Version history and recent updates. For detailed development phase records, see `history/PHASES.md`.

---

## Version 0.3.4 (December 2025)

### DSP Improvements

**ModBus Routing Fix [COMPLETE]**
- All modbus routes now use audio-rate modOsc signal instead of DC offset
- Routes affected: MP, MD, MM, MC, MA
- Enables actual vibrato, tremolo, filter sweeps, and dynamic modulation

**ModBus Envelope [COMPLETE]**
- MBA <0-16383> / MBEV.AMT - ModBus envelope amount
- MBD <1-10000> / MBEV.DEC - ModBus envelope decay (milliseconds)

**MC/MQ Filter Routing [COMPLETE]**
- MC <0|1> - ModBus → Filter cutoff routing (renamed from MF.F)
- MQ <0|1> - ModBus → Filter resonance routing (new)
- Follows M + target naming convention (MC = mod→cutoff, MQ = mod→Q)

### UI/UX Improvements

**Script Undo/Redo [COMPLETE]**
- Ctrl+Z / Ctrl+Shift+Z for undo/redo
- Page-local: 11 independent stacks, max 50 actions
- Non-destructive editing with full history

**Frappe Theme [COMPLETE]**
- Added Frappe theme to theme system
- Available via THEME command

**ER/NR Rhythm Operators [COMPLETE]**
- ER <fill> <length> <step> - Euclidean rhythm generator
- NR <prime> <mask> <factor> <step> - Numeric repetitor
- Enhanced pattern generation capabilities

### Build Pipeline Improvements

**Bundle Script Auto-Compilation [COMPLETE]**
- scripts/bundle.sh now automatically compiles SynthDefs before bundling

---

## Version 0.3.3 (December 2025)

### New Features

**VERSION Command [COMPLETE]**
- Added VERSION/VER command to display version
- Version shown in startup message and help page header

**Line Duplicate Behavior [COMPLETE]**
- Ctrl+D now pushes lines down instead of overwriting
- Non-destructive line duplication

### Bug Fixes

**TOG Zero Parsing Fix [COMPLETE]**
- Fixed display corruption with TOG using zero
- Added `find_whole_word()` word-boundary matcher

**AUDIO Device Query Fix [COMPLETE]**
- Fixed `AUDIO 1` failing without prior `AUDIO` call
- Populate audio_devices on App initialization

---

## Version 0.3.1 (December 2025)

### New Commands
- `LOAD.CLR <0|1>` - Clear REPL on load
- `AUTOLOAD <0|1>` - Auto-load last scene on startup
- `LIMIT <0|1>` - Limiter on/off
- `SYNC` - Reset all stateful elements to starting position
- `SYNC.SEQ` - Reset SEQ sequences to first element
- `SYNC.TOG` - Reset TOG toggles to first value
- `SYNC.PAT` - Reset pattern indices to 0

### Config Command Output Fixes [COMPLETE]
All config and query commands now output to REPL regardless of DEBUG level.

---

## Recent Updates (December 2025)

### Envelope Parameter Scaling Fix [COMPLETE]
- FA (FM envelope amount): 0-16 float → 0-16383 int
- DA (Disc envelope amount): 0-16 float → 0-16383 int

### SynthDef Default Parameter Alignment [COMPLETE]
- pf: 200 → 131 (C3, ~131 Hz)
- mf: 50 → 262 (C4, ~262 Hz)
- pa: 4 → 0 (no pitch envelope by default)
- fc: 1000 → 10000 (filter wide open)
- cr: 4 → 1 (compressor off by default)

### Control Flow Investigation [COMPLETE]
- IF/ELSE/ELIF scope is SCRIPT-LOCAL
- All boolean operators verified working
- Nested IF in loops fixed
- SEQ quote parsing fixed

### Direct scsynth Integration [COMPLETE]
- Monokit now runs without sclang, spawning scsynth directly
- Bundle structure: monokit binary + scsynth + plugins + synthdefs
- Total bundle size: ~13 MB (vs ~200 MB for full SC)

### Recording in scsynth-direct Mode [COMPLETE]
- Uses DiskOut UGen for streaming audio to disk
- Output format: 24-bit stereo WAV @ 48kHz

### Phase 6: Release Preparation [COMPLETE]
- Automated release workflow via GitHub Actions
- Homebrew formula auto-updates after releases
- GPL-2.0 license added
- 47 themes bundled at compile time

### Terminal Compatibility System [COMPLETE]
- Terminal capability detection at startup
- 256-color theme fallback when true color unavailable
- COMPAT, COMPAT.MODE, METER.ASCII commands

### Script Undo/Redo [COMPLETE]
- Ctrl+Z / Ctrl+Shift+Z for undo/redo
- Page-local: 11 independent stacks, max 50 actions

### NR and ER Operators [COMPLETE]
- ER <fill> <length> <step> - Euclidean rhythm generator
- NR <prime> <mask> <factor> <step> - Numeric repetitor

---

For complete historical records, see:
- `history/CHANGELOG.md` - Detailed completion records
- `history/PHASES.md` - Development phases 1-6
- `history/FUTURE.md` - Phase 7-8 plans
