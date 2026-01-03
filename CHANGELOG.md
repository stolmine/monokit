# Changelog

## v0.4.3 (January 2026) - Stability & Polish

### New Features

**GRID.MODE 2 FX Visualization**
- EQ: 3-band numeric display with frequencies, gain bars, and dB values
- Compressor: matched IN/OUT meters with gain reduction (GR) and makeup (MU) display
- Fixed compressor OSC connection for scsynth-direct mode
- Fixed COMP.AUTO/CAU command (was missing from synthdef)
- Auto-makeup now proportional to actual GR (not just threshold)
- Auto mode completely overrides manual CM (no longer multiplied together)
- **KNOWN ISSUE:** NaN readings on comp meters and audio cuts after extended play - needs investigation

### Quick Polish Features

**Scope Gain & Reset**
- Added SCOPE.GAIN parameter for independent scope output volume control
- Added SCOPE.RST command to reset scope state/buffer
- Enables accurate monitoring of quiet signals without affecting main output

**Programmable EQ Shelving Frequencies**
- Added ELF command for low shelf frequency control (default 200Hz)
- Added EHF command for high shelf frequency control (default 2500Hz)
- Allows user customization of EQ shelf points for better tonal shaping

**Compressor Auto-Makeup Toggle**
- Added COMP.AUTO/CAU command to enable/disable automatic makeup gain
- When enabled (1), compressor automatically compensates for gain reduction
- When disabled (0), CM parameter provides manual makeup control
- Default: enabled for convenience

**Confirmation Dialogs**
- Added CFM.QUIT setting (0/1) for quit confirmation when unsaved changes exist
- Added CFM.SAVE setting (0/1) for save overwrite confirmation
- Quit protection: prevents accidental quit when metro active + named scene loaded
- Settings persist in config.toml
- Config path standardized to ~/.config/monokit

**Help System Reorganization**
- Moved N1-N4 counter documentation from PATTERNS to VARIABLES & MATH section
- Improves discoverability and logical grouping (counters are variables, not patterns)

**Build Configuration**
- scsynth-direct now default build feature for bundled binary distribution

### Critical Bug Fixes

**Scene Loading Audio Crash Prevention**
- Fixed audio system crashes during scene loading caused by OSC parameter flooding
- Added configurable delays between RST parameter sends
- Manual RST: 0ms delays (instant, minimal disruption)
- LOAD.RST: 1ms delays with 160ms synchronization wait
- Prevents UDP packet loss and SuperCollider buffer overflow
- Eliminates race condition between RST and Init script execution
- Root cause: RST sent 157 parameters with zero delays, overwhelming SuperCollider

**Plaits Artifact Elimination**
- Set Plaits volume (PLV) to 0 in RST defaults
- Eliminates high-pitched squeal from Plaits during RST command
- Issue was introduced in v0.4.11 when Plaits parameters were added to RST
- Users must manually set PLV after RST to use Plaits
- Scenes with LOAD.RST work normally (scene sets PLV value)

**Script Validation Refactor**
- Centralized command registry with 408 command definitions
- Single source of truth for validation and dispatch
- Removed phantom commands (NA, NC, ND, NE, NG, ENV.*)
- Added missing validation for FBEV.AMT, M.SYNC
- Aliases now derived from registry (no duplicate maintenance)
- Reduced validate.rs by ~63% (707 lines to 10 lines)
- Reduced aliases.rs by ~95% (193 lines to 10 lines)
- Total reduction: ~890 lines of validation code

### Implementation Details

**Modified `handle_rst()` function:**
- Accepts configurable `delay_ms` parameter for flexible timing
- Manual RST optimized for speed (0ms = instant execution)
- LOAD.RST optimized for stability (1ms delays between parameters)
- Conditional sleep only when delay_ms > 0 to avoid unnecessary overhead

**Command Registry Module:**
- New module structure: `src/commands/registry/`
- Category modules: variables, counters, patterns, synth, effects, system, control, ui
- CommandDef struct with name, canonical, args, help text
- ArgCount enum for flexible validation (None, Exactly, AtLeast, Range, Custom)
- COMMAND_REGISTRY static HashMap as single source of truth
- Aliases automatically derived from registry canonical mappings

**Files modified:**
- `src/commands/system/misc.rs` - RST delay logic and PLV default
- `src/commands/mod.rs` - RST and LOAD.RST call sites
- `src/commands/validate.rs` - Registry-based validation (63% reduction)
- `src/commands/aliases.rs` - Registry-derived aliases (95% reduction)

**Files created:**
- `src/commands/registry/mod.rs` - Registry module coordinator
- `src/commands/registry/commands.rs` - Central registry aggregation
- `src/commands/registry/validate.rs` - Registry-based validation logic
- `src/commands/registry/{variables,counters,patterns,synth,effects,system,control,ui}.rs` - Category modules

---

## v0.4.21 (December 2025) - MiClouds Gain Boost

### Improvements

**MiClouds Output Level**
- Increased MiClouds output gain from 1x to 2.69x
- Makes the granular effect much more present in the mix
- Equivalent to running CLG (input gain) at 11000 on the output
- Users can still control level via CL.WET (dry/wet mix) and CL.GAIN (input level)

---

## v0.4.3 (December 2025) - MiClouds Granular Effect

### New Features

**MiClouds Granular Effect**
- Integrated MiClouds granular synthesis/processing UGen from mi-UGens
- 14 parameters for complete granular control: pitch, position, size, density, texture, feedback, freeze, mode
- Continuously records incoming audio into buffer; CL.TRIG triggers grain playback
- 4 processing modes: Granular, Pitch Shifter, Looping Delay, Spectral
- Internal reverb, stereo spread, lo-fi processing, and feedback (with self-oscillation capability)
- Placed in signal chain after pan, before beat repeat
- Full command set with CL.* prefix and short aliases (CLP, CLO, CLS, etc.)
- Integrated with RND.FX and RST commands
- Complete documentation in MANUAL.md and in-app help system

**Commands Added:**
- `CL.TRIG` / `CLTR` - Trigger grain playback
- `CL.PITCH` / `CLP` - Pitch control (0-16383, 8192=center)
- `CL.POS` / `CLO` - Buffer read position
- `CL.SIZE` / `CLS` - Grain size
- `CL.DENS` / `CLD` - Grain density
- `CL.TEX` / `CLT` - Texture/character
- `CL.WET` / `CLW` - Wet/dry mix (activates effect)
- `CL.GAIN` / `CLG` - Input gain (8192=unity)
- `CL.SPREAD` / `CLSP` - Stereo spread
- `CL.RVB` / `CLR` - Internal reverb
- `CL.FB` / `CLF` - Feedback amount
- `CL.FREEZE` / `CLFZ` - Freeze buffer recording
- `CL.MODE` / `CLM` - Processing mode (0-3)
- `CL.LOFI` / `CLLO` - Lo-fi sample rate reduction

**Signal Flow Update:**
```
...Compressor → Pan → MiClouds Granular → Beat Repeat → Pitch Shift...
```

---

## v0.4.2.1 (December 2025) - UX Fixes

### Bug Fixes

**Hotkey Compatibility**
- Fixed script mute hotkeys not working in terminal emulators
- Changed from Alt+Shift to Ctrl+Shift combinations (reliable across all terminals)
- New: Ctrl+Shift+1-8 (toggle script mutes), Ctrl+Shift+M (metro), Ctrl+Shift+I (init)
- Added Ctrl+Q hotkey to quit application immediately (works during any operation)

**Error Messages**
- Fixed parameter validation errors exceeding 46-character terminal width limit
- Changed format from "ERROR: DISPLAY NAME MUST BE BETWEEN X AND Y" to "CMD: RANGE X-Y"
- Updated all parameter macros system-wide (60+ parameters)
- Example: "CR.MIX: RANGE 0-16383" (was 57 chars, now 22 chars)
- All error messages now ≤ 46 characters (Teletype-inspired terseness)

**Documentation**
- Added PAGE command documentation to help system
- Enhanced N1-N4 counter command documentation with examples
- Added MUTE command documentation to help system
- Updated hotkey documentation (Ctrl+Shift, Ctrl+Q)

### Testing
- All 602 tests passing
- Verified hotkeys work in iTerm2, Terminal.app
- Verified error messages comply with 46-char limit

---

## v0.4.2 (December 2025) - Workflow Enhancement

### New Features

**MiClouds Granular Effect (14 Commands)**
- All 14 Clouds commands added and validated
- CL.TRIG / CLTR - Trigger grain playback from buffer
- CL.PITCH / CLP - Pitch control (0-16383, 8192=center)
- CL.POS / CLO - Buffer read position
- CL.SIZE / CLS - Grain size
- CL.DENS / CLD - Grain density
- CL.TEX / CLT - Texture/character
- CL.WET / CLW - Wet/dry mix (activates effect)
- CL.GAIN / CLG - Input gain (8192=unity)
- CL.SPREAD / CLSP - Stereo spread
- CL.RVB / CLRV - Internal reverb (alias fixed)
- CL.FB / CLF - Feedback amount
- CL.FREEZE / CLFZ - Freeze buffer recording
- CL.MODE / CLM - Processing mode (0-3)
- CL.LOFI / CLLO - Lo-fi sample rate reduction
- Integrated with RND.FX and RST commands
- Placed after pan, before beat repeat in signal chain

**Script Mutes**
- Individual mute toggles for scripts 1-8, M (metro), and I (init)
- Muted scripts skip execution but preserve content
- Hotkeys: Alt+Shift+1-8, Alt+Shift+M, Alt+Shift+I for quick toggling
- Commands: MUTE (query), MUTE <id> (toggle), MUTE <id> <0|1> (set)
- Direct commands: MUTE.1-8, MUTE.M, MUTE.I
- Visual [MUTED] indicators in script page titles
- Mute state persists in scenes
- Enables workflow where scripts can be prepared without executing

**Page Navigation Commands**
- Programmatic page switching via PAGE command
- Supported pages: PAGE LIVE, PAGE 1-8, PAGE M, PAGE I, PAGE P, PAGE V, PAGE N, PAGE S, PAGE HELP, PAGE GRID
- Short aliases: PG (page), L/LIVE, H/HELP, G/GRID
- Script-controlled UI navigation (works from metro scripts)
- Enables future animated transitions
- PAGE GRID sets grid view mode

**Compressor Dry/Wet Mix**
- Added CR.MIX / CRMIX parameter (0-16383) for parallel compression
- 0 = 100% dry (bypass compression), 16383 = 100% wet (fully compressed)
- Dry/wet blending uses XFade2 for smooth crossfading
- Default 16383 maintains backward compatibility
- Added to RST defaults and RND.FX randomization
- Enables parallel compression mixing techniques

### Bug Fixes

**DC (Discontinuity) Command Fix**
- Fixed DC command modulator routing issue
- DC parameter now correctly modulates discontinuity amount
- Discontinuity effect now works as expected

**CLR Alias Conflict Resolution**
- CLR alias reserved for CLEAR command only
- Clouds reverb uses CLRV alias (CL.RVB canonical form)
- Eliminates command name collision

**Title Timer Persistence**
- Fixed TITLE.TIMER setting not persisting correctly on startup
- Timer now initializes properly when enabled from config
- title_timer_last_toggle now set to current time when loading enabled timer
- Eliminates need to "kickstart" timer after restart

### Testing

- All 602 tests passing
- Updated fx_randomization test to account for CR.MIX parameter
- Script mutes tested with hotkeys, commands, and scene persistence
- Page navigation tested across all page types
- All 14 Clouds commands validated and tested

## v0.4.12 (December 2025) - Bug Fixes

### Bug Fixes

**Title Timer Persistence**
- Fixed TITLE.TIMER setting not persisting correctly on startup
- Timer now initializes properly when enabled from config
- title_timer_last_toggle now set to current time when loading enabled timer from config
- Eliminates need to "kickstart" timer after restart

**Beat Repeat and Pitch Shift Short Aliases**
- Fixed missing short aliases for beat repeat and pitch shift commands
- Registered aliases: BRL, BRR, BRW, BRX for beat repeat (len, rev, win, mix)
- Registered aliases: PSM, PSS, PSG, PSX, PST for pitch shift (mode, semi, grain, mix, targ)
- These commands were functional but missing from alias registration system
- Alias system now recognizes both full and short command names

**Envelope Parameter Error Output**
- Fixed envelope parameter validation errors not respecting tier system
- Error messages now properly filtered by TIER_ERRORS (tier 1)
- Affects all envelope parameters across voice, filter, FM, and disc envelopes
- Ensures DEBUG 0 fully silences error output as expected

**Test Suite Updates**
- Updated all 602 tests to use ExecutionContext refactor
- All tests passing with new command execution architecture
- Ensures regression coverage for ExecutionContext changes

**GitHub Release Workflow**
- Fixed hardcoded bundle size in release notes
- Bundle size now calculated dynamically from artifact
- Release notes show accurate bundle file size

## v0.4.11 (December 2025) - Maintenance Release

### Internal Refactoring

**Debug Tier & Output Routing Refactor**
- Created ExecutionContext struct to group 47+ command parameters
- Reduced process_command signature from 109 → 3 parameters
- Eliminated 165 duplicate tier checks across command handlers
- Centralized output control through ExecutionContext.output() method
- Removed unused TIER_VERBOSE constant (tier 5)
- Migrated all variable, pattern, and system commands to use ExecutionContext
- Fixed tier violations: SLEW (tier 1 → 4), PRINT (tier 1 → 2)
- Added missing tier gates to scene commands (SAVE, LOAD, SCENES, DELETE)
- Added missing tier gates to preset commands (PSET, PSET.SAVE, PSET.DEL, PSETS)
- Added missing tier gates to recording commands (REC, REC.STOP, REC.PATH)
- All commands now properly respect debug level settings at tier 0 (TIER_SILENT)
- Documentation: DEBUG_TIERS.md, TIER_0_COMMANDS.md, TIER_FIXES_SUMMARY.md
- No behavioral changes - purely architectural improvement for maintainability

### Bug Fixes

**Redo Keybinding**
- Fixed Ctrl+Shift+Z redo command not working
- Issue: Shift+Z produces uppercase 'Z', keybinding only matched lowercase 'z'
- Both undo and redo now handle both 'z' and 'Z' characters
- Redo functionality now works as expected on all platforms

### Documentation & Help System

**Deprecated Noise Parameters Removed**
- Removed non-functional noise envelope parameters from help system
- Cleaned up: NA (attack), ND (decay), NC (curve), NE (envelope amount), NG (gate)
- These were removed during multi-synth architecture split in v0.3.5
- Help now accurately reflects working parameters: NW, NP, NM, NV

**Filter Routing Documentation**
- Added missing MC/MQ toggle commands to help system
- ROUT.MC / MC - Enable/disable modulation to filter cutoff
- ROUT.MQ / MQ - Enable/disable modulation to filter resonance
- Clarified relationship between toggles (MC/MQ) and amounts (MF_F/MF_Q)

**UI Improvements**
- Changed trigger indicator from 'C' to 'H' for Complex oscillators
- Added '|' separator between H and P indicators for better readability
- Header now displays "H|P" instead of "CP"

### Build & Release Pipeline

**GitHub Actions Workflow Fixes**
- Added mi-UGens v0.0.8 installation for Plaits integration support
- Applied macOS sclang workaround (must run from SC app directory)
- Removed timeout command dependency (not available on macOS by default)
- Added retry logic for SuperCollider installation (handles CDN failures)
- Release builds now include full Plaits support with MiPlaits.scx

**Cross-Platform Preparation**
- Fixed hardcoded `/tmp` paths to use `std::env::temp_dir()`
- Improved platform-agnostic temporary file handling

## v0.4.0 (December 2025)

### Plaits Voice Enhancements

**Pitch Control**
- Added PL.FREQ / PLF command for Plaits pitch control (20-20000 Hz)
- Supports Hz values and N syntax for note-to-frequency conversion
- Works with expressions, TOG, EITH, SEQ patterns
- Independent pitch control from complex oscillators

**3-Letter Parameter Aliases**
- Added concise aliases for all Plaits parameters to improve legibility
  - PLH → PL.HARM (harmonics)
  - PLT → PL.TIMB (timbre)
  - PLE → PL.ENG (engine)
  - PLM → PL.MORPH (morph)
  - PLD → PL.DEC (decay)
  - PLL → PL.LPG (lowpass gate)
  - PLF → PL.FREQ (pitch)
- Distinguishes Plaits commands from complex oscillator commands

**PLTR Trigger Readout**
- Added confirmation output when PLTR command is triggered
- Shows "PLAITS TRIGGERED" message
- Respects debug level settings (TIER_CONFIRMS)

**RND.PL Fixes**
- Fixed parameter scaling for harmonics/timbre/morph/decay/lpg
- Changed from OscType::Int(0-16383) to OscType::Float(0.0-1.0)
- Now properly randomizes all Plaits parameters
- Added RND.PL to command validation (was showing "unknown command")

### UI Enhancements

**Multi-Voice Trigger Indicators**
- Added separate trigger indicators in header for Complex and Plaits voices
  - H = Complex oscillators (TR command)
  - P = Plaits macro oscillator (PLTR command)
- Replaced single "TR" indicator with multi-voice awareness
- Both indicators can be active simultaneously
- Updated HEADER command descriptions to reflect H/P terminology

### Bug Fixes

**Plaits Parameter Routing**
- Fixed "pitch" and "detune" parameter routing to PLAITS_NODE_ID
- Previously pitch was being sent to MAIN_NODE_ID instead of Plaits synth
- Ensures PLF/PL.FREQ commands properly affect Plaits voice

## v0.3.61 (December 2025)

### Architecture Changes

**Validation-Aware Highlighting**
- Implemented two-tier highlighting system for stateful operators
- Variables (A, B, C, etc.) always highlight based on toggle_state
  - State advances regardless of validation in usage contexts
  - Exempt from validation checks (multi-context usage)
- Direct parameter usage (PF TOG 100 200) tracks validation results
  - Added direct_validation HashMap to PatternStorage
  - Only highlights if validation succeeded
  - Skips highlighting on validation failure
- All 6 parameter macros record validation results for TOG/SEQ/EITH
- Added is_variable_assignment() helper to distinguish contexts

**State Highlight Mitigation**
- Removed dual state mutations in random_ops.rs
  - handle_eith/handle_tog no longer mutate state twice
  - Single mutation point in eval_expression
- Added state snapshots for UI highlighting isolation
  - script.rs, metro.rs, init.rs snapshot toggle_state/toggle_last_value
  - Highlighting reads from immutable snapshots
- Completed rollback coverage in all parameter macros
  - Added rollback to define_int_param_ms, define_bool_param, define_mode_param_with_names
  - All 6 macros restore state on validation failure
- Removed StateChanges struct from eval return type

### Bug Fixes

**Stateful Operator Highlighting**
- Fixed missing prefix text in validation failure paths
  - Commands like "PF TOG 0 100" now display full text when validation fails
  - Added prefix segment handling to SEQ, TOG, and EITH operators
- Fixed inconsistency between toggle_state and toggle_last_value
  - Validation-aware system ensures accurate highlighting
  - Variables advance independently of validation results
- Fixed critical bug where highlighting showed incorrect values for stateful operators
  - Part 1: Store actual returned value in toggle_last_value HashMap
  - Previously derived value from incremented state (showed NEXT, not CURRENT)
  - Now stores actual returned value before incrementing state
- Eliminated 1-frame delay for interactive commands
  - Event loop rendered UI before executing commands
  - Added immediate re-render after command execution
  - Metro-triggered scripts already worked correctly
- Highlighting now shows correct current value for both interactive and scripted commands
- All 602 tests pass including 17 highlighting-specific tests

---

## v0.3.6 (December 2025)

### New Features

**Plaits Integration (Mutable Instruments)**
- Added Plaits as 5th parallel sound source (node 1004)
- 16 synthesis engines: VA, FM, wavetable, granular, percussion, physical modeling
- 9 new commands: PL.ENG, PL.HARM, PL.TIMB, PL.MORPH, PL.DEC, PL.LPG, PLV, PAV, PLTR
- Dual outputs (main + aux) with independent volume control
- RND.PL randomization command for exploring sounds
- Post-VCA routing: Plaits bypasses HD2 filter/distortion, shares effects chain
- mi-UGens (MiPlaits.scx) bundled in release

### Bug Fixes

**Plaits Node ID Conflict**
- Fixed node 1004 conflict by establishing clear 5-synth architecture
- Node assignments: 1000=noise, 1001=mod, 1002=primary, 1003=main, 1004=plaits

**Plaits Volume Double-Scaling**
- Fixed PLV/PAV being nearly inaudible at max values
- Volume parameters now scaled once (in SuperCollider) not twice
- Matches pattern used by other volume parameters (PV, MV, NV)

**Plaits VCA Bypass**
- Fixed master volume (VOL) not affecting Plaits output
- Moved Plaits mixing to post-VCA stage
- Master volume now controls entire mix consistently
- Plaits maintains independent character by bypassing filter/distortion

**CPU Monitoring**
- Fixed inactive CPU readout in scsynth-direct mode
- Previous approach used /cmd to execute sclang code (doesn't work with scsynth)
- New approach: meter_thread polls /status every 500ms (2Hz)
- Parses /status.reply for avgCPU (index 5) and peakCPU (index 6)
- CPU percentage now displays correctly in header and live page

---

## v0.3.5 (December 2025)

### Architecture Changes

**Multi-Synth Architecture**
- Restructured from monolithic SynthDef into 4 modular synths
- monokit_noise (node 1000, bus 18): Noise generator
- monokit_mod (node 1001, bus 17): Modulator oscillator
- monokit_primary (node 1002, bus 16): Primary oscillator
- monokit_main (node 1003): Effects processor and mixer
- Fixes SuperCollider optimizer bug causing parameter cross-talk (NV/PV/MV)
- Each source has isolated volume parameter graph preventing conflicts

**Parameter Routing System**
- Intelligent parameter routing to correct synth nodes
- Noise params (NW, NV) → node 1000
- Mod params (MF, MW, MV, FB, etc.) → node 1001
- Primary params (PF, PW, PV, FM, etc.) → node 1002
- Effects params (FC, FQ, etc.) → node 1003
- Trigger (t_gate) broadcasts to all 4 synths for envelope sync

### Bug Fixes

**Build System Reliability**
- Fixed race condition in sclang compilation causing intermittent hangs
- Added process cleanup (pkill sclang/scsynth) before compilation
- Added 2-second delay for clean state
- Both bundle.sh and verified_build.sh now reliable

**Scene Path Consistency**
- Removed XDG location (~/.config/monokit/scenes) fallback
- Now consistently uses platform-native Application Support location
- macOS: ~/Library/Application Support/monokit/scenes
- Ensures all user scenes accessible from bundle

**Noise Implementation**
- Simplified to basic noise source (NW waveform, NV volume only)
- Removed unsupported envelope/gate/FM parameters from Rust layer
- Aligned command reference with actual SynthDef capabilities

### Infrastructure

**Documentation**
- Added build race condition investigation docs
- Added noise restoration documentation
- Updated command reference to reflect current noise parameters

---

## v0.3.4 (December 2025)

### New Features

**Additional Filter Types (FT 0-13)**
- Expanded filter system from 4 to 14 filter types
- FT 0-3: SVF filters (LP, HP, BP, Notch)
- FT 4: MoogFF (warm, self-oscillating ladder LP)
- FT 5: RLPF (resonant LP 12dB, punchy)
- FT 6: RHPF (resonant HP 12dB, tight)
- FT 7: DFM1 LP (diode filter LP, scuzzy)
- FT 8: DFM1 HP (diode filter HP)
- FT 9: BMoog LP (24dB saturating LP)
- FT 10: BMoog HP (24dB saturating HP)
- FT 11: BMoog BP (24dB saturating BP)
- FT 12: Latch-SC LP (switched-cap LP)
- FT 13: Latch-SC HP (switched-cap HP)
- All filters share FC, FQ, FE, FED, FK params
- MC renamed from MF.F (modbus to filter cutoff)
- MQ added (modbus to filter resonance)

**Noise Source Integration**
- Added noise generator as third sound source
- NW: Waveform selection (0=white, 1=pink, 2=brown)
- NV: Noise volume (0-16383)
- Simple mixing with primary and modulator oscillators

**RND.FX Expanded**
- Now randomizes all FX blocks: filter, lo-fi, ring mod, resonator, delay, EQ, reverb

**ModBus Envelope (MBA/MBD)**
- Added envelope control for modbus depth
- MBA: modbus envelope amount (0-16383)
- MBD: modbus envelope decay (ms)
- Allows per-trigger modulation intensity sweeps

**Script Undo/Redo**
- Ctrl+Z: Undo last script edit
- Ctrl+Y: Redo undone edit
- Per-script undo history (up to 50 states)

**Frappe Theme**
- Added Catppuccin Frappe theme variant

**Recording Timestamps**
- Auto-generated filenames now include timestamp
- Format: monokit_YYYYMMDD_HHMMSS.wav

### Bug Fixes

**Latch-SC Filter Clock Rate**
- Fixed silence issue with FT 12-13 at low cutoff frequencies
- Clock rate now properly clamped to 100-20000 Hz

**Audio-Rate ModBus Routing**
- MC/MQ now use audio-rate modOsc signal for smooth modulation

---

## v0.3.3 (December 2025)

### New Features

**ER and NR Operators**
- Added ER (Euclidean Rhythm) operator
- `ER <fill> <length> <step>` returns 0 or 1
- Distributes fill beats evenly across length steps
- Uses Bjorklund's algorithm for euclidean patterns
- Added NR (Numeric Repetitor) operator
- `NR <prime> <mask> <factor> <step>` returns 0 or 1
- 32 prime patterns from Noise Engineering
- Per-line state tracking for both operators
- Example: `IF ER 5 8 I: TR` or `IF NR 0 0 1 I: TR`

**VERSION Command**
- Added VERSION/VER command to display current version
- Version information now shown in startup message
- Version displayed in help page header
- Provides quick access to version information

**Line Duplicate Push Behavior**
- Ctrl+D now pushes lines down instead of overwriting
- Improved editing workflow for duplicating script lines
- Non-destructive line duplication behavior

### Bug Fixes

**TOG Zero Parsing**
- Fixed display corruption when TOG uses zero as argument
- Example: `DC TOG 2000 0` was displaying as `DC TOG 2000 000 0`
- Added `find_whole_word()` helper for word-boundary matching
- Prevents "0" being found inside "2000" during highlighting
- File: src/ui/state_highlight.rs

**AUDIO Command Device Query**
- Fixed `AUDIO 1` failing if `AUDIO` not called first
- Audio devices now populated during App initialization
- Applies to macOS with scsynth-direct feature enabled
- File: src/app/mod.rs

---

## v0.3.2 (December 2025)

### Bug Fixes

**Beat Repeat**
- Fixed beat repeat not working in bundle mode
- Synced SynthDef between compile_synthdefs.scd and monokit_server.scd
- Beat repeat activation now derives from br_mix > 0

**Config Command Output**
- All config/query commands now output regardless of DEBUG level
- Fixed: THEME, HEADER, CPU, GRID, METER.*, SCOPE.*, OUT.*, SCRMBL.*
- Fixed: LOAD.RST, LOAD.CLR, AUTOLOAD, VCA, LIMIT
- Fixed: TITLE, TITLE.TIMER, BPM, SCENES, PSETS
- Fixed: REC, REC.STOP, REC.PATH recording commands
- Fixed: SAVE, LOAD, DELETE scene commands

---

## v0.3.0 (December 2025)

### New Features

**SYNC Command**
- `SYNC` resets all stateful elements to starting position
- SEQ sequences reset to first element
- TOG toggles reset to first value
- EITH and inline random choice states cleared
- EV/SKIP counters reset to 0
- Pattern indices (P.I) reset to 0
- N1-N4 counters reset to MIN values
- Partial variants: `SYNC.SEQ`, `SYNC.TOG`, `SYNC.PAT`

**AUTOLOAD Setting**
- `AUTOLOAD 0/1` enables auto-load of last scene on startup
- Last loaded scene tracked in config.toml
- Missing scenes handled gracefully

**Script Validation Overhaul**
- Invalid commands now rejected at script entry time, before save
- Paste validation added (previously bypassed validation)
- Expression syntax validation without evaluation
- Control flow validation for loops, conditionals, DEL commands
- SEQ pattern content validation (bracket balancing, valid tokens)
- Reference range validation (patterns 0-5, scripts 1-8/M/I)
- Extra tokens after expressions now rejected
- Clear error messages for all validation failures

**Pattern Operations as Expressions**
- 32 pattern operations now return values when used in expressions
- P.PUSH, P.POP, P.REV, P.SHUF, P.SORT, P.ROT with return values
- P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE with return values
- P.INS, P.RM, P.CLR, P.RND with return values
- All PN.* variants support pattern number as first argument
- Enables Teletype-style patterns like `A P.PUSH 42`

### Bug Fixes

**Envelope Parameter Scaling**
- FA (FM envelope amount) now accepts 0-16383 range (was 0-16)
- DA (Disc envelope amount) now accepts 0-16383 range (was 0-16)
- Root cause: SynthDef divided by 16383 but Rust limited input range

**SynthDef/RST Default Alignment**
- RST command now matches SynthDef default values
- pf: 200 → 131 (C3)
- mf: 50 → 262 (C4)
- pa: 4 → 0 (no pitch envelope by default)
- fc: 1000 → 10000 (filter wide open)
- cr: 4 → 1 (compressor off by default)

**Nested IF in Loops**
- Fixed `L 1 6: IF GT I 2: IF LT I 5: PRINT I` giving "UNKNOWN COMMAND: IF"
- Changed colon splitting from find(':') to rfind(':')
- Added recursive nested conditional handling

**SEQ Quote Parsing**
- Fixed `A SEQ "A B C D"` giving "FAILED TO PARSE VALUE"
- Added quote-respecting whitespace splitting
- Variable assignment now handles quoted strings correctly

**TOG Zero Parsing**
- Fixed display corruption when TOG uses zero as argument
- Key format mismatch between state storage and highlighting resolved

**Beat Repeat Activation**
- Removed BR.ACT command
- Beat repeat now activates automatically when BR.MIX > 0
- Fixes stickiness bug where BR wouldn't turn off reliably

### CLI Enhancements

- `--dry-run --run <scene>` runs without SuperCollider/audio
- Enables headless testing of command logic
- Batch mode auto-starts metro after loading scene

### Test Coverage

- 9 new validation test scenes
- 583 unit tests passing
- Comprehensive coverage for pattern operations, expressions, control flow

---

## v0.2.0 (December 2025)

- scsynth-direct integration (bundled binary distribution)
- Recording via DiskOut UGen
- Audio device handling improvements
- sc3-plugins extraction path fix

## v0.1.1 (December 2025)

- Initial Homebrew tap release
- Bundle signing and distribution fixes

## v0.1.0 (December 2025)

- Initial release
- Core voice with 77 parameters
- Pattern system (6 patterns x 64 steps)
- SEQ mini-notation
- Scale quantization
- MIDI clock sync
- Preset system
- Full effects chain
