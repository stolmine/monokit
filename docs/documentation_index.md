# Monokit Documentation Index

## Recent Updates (December 2025)

### List Output Formatting [COMPLETE]
**Vertical Display for Theme List:**
- THEMES command now displays one theme per line vertically
- Improved readability and terminal compatibility
- Respects 46-character width constraint
- Consistent with other list commands (SCENES, PSETS, MIDI.IN, AUDIO.OUT)

### VCA Reset Coverage [COMPLETE]
**VCA Mode in Reset Commands:**
- RST command now resets VCA to default mode 1 (gated)
- LOAD.RST behavior includes VCA reset automatically
- VCA state properly restored on scene load
- Ensures predictable behavior after reset operations

### EITH Selection Highlighting [COMPLETE]
**Random Choice Visual Feedback:**
- EITH (random choice) now tracks last selected value
- Selected option highlighted in script display
- State persists across evaluations like TOG does
- Nested random choice `{a b}` in SEQ notation shows last selection
- Random choice state tracking with `seq_rnd_` keys in toggle_state

### Direct scsynth Integration [COMPLETE]

**Self-contained Audio Engine:**
- Monokit now runs without sclang, spawning scsynth directly
- Send parameters to scsynth via OSC (port 57110)
- Receive meter/spectrum/scope data on separate port
- Simplified by eliminating sclang interpretation layer

**Build & Distribution:**
- `cargo build --features scsynth-direct` enables feature
- Bundle structure: monokit binary + scsynth + plugins + synthdefs
- Resource directory: `/Resources/` contains scsynth executable
- SynthDef precompilation: `sc/synthdefs/` holds .scsyndef files
- Total bundle size: ~13 MB (vs ~200 MB for full SC)

**t_gate Parameter Change:**
- SynthDef gate parameter renamed to `t_gate` (TrigControl)
- Ensures reliable triggering via OSC `/s_new` messages
- Pitch, FM, discontinuity, feedback envelopes trigger on gate
- Gate responds to TR command (trigger)

**Bundled Resources:**
- scsynth binary and core plugins in bundle Resources/
- Monokit detects bundled resources via executable path
- Fallback to system scsynth if bundled not found
- Plugin path: `-U` flag passed to scsynth at boot

**Feature Flag Architecture:**
- `scsynth-direct` cargo feature enables new mode
- Conditional compilation: `#[cfg(feature = "scsynth-direct")]`
- Sclang compat as fallback (requires SuperCollider installed)

See: `docs/scsynth_direct_integration.md` for detailed spec

### Recording in scsynth-direct Mode [COMPLETE]

**DiskOut UGen Implementation:**
- Recording now works in bundled scsynth-direct mode
- Uses DiskOut UGen for streaming audio to disk
- Requires DiskIO_UGens.scx plugin (now bundled)
- Separate monokit_recorder SynthDef on node 1001

**Key Fixes:**
- Upgraded to scsynth 3.14.1 (fixes CoreAudio input query bug with -i 0)
- Added DiskIO_UGens.scx to bundle (contains DiskOut UGen)
- Fixed /b_write OSC command with proper parameters (leaveOpen=1 for streaming)
- Bundled required dylibs: libsndfile, libfftw3f, libreadline
- Added xattr clearing and code signing to bundle process

**Recording Workflow:**
- Start: /b_alloc → /b_write (leaveOpen=1) → /s_new monokit_recorder
- Stop: /n_free 1001 → /b_close → /b_free
- Output format: 24-bit stereo WAV @ 48kHz
- File naming: monokit_audio_0.wav, monokit_audio_1.wav, etc.
- Files saved to current working directory

**Bundle Updates:**
- scripts/bundle.sh copies DiskIO_UGens.scx from plugins directory
- All required dylibs copied from SuperCollider.app/Contents/Frameworks/
- Bundle script now always clears quarantine with xattr -cr
- Code signing: all binaries and dylibs signed with codesign --force --deep -s -

### BRK Command (December 2025)

**Script Break Control:**
- BRK stops current script execution immediately
- Zero arguments, works anywhere in a script
- Works with conditionals: IF A: BRK
- Works in loops: L 0 5: IF GT I 2: BRK
- BRK affects current script only, not parents
- Nested script reset: break flag cleared after SCRIPT command returns
- Implementation: script_break flag in App struct

### Audio Engine Confirmation (December 2025)

**AUDIO.OUT Device Change Feedback:**
- "AUDIO ENGINE ONLINE" displays after SC restart
- Confirms SuperCollider audio engine is ready
- Direct command response, always visible
- Not gated by DEBUG level
- Implementation in src/commands/system/audio.rs

### Error Tier Filtering Fixes (December 2025)

**DEBUG Level Compliance:**
- Fixed 7 error bypass locations at DEBUG 0
- control_flow.rs: Validation, file I/O errors
- mod.rs: Command execution, invalid script index, recursion depth errors
- interactive.rs: Validation, file I/O errors
- All errors now respect should_output(OutputCategory::Error)
- Consistent error display across REPL modes

### Header ASCII Meters (December 2025)

**Distinct Meter Characters:**
- Header meters use new charset: `.oO0@###`
- Rounder, easier to read at a glance
- Grid meters keep original `.:-=+###` pattern
- Better for vertical stacking
- Implementation in src/ui/header.rs

### XDG Configuration Migration (December 2025)

**Config Location Change:**
- Migrated from `~/.monokit/` to `~/.config/monokit/` (XDG Base Directory compliance)
- All configuration and user data now follows XDG specification
- Config file: `~/.config/monokit/config.toml`
- Scenes: `~/.config/monokit/scenes/`
- User presets: `~/.config/monokit/presets/`

### Phase 6: Release Preparation (December 2025) - COMPLETE

**Automated Release Infrastructure:**
- Terminal Compatibility (Phase 1 complete)
- Automated release workflow via `.github/workflows/release.yml`
- GitHub Release creation with pre-built bundles
- Homebrew formula auto-updates after releases
- Bundle creation via `scripts/bundle.sh`
- Symlink resolution for Homebrew compatibility (`get_exe_dir()`)
- GPL-2.0 license added
- 47 themes bundled at compile time (themes/themes.toml)
- Advanced DSP renumbered to Phase 7
- Distribution renumbered to Phase 8

**Homebrew Distribution:**
- Tap: `stolmine/monokit`
- Formula: Pre-built bundles, no dependencies
- Structure: libexec with symlink to bin
- Auto-updates via GitHub Actions

### Terminal Compatibility System (December 2025)

**Terminal Capability Detection:**
- Detects terminal capabilities at startup
- 256-color theme fallback when true color unavailable
- Startup warning for Terminal.app users
- High-contrast cursor (white/black) in 256-color mode
- Implementation in src/terminal.rs

**COMPAT Command:**
- COMPAT shows terminal capabilities
- Displays color support, terminal type
- Command handler in src/commands/system/misc.rs

**COMPAT.MODE Command:**
- COMPAT.MODE 0 = standard mode (true color)
- COMPAT.MODE 1 = compatibility mode (256-color)
- Bundles compatibility settings
- Theme changes apply 256-color conversion

**METER.ASCII Command:**
- METER.ASCII 0 = Unicode block characters
- METER.ASCII 1 = ASCII characters (.:-=+#)
- Applies to both grid and header mini-meters
- Consistent meter rendering across UI

### SCOPE.CLR Color Labels (December 2025)

**Color Name Support:**
- Accepts color names instead of just numbers
- Valid names: FOREGROUND, SECONDARY, HIGHLIGHT_BG, HIGHLIGHT_FG, BORDER, ERROR, ACCENT, SUCCESS, LABEL
- Aliases: FG, SEC, HL_BG, HL_FG, ERR, ACC, SUC, LBL
- Backwards compatible with 0-8
- All 9 theme colors now available (was only 4)
- Command handler in src/commands/scope/mod.rs

### REPL Output Consistency (December 2025)

**UI/Settings Commands Always Display:**
- UI/settings commands now output regardless of DEBUG level
- Fixed: LIMIT, SCOPE.TIME, SCOPE.CLR, SCOPE.MODE, SCOPE.UNI
- Direct queries and setting confirmations always visible
- DEBUG level only gates background/automated output
- Implementation in src/commands/scope/mod.rs

### Grid Scramble Disabled (December 2025)

**Grid Icon Behavior:**
- Grid icons no longer scramble on tab switch
- Title scramble remains active
- Implementation in src/ui/mod.rs

### Audio Device Selection (December 2025)

**AUDIO.OUT Command:**
- AUDIO.OUT lists available output devices with numbers
- AUDIO.OUT <N> selects device by number (1-based)
- Numbered selection solves case-sensitivity issue
- Monokit uppercases input, device names are case-sensitive
- Device change restarts SuperCollider audio engine
- Selected device saved to config for next launch
- Also works with direct device name for scripting
- Output always displays (direct command response, not gated by debug level)

**Implementation Details:**
- Command handler in src/commands/system/audio.rs
- Validates numbered selection against device list
- Sends MetroCommand::SetAudioOutDevice to SC
- src/app/mod.rs: Added audio_devices field
- src/app/mod.rs: Added audio_device_current field
- src/ui/mod.rs: Stores device list
- src/ui/mod.rs: Displays numbered items
- src/commands/mod.rs: Added audio_devices parameter
- Help text: Updated in src/ui/pages/help_content.rs

**User Flow:**
1. Run AUDIO.OUT to see numbered device list
2. Run AUDIO.OUT 2 to select device #2
3. SC restarts with new device
4. Device saved to config automatically

### DRY Refactors (December 2025)

**Boolean Toggle Macro:**
- Created boolean_toggle_handler! macro in src/commands/system/misc.rs
- Consolidated 15 handler functions (337 lines saved)
- Handles: HEADER, METER.HDR, METER.GRID, SPECTRUM, ACTIVITY, GRID, GRID.MODE, HL.SEQ, HL.COND, SCRMBL, SCRMBL.MODE, SCRMBL.CRV, SCOPE.VIS, TITLE.TIMER enable/disable
- Generates validation, execution, config persistence, and confirmation messages
- Pattern: boolean_toggle_handler!(command, field, on_msg, off_msg)

**Integer Enum Macro:**
- Created integer_enum_handler! macro in src/commands/system/misc.rs
- Consolidated 6 handler functions (416 lines saved)
- Handles: DEBUG, SCRMBL.SPD, SCOPE.MODE, SCOPE.DIV, SCOPE.RES, VCA
- Generates validation, execution, config persistence, confirmation with mode names
- Pattern: integer_enum_handler!(command, field, max, mode_names)

**F-Key Handler Refactor:**
- Refactored F-key input handling in src/ui/mod.rs
- Added Page::from_script_number() helper function
- Consolidated duplicate logic (36 lines saved)
- Handles F1-F8 keys for script page navigation

**Pattern Macros Extraction:**
- Extracted pattern matching macros to src/commands/patterns/macros.rs
- Moved from src/commands/patterns/common.rs
- Reduced common.rs from 1,132 lines to 361 lines
- Macros: pattern_arg!, optional_pattern_arg!, require_pattern_loaded!
- Better code organization and reusability

**Settings Struct Bundling:**
- Created ScopeSettings struct (mode, division, resolution, visible) in src/types.rs
- Replaced 4 individual parameters with single struct
- Created OutputSettings struct (err, ess, qry, cfm) in src/types.rs
- Created ScrambleSettings struct (enabled, mode, speed, curve) in src/types.rs
- Created UIToggles struct (header, meter_hdr, meter_grid, spectrum, activity, grid, grid_mode, hl_seq, hl_cond) in src/types.rs
- Reduced function parameter counts across codebase
- Improved code maintainability and consistency

### VCA Command (December 2025)

**Voice Control Amplifier Mode:**
- VCA 0 = DRONE mode (VCA stays open, no envelope)
- VCA 1 = GATED mode (envelope controls amplitude)
- Default: GATED mode
- Supports expressions (VCA A, VCA RND 0 1)
- Config persistence to config.toml
- Command handler in src/commands/synth/output.rs

**SuperCollider Implementation:**
- Uses Select.kr(vca_mode, [1.0, ampEnv]) to switch between modes
- Mode 0: Constant amplitude (1.0)
- Mode 1: Envelope-controlled amplitude (ampEnv)
- Updated in monokit_server.scd

**Use Cases:**
- DRONE mode: Continuous tones, ambient textures, no envelope
- GATED mode: Standard percussive/gated synthesis

### TITLE.TIMER Command (December 2025)

**Title Cycling System:**
- TITLE.TIMER - Query current state (enabled/disabled, interval)
- TITLE.TIMER 0 - Disable cycling
- TITLE.TIMER 1 <secs> - Enable cycling with interval (1-1800 seconds)
- Cycles between "MONOKIT" and scene name
- Triggers scramble animation on each toggle
- Both arguments support expressions (TITLE.TIMER A B)
- Config persistence to config.toml

**Implementation Details:**
- Command handler in src/commands/system/misc.rs
- Uses integer_enum_handler! macro for state management
- Validates interval range (1-1800 seconds)
- Timer state stored in App struct
- Integrates with existing scramble animation system

**User Flow:**
1. TITLE.TIMER 1 5 - Enable cycling every 5 seconds
2. Header alternates between "MONOKIT" and scene name
3. Each toggle triggers scramble animation
4. TITLE.TIMER 0 - Disable cycling, stays on current title

### Terminal Window Title (December 2025)

**Window Title Update:**
- Terminal window title now displays "monokit" instead of "scsynth"
- Set via ANSI escape sequence in src/main.rs
- Improves application identity in window manager/taskbar
- Uses standard ANSI escape sequence: \x1b]0;monokit\x07

### Command Registration Audit (December 2025)

**Missing Command Registrations Fixed:**
- Fixed MIDI.IN command missing from src/commands/validate.rs
- Fixed MIDI command missing from src/commands/validate.rs
- Fixed LOAD.RST command missing from src/commands/validate.rs
- Fixed 7 GATE commands missing from dispatcher in src/commands/mod.rs:
  - GATE (global gate)
  - AENV.GATE (amp envelope gate)
  - PENV.GATE (pitch envelope gate)
  - FMEV.GATE (FM envelope gate)
  - DENV.GATE (discontinuity envelope gate)
  - FBEV.GATE (feedback envelope gate)
  - FLEV.GATE (filter envelope gate)

**Impact:**
- GATE commands now properly validated and dispatched
- MIDI commands now validate correctly
- LOAD.RST now validates correctly
- Prevents "UNKNOWN COMMAND" errors for valid commands
- Ensures consistent validation across all commands

### Rolling Text Scramble Animation (December 2025)

**Header Animation Feature:**
- Rolling text scramble effect on header title
- Each character position has independent scramble timing for visual identity
- Triggers on: startup, scene load/save, TITLE command
- Rolling reveal effect uses 5-character scramble buffer
- Integrated into App struct and main event loop

**Commands:**
- SCRMBL 0/1: Enable or disable scramble effect
- SCRMBL.MODE 0-3: Set animation style (0=Regular, 1=Smash, 2=Rolling, 3=Overshoot)
- SCRMBL.SPD 1-10: Control animation speed (1=slowest, 10=fastest)
- SCRMBL.CRV 0/1: Animation curve (0=Linear, 1=Settle - chars land at 70% and pause before reveal)
- All settings persist to config.toml

**UI Scramble Elements:**
- CPU and BPM indicators scramble on startup
- Same animation modes and speed apply to all UI elements
- Creates cohesive visual identity across interface

**Grid Scramble Animation:**
- Grid labels/icons scramble when switching to grid view (Tab key)
- Gated by SCRMBL 0/1 setting
- Text mode: Uses same scramble as header
- Icon mode (GRID.MODE 1): Unicode traversal
- Icons start 100-500 positions from target
- Traverse adjacent Unicode chars to target
- Uses same timing as title (~1.4s at speed 5)
- Shares SCRMBL.SPD, SCRMBL.CRV, SCRMBL.MODE settings
- Synchronized duration with title animation

**Implementation Details:**
- Core animation logic in src/scramble.rs
- TitleScrambler struct with per-character state tracking
- Header rendering in src/ui/header.rs calls scrambler
- Animation trigger methods in src/app/mod.rs
- Scene load/save triggers in src/commands/system/scene.rs
- TITLE command triggers in src/commands/system/misc.rs
- Script execution triggers in src/app/script_exec/mod.rs
- Test infrastructure in src/tests/common.rs
- Command handlers in src/commands/system/misc.rs

**Animation Behavior:**
- Each character randomly scrambles with ASCII printables
- Characters reveal in rolling sequence, not all at once
- Creates dynamic, organic reveal effect
- Animation state stored in App for event loop updates
- Four distinct animation modes provide visual variety
- Speed control scales animation duration proportionally
- Curve option: Linear (constant rate) or Settle (chars find target at 70% and hold before lock)

### SuperCollider Process Management (December 2025)

**Automatic SC Lifecycle:**
- Monokit now spawns and manages sclang automatically on startup
- No need to manually start SuperCollider first
- Graceful shutdown: sends `Server.quitAll; 0.exit;` before killing
- Kills both sclang and scsynth processes on exit
- 20-second timeout waiting for SC to boot

**New Files:**
- `src/sc_process.rs` - SC process spawn/stop/restart management
- `src/utils.rs` - Shared utilities (quote-respecting string split)
- `CROSS_PLATFORM_AUDIO.md` - Future cross-platform audio design notes
- `AUDIO_DEVICE_PLAN.md` - Audio device selection implementation plan

**SC Script Changes (monokit_server.scd):**
- Reads `MONOKIT_AUDIO_OUT` environment variable for device selection
- Sends `/monokit/ready` OSC message when server is booted
- Adds `/monokit/audio/out/query` responder for device enumeration

**Ready Detection:**
- Meter thread parses `/monokit/ready` and `AudioDeviceList` events
- Main loop waits for SC ready signal before starting TUI

### Error Display Consistency (December 2025)

**Unified Error Formatting:**
- All errors now consistently use "ERROR:" prefix in uppercase
- REPL view: Detects ERROR:/Error:, strips prefix, uppercases, re-adds with red color
- Script view: Adds "ERROR:" prefix and uppercases validation errors
- Removed duplicate "ERROR: ERROR:" issue

**Error Sources Fixed:**
- Removed ERROR: from Err() returns in validate.rs and patterns/common.rs
- Added ERROR: to all direct output() error calls in misc.rs (25+ messages)
- Standardized loops.rs and mod.rs to use "ERROR:" not "Error:"

### Negative Number Handling (December 2025)

**Crash Prevention:**
- TOG/EITH in eval/logic.rs now check bounds before second argument eval
- Pattern expressions (PN.NEXT, PN.*, etc.) validate negative values before usize conversion
- Pattern commands validate negative indices/lengths with clear error messages
- SCRIPT command validates range 1-8

### SEQ Highlight Fix (December 2025)

**Quote-Respecting Split:**
- Created shared `split_respecting_quotes()` in src/utils.rs
- State highlight now uses same logic as execution layer
- Fixes: `PF SEQ "5000*15 1250"; AD 5; PA 0` now highlights correctly
- Semicolons after quoted strings no longer break highlighting

### Command Validation & Error Display Fixes (December 2025)

**Command Validation Audit (20+ missing commands):**
- CPU, BPM, HEADER - System query commands now validated
- METER.HDR, METER.GRID, SPECTRUM, ACTIVITY, GRID, GRID.MODE - UI toggles now validated
- HL.SEQ, HL.COND - Highlighting commands now validated
- N1, N2, N3, N4 counter queries now validated
- N1.MIN, N1.MAX (and N2-N4 equivalents) now validated
- TITLE, FLASH, CLEAR - Utility commands now validated
- REPL.DUMP - REPL export command now validated
- OUT.ERR, OUT.ESS, OUT.QRY, OUT.CFM - Output control commands now validated

**PN Expression Validation:**
- PN.* commands now require pattern argument even when used as expressions
- Prevents cryptic "not enough arguments" errors during evaluation
- Examples: PN.NEXT, PN.MIN, PN.MAX, PN.SUM, PN.AVG properly validated

**Error Display Improvements:**
- Errors now display in bottom border of script blocks (fits 50x18 terminal)
- Error message prefix fix: "ERROR: ERROR:" duplicate removed
- Validation errors show immediately on script page line edit

**UI Bug Fixes:**
- Ctrl+U now works on Notes page (was missing from input handler)

**Parser & Validation Improvements:**
- Semicolon in quoted strings now preserved (`PRINT "hello;world"` works correctly)
- Negative parameter values properly validated across all synth parameters
- DEL/DEL.X/DEL.R bounds validation enforced (max 16000ms, count >= 1)
- Bool parameters reject values > 1 consistently
- SEQ syntax errors now descriptive instead of generic "UNKNOWN COMMAND"
- Extra arguments rejected for zero-arg commands (TOSS, TR, RST, etc.)
- Pattern bounds fixed in eval (patterns 4-5 now work in expressions)
- DEL commands with colons no longer parsed as conditionals

**Pattern Query Expression Support:**
- P.N, P.L, P.I now work in expressions (`A P.N`, `PRINT P.L`)
- P.MIN, P.MAX, P.SUM, P.AVG, P.FND now expression-compatible
- PN.MIN, PN.MAX, PN.SUM, PN.AVG, PN.FND for explicit patterns
- M.SCRIPT M now accepts "M" as alias for script 8

### REPL Test Suite (December 2025)

**Comprehensive Test Coverage:**
- 12 test scenes created in `repl_tests/` directory
- Each scene exercises specific command categories
- All scenes include REPL.DUMP commands for output verification
- Detailed findings documented in `repl_tests/TEST_FINDINGS.md`

**Test Coverage Areas:**
- Buffer FX: Beat Repeat (BR.*), Pitch Shift (PS.*)
- Effects: Filter, Resonator, Delay, Reverb, EQ, LoFi, Ring Mod, Compressor, Routing
- Envelopes: All 6 envelope types (AENV, PENV, FMEV, DENV, FBEV, FLEV) with aliases
- Math/Logic: ADD/SUB/MUL/DIV/MOD/MAP, comparisons, IF/ELIF/ELSE, PROB/EV/SKIP, L loops
- Metro/Timing: M, M.BPM, M.ACT, M.SCRIPT, DEL commands
- Patterns: P.* and PN.* operations (state, query, manipulation, math)
- Quantize: N operator, Q quantization, Q.ROOT, Q.SCALE, Q.BIT custom scales
- Randomization: RND.*, pattern randomization, TOG/TOSS/EITH
- SEQ/Random: Inline sequences with all Phase 2 features (repeat, toggle, random choice)
- Synth/Osc: Primary/modulator oscillators, FM, feedback, discontinuity, mix controls
- UI/System: DEBUG tiers, OUT.* overrides, HEADER, CPU, METER.*, SCOPE.*, SLEW
- Vars/Counters: Variables A-D, X-Z, T, J, K; Counters N1-N4 with MIN/MAX/RST

**Test Results:**
- 7 tests passed fully
- 5 tests passed with minor issues (documented in findings)
- No critical bugs found

**Key Findings:**
- **REPL.DUMP Fix Applied:** Now works in script context, not just interactive mode
- **Pattern Query Limitation:** P.MIN, P.MAX, P.SUM, P.AVG, P.FND (and PN.* equivalents) not valid as expressions
- **Metro Query Issue:** `PRINT M` doesn't work (M query not expression-compatible)
- **Bool Param Edge Case:** Minor validation issue in command chains
- Several test script authoring errors identified and documented

**Files:**
- 12 scene files: `test_*.json` (buffer_fx, effects, envelopes, math_logic, metro_timing, patterns, quantize, randomization, seq_random, synth_osc, ui_system, vars_counters)
- 12 dump files: `dump_*.txt` (REPL output from each test)
- `TEST_FINDINGS.md` (comprehensive analysis and recommendations)

### Error Test Suite & Batch Mode (December 2025)

**Batch Mode Implementation:**
- Added `--run <scene>` flag to main.rs for automated testing
- Non-interactive mode: runs scene without TUI
- Exit after execution completes or on error
- Enables CI/CD testing and automation workflows

**Error Test Coverage:**
- 10 comprehensive error test scenes in `repl_tests/`
- Tests validate error handling across all command categories
- Systematic coverage of edge cases and invalid inputs
- Test runner scripts: `run_error_tests.sh`, `analyze_error_tests.sh`

**Key Findings:**
- Detailed error test report: `repl_tests/ERROR_TEST_REPORT.md`
- Documents expected vs actual error behavior
- Identifies validation gaps and improvement opportunities
- Comprehensive test methodology for regression prevention

**Files:**
- 10 error scene files: `error_test_*.json`
- Test automation: `run_error_tests.sh`, `analyze_error_tests.sh`
- Analysis: `ERROR_TEST_REPORT.md`

### Tiered REPL Verbosity System (December 2025)

**DEBUG Command Enhancement:**
- DEBUG now supports tiers 0-5 (was 0-2)
- Tier 0: SILENT - Nothing (use category overrides)
- Tier 1: ERRORS - All error messages
- Tier 2: ESSENTIAL - State changes (scene, metro, rec)
- Tier 3: QUERIES - Value read responses
- Tier 4: CONFIRMS - Set confirmations
- Tier 5: VERBOSE - All output

**Category Override Commands:**
- OUT.ERR <0|1> - Override: show errors
- OUT.ESS <0|1> - Override: show essential
- OUT.QRY <0|1> - Override: show queries
- OUT.CFM <0|1> - Override: show confirms

**REPL.DUMP Command:**
- REPL.DUMP [filename] - Export REPL contents to text
- Default filename: repl_dump.txt

**Background Error Routing:**
- Meter/MIDI errors now route to REPL (was stderr)
- Uses MetroEvent::Error for thread communication

### UI Polish (December 2025)

**Search Input Display:**
- Search input now displays in uppercase (Teletype aesthetic)
- Maintains consistent visual style with rest of UI

**Tab Hotkey Enhancement:**
- Tab now navigates to Live page from any other page
- On Live page, Tab continues to toggle grid view
- Dual function improves navigation flow

### Phase 4: Polish & UI Preferences

**Config Persistence (December 2025)**
- Added 16 new DisplayConfig fields for UI preferences persistence
- All UI toggles now save to ~/.config/monokit/config.toml
- Settings persist across sessions
- Config automatically created on first launch if missing

**New Persisted Settings:**
- show_header_meters: Audio meters in header (default: true)
- show_grid_meters: Audio meters in grid view (default: true)
- show_spectrum: Spectrum analyzer in grid view (default: true)
- show_activity: Script activity indicators (default: true)
- show_grid: Parameter activity grid (default: true)
- grid_icons_mode: Grid labels (0) vs icons (1) (default: true)
- highlight_seq: SEQ/TOG state highlighting (default: true)
- header_verbosity: Header display mode 0-4 (default: 4)
- show_cpu_meter: CPU percentage display (default: true)
- highlight_cond: Conditional execution highlighting (default: true)
- load_reset_mode: Scene load behavior (default: false)
- debug_level: REPL output verbosity (default: 2)
- scope_time_ms: Oscilloscope timespan (default: 30)
- scope_color: Waveform color 0-3 (default: 0)
- scope_mode: Display mode 0-4 (default: 0)
- scope_unipolar: Unipolar mode (default: false)

**UI Toggle Commands (December 2025)**
- METER.HDR <0|1> - Toggle header audio meters
- METER.GRID <0|1> - Toggle grid view audio meters
- SPECTRUM <0|1> - Toggle spectrum analyzer in grid view
- ACTIVITY <0|1> - Toggle script activity indicators (Phase 4.1)
- GRID <0|1> - Toggle parameter activity grid (Phase 4.4)
- GRID.MODE <0|1> - Grid display: 0=labels, 1=icons (Phase 4.4)
- HL.SEQ <0|1> - Toggle SEQ/TOG state highlighting (Phase 4.2)
- HL.COND <0|1> - Toggle conditional execution highlighting (Phase 4.7)

**Grid View Layout Stability (December 2025)**
- Fixed spectrum/meters positioning when GRID visibility toggles
- Grid reserves fixed 30 chars regardless of GRID.MODE (labels/icons)

**New Command Aliases (December 2025)**
Added envelope parameter aliases for consistency:
- AA → AENV.ATK (Amplitude envelope attack)
- PAA → PENV.ATK (Pitch envelope attack)
- FAA → FMEV.ATK (FM envelope attack)
- DAA → DENV.ATK (Discontinuity envelope attack)
- FBAA → FBEV.ATK (Feedback envelope attack)
- FLAA → FLEV.ATK (Filter envelope attack)
- AC → AENV.CRV (Amplitude envelope curve)
- PC → PENV.CRV (Pitch envelope curve)
- FBC → FBEV.CRV (Feedback envelope curve)
- FLC → FLEV.CRV (Filter envelope curve)

Note: FMEV.CRV and DENV.CRV have no short aliases (FC=FILT.CUT, DC=DISC.AMT conflicts)

**Scene Name Header Display (December 2025)**
- TITLE <0|1> command toggles header between "MONOKIT" and scene name
- Shows "[UNSAVED]" when no scene loaded
- Scene names truncated to 15 characters in header
- Setting persists to config.toml

**BPM Header Display (December 2025)**
- "BPM XXX" shown in header at HEADER levels 1-4
- Real-time updates with formula: 15000 / interval_ms
- Displays current tempo based on metro interval

**Global Text Audit (December 2025)**
- All UI text now complies with 46-char width limit
- Fixed 70+ violations across system (errors, help, footer, startup)
- Common term abbreviations: DEF, MS, ENV, etc.

**Envelope ATK/CRV Alias Integration (December 2025)**
- ATK/CRV aliases fully integrated with validator and handlers
- Short forms work: AA, PAA, FAA, DAA, FBAA, FLAA, AC, PC, FBC, FLC
- Complete alias system for envelope parameters

**Help System Updates (December 2025)**
- Added "UI & DISPLAY" category documenting all UI toggle commands
- Added "OSCILLOSCOPE" category documenting scope commands
- Help system now covers all 7 UI control commands
- Comprehensive documentation for meter, spectrum, activity, and grid toggles

**Files Changed:**
- src/config.rs - Added 16 DisplayConfig fields with persistence
- src/app/mod.rs - Read config on startup, write on command changes
- src/commands/system/misc.rs - Added toggle command handlers (METER.HDR, METER.GRID, SPECTRUM, ACTIVITY, GRID, GRID.MODE, HL.SEQ)
- src/commands/aliases.rs - Added 10 new envelope aliases
- src/help_content.rs - Added "UI & DISPLAY" and "OSCILLOSCOPE" categories

---

## Recent Updates (November 2025)

### Timing Infrastructure Improvements

**Enhanced Timing Precision:**
- Platform-calibrated hybrid sleep timing with `spin_sleep` crate
- Real-time thread priority elevation on supported platforms via `audio_thread_priority`
- All OSC messages sent as timestamped bundles with NTP timestamps
- Sample-accurate scheduling in SuperCollider
- Created `src/osc_utils.rs` for timestamped OSC bundle generation

**Technical Details:**
- SpinSleeper automatically calibrates to platform sleep characteristics
- RT thread priority reduces scheduling latency on macOS, Linux, Windows
- OSC timestamps calculated relative to immediate time (`rosc::Instant::Immediate.whole_millis()`)
- All parameter updates and triggers bundled with timestamps for SC scheduler
- Improved stability and timing accuracy for metro thread

**Files Changed:**
- `Cargo.toml` - Added `spin_sleep` 1.2 and `audio_thread_priority` 0.32 dependencies
- `src/osc_utils.rs` - New utility module for timestamped OSC bundles
- `src/metro.rs` - SpinSleeper and RT priority integration
- `src/main.rs` - Module declaration for `osc_utils`

### MIDI Clock Sync

**MIDI Clock Input:**
- External tempo sync from DAW or MIDI clock devices
- Transport start/stop follows MIDI messages
- 16th note resolution (each MIDI clock tick = metro tick)
- Auto-detection of available MIDI inputs
- `M.SYNC <0|1>` - Set sync mode (0=internal, 1=MIDI clock)
- `MIDI.IN` - List available MIDI input devices
- `MIDI.IN <name>` - Connect to specific MIDI device

When M.SYNC is enabled (1), the metro follows external MIDI clock. Transport start/stop messages control metro activation.

### REPL Scrolling and UI Enhancements

**REPL Output Scrolling (Live Page):**
- Ctrl+Up/Down scrolls through REPL output history
- Shows scroll indicator [↑N] in title when scrolled up from bottom
- Auto-resets to bottom when new output arrives
- Smooth navigation through command history and output

**Help System Updates:**
- Added SEQ inline sequence documentation
- Added PSET preset system documentation
- Added REPL scrolling keybinding documentation
- Comprehensive coverage of all system features

**Roadmap Updates:**
- "Live Sequence State Highlighting" feature added to Phase 4 (UI enhancements)

### SEQ/TOG State Highlighting (December 2025)

**Phase 4.2: State Highlighting - COMPLETE**
- Color-only highlighting for current step in SEQ patterns
- Current TOG option highlighted
- Nested alternation `<a b>` shows active option based on stored state
- Nested random choice `{a b}` shows last selected option (state now tracked)
- Integrated into Script, Metro, and Init pages
- Color strategy: non-selected lines use foreground/secondary, selected lines use success/highlight_fg

**SEQ Validation:**
- Added validation to reject `SEQ"..."` (no space before quote)
- Added validation to reject `SEQ "...` (unclosed quote)
- Invalid lines are not saved to scripts

**Random Choice State Tracking:**
- Random choice `{}` now stores selected index in toggle_state with key `seq_rnd_{script}_{pattern}_{step}`
- UI can now accurately highlight which option was randomly selected

**Files Created:**
- `src/ui/state_highlight.rs` - New module for highlighting stateful operators

### Variables Page (December 2025)

**Phase 4.3: Variable State Monitoring - COMPLETE**
- New Variables page shows all variable state in Teletype-style monitor layout
- Access via `Alt+V` or `[`/`]` navigation
- Displays global variables: A, B, C, D, X, Y, Z, T
- Displays counters N1-N4 with their min/max bounds
- Displays per-script local variables: J, K for all 10 scripts (Scripts 1-8, M, I)
- Clean three-column layout for easy scanning
- Added `Page::Variables` to navigation system

**Notes Page (December 2025)

**Phase 4.6: Notes Page Redesign - COMPLETE**
- Redesigned to use 8 fixed lines like script pages
- Line navigation with Up/Down arrows
- Same editing commands: Ctrl+D/K/C/X/V
- Notes saved and loaded with scenes
- Consistent with script page UX

**Bug Fixes:**
- SLEW and SLEW.ALL commands now accept expressions in scripts (fixed strict arg count validation)
- Empty line cursor visibility restored on Script 1-8, Metro, and Init pages

**Files Created:**
- `src/ui/pages/variables.rs` - New page implementation

### Parameter Activity Grid (December 2025)

**Phase 4.4: Parameter Activity Grid - COMPLETE**
- Alternate view on Live page showing parameter activity as grid
- Press Tab to toggle between REPL view and Grid view
- 8x6 grid of 48 unicode icons representing synth parameters
- Icons light up and decay when their parameters change (reuses activity_color())
- Grid is center-justified in the content area with 3-space gaps between icons
- Activity tracked for: oscillators (PF, PW, MF, MW), FM/FB, filter, effects, envelopes, modulation, output

**Files Changed:**
- `src/types.rs` - Added ParamActivity struct, GRID_ICONS const
- `src/app/mod.rs` - Added param_activity and show_grid_view fields
- `src/ui/pages/live.rs` - Added grid view rendering
- `src/ui/mod.rs` - Added Tab keybinding
- `src/app/script_exec/mod.rs` - Mark param activity on script execution
- `src/app/script_exec/interactive.rs` - Mark param activity on REPL commands

### Audio Metering (December 2025)

**Phase 4.5: Audio Metering - COMPLETE**
- Real-time stereo peak/RMS metering via bidirectional OSC
- SuperCollider sends meter data at 20Hz via SendPeakRMS to port 57121
- Meter thread receives OSC and updates MeterData
- Header displays compact meters: `L▅▆ R▅▅`
- Clip indicator (error color) resets when level drops
- REC indicator moved to right-aligned border title
- Vertical 8-row meters on grid view (Tab on Live page) with 64 levels of resolution
- 15-band spectrum analyzer on grid view below parameter grid, 2 rows tall
- Logarithmically-spaced frequency bands (25Hz to 16kHz)
- Square root scaling for better visual response with smooth decay (0.85 rate)
- Per-band clipping: each spectrum band shows red when exceeding 0.95 threshold
- SuperCollider uses BPF filters + Amplitude followers at 20Hz, OSC to port 57121
- CPU percentage display in header border (right-aligned, toggleable with CPU 1/0 command)
- CPU percentage text on grid view (bottom-right, aligned with spectrum bottom): right-aligned "   XX%"
- CPU label on grid view (same row as SPECTRUM label, right side)
- CPU color scheme: secondary normally, error when >= 80%
- SuperCollider sends CPU data at 2Hz via /monokit/cpu OSC message

**Files Created/Changed:**
- `src/meter.rs` - New receiver thread for meter data
- `src/types.rs` - Added MeterData struct, MetroEvent::MeterUpdate
- `src/app/mod.rs` - Added meter_data field
- `src/main.rs` - Spawns meter thread
- `src/ui/mod.rs` - Handles MeterUpdate events
- `src/ui/header.rs` - Bargraph rendering, REC in border title
- `src/ui/pages/live.rs` - Vertical meters on grid view, spectrum analyzer rendering
- `sc/monokit_server.scd` - Added SendPeakRMS and OSCdef forwarder, spectrum analysis

### Conditional Execution Highlighting (December 2025)

**Phase 4.7: Conditional Execution Highlighting - COMPLETE**
- Visual feedback when PRE conditionals pass and execute their body commands
- Segment-based highlighting: only the PRE portion highlights, not the entire line
- Supported PREs: IF, ELIF, ELSE, PROB, EV, SKIP (NOT L loops - they would flash constantly)
- Example: `$ 2; IF PN.NEXT 2: TR` - only `IF PN.NEXT 2:` highlights when condition passes
- Multiple PREs on same line highlight independently
- Nested PREs (like `EV 4: IF A: TR`) each highlight their own segment
- Reuses activity_color() decay animation system from Phase 4.1
- Color strategy: unselected lines use foreground->secondary decay, selected lines use success->highlight_fg decay
- `HL.COND <0|1>` command to toggle conditional highlighting on/off
- State tracked per script line in App struct

**Files Changed:**
- `src/app/mod.rs` - Added cond_highlight_enabled and conditional activity tracking
- `src/app/script_exec/control_flow.rs` - Mark conditional segments on execution
- `src/ui/state_highlight.rs` - Extended with conditional highlighting logic
- `src/ui/pages/script.rs` - Render conditional highlights on script pages
- `src/ui/pages/metro.rs` - Render conditional highlights on metro page
- `src/ui/pages/init.rs` - Render conditional highlights on init page
- `src/commands/system/misc.rs` - Added HL.COND command handler

### Scope Page (December 2025)

**Phase 4.8: Oscilloscope Page - COMPLETE**
- Dedicated oscilloscope page showing real-time waveform display
- 128 samples at 20Hz from SuperCollider via OSC
- Multiple character rendering modes with unicode support
- Configurable timespan, color, and display modes
- DC blocking added to synth output via LeakDC
- Info displayed on bottom border (TIME, SAMPLES)

**Commands:**
- `SCOPE.TIME <5-500>` - Set waveform timespan in milliseconds (default 30, accepts expressions)
- `SCOPE.CLR <0-3>` - Set waveform color (accepts expressions):
  - 0 = success (green)
  - 1 = error (red)
  - 2 = foreground
  - 3 = accent
- `SCOPE.MODE <0-4>` - Set display mode (accepts expressions):
  - 0 = BRAILLE (2×4 dots, highest resolution)
  - 1 = BLOCK (vertical bars ▁▂▃▄▅▆▇█)
  - 2 = LINE (line drawing ─╱╲)
  - 3 = DOT (scatter plot ●)
  - 4 = QUADRANT (2×2 blocks ▖▗▘▙▚▛▜▝▞▟)
- `SCOPE.UNI <0|1>` - Unipolar mode: 0=bipolar (±1 range), 1=unipolar (rectified, 2× resolution, accepts expressions)

**Navigation:**
- `Alt+S` - Jump to Scope page
- `[` / `]` - Cycle through pages including Scope

**Files Created/Changed:**
- `src/ui/pages/scope.rs` - New page implementation
- `src/ui/braille.rs` - Character rendering utilities
- `src/meter.rs` - Scope data receiver (20Hz updates via OSC)
- `src/types.rs` - Added ScopeData struct, SCOPE_SAMPLES const
- `src/app/mod.rs` - Added scope state fields
- `sc/monokit_server.scd` - Added LeakDC to synth output, scope data sender

### DRY Refactoring Complete (All Phases)

**Total lines removed: ~5,942 lines**

| Phase | Target | Reduction | Status |
|-------|--------|-----------|--------|
| Phase 0 | Reorganization | - | ✓ Complete |
| Phase 1 | Envelopes | 918 lines (81%) | ✓ Complete |
| Phase 2 | Patterns | 1,573 lines (78%) | ✓ Complete |
| Phase 3 | Synth Params | 2,325 lines (80%) | ✓ Complete |
| Phase 4A | Variable/Counter Macros | 489 lines (66%) | ✓ Complete |
| Phase 4B | Expression Helpers | +39 lines (infra) | ✓ Complete |
| Phase 4C | Test Fixtures | 637 lines | ✓ Complete |

**Macro Systems Created:**
- `synth/envelopes/common.rs` - Envelope parameter macros
- `patterns/common.rs` - Pattern operation macros
- `synth/common.rs` - Synth parameter macros (6 types)
- `core/variables.rs` - Variable handler macros
- `core/counters.rs` - Counter handler macros
- `commands/common.rs` - Expression parsing helpers
- `tests/common.rs` - Test setup macro

All 558 tests pass. Codebase reduced by ~28% while maintaining full functionality.

---

### Phase 1 DRY Refactoring Complete
**Changes:** Envelope handler consolidation using `define_int_param!` and `define_float_param!` macros.

**Results:**
- Created `synth/envelopes/common.rs` with shared macro definitions
- Refactored all 6 envelope files to use macros (amp, pitch, fm, disc, feedback, filter)
- Removed dead code: `handle_*_mode` handlers, `global.rs`
- Line reduction: ~1,141 lines → 223 lines (~918 line reduction, 81% decrease)
- All 558 tests pass

**Technical Details:**
- Each envelope file reduced from ~140 lines to ~10-12 lines
- Shared parameter generation via macros eliminates boilerplate
- Maintains exact same functionality and API
- Foundation for Phase 2 (Pattern Operation DRY) and Phase 3 (Synth Parameter DRY)

---

### Envelope System Simplification
**Changes:** Envelope system refactored to use simple `Env.perc` with controllable attack and curve parameters.

**What Changed:**
- Removed gate-based envelope triggering (no gate parameter, no mode switching)
- Simplified to single percussive envelope type per parameter
- Each envelope has: decay time, attack time, curve, and amount
- Fixed pitch envelope parameter routing with `Lag.kr` control capture

**Removed Commands:**
- `ENV.ATK`, `ENV.DEC`, `ENV.CRV`, `ENV.MODE` - Global envelope controls
- `GATE` - Global gate duration
- `*.MODE`, `*.GATE` - Per-envelope mode and gate overrides (AENV.MODE, PENV.GATE, etc.)

**Added Commands:**
- `FBEV.AMT` - Alias for FBA (feedback envelope amount)

**Current Envelope Commands:**

| Envelope | Decay | Amount | Attack | Curve |
|----------|-------|--------|--------|-------|
| Amp | AD | - | AENV.ATK | AENV.CRV |
| Pitch | PD | PA | PENV.ATK | PENV.CRV |
| FM | FD | FA | FMEV.ATK | FMEV.CRV |
| Disc | DD | DA | DENV.ATK | DENV.CRV |
| Feedback | FBD | FBA/FBEV.AMT | FBEV.ATK | FBEV.CRV |
| Filter | FED | FE | FLEV.ATK | FLEV.CRV |

**Technical Details:**
- Exponential pitch envelope scaling: `pow(2, pitchEnv * pa)` for proper octave behavior
- PA parameter represents octaves (PA=4 = 4 octaves of pitch sweep)
- Control capture using `Lag.kr(param, 0)` prevents UGen graph optimization issues
- All envelopes trigger on each `TR` command

**Files Changed:**
- `sc/monokit_server.scd` - Envelope implementation and control captures
- `src/commands/synth/envelopes/*.rs` - Envelope command handlers

---

## Documentation

### User Documentation
- **MANUAL.md** - Comprehensive user manual with command reference and tutorials
- **README.md** - Project overview, installation instructions, feature rundown

### Development Documentation
- **ROADMAP.md** - Single source of truth for all future development plans and roadmap
- **CONCEPT.md** - Project overview, architecture, MVP implementation
- **PLAN.md** - UI refactor plan: Teletype-style interface with page system, script storage, patterns, and control flow

### Release & Distribution
- **RELEASE_PIPELINE.md** - Automated release workflow: local builds, GitHub Actions, Homebrew updates
- **HOMEBREW_BUNDLE_FORMULA.md** - Homebrew formula documentation for monokit tap
- **BUNDLE_QUICK_START.md** - Quick start guide for bundle distribution

### Technical Specifications
- **scsynth_direct_integration.md** - Direct scsynth integration design and implementation
- **DSP_TIER1_IMPLEMENTATION_PLAN.md** - Detailed implementation plan for Filter, Resonator, Delay, and Reverb DSP blocks
- **DSP_TIER3_BUFFER_EFFECTS_PLAN.md** - Implementation plan for Beat Repeat and Pitch Shift buffer effects
- **EFFECT_ROUTING_DESIGN.md** - Design document for flexible effect routing system
- **DRY_REFACTOR_PLAN.md** - Comprehensive refactoring plan for codebase reorganization and DRY consolidation
- **DRY_ANALYSIS_REPORT.md** - Detailed analysis of code duplication and refactoring opportunities (~4,000-5,000 line reduction potential)
- **UI_REFINEMENT_PLAN.md** - Detailed implementation plan for UI enhancements: activity indicators, variable monitoring, SEQ/TOG highlighting, audio metering
- **MIDI_CLOCK_TIMING_LESSONS.md** - MIDI clock sync timing diagnostics and lessons learned

### Deferred Features
- **ON_HOLD.md** - Features deferred indefinitely due to technical constraints (LFO system, Aux Envelopes, Extended Envelopes, MIDI Clock Output, Initial Window Sizing)

### Cross-Reference
- **documentation_index.md** - This file, listing all documentation and key project files

## Key Project Files

### Configuration

- **Cargo.toml** - Rust project manifest with dependencies (rosc, ratatui, crossterm, nom, anyhow, thiserror, serde)
- **Cargo.lock** - Dependency lock file

### Source Code

Modular Rust implementation (~17,300 total lines across 93 files, after Phase 1 DRY refactoring):

- **src/main.rs** (69 lines) - Application entry point, initializes TUI and starts main loop
- **src/metro.rs** (112 lines) - Metro thread implementation with absolute timing
- **src/types.rs** (233 lines) - Core data structures, enums, constants, and type definitions
- **src/eval/** - Expression evaluation module (~720 lines across 4 files)
  - **mod.rs** - Core dispatch, resolve_value, quantize_note, eval_condition
  - **patterns.rs** - Pattern expression handling (PN.*, P.*)
  - **math.rs** - Math operators (ADD, SUB, MUL, DIV, MOD, Q, N)
  - **logic.rs** - Comparisons, RND, TOG, MAP, counters
- **src/ui/** - TUI rendering module (~1,370 lines across 10 files)
  - **mod.rs** - Module coordinator
  - **header.rs** - Header rendering
  - **footer.rs** - Footer rendering
  - **pages/** - Page implementations (7 files: live, script, metro, init, pattern, help, mod)
- **src/scene.rs** (169 lines) - Scene persistence, file I/O
- **src/theme.rs** (67 lines) - Theme struct and built-in themes (dark, light, system)
- **src/config.rs** (170 lines) - Configuration loading, named theme support
- **src/app/** - Application module (~750 lines across 6 files)
  - **mod.rs** - App struct, constructor, navigation
  - **input.rs** - Input handling methods
  - **script_exec/** - Script/command execution module (4 files)
    - **mod.rs** - Core execution, metro updates
    - **control_flow.rs** - If/elif/else, conditionals
    - **loops.rs** - Loop processing
    - **interactive.rs** - User command execution
- **src/commands/** - Command processing module (48 files, ~8,167 lines, -918 from Phase 1)
  - **mod.rs** - Main dispatcher with command routing
  - **validate.rs** - Command validation
  - **aliases.rs** - Alias resolution for PREFIX.SUFFIX → short form mapping (93 aliases)
  - **core/** - Core language primitives (7 files)
    - **mod.rs** - Module coordinator
    - **variables.rs** - Variable handlers (A-D, X-Z, T, I, J, K)
    - **counters.rs** - Auto-increment counters (N1-N4 with MIN/MAX/RST)
    - **math_ops.rs** - Math operations (ADD, SUB, MUL, DIV, MOD, MAP)
    - **random_ops.rs** - Random operations (RND, RRND, TOSS, EITH, TOG)
    - **scale.rs** - Quantization (Q, Q.ROOT, Q.SCALE, Q.BIT)
    - **scheduling.rs** - Delayed execution (DEL, DEL.CLR, DEL.X, DEL.R)
  - **patterns/** - Pattern operations module (9 files)
    - **mod.rs** - Module coordinator
    - **working.rs** - Working pattern state (P.N, P.L, P.I)
    - **working_query.rs** - Working pattern queries (P.HERE, P.NEXT, P.PREV)
    - **working_manip.rs** - Working pattern manipulation (P.PUSH, P.POP, P.INS, P.RM, P.REV, P.ROT, P.SHUF, P.SORT, P.RND)
    - **working_math.rs** - Working pattern math (P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE, P.MIN, P.MAX, P.SUM, P.AVG, P.FND)
    - **explicit.rs** - Explicit pattern state (PN.N, PN.L, PN.I)
    - **explicit_query.rs** - Explicit pattern queries (PN.HERE, PN.NEXT, PN.PREV)
    - **explicit_manip.rs** - Explicit pattern manipulation (PN.PUSH, PN.POP, PN.INS, PN.RM, PN.REV, PN.ROT, PN.SHUF, PN.SORT, PN.RND)
    - **explicit_math.rs** - Explicit pattern math (PN.ADD, PN.SUB, PN.MUL, PN.DIV, PN.MOD, PN.SCALE, PN.MIN, PN.MAX, PN.SUM, PN.AVG, PN.FND)
  - **system/** - System and session commands (4 files)
    - **mod.rs** - Module coordinator
    - **metro.rs** - Metro commands (M, M.BPM, M.ACT, M.SCRIPT)
    - **scene.rs** - Scene commands (SAVE, LOAD, SCENES, DELETE)
    - **misc.rs** - Other system commands (TR, RST, SCRIPT, THEME, DEBUG, PRINT, HELP, CLEAR, REC)
  - **synth/** - Synth parameter handlers (18 files, reorganized from synth_params/)
    - **mod.rs** - Module coordinator
    - **oscillator.rs** - Oscillator parameters (PF, PW, MF, MW, FB)
    - **modulation.rs** - Modulation and tracking (TK, MB, MP/MD/MT/MA, FM, MX, MM, ME)
    - **discontinuity.rs** - Discontinuity/waveshaping (DC, DM)
    - **slew.rs** - Parameter slewing (SLEW.ALL global, SLEW per-parameter)
    - **output.rs** - Output parameters (VOL, PAN)
    - **envelopes/** - Envelope module (8 files, 223 lines total after Phase 1 DRY)
      - **mod.rs** - Module coordinator
      - **common.rs** - Shared macro definitions (define_int_param!, define_float_param!)
      - **amp.rs** - Amplitude envelope (AD, AENV.ATK, AENV.CRV) - 11 lines
      - **pitch.rs** - Pitch envelope (PD, PA, PENV.ATK, PENV.CRV) - 12 lines
      - **fm.rs** - FM envelope (FD, FA, FMEV.ATK, FMEV.CRV) - 12 lines
      - **disc.rs** - Discontinuity envelope (DD, DA, DENV.ATK, DENV.CRV) - 11 lines
      - **feedback.rs** - Feedback envelope (FBD, FBA/FBEV.AMT, FBEV.ATK, FBEV.CRV) - 10 lines
      - **filter.rs** - Filter envelope (FED, FE, FLEV.ATK, FLEV.CRV) - 10 lines
    - **filter.rs** - SVF filter parameters (FC, FQ, FT, FK, MF.F)
    - **resonator.rs** - Comb resonator (RF, RD, RM, RK)
    - **effects/** - Time-based and processing effects (9 files)
      - **mod.rs** - Module coordinator
      - **delay.rs** - Stereo delay (DT, DF, DLP, DW, DS, D.MODE, D.TAIL)
      - **reverb.rs** - Plate reverb (RV, RP, RH, RW, R.MODE, R.TAIL)
      - **eq.rs** - 3-band EQ (EL, EM, EF, EQ, EH)
      - **lofi.rs** - Lo-Fi processor (LB, LS, LM)
      - **ring_mod.rs** - Ring modulator (RGF, RGW, RGM)
      - **compressor.rs** - Compressor (CT, CR, CA, CL, CM)
      - **beat_repeat.rs** - Beat repeat (BR.LEN, BR.REV, BR.WIN, BR.MIX)
      - **pitch_shift.rs** - Pitch shift (PS.MODE, PS.SEMI, PS.GRAIN, PS.MIX, PS.TARG)
  - **randomization.rs** - Randomization commands (RND.VOICE, RND.OSC, RND.FM, RND.MOD, RND.ENV, etc.)
- **src/tests/** - Test suite module (21 files, ~5,288 lines, 558 tests)
  - Organized by category: envelope, counter, slew, tog, rnd, toss_eith, expr, condition, pattern, pattern_ops, variable, validation, math, map, comparison, scene, debug, buffer_effects

### REPL Test Scenes

- **repl_tests/** - Comprehensive REPL command test suite (27 files)
  - 12 test scene files (.json): buffer_fx, effects, envelopes, math_logic, metro_timing, patterns, quantize, randomization, seq_random, synth_osc, ui_system, vars_counters
  - 12 dump files (.txt): REPL output verification from each test
  - TEST_FINDINGS.md: Detailed analysis of test results and issues found
  - Created December 2025 to validate all major command categories
  - 7 full passes, 5 partial passes, no critical bugs
  - Identified REPL.DUMP fix need (now applied)
  - Documented pattern query expression limitation

Key features:
- Page-based interface: Live, Script 1-8, Metro (M), Init (I), Pattern (P), Help (paginated with 10 category pages)
- Script storage: 10 scripts × 8 lines (Scripts 1-8, M, I)
- Pattern storage: 6 patterns × 64 steps (i16 values, patterns 0-5)
- Variables: A-D, X-Y-Z-T (global), J-K (per-script local), I (loop counter)
- Control flow: IF/ELIF/ELSE conditions, PROB probabilistic, EV/SKIP every-N-tick
- Comparison operators: EZ, NZ, EQ, NE, GT, LT, GTE, LTE (return 1/0)
- N operator: Semitone to frequency conversion (N 0 = C3 = 131 Hz)
- Expression evaluation in all numeric arguments
- Metro thread sends script execution requests to main thread
- OSC client sending to SuperCollider (127.0.0.1:57120)

- **sc/monokit_server.scd** (626 lines) - SuperCollider sound engine
  - `\monokit` SynthDef: HD2-style dual oscillator with FM, discontinuity, comprehensive DSP effects, and multi-stage processing
  - Additive envelope model: output = base + env * amount
  - Signal chain: Oscillators → FM → Mix → Discontinuity → Lo-Fi → SVF Filter → Ring Mod → Comb Resonator → Amp → Compressor → Pan → Beat Repeat → Pitch Shift → Stereo Delay → 3-Band EQ → Plate Reverb → Out
  - 77 synth parameters (25 oscillator/envelope + 48 DSP + 4 routing)
  - Includes implemented Beat Repeat and Pitch Shift buffer effects
  - Global parameter slew with Lag.kr smoothing for artifact-free transitions
  - OSC responders:
    - `/monokit/trigger` - Gate trigger (no args)
    - `/monokit/param` - Generic parameter setter (string name, float/int value)
    - `/monokit/slew` - Set global slew time (ms)
    - `/monokit/rec` - Start recording
    - `/monokit/rec/stop` - Stop recording
    - `/monokit/rec/path` - Set recording path prefix
  - Stateless sound engine (no metro logic)

### Build Artifacts

- **target/** - Rust build output (ignored by git)

---

## DRY Refactoring Plan

### Overview
The codebase contains ~6,500-7,000 lines of systematic duplication (30-33% of total code). A comprehensive refactoring plan targets **~4,000-5,000 line reduction** through reorganization and macro consolidation.

### Four-Phase Approach

**Phase 0: Codebase Reorganization [COMPLETE]**
- Reorganized command structure by logical domain (core, patterns, system, synth)
- Consolidated scattered envelope parameters into proper locations
- Moved envelope decay/amount params to respective envelope files:
  - DD from `discontinuity.rs` → `envelopes/disc.rs`
  - FBA, FBD from `oscillator.rs` → `envelopes/feedback.rs`
  - FE, FED from `filter.rs` → `envelopes/filter.rs`
- Created `synth/output.rs` to consolidate VOL and PAN
- Deleted dead code (MODE handlers, deprecated GATE commands, unused global.rs)
- Renamed `delay.rs` → `core/scheduling.rs` (handles DEL commands, not synth delay)
- Split `effects.rs` into modular effect files (lofi.rs, ring_mod.rs, compressor.rs, etc.)
- Renamed `synth_params/` → `synth/` for clarity
- All 558 tests pass

**Phase 1: Envelope Handler DRY [COMPLETE - 918 line reduction]**
- Created `synth/envelopes/common.rs` with `define_int_param!` and `define_float_param!` macros
- Generated DEC, AMT, ATK, CRV handlers for all 6 envelopes from macro definitions
- Each envelope file reduced from ~140 lines → ~10-12 lines
- Total reduction: ~1,141 lines → 223 lines (81% decrease)
- All 558 tests pass

**Phase 2: Pattern Operation DRY [COMPLETE - 1,573 line reduction in wrappers]**
- Created `patterns/common.rs` (902 lines) with `PatternRef` enum, shared implementations, and macro system
- Aggressive macro approach: 10 macros generate both P.* and PN.* handlers from single definitions
- Unified P.* (working) and PN.* (explicit) operations via PatternRef::Working/Explicit
- Wrapper code reduced from 2023 → 450 lines (78% reduction)
- Explicit files now just re-export from working files (~10 lines each)
- All 558 tests pass

**Phase 3: Synth Parameter DRY [PLANNED - ~2,000 line reduction]**
- Create `synth/param_macro.rs` with `define_param!` macro
- Generate parameter handlers from declarative specifications
- Consolidate 70+ nearly-identical parameter handlers into config-driven system

### Results So Far
- Clear, logical file organization by domain (Phase 0)
- 918 line reduction from envelope consolidation (Phase 1)
- 1,573 line reduction in pattern wrappers via macro system (Phase 2)
- Total reduction: ~2,491 lines
- Easier to add new commands (single macro invocation for both P.* and PN.*)
- All 558 tests continue to pass
- Maintains backward compatibility

### Expected Final Results
- ~4,500+ line reduction with Phase 3 (synth parameter macros)
- Further simplified maintenance through macro-driven parameter handlers

See **DRY_REFACTOR_PLAN.md** for complete implementation details and **DRY_ANALYSIS_REPORT.md** for duplication analysis.

---

## Architecture Overview

```
Rust TUI (src/main.rs)
    |
    +-- Main Thread
    |    - TUI rendering (ratatui)
    |    - Command processing
    |    - Script execution (with App context)
    |    - Pattern/variable access
    |    - Recording management (WAV int24)
    |    |
    |    v OSC messages
    |    127.0.0.1:57120
    |
    +-- Metro Thread (absolute timing)
         - Sends ExecuteScript events to main thread
         - No direct script execution (thread safety)

SuperCollider Sound Engine (sc/monokit_server.scd)
    |
    v
Audio output → Recording (optional)
```

## Command Reference

### Naming Convention

Monokit uses a **PREFIX.SUFFIX** naming convention for canonical command forms:

**Category Prefixes:**
- `POSC` - Primary Oscillator (POSC.FREQ → PF, POSC.WAVE → PW)
- `MOSC` - Modulator Oscillator (MOSC.FREQ → MF, MOSC.WAVE → MW, MOSC.FB → FB)
- `DISC` - Discontinuity/Waveshaping (DISC.AMT → DC, DISC.MODE → DM)
- `FILT` - SVF Filter (FILT.CUT → FC, FILT.RES → FQ, FILT.TYP → FT)
- `RESO` - Comb Resonator (RESO.FRQ → RF, RESO.DEC → RD, RESO.MIX → RM)
- `DLY` - Stereo Delay (DLY.TIME → DT, DLY.FB → DF, DLY.WET → DW)
- `REV` - Plate Reverb (REV.DEC → RV, REV.PRE → RP, REV.WET → RW)
- `LOFI` - Lo-Fi Processor (LOFI.BIT → LB, LOFI.SMP → LS, LOFI.MIX → LM)
- `RING` - Ring Modulator (RING.FRQ → RGF, RING.WAV → RGW, RING.MIX → RGM)
- `COMP` - Compressor (COMP.THR → CT, COMP.RAT → CR, COMP.ATK → CA)
- `EQ` - 3-Band EQ (EQ.LOW → EL, EQ.MID → EM, EQ.HI → EH)
- `MBUS` - Modulation Bus (MBUS.AMT → MB, MBUS.TRK → TK, MBUS.FM → FM)
- `ROUT` - Routing Matrix (ROUT.MP → MP, ROUT.MD → MD, ROUT.MF → MF.F)
- `OUT` - Output (OUT.VOL → VOL, OUT.PAN → PAN)
- `AENV` - Amplitude Envelope (AENV.ATK, AENV.CRV, decay → AD)
- `PENV` - Pitch Envelope (PENV.ATK, PENV.CRV, decay → PD, amount → PA)
- `FMEV` - FM Envelope (FMEV.ATK, FMEV.CRV, decay → FD, amount → FA)
- `DENV` - Discontinuity Envelope (DENV.ATK, DENV.CRV, decay → DD, amount → DA)
- `FBEV` - Feedback Envelope (FBEV.ATK, FBEV.CRV, FBEV.AMT → FBA, decay → FBD)
- `FLEV` - Filter Envelope (FLEV.ATK, FLEV.CRV, decay → FED, amount → FE)

**Alias System:**
- Current short forms (PF, FC, AD, etc.) remain as **aliases** to canonical forms
- You can use either form: `PF 440` or `POSC.FREQ 440` (both work identically)
- Legacy commands in existing scenes continue to work
- Use whichever form you prefer (terse aliases or explicit canonical names)

### Current Commands (v0.1.0)

#### Trigger & Volume
- `TR` - Trigger voice (sends gate pulse)
- `VOL <0.0-1.0>` - Set master volume

#### Metro/Timing
- `M` - Show current metro interval (milliseconds)
- `M <ms>` - Set metro interval in milliseconds
- `M.BPM <bpm>` - Set metro tempo as BPM
- `M.ACT <0|1>` - Activate (1) or deactivate (0) metro
- `M.SCRIPT <1-8>` - Set which script metro calls on each tick (default: M script)
- `M.SYNC <0|1>` - Set sync mode (0=internal, 1=MIDI clock)
- `MIDI.IN` - List available MIDI input devices
- `MIDI.IN <name>` - Connect to MIDI device for clock sync

#### Delayed Execution
- `DEL <ms>: <cmd>` - Execute command after delay (max 16000ms)
- `DEL.CLR` - Clear all pending delayed commands
- `DEL.X <count> <ms>: <cmd>` - Queue command N times at intervals
  - Example: `DEL.X 4 100: TR` fires at 100ms, 200ms, 300ms, 400ms
- `DEL.R <count> <ms>: <cmd>` - Execute immediately, then repeat
  - Example: `DEL.R 4 100: TR` fires now, then at 100ms, 200ms, 300ms
- Supports expressions in delay time and counts
- Commands execute on metro thread at scheduled time

#### Scripts
- `SCRIPT <1-8>` - Execute stored script (can be called from other scripts, supports expressions)
  - `SCRIPT 1` - Direct script number
  - `SCRIPT A` - Variable reference
  - `SCRIPT ADD 1 1` - Math expression
  - `SCRIPT PN.NEXT 0` - Pattern operation
- Scripts 1-8: User scripts
- M script (index 8): Called on each metro tick
- I script (index 9): Called on startup

#### Scenes
- `SAVE <name>` - Save current scene (scripts + patterns) to ~/.config/monokit/scenes/
- `LOAD <name>` - Load scene from file, resets variables
- `SCENES` - List available saved scenes
- `DELETE <name>` - Delete a saved scene

#### Variables
- `A`, `B`, `C`, `D` - General accumulators (get/set: `A` or `A 100`)
- `X`, `Y`, `Z`, `T` - General accumulators
- `J`, `K` - Per-script local variables (each script has its own J and K)
- `I` - Loop counter (scoped to L loops, read-only)
- Variables accept expressions for their value: `A ADD 1 1`, `J RND 100`, `X PN.NEXT 0`, `B MUL A 2`
- Variables can be used in expressions: `PF A`, `DC X`

#### Counters (N1-N4)
- `N1`, `N2`, `N3`, `N4` - Read current value and auto-increment (works in expressions)
- `N1.MIN <n>` - Set minimum value (default 0, accepts expressions)
- `N1.MAX <n>` - Set maximum value (wraps to MIN when exceeded, 0=disabled/no wrap, accepts expressions)
- `N1.RST` - Reset counter to MIN value
- Same operations available for N2, N3, N4
- Example usage:
  ```
  N1.MIN 10; N1.MAX 14    # Counter cycles 10,11,12,13,14,10...
  PF N N1                 # Use counter for pitch (semitones)
  N1.RST                  # Reset to 10
  ```

#### Patterns (Working Pattern - P)
**State & Query:**
- `P.N` / `P.N <0-5>` - Get/set working pattern
- `P.L` / `P.L <1-64>` - Get/set pattern length
- `P.I` / `P.I <0-63>` - Get/set playhead index
- `P <idx>` / `P <idx> <val>` - Get/set value at index
- `P.HERE` - Get value at playhead
- `P.NEXT` - Advance playhead, return value
- `P.PREV` - Reverse playhead, return value

**Manipulation:**
- `P.PUSH <val>` - Push value to end, shift all values left
- `P.POP` - Return value at end
- `P.INS <idx> <val>` - Insert value at index, shift right
- `P.RM <idx>` - Remove value at index, shift left
- `P.REV` - Reverse pattern order
- `P.ROT <n>` - Rotate pattern by n positions
- `P.SHUF` - Shuffle pattern randomly
- `P.SORT` - Sort pattern ascending

**Math Operations:**
- `P.ADD <val>` - Add value to all steps
- `P.SUB <val>` - Subtract value from all steps
- `P.MUL <val>` - Multiply all steps by value
- `P.DIV <val>` - Divide all steps by value
- `P.MOD <val>` - Modulo all steps by value
- `P.SCALE <min> <max>` - Scale pattern to new range
- `P.RND [min] [max]` - Randomize all steps (default: 0-127)

**Query Operations:**
- `P.MIN` - Return minimum value in pattern
- `P.MAX` - Return maximum value in pattern
- `P.SUM` - Return sum of all values
- `P.AVG` - Return average of all values
- `P.FND <val>` - Find value, return index (-1 if not found)

#### Patterns (Explicit Pattern - PN)
**State & Query:**
- `PN <pat> <idx>` / `PN <pat> <idx> <val>` - Get/set value (pat: 0-5)
- `PN.L <pat>` / `PN.L <pat> <len>` - Get/set length
- `PN.I <pat>` / `PN.I <pat> <idx>` - Get/set playhead
- `PN.HERE <pat>` - Get value at playhead
- `PN.NEXT <pat>` - Advance playhead, return value
- `PN.PREV <pat>` - Reverse playhead, return value

**Manipulation:**
- `PN.PUSH <pat> <val>` - Push value to end, shift all values left
- `PN.POP <pat>` - Return value at end
- `PN.INS <pat> <idx> <val>` - Insert value at index, shift right
- `PN.RM <pat> <idx>` - Remove value at index, shift left
- `PN.REV <pat>` - Reverse pattern order
- `PN.ROT <pat> <n>` - Rotate pattern by n positions
- `PN.SHUF <pat>` - Shuffle pattern randomly
- `PN.SORT <pat>` - Sort pattern ascending

**Math Operations:**
- `PN.ADD <pat> <val>` - Add value to all steps
- `PN.SUB <pat> <val>` - Subtract value from all steps
- `PN.MUL <pat> <val>` - Multiply all steps by value
- `PN.DIV <pat> <val>` - Divide all steps by value
- `PN.MOD <pat> <val>` - Modulo all steps by value
- `PN.SCALE <pat> <min> <max>` - Scale pattern to new range
- `PN.RND <pat> [min] [max]` - Randomize all steps (default: 0-127)

**Query Operations:**
- `PN.MIN <pat>` - Return minimum value in pattern
- `PN.MAX <pat>` - Return maximum value in pattern
- `PN.SUM <pat>` - Return sum of all values
- `PN.AVG <pat>` - Return average of all values
- `PN.FND <pat> <val>` - Find value, return index (-1 if not found)

Note: All PN and P operations accept variables/expressions as arguments (e.g., `DC PN.NEXT 0`, `P I`, `PN A B`)

#### Expression Support
All numeric arguments accept nested expressions, including:
- Math operations: `PF ADD A 100`, `DC MUL X 2`
- Pattern operations: `PF PN.NEXT 0`, `DC SUB PN.HERE 0 PN.HERE 1`
- Random operations: `PF RND 1000`, `A RRND 0 127`
- Sequence operations: `PF N SEQ "C3 E3 G3"`, `IF SEQ "x _": TR`
- Variables: `PF A`, `DC X`, `MF J`
- Nested combinations: `PF ADD PN.NEXT 0 RND 100`

#### Inline Sequences (SEQ) - Phase 2 Complete
- `SEQ "<pattern>"` - Cycle through values on each evaluation
  - **Phase 1 Tokens:**
    - `x` = trigger (returns 1)
    - `_` or `.` = rest (returns 0)
    - Numbers: `100`, `-12`, `0`
    - Note names: `C3`, `E3`, `F#4`, `Bb2` (returns semitones relative to C3)
  - **Phase 2 Features:**
    - `*n` = repeat token n times (e.g., `C3*4` expands to `C3 C3 C3 C3`)
    - `?` = random trigger (50% chance of 1, 50% chance of 0)
    - `<a b>` = toggle/cycle (deterministic, like TOG operator - returns A, then B, then A, then B...)
    - `{a b}` = random choice (unpredictable, like EITH - randomly picks A or B each time)
    - Combinable modifiers (e.g., `<C3 E3>*2` toggles twice, `{C3 E3}*2` picks randomly twice)

**Distinction Between Toggle and Random:**
- `<a b>` = **Toggle/Cycle** - deterministic state machine (like TOG)
  - `SEQ "<C3 E3>"` → C3, E3, C3, E3, C3, E3...
  - State persists across calls (remembers last value)

- `{a b}` = **Random Choice** - unpredictable selection (like EITH)
  - `SEQ "{C3 E3}"` → randomly C3 or E3, then randomly again
  - No state (picks fresh each time)

- State persists per-script and per-pattern (independent counters)
- Use with N operator for Hz: `PF N SEQ "C3 E3 G3"`
- Use with Q for quantization: `PF N Q SEQ "0 3 5 7"`

Examples:
```
IF SEQ "x _ x _": TR           # Trigger on beats 1 and 3
PF N SEQ "C3 E3 G3 C4"         # Arpeggiate C major
A SEQ "0 1 2 3"                # Store in variable
SEQ "C3*4 E3*2"                # Repeated notes (C3 C3 C3 C3 E3 E3)
SEQ "<C3 E3> G3"               # Toggle C3/E3, then G3
SEQ "{C3 E3} G3"               # Random C3 or E3, then G3
SEQ "x ? x ?"                  # Random triggers (50% chance each)
SEQ "<C3 E3>*2"                # Two toggles
SEQ "{C3 E3}*2"                # Two random choices
SEQ "<C3 E3> {G3 B3}"          # First toggles, second random
```

#### Preset System (PSET)
- `PSET <script> <name>` - Load preset into script 1-8
- `PSET.SAVE <script> <name>` - Save script as user preset
- `PSET.DEL <name>` - Delete user preset
- `PSETS` - List all presets ([F] factory, [U] user)

Factory Presets (22 total):
- **Drums (10):** 808-kick, punch-kick, sub-kick, basic-snare, snap-snare, hat-closed, hat-open, fm-hat, clap, rim
- **Bass (3):** sub-bass, saw-bass, fm-bass
- **Lead (3):** saw-lead, fm-lead, pluck-lead
- **Percussion (3):** metal-hit, conga, tom
- **FX (3):** noise, zap, rise

User presets stored in `~/.config/monokit/presets/`

Examples:
```
PSET 1 808-kick              # Load kick preset into script 1
PSET.SAVE 2 my-bass          # Save script 2 as user preset
PSET.DEL old-sound           # Delete user preset
PSETS                        # List all available presets
```

#### Control Flow (PRE separator)
- `IF <expr>: <cmd>` - Execute cmd if expr != 0 (truthy). Example: `IF PN.HERE 0: TR`
- `IF <cond>: <cmd>` - With comparison: `IF A > 5: TR`, `IF GT A 5: TR`
- `ELIF <cond>: <cmd>` - Else-if, executes if previous IF/ELIF was false and condition is true
- `ELSE: <cmd>` - Else branch, executes if all previous IF/ELIF were false
- `PROB <0-100>: <cmd>` - Execute cmd with probability
- `EV <n>: <cmd>` - Execute cmd every Nth tick (applies to whole line including semicolons)
- `SKIP <n>: <cmd>` - Skip every Nth tick (inverse of EV, executes on all other ticks)
- `L <start> <end>: <commands>` - Loop from start to end (inclusive), supports forward/backward iteration, I is loop counter
- `BRK` - Stop current script execution immediately (works in conditionals/loops, doesn't affect parent scripts)
- Sub-commands: `cmd1; cmd2; cmd3` - Multiple commands on one line

#### Comparison Operators (return 1 for true, 0 for false)
- `EZ <x>` - Equals zero (x == 0)
- `NZ <x>` - Not zero (x != 0)
- `EQ <a> <b>` - Equals (a == b)
- `NE <a> <b>` - Not equals (a != b)
- `GT <a> <b>` - Greater than (a > b)
- `LT <a> <b>` - Less than (a < b)
- `GTE <a> <b>` - Greater than or equal (a >= b)
- `LTE <a> <b>` - Less than or equal (a <= b)
- Infix comparisons also supported in conditions: `>`, `<`, `>=`, `<=`, `==`, `!=`

Examples:
- `IF PN.HERE 0: TR` - Trigger if pattern value is non-zero
- `IF EZ A: TR` - Trigger if A equals zero
- `IF GT A 5: TR` - Trigger if A > 5
- `IF A > 5: TR` - Same as above (infix syntax)

#### Synth Parameters (77 total)

**Primary Oscillator**
- `PF <hz>` - Primary frequency (20-20000)
- `PW <0-2>` - Primary waveform (0=sine, 1=triangle, 2=sawtooth)

**Modulator Oscillator**
- `MF <hz>` - Modulator frequency (20-20000)
- `MW <0-3>` - Modulator waveform (0=sine, 1=triangle, 2=sawtooth, 3=feedback)

**Feedback FM (Noise Generation)**
- `FB <0-16383>` - Feedback amount (0=clean, >8191=chaotic/noise)
- `FBA <0-16383>` - Feedback envelope amount
- `FBD <ms>` - Feedback envelope decay time (1-10000 ms)

**Discontinuity (Waveshaping)**
- `DC <0-16383>` - Discontinuity amount (mix of modulator into shaper)
- `DM <0-6>` - Discontinuity mode (0=fold, 1=tanh, 2=softclip, 3=hard, 4=asym, 5=rectify, 6=crush)
- `DD <ms>` - Discontinuity envelope decay time (milliseconds, 1-10000)

**Lo-Fi Processor**
- `LB <1-16>` - Bit depth (1=crushed, 16=clean)
- `LS <100-48000>` - Sample rate reduction (Hz)
- `LM <0-16383>` - Lo-fi mix (dry/wet)

**Tracking & Modulation Bus**
- `TK <0-16383>` - Tracking amount (modulator frequency follows pitch envelope)
- `MB <0-16383>` - Modulation bus amount (general modulation depth)
- `MP <0|1>` - Enable modulation -> primary frequency (FM-independent)
- `MD <0|1>` - Enable modulation -> discontinuity amount
- `MT <0|1>` - Enable modulation -> tracking
- `MA <0|1>` - Enable modulation -> amplitude

**FM Synthesis**
- `FM <0-16383>` - FM index (modulator phase modulates primary frequency, additive with mod bus routing)

**Mix Controls (Additive Routing)**
- `MX <0-16383>` - Mix amount (modulator output to discontinuity input)
- `MM <0-16383>` - Mix modulation amount (depth of mod bus modulation on mix)
- `ME <0|1>` - Mix modulation enable (route mod bus to mix amount)

**Envelope Controls**
All envelopes use consistent *ENV prefix naming:
- `*.DEC` - Decay time (1-10000 ms) - short forms: AD, PD, FD, DD, FBD, FED
- `*.ATK` - Attack time (1-10000 ms)
- `*.CRV` - Curve shape (-8.0 to 8.0, -8=log, 0=linear, 8=exp)
- `*.AMT` - Envelope amount (where applicable) - short forms: PA, FA, DA, FBA, FE

Available envelope prefixes:
- `AENV` - Amplitude envelope (AENV.DEC→AD, AENV.ATK, AENV.CRV)
- `PENV` - Pitch envelope (PENV.DEC→PD, PENV.AMT→PA, PENV.ATK, PENV.CRV)
- `FMEV` - FM envelope (FMEV.DEC→FD, FMEV.AMT→FA, FMEV.ATK, FMEV.CRV)
- `DENV` - Discontinuity envelope (DENV.DEC→DD, DENV.AMT→DA, DENV.ATK, DENV.CRV)
- `FBEV` - Feedback envelope (FBEV.DEC→FBD, FBEV.AMT, FBEV.ATK, FBEV.CRV)
- `FLEV` - Filter envelope (FLEV.DEC→FED, FLEV.AMT→FE, FLEV.ATK, FLEV.CRV)

Examples:
- `AENV.ATK 50` - Set amp envelope attack to 50ms
- `PENV.CRV -4` - Set pitch envelope to logarithmic curve
- `AD 500` or `AENV.DEC 500` - Set amplitude decay to 500ms
- `PA 4` or `PENV.AMT 4` - Set pitch envelope to 4 octaves
- `FE 8000` or `FLEV.AMT 8000` - Set filter envelope amount

**Decay Time Short Forms**
- `AD <ms>` - Amplitude decay (AENV.DEC)
- `PD <ms>` - Pitch decay (PENV.DEC)
- `FD <ms>` - FM decay (FMEV.DEC)
- `DD <ms>` - Discontinuity decay (DENV.DEC)
- `FBD <ms>` - Feedback decay (FBEV.DEC)
- `FED <ms>` - Filter decay (FLEV.DEC)

**Envelope Amounts (Additive Model: output = base + env*amount)**
- `PA <0-16>` - Pitch envelope amount in octaves (PENV.AMT)
- `FA <0-16>` - FM envelope amount (FMEV.AMT)
- `DA <0-16>` - Discontinuity envelope amount (DENV.AMT)
- `FBA <0-16383>` - Feedback envelope amount (FBEV.AMT)
- `FE <0-16383>` - Filter envelope amount (FLEV.AMT)

**SVF Multi-Mode Filter**
- `FC <hz>` - Filter cutoff frequency (20-20000)
- `FQ <0-16383>` - Filter resonance (0-16383)
- `FT <0-3>` - Filter type (0=LP, 1=HP, 2=BP, 3=Notch)
- `FE <0-16383>` - Filter envelope amount
- `FED <ms>` - Filter envelope decay (1-10000 ms)
- `FK <0-16383>` - Filter key tracking amount
- `MF.F <0|1>` - ModBus -> Filter cutoff routing

**Ring Modulator**
- `RGF <20-2000>` - Ring mod frequency (Hz)
- `RGW <0-3>` - Ring mod waveform (0=sine, 1=triangle, 2=sawtooth, 3=square)
- `RGM <0-16383>` - Ring mod mix (0=dry, 16383=100% modulated)

**Comb Resonator (Karplus-Strong)**
- `RF <hz>` - Resonator frequency (20-5000)
- `RD <ms>` - Resonator decay time (10-5000 ms)
- `RM <0-16383>` - Resonator mix (dry/wet)
- `RK <0-16383>` - Resonator key tracking

**Compressor**
- `CT <0-16383>` - Threshold
- `CR <1-20>` - Ratio (1=off, 20=limiting)
- `CA <1-500>` - Attack (ms)
- `CL <10-2000>` - Release (ms)
- `CM <0-16383>` - Makeup gain

**Pan**
- `PAN <-16383 to +16383>` - Stereo position (-L, 0=center, +R)

**Beat Repeat**
Beat repeat activates automatically when BR.MIX > 0.
- `BR.LEN <0-7>` - Loop division (0=1/16, 1=1/8, 2=1/4, 3=1/2, 4=1, 5=2, 6=4, 7=8 beats)
- `BR.REV <0|1>` - Reverse playback
- `BR.WIN <1-50>` - Window/capture size (ms)
- `BR.MIX <0-16383>` - Dry/wet mix (activates when > 0)

**Pitch Shift**
- `PS.MODE <0|1>` - Mode (0=normal, 1=granular)
- `PS.SEMI <-24 to 24>` - Pitch shift (semitones)
- `PS.GRAIN <5-100>` - Grain size (ms)
- `PS.MIX <0-16383>` - Dry/wet mix
- `PS.TARG <0|1>` - Target (0=input, 1=output)

**Stereo Delay**
- `DT <ms>` - Delay time (1-2000 ms)
- `DF <0-16383>` - Delay feedback amount
- `DLP <hz>` - Delay lowpass filter cutoff (100-20000)
- `DW <0-16383>` - Delay wet mix (INSERT mode) or send level (SEND mode)
- `DS <0-1>` - Delay sync (0=free, 1=tempo - not implemented)

**3-Band EQ (Post-Delay)**
- `EL <-24 to +24>` - Low shelf gain (dB at 200Hz)
- `EM <-24 to +24>` - Mid peak gain (dB)
- `EF <200-8000>` - Mid center frequency (Hz)
- `EQ <0.1-10>` - Mid Q/bandwidth
- `EH <-24 to +24>` - High shelf gain (dB at 4000Hz)

**Plate Reverb**
- `RV <0-16383>` - Reverb size/decay time
- `RP <ms>` - Reverb pre-delay (0-100 ms)
- `RH <0-16383>` - Reverb high damping
- `RW <0-16383>` - Reverb wet mix (INSERT mode) or send level (SEND mode)

**Effect Routing**
- `D.MODE <0-2>` - Delay routing mode (0=BYPASS, 1=INSERT, 2=SEND)
- `D.TAIL <0-2>` - Delay tail behavior (0=CUT, 1=RING, 2=FREEZE)
- `R.MODE <0-2>` - Reverb routing mode (0=BYPASS, 1=INSERT, 2=SEND)
- `R.TAIL <0-2>` - Reverb tail behavior (0=CUT, 1=RING, 2=FREEZE)

Effect routing modes:
- **BYPASS (0)**: Effect disabled, signal passes through unchanged
- **INSERT (1)**: Traditional series processing with wet/dry mix
- **SEND (2)**: Parallel processing where wet parameter controls send level

Tail behaviors:
- **CUT (0)**: Tails cut immediately when wet=0 (gated output)
- **RING (1)**: Tails decay naturally (default behavior)
- **FREEZE (2)**: Stop new input, sustain current tail indefinitely

Note: In SEND mode with RING or FREEZE tail modes, the effect output remains at full level when wet=0, allowing tails to continue naturally. In CUT mode, output is gated by the wet parameter.

#### Math Operations
- `ADD <a> <b>` or `+ <a> <b>` - Add two values (works as command and in expressions)
- `SUB <a> <b>` or `- <a> <b>` - Subtract b from a (works as command and in expressions)
- `MUL <a> <b>` or `* <a> <b>` - Multiply two values (works as command and in expressions)
- `DIV <a> <b>` or `/ <a> <b>` - Divide a by b (works as command and in expressions)
- `MOD <a> <b>` or `% <a> <b>` - Modulo a by b (works as command and in expressions)
- `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Range mapping with clamping (works as command and in expressions)
  - Maps input value from input range to output range
  - Automatically clamps result to output range
  - Example: `MAP 50 0 100 0 1000` maps 50 from 0-100 range to 500 in 0-1000 range
  - Works with reversed ranges: `MAP 25 0 100 1000 0` maps and inverts

#### Random Number Generation
- `RND <max>` - Random integer from 0 to max inclusive (works as command and in expressions)
- `RRND <min> <max>` - Random integer from min to max inclusive (works as command and in expressions)
- `TOSS` - Random 0 or 1 (coin flip, works as command and in expressions)
- `EITH <a> <b>` - Random choice between a and b (works as command and in expressions)
- `TOG <a> <b>` - Alternates between a and b on each call (works as command and in expressions)
  - State is per-script and per-line
  - First call returns a, second returns b, third returns a, etc.
  - Example: `PF TOG N 0 N 7` alternates between C3 and G3

#### Randomization Commands
**Voice Randomization:**
- `RND.VOICE` - Randomize all oscillator/FM parameters within musical ranges
- `RND.OSC` - Randomize oscillator params only (PF, PW, MF, MW)
- `RND.FM` - Randomize FM-related params (FM, FB, FBA, FBD)

**Modulation Randomization:**
- `RND.MOD` - Randomize modulation routing (MB, TK, MP, MD, MT, MA)
- `RND.ENV` - Randomize envelope times and amounts (ATK, DEC, CRV, PA, FA, DA)

**FX Randomization:**
- `RND.FX` - Randomize all effect parameters (filter + delay + reverb)
- `RND.FILT` - Randomize filter (FC, FQ, FT, FE)
- `RND.DLY` - Randomize delay (DT, DF, DLP, DW)
- `RND.VERB` - Randomize reverb (RV, RP, RH, RW)

**Pattern Randomization:**
- `RND.P [min] [max]` - Randomize working pattern values (default: 0-127)
- `RND.PN <n> [min] [max]` - Randomize specific pattern n
- `RND.PALL [min] [max]` - Randomize all patterns

#### Note/Pitch Conversion
- `N <semitones>` - Convert semitones to frequency in Hz (12-TET, works in expressions)
  - N 0 = C3 (131 Hz) - matches Teletype reference pitch
  - N 12 = C4 (262 Hz)
  - N 21 = A4 (440 Hz)
  - N -12 = C2 (65 Hz)
  - Usage: `PF N 0` (set primary freq to C3), `PF N ADD A 7` (C3 + A semitones + perfect 5th)

#### Scale Quantization
- `Q <note>` - Quantize note (semitone value) to current scale (works in expressions)
  - Returns nearest note in scale as semitone value
  - Use with N operator to convert to Hz: `PF N Q A`
  - Works with pattern data: `PF N Q P.NEXT`
- `Q.ROOT <0-11>` - Set scale root note (C=0, C#=1, D=2, ..., B=11)
- `Q.SCALE <0-11>` - Set scale type
  - 0 = Chromatic (all notes)
  - 1 = Major
  - 2 = Minor (natural)
  - 3 = Dorian
  - 4 = Phrygian
  - 5 = Lydian
  - 6 = Mixolydian
  - 7 = Pentatonic Major
  - 8 = Pentatonic Minor
  - 9 = Blues
  - 10 = Whole Tone
  - 11 = Diminished
- `Q.BIT <binary>` - Set custom scale mask as binary string
  - Each bit represents a semitone (1=in scale, 0=not in scale)
  - Examples:
    - `Q.BIT 101010110101` - Major scale (12-TET)
    - `Q.BIT 10101` - Pentatonic (5-EDO)
    - `Q.BIT <24 bits>` - Quarter-tone systems
  - Allows arbitrary microtonal scales and EDO systems

Example usage:
```
Q.ROOT 0          // Set root to C
Q.SCALE 1         // Set to Major scale
PF N Q A          // Quantize variable A to C Major, convert to Hz
PF N Q P.NEXT     // Quantize pattern value to scale
Q.BIT 10101       // Custom 5-note scale
```

#### Recording
- `REC` - Start recording to current working directory
- `REC.STOP` - Stop recording (automatically called on quit)
- `REC.PATH <prefix>` - Set custom recording path prefix
- Files saved as WAV 24-bit stereo @ 48kHz
- Sequential file naming: monokit_audio_0.wav, monokit_audio_1.wav, etc.
- UI shows red "● REC MM:SS" indicator when recording
- Recording auto-stops on quit to prevent file corruption
- Works in both sclang and scsynth-direct modes
- scsynth-direct: uses DiskOut UGen with streaming buffer writes

#### Beat Repeat
Beat repeat activates automatically when BR.MIX > 0.

- `BR.LEN <0-7>` - Loop division/length setting
  - 0 = 1/16 beat (shortest loop)
  - 1 = 1/8 beat
  - 2 = 1/4 beat
  - 3 = 1/2 beat
  - 4 = 1 beat
  - 5 = 2 beats
  - 6 = 4 beats
  - 7 = 8 beats (longest loop)
- `BR.REV <0|1>` - Reverse playback (0=normal, 1=reversed)
- `BR.WIN <1-50>` - Window/capture size in milliseconds (1-50ms)
- `BR.MIX <0-16383>` - Dry/wet mix (0=dry, 16383=100% wet, activates when > 0)
  - Uses separate L/R buffers for proper stereo operation

Example usage:
```
BR.MIX 16383      // Enable beat repeat at 100% wet
BR.LEN 2          // Set to 1/4 beat loop
BR.WIN 10         // 10ms window
BR.REV 1          // Reverse playback
BR.MIX 8192       // Adjust mix to 50%
```

#### Pitch Shift
- `PS.MODE <0|1>` - Pitch shift mode (0=normal, 1=granular)
- `PS.SEMI <-24 to 24>` - Pitch shift amount in semitones
  - Negative values shift down
  - Positive values shift up
  - Range: -24 (2 octaves down) to +24 (2 octaves up)
- `PS.GRAIN <5-100>` - Grain size in milliseconds (5-100ms)
  - Smaller grains = more artifacts but tighter timing
  - Larger grains = smoother but more latency
- `PS.MIX <0-16383>` - Dry/wet mix (0=dry, 16383=100% wet)
- `PS.TARG <0|1>` - Processing target (0=input signal, 1=output signal)

Example usage:
```
PS.MODE 1         // Granular mode
PS.SEMI 12        // Shift up one octave
PS.GRAIN 20       // 20ms grain size
PS.MIX 8192       // 50% mix
PS.TARG 0         // Process input signal
```

#### Parameter Slew
- `SLEW.ALL <0-10000>` - Set global slew time in milliseconds
  - 0 = instant changes (default, backward compatible)
  - Higher values = smoother transitions
  - Applies to all smoothable parameters via SC-side Lag.kr
  - Example: `SLEW.ALL 200` makes all param changes glide over 200ms
- `SLEW <param> <0-10000>` - Set slew time for specific parameter
  - Per-parameter slew time overrides SLEW.ALL
  - Only affects the specified parameter
  - Example: `SLEW PF 500` sets only PF to 500ms slew time
  - Example: `SLEW.ALL 100; SLEW FC 1000` sets global 100ms, but FC gets 1000ms
- Smoothed parameters: PF, MF, FC, FM, MX, DC, FB, FQ, FK, FE, RF, RM, DT, DF, DW, RV, RW, volume, pan, LB, LS, LM, RGF, RGM, CT, CM, EL, EM, EH, EF
- NOT smoothed (discrete): PW, MW, FT, DM, mode switches, triggers

#### Output
- `PRINT "<text>"` or `PRINT '<text>'` - Print literal string to REPL output
- `PRINT <expr>` - Evaluate expression and print result to REPL
  - Examples: `PRINT A`, `PRINT ADD 1 2`, `PRINT PN.NEXT 0`
  - Works with variables, math operations, pattern operations, etc.

#### Oscilloscope
- `SCOPE.TIME <5-500>` - Set waveform timespan in milliseconds (default 30, accepts expressions)
- `SCOPE.CLR <0-3>` - Set waveform color (accepts expressions):
  - 0 = success (green)
  - 1 = error (red)
  - 2 = foreground
  - 3 = accent
- `SCOPE.MODE <0-4>` - Set display mode (accepts expressions):
  - 0 = BRAILLE (2×4 dots, highest resolution)
  - 1 = BLOCK (vertical bars ▁▂▃▄▅▆▇█)
  - 2 = LINE (line drawing ─╱╲)
  - 3 = DOT (scatter plot ●)
  - 4 = QUADRANT (2×2 blocks ▖▗▘▙▚▛▜▝▞▟)
- `SCOPE.UNI <0|1>` - Unipolar mode (0=bipolar ±1, 1=unipolar rectified, accepts expressions)

#### Notes
- `NOTE "text"` - Append quoted text to Notes page (error if all 8 lines full)
- `NOTE.CLR` - Clear all notes

#### System
- `RST` - Reset all parameters to defaults:
  - PF: 131 Hz (C3), MF: 262 Hz (C4)
  - PA: 0 (pitch env off), FC: 10000 Hz
  - D.MODE: 2 (SEND), R.MODE: 2 (SEND)
  - CR: 1 (compressor off)
- `LOAD.RST` - Show current reset mode setting
- `LOAD.RST 0` - Persist synth params on scene load (default)
- `LOAD.RST 1` - Reset all params before loading scene
- `CLEAR` - Clear REPL output history
- `DEBUG <level>` - Set debug verbosity level:
  - `DEBUG 0` - Silent mode (no REPL output except errors and PRINT commands)
  - `DEBUG 1` - Important messages (metro status, PRINT commands) - minimal verbosity
  - `DEBUG 2` - Verbose mode (all parameter changes) - default level
- `CPU` - Show current CPU display state (0 or 1)
- `CPU 0` - Hide CPU meter in header border
- `CPU 1` - Show CPU meter in header border
- `HEADER <0-4>` - Set header verbosity level (persists to config):
  - `HEADER 0` - Dynamic nav label only (just current page, e.g., [LIVE])
  - `HEADER 1` - Dynamic nav label + L/R meters
  - `HEADER 2` - Dynamic nav label + TR indicator + meters
  - `HEADER 3` - Full nav (all page labels) + TR + meters (no CPU)
  - `HEADER 4` - Full nav + TR + meters + CPU (default)
  - REC indicator always visible at all levels (safety-critical)
  - CPU 1 command works as override at any level
  - UI elements maintain consistent positioning across all levels
- `q`, `quit`, or `exit` - Quit application (typed in REPL)

#### Themes
- `THEME` - Show current theme and list all available themes
- `THEME <name>` - Switch to theme by name (case-insensitive)
- Built-in themes: `dark`, `light`, `system`
- 47 bundled themes compiled at build time from themes/themes.toml
- Custom themes defined in `~/.config/monokit/config.toml` under `[themes.name]` sections
- Example themes: dracula, solarized, coral, copper, neo_peachio_dark, nougat_light, and many more

### Navigation (Keybindings)

#### Page Cycling
- `[` / `]` - Cycle through pages (Live → 1-8 → M → I → P → S → V → wrap)

#### Direct Page Access (Function Keys)
- `F1` through `F8` - Script pages 1-8
- `F9` - Live page
- `F10` - Metro page
- `F11` - Init page
- `F12` - Pattern page
- `ESC` - Toggle Help (overlay, scrollable with arrow keys)

#### Alternative: Alt+key (requires iTerm2 configuration)
- `Alt+L` - Live page
- `Alt+1` through `Alt+8` - Script pages 1-8
- `Alt+M` - Metro page
- `Alt+I` - Init page
- `Alt+P` - Pattern page
- `Alt+S` - Scope page
- `Alt+V` - Variables page
- `Alt+H` - Toggle Help

**iTerm2 Note:** Alt+key combinations require setting "Left Option key = Esc+" in iTerm2 Preferences > Profiles > Keys > General. Function keys (F1-F12) work in all terminals without configuration.

#### Input
- `Enter` - Execute command
- `Up/Down` - Command history (on non-Help pages) / REPL scroll (Live page with Ctrl modifier)
- `Ctrl+Up/Down` - Scroll REPL output (Live page only)
  - Scrolls through command output history
  - Shows [↑N] indicator in title when scrolled
  - Auto-resets to bottom on new output
- `Left/Right` - Cursor movement
- `Ctrl+Left/Right` - Word-by-word cursor movement
- `Ctrl+D` - Duplicate line (script pages)
- `Ctrl+K` - Delete entire line (script pages)
- `Ctrl+C` - Copy line (script pages)
- `Ctrl+X` - Cut line (script pages)
- `Ctrl+V` - Paste line (script pages)
- Script pages show validation errors that auto-clear after 3 seconds or on successful save

#### Search
- `Ctrl+F` - Enter search mode (isolated scope based on current page context)
  - Search bar shows `/` prefix to indicate search mode active
  - Help search scope: searches help pages only
  - Script search scope: searches scripts 1-8, M, I (no patterns)
- `Enter` - Jump to next match
- `Shift+Enter` - Jump to previous match
- `ESC` - Exit search mode (user position preserved)
- Match highlighting:
  - Current match: highlighted with highlight_bg/highlight_fg colors
  - Other matches: highlighted with accent color
- Match count indicator: displays as `[N/M]` (current match / total matches)
- Navigation hotkeys (Alt+key, F1-F12) exit search and navigate to new page
- Search queries are case-insensitive

### UI Style (Teletype)
- **Uppercase text:** All UI text (commands, labels, output) displays in uppercase for a Teletype-style aesthetic
- **User input conversion:** User input is automatically converted to uppercase on entry
- **Script page highlighting:** Selected line shows white background with black text for brightness-based distinction
- **Line numbers:** Removed from script pages for a cleaner display

### Theme System
The UI uses a comprehensive theme system with RGB colors for terminal compatibility.

**Named Theme Support:** Config file supports multiple named themes:
```toml
[display]
theme = "dracula"    # Active theme name

[themes.dracula]
background = "#282a36"
foreground = "#f8f8f2"
# ... other colors

[themes.coral]
background = "#2d3748"
foreground = "#fa8072"
# ... other colors
```

**Theme Colors:**
- `background` - Main background color
- `foreground` - Primary text color
- `secondary` - Secondary/dimmed text (footer hints, etc.)
- `highlight_bg` / `highlight_fg` - Selection highlighting (cursor, selected line)
- `border` - Border elements
- `error` - Error messages
- `accent` - Selected items and active indicators
- `success` - Positive states (metro active, etc.)
- `label` - Section headers and labels

**Rendering:** Uses buffer-based background rendering for proper theme support across different terminal emulators. Themes use RGB color values (Color::Rgb) for consistent cross-platform display.

**Config Location:** `~/.config/monokit/config.toml`
**Bundled Themes:** 47 themes compiled into binary (themes/themes.toml), plus 3 built-in (dark, light, system) = 50 total available
**Example Config:** See `config.toml.example` in repo for additional theme examples

## OSC Protocol

All communication from Rust CLI to SuperCollider server uses UDP over localhost (127.0.0.1:57120).

**Message Format**
- **Trigger:** `/monokit/trigger` (no arguments)
- **Master Volume:** `/monokit/volume` with float value (0.0-1.0)
- **Parameter Control:** `/monokit/param <name> <value>` where:
  - `<name>` = parameter name (string):
    - Oscillator/FM: pf, pw, mf, mw, fb, fba, fbd, dc, dm, dd, tk, mb, mp, md, mt, ma, fm, mx, mm, me
    - Envelopes: ad, pd, fd, dd, pa, fa, da
    - Lo-Fi: lb, ls, lm
    - Filter: fc, fq, ft, fe, fed, fk, mf_f
    - Ring Mod: rgf, rgw, rgm
    - Resonator: rf, rd, rm, rk
    - Compressor: ct, cr, ca, cl, cm
    - Pan: pan
    - Beat Repeat: br_act, br_len, br_rev, br_win, br_mix
    - Pitch Shift: ps_mode, ps_semi, ps_grain, ps_mix, ps_targ
    - Delay: dt, df, dlp, dw, ds, dmode, dtail
    - EQ: el, em, ef, eq, eh
    - Reverb: rv, rp, rh, rw, rmode, rtail
  - `<value>` = float or int depending on parameter type
- **Reset:** `/monokit/reset` (no arguments, resets all parameters to defaults)
- **Recording:**
  - `/monokit/rec` - Start recording (with optional directory path)
  - `/monokit/rec/stop` - Stop recording
  - `/monokit/rec/path` - Set custom recording path prefix

All parameter updates are validated in Rust CLI before sending and applied immediately on SuperCollider voice.

## Dependencies

### Rust
- rosc 0.10 - OSC protocol
- ratatui 0.29 - Terminal UI framework
- crossterm 0.28 - Terminal backend
- rand 0.8 - Random number generation (for PROB, RND, RRND)
- anyhow 1 - Error handling
- thiserror 1 - Error types

### SuperCollider
- SuperCollider 3.x with scsynth

## Project Metadata

### License
- GPL-2.0 (GNU General Public License v2.0)

### Release Process
- Release script: `scripts/release.sh`
- Automates version tagging and build process
- Creates GitHub releases with compiled binaries

### Distribution
- Homebrew tap: `brew tap stolmine/monokit` (pre-built bundles)
- Formula: `homebrew-monokit/Formula/monokit.rb`
- Install location: `/opt/homebrew/libexec/monokit/` (symlink from `/opt/homebrew/bin/monokit`)
- Bundle structure: monokit binary + Resources/ (scsynth, plugins, synthdefs) + Frameworks/ (dylibs)
- Automated release updates via `.github/workflows/release.yml`
- Binary installation paths follow XDG Base Directory specification
- User config: `~/.config/monokit/`
