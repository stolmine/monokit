# Monokit Development Progress

## November 2025

### Timing Infrastructure Improvements - COMPLETE
**Date:** November 2025
**Status:** Complete

**Implemented:**
- Platform-calibrated hybrid sleep timing using `spin_sleep` crate
- Real-time thread priority elevation via `audio_thread_priority` crate
- Timestamped OSC bundles with NTP timestamps for sample-accurate scheduling
- Created `src/osc_utils.rs` utility module for OSC bundle generation
- SpinSleeper integration in metro thread for precise timing
- RT priority elevation on macOS, Linux, and Windows (best-effort)

**Technical Details:**
- SpinSleeper uses calibrated spin thresholds based on platform sleep characteristics
- RT priority reduces OS scheduling latency (requires privileges on some platforms)
- All OSC messages bundled with timestamps relative to immediate time
- SuperCollider receives timestamped bundles for sample-accurate scheduling
- Improved metro stability and timing accuracy

**Dependencies Added:**
- `spin_sleep = "1.2"` - Hybrid sleep with platform calibration
- `audio_thread_priority = "0.32"` - Cross-platform RT priority elevation

**Files Modified:**
- `Cargo.toml` - Added timing dependencies
- `src/osc_utils.rs` - New utility module
- `src/metro.rs` - SpinSleeper and RT priority integration
- `src/main.rs` - Module declaration

---

### MIDI Clock Sync - COMPLETE (Input Only)
**Date:** November 2025
**Status:** Input Complete

**Implemented:**
- `M.SYNC <0|1>` - Set sync mode (0=internal, 1=MIDI clock)
- `MIDI.IN` - List available MIDI input devices
- `MIDI.IN <name>` - Connect to MIDI device for clock sync
- Auto-detection of available MIDI inputs
- External tempo sync from DAW or MIDI clock devices (24 PPQN standard)
- Transport start/stop follows MIDI messages
- 16th note resolution (each MIDI clock tick = metro tick)

**Usage:**
```
MIDI.IN                      # List available MIDI devices
MIDI.IN "My MIDI Device"     # Connect to specific device
M.SYNC 1                     # Enable MIDI clock sync
M.ACT 1                      # Activate metro (will follow external clock)
```

When M.SYNC is enabled, the metro follows external MIDI clock timing. Transport start/stop messages automatically control metro activation.

**Not Implemented:**
- MIDI clock output
- Clock division/multiplication
- Explicit transport control commands

---

### SEQ Mini Notation System - COMPLETE (Phase 2)
**Date:** November 2025
**Status:** Phase 2 Complete

**Implemented (Phase 1):**
- `SEQ "pattern"` - Inline sequence notation operator
- Token support:
  - `x` = trigger (returns 1)
  - `_` or `.` = rest (returns 0)
  - Numbers: `100`, `-12`, `0`
  - Note names: `C3`, `E3`, `F#4`, `Bb2` (returns semitones relative to C3)
  - Accidentals: sharps (#) and flats (b)
- Per-script, per-pattern independent state tracking
- Works in all expression contexts
- Integration with N operator for Hz conversion
- Integration with Q operator for scale quantization

**Implemented (Phase 2):**
- `*n` - Repeat modifier (e.g., `C3*4` expands to `C3 C3 C3 C3`)
- `?` - Random trigger token (50% chance of 1, 50% chance of 0)
- `<a b>` - Toggle/Cycle (deterministic, like TOG - cycles A, B, A, B...)
- `{a b}` - Random Choice (unpredictable, like EITH - randomly picks A or B each time)
- Combinable modifiers (e.g., `<C3 E3>*2` toggles twice, `{C3 E3}*2` picks randomly twice)

**Usage Examples:**
```
IF SEQ "x _ x _": TR           # Trigger on beats 1 and 3
PF N SEQ "C3 E3 G3 C4"         # Arpeggiate C major
A SEQ "0 1 2 3"                # Store in variable
SEQ "C3*4 E3*2"                # Repeated notes (C3 C3 C3 C3 E3 E3)
SEQ "<C3 E3> G3"               # Random C3 or E3, then G3
SEQ "x ? x ?"                  # Random triggers (50% chance)
SEQ "<C3 E3>*2"                # Two random choices
```

**Future (Phase 3):**
- Subdivision brackets `[a b]` for metric subdivisions
- Euclidean rhythm syntax
- Probability modifiers `@p` for per-step probabilities

---

### PSET Preset System - COMPLETE
**Date:** November 2025
**Status:** Complete

**Implemented:**
- `PSET <script> <name>` - Load preset into script slot
- `PSET.SAVE <script> <name>` - Save script as user preset
- `PSET.DEL <name>` - Delete user preset
- `PSETS` - List all available presets with [F] factory and [U] user markers
- 22 factory presets organized by category
- User preset storage in `~/.monokit/presets/`

**Factory Presets (22 total):**
- **Drums (10):** 808-kick, punch-kick, sub-kick, basic-snare, snap-snare, hat-closed, hat-open, fm-hat, clap, rim
- **Bass (3):** sub-bass, saw-bass, fm-bass
- **Lead (3):** saw-lead, fm-lead, pluck-lead
- **Percussion (3):** metal-hit, conga, tom
- **FX (3):** noise, zap, rise

**Usage:**
```
PSET 1 808-kick              # Load kick into script 1
PSET.SAVE 2 my-bass          # Save script 2 as preset
PSET.DEL old-sound           # Delete user preset
PSETS                        # List all presets
```

---

### DRY Refactoring Program - COMPLETE
**Date:** November 2025
**Status:** All Phases Complete

**Total Line Reduction: ~5,942 lines (28% of original codebase)**

**Phase 0: Codebase Reorganization** - COMPLETE
- Created `core/`, `system/`, `synth/` directory structure
- Moved command handlers to logical domains
- Split effects into modular files

**Phase 1: Envelope Handler DRY** - COMPLETE
- Created macro system for envelope parameters
- Reduced envelope code from ~1,141 lines → 223 lines
- **Line reduction: 918 lines (81% decrease)**

**Phase 2: Pattern Operation DRY** - COMPLETE
- Created unified pattern operation system
- Reduced wrapper code from 2,023 → 450 lines
- **Line reduction: 1,573 lines (78% decrease)**

**Phase 3: Synth Parameter DRY** - COMPLETE
- Consolidated 70+ parameter handlers
- **Line reduction: ~2,325 lines**

**Phase 4: Variables, Counters, Test Fixtures** - COMPLETE
- Phase 4A: Variable/Counter macros - 489 lines
- Phase 4B: Expression helpers - Infrastructure
- Phase 4C: Test fixture optimization - 637 lines
- **Total Phase 4: ~1,126 lines**

**Results:**
- All 411 tests pass
- Easier to add new commands
- Significantly reduced maintenance burden
- Clear, logical file organization

---

## Previous Features

### Envelope System Simplification - COMPLETE
**Date:** November 2025
- Removed gate-based envelope triggering
- Simplified to single percussive envelope type per parameter
- Each envelope has: decay time, attack time, curve, and amount
- Fixed pitch envelope parameter routing

### Scale Quantization System - COMPLETE
- `Q <note>` - Quantize to current scale
- `Q.ROOT <0-11>` - Set scale root
- `Q.SCALE <0-11>` - Set scale type (12 presets)
- `Q.BIT <binary>` - Custom scale mask for microtonal systems

### Counter System - COMPLETE
- `N1`, `N2`, `N3`, `N4` - Auto-increment counters
- `N1.MIN <n>`, `N1.MAX <n>`, `N1.RST` - Counter control

### Delayed Execution - COMPLETE
- `DEL <ms>: <cmd>` - Execute after delay
- `DEL.X <count> <ms>: <cmd>` - Queue N times
- `DEL.R <count> <ms>: <cmd>` - Execute then repeat
- `DEL.CLR` - Clear pending commands

### Parameter Slewing - COMPLETE
- `SLEW.ALL <ms>` - Global slew time
- `SLEW <param> <ms>` - Per-parameter slew

### Buffer Effects - COMPLETE
- Beat Repeat with buffer freeze
- Pitch Shift with normal and granular modes
- Effect routing: BYPASS/INSERT/SEND modes
- Tail behaviors: CUT/RING/FREEZE

---

## Active Development

No features currently in active development.

---

## Upcoming (Roadmap)

See ROADMAP.md for planned features organized by phase:
- Phase 4: Modulation System (LFO, Aux Envelopes)
- Phase 5: UI/Feedback (Visual enhancements, activity indicators)
- Phase 6: Advanced DSP (Additional voice types, sample playback)
- Phase 7: Distribution (Unified installer, packaging)
