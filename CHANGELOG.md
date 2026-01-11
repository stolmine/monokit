# Changelog

All notable changes to monokit. Versions ordered newest to oldest.

---

## v0.5.1 (January 2026)

### New Features

**REC.SPINNER Command**
- Recording indicator now shows animated spinner synced to metro beat
- 7 configurable spinner styles via REC.SPINNER 0-6:
  - 0: CIRCLED (⊕⊖⊗⊘⊙⊚⊛⊜⊝)
  - 1: FILL (○◔◑◕●◕◑◔)
  - 2: BREATHE (▉▊▋▌▍▎▏▎▍▌▋▊▉)
  - 3: BRAILLE (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
  - 4: DOT (⠁⠂⠄⡀⢀⠠⠐⠈)
  - 5: STAR (✶✸✹✺✹✸)
  - 6: HALF (◐◓◑◒)

### Bug Fixes

- **Noise VCA routing** - Noise now routes through HD2's shared VCA/EG; respects VCA.MODE (0=drone, 1=enveloped)
- **Recording silence** - Fixed silence at start of recordings by pre-filling DiskOut buffer before opening file
- **Duplicate REC messages** - Removed duplicate start/stop messages (now only from UI handler)
- **CPU readout width** - CPU percentage now right-aligned with 3-character width for stable display

---

## v0.5.0 (January 2026)

### Major Features

**Sampler System with MiRings Resonator**
- Complete sample playback engine with kit loading (KIT), slice triggering (STR)
- S.ONSET auto-slicing with transient detection (pure Rust, no dependencies)
- S.SLICE for equal division slicing (2-128 slices)
- MiRings physical modeling resonator (7 commands: SRINGS.*)
- 14 filter types ported from HD2 main synth
- Modbus routing for rate, pitch, filter cut/res
- 25+ new sampler commands

**FX Chain Reorder**
- New signal chain: Mix → EQ → Pan → Pitch Shift → Beat Repeat → Clouds → Delay → Comp → Reverb
- EQ moved to front (shape source before mangling)
- Compressor moved after delay (tame dynamics after creative FX)
- Removed non-functional comb resonator (RM/RF/RD/RK)

**Delay Clock Sync**
- DS/DT now 14-bit input (0-16383) with tempo sync mode
- DS 0 (FREE): 1-2000ms with t^0.7 curve for fine control at short times
- DS 1 (SYNC): 1/16 beat through 4 beats synced to BPM
- Fixed CombC delay bug (inline Lag.kr at call site)

**Grid Visualizations**
- GRID.MODE 4: FX dry/wet levels (11 FX displayed)
- GRID.MODE 5: Sampler state visualization
- Ctrl+G hotkey for cycling grid modes 0-5
- SCRMBL.GRID/SG for independent grid scramble toggle

### Code Quality

- DRY pass: EQ handlers reduced 82% (446→80 lines)
- DRY pass: FX mix handlers consolidated (-243 lines)
- Shared utilities module with conversion helpers
- Script rendering consolidation

### Bug Fixes

- Compressor auto makeup hunting (replaced feedback-loop with static formula)
- $ command not executing scripts from metro (was discarding return value)
- MiRings OSC routing (params were falling through to wrong node)
- SC optimizer zeroing decay params (changed to NamedControl)
- Sampler end_frame=0 producing no audio

---

## v0.4.4 (January 2026)

### New Features

**GRID.MODE 2 FX Visualization**
- EQ: 3-band numeric display with frequencies, gain bars, and dB values
- Compressor: matched IN/OUT meters with gain reduction (GR) and makeup (MU) display

### Bug Fixes

- **Compressor auto-makeup** - Now proportional to actual gain reduction (not just threshold)
- **Compressor NaN guard** - Protected against -inf from 0.ampdb
- **Compressor OSC connection** - Fixed for scsynth-direct mode
- **COMP.AUTO/CAU** - Added missing synthdef parameter

---

## v0.4.3 (January 2026)

### New Features

**Scope Gain & Reset**
- SCOPE.GAIN for independent scope output volume control
- SCOPE.RST to reset scope state/buffer

**Programmable EQ Shelving Frequencies**
- ELF command for low shelf frequency (default 200Hz)
- EHF command for high shelf frequency (default 2500Hz)

**Compressor Auto-Makeup Toggle**
- COMP.AUTO/CAU to enable/disable automatic makeup gain

**Confirmation Dialogs**
- CFM.QUIT setting for quit confirmation
- CFM.SAVE setting for save overwrite confirmation
- Quit protection when metro active + named scene loaded

**Help System Reorganization**
- N1-N4 counters moved to VARIABLES & MATH section

### Critical Bug Fixes

**Scene Loading Audio Crash Prevention**
- Added configurable delays between RST parameter sends
- Manual RST: 0ms delays (instant)
- LOAD.RST: 1ms delays with 160ms sync wait
- Prevents UDP packet loss and SC buffer overflow

**Plaits Artifact Elimination**
- PLV set to 0 in RST defaults
- Eliminates high-pitched squeal during RST

**Script Validation Refactor**
- Centralized command registry with 408 commands
- Reduced validate.rs by 63%, aliases.rs by 95%
- Total reduction: ~890 lines

---

## v0.4.22 (December 2025)

### New Features

**MiClouds Granular Effect**
- Integrated MiClouds UGen from mi-UGens
- 14 parameters: pitch, position, size, density, texture, feedback, freeze, mode
- 4 processing modes: Granular, Pitch Shifter, Looping Delay, Spectral
- Full command set with CL.* prefix and short aliases
- Placed in signal chain after pan, before beat repeat

**Commands Added:**
- CL.TRIG/CLTR, CL.PITCH/CLP, CL.POS/CLO, CL.SIZE/CLS
- CL.DENS/CLD, CL.TEX/CLT, CL.WET/CLW, CL.GAIN/CLG
- CL.SPREAD/CLSP, CL.RVB/CLRV, CL.FB/CLF, CL.FREEZE/CLFZ
- CL.MODE/CLM, CL.LOFI/CLLO

---

## v0.4.21 (December 2025)

### Improvements

**MiClouds Output Level**
- Increased output gain from 1x to 2.69x for better mix presence

---

## v0.4.2.1 (December 2025)

### Bug Fixes

**Hotkey Compatibility**
- Changed from Alt+Shift to Ctrl+Shift combinations
- New: Ctrl+Shift+1-8, Ctrl+Shift+M, Ctrl+Shift+I
- Added Ctrl+Q to quit immediately

**Error Messages**
- Fixed parameter errors exceeding 46-char limit
- Format: "CMD: RANGE X-Y" (was verbose sentence)

---

## v0.4.2 (December 2025)

### New Features

**Script Mutes**
- Individual mute toggles for scripts 1-8, M (metro), I (init)
- Hotkeys: Ctrl+Shift+1-8, Ctrl+Shift+M, Ctrl+Shift+I
- Commands: MUTE, MUTE <id>, MUTE <id> <0|1>
- Visual [MUTED] indicators in script page titles

**Page Navigation Commands**
- PAGE command for programmatic page switching
- Supported: PAGE LIVE, PAGE 1-8, PAGE M, PAGE I, PAGE P, PAGE V, PAGE N, PAGE S, PAGE HELP, PAGE GRID

**Compressor Dry/Wet Mix**
- CR.MIX/CRMIX parameter (0-16383) for parallel compression

### Bug Fixes

- DC (Discontinuity) command modulator routing
- CLR alias conflict (CLR=CLEAR, clouds reverb=CLRV)
- Title timer persistence on startup

---

## v0.4.12 (December 2025)

### Bug Fixes

- Beat repeat/pitch shift short aliases not registered
- Envelope parameter error output not respecting tier system
- GitHub release bundle size now calculated dynamically

---

## v0.4.11 (December 2025)

### Internal Refactoring

**ExecutionContext Refactor**
- Created ExecutionContext struct grouping 47+ parameters
- Reduced process_command signature from 109 → 3 parameters
- Eliminated 165 duplicate tier checks

### Bug Fixes

- Ctrl+Shift+Z redo keybinding (Shift+Z = uppercase)
- Removed deprecated noise params (NA, ND, NC, NE, NG) from help
- Added MC/MQ toggle commands to help

### Build & Release

- Added mi-UGens installation to GitHub Actions
- Fixed macOS sclang workaround
- Added retry logic for SuperCollider installation

---

## v0.4.0 (December 2025)

### Plaits Voice Enhancements

**Pitch Control**
- PL.FREQ/PLF command (20-20000 Hz)
- Supports N syntax for note-to-frequency conversion

**3-Letter Aliases**
- PLH, PLT, PLE, PLM, PLD, PLL, PLF

**Other**
- PLTR trigger readout
- Multi-voice trigger indicators (H/P in header)
- RND.PL fixes (parameter scaling)

---

## v0.3.61 (December 2025)

### Bug Fixes

**Stateful Operator Highlighting**
- Fixed validation-aware highlighting system
- State snapshots for UI highlighting isolation
- Rollback coverage in all parameter macros
- Fixed 1-frame delay for interactive commands

---

## v0.3.6 (December 2025)

### New Features

**Plaits Integration**
- Added as 5th parallel sound source (node 1004)
- 16 synthesis engines
- 9 commands: PL.ENG, PL.HARM, PL.TIMB, PL.MORPH, PL.DEC, PL.LPG, PLV, PAV, PLTR
- RND.PL randomization

### Bug Fixes

- Plaits node ID conflict
- Plaits volume double-scaling
- Plaits VCA bypass (master volume now affects Plaits)
- CPU monitoring in scsynth-direct mode

---

## v0.3.5 (December 2025)

### Architecture Changes

**Multi-Synth Architecture**
- Restructured from monolithic SynthDef into 4 modular synths
- Fixes SC optimizer bug causing parameter cross-talk (NV/PV/MV)
- Node assignments: 1000=noise, 1001=mod, 1002=primary, 1003=main

### Bug Fixes

- Build system race condition
- Scene path consistency (removed XDG fallback)

---

## v0.3.4 (December 2025)

### New Features

- Additional filter types (FT 0-13, 14 total)
- Noise source integration (NW, NV)
- ModBus envelope (MBA/MBD)
- Script undo/redo (Ctrl+Z/Ctrl+Y)
- Frappe theme
- Recording timestamps

### Bug Fixes

- Latch-SC filter clock rate
- Audio-rate ModBus routing

---

## v0.3.3 (December 2025)

### New Features

- ER (Euclidean Rhythm) operator
- NR (Numeric Repetitor) operator
- VERSION command
- Line duplicate push behavior (Ctrl+D)

### Bug Fixes

- TOG zero parsing
- AUDIO command device query

---

## v0.3.2 (December 2025)

### Bug Fixes

- Beat repeat not working in bundle mode
- Config command output regardless of DEBUG level

---

## v0.3.0 (December 2025)

### New Features

- SYNC command (reset all stateful elements)
- AUTOLOAD setting
- Script validation overhaul
- Pattern operations as expressions (32 ops return values)

### Bug Fixes

- Envelope parameter scaling (FA, DA range)
- SynthDef/RST default alignment
- Nested IF in loops
- SEQ quote parsing
- Beat repeat activation

### CLI Enhancements

- `--dry-run --run <scene>` for headless testing

---

## v0.2.0 (December 2025)

- scsynth-direct integration (bundled binary)
- Recording via DiskOut UGen
- Audio device handling improvements

---

## v0.1.1 (December 2025)

- Initial Homebrew tap release
- Bundle signing fixes

---

## v0.1.0 (December 2025)

- Initial release
- Core voice with 77 parameters
- Pattern system (6 patterns × 64 steps)
- SEQ mini-notation
- Scale quantization
- MIDI clock sync
- Preset system
- Full effects chain
