# Monokit Development Phases

Detailed completion records for development phases 1-6.

---

## Phase 1: Core Utilities [COMPLETE]

### Slewing & Interpolation [Medium] - COMPLETE
- [x] `SLEW.ALL <ms>` - Global slew time for all parameters
- [x] `SLEW <param> <ms>` - Per-parameter slew override
- [x] SC-side Lag.kr smoothing for 30+ parameters

### Envelope Shaping [Medium] - COMPLETE
- [x] Per-envelope attack time (*.ATK) for all 6 envelope types
- [x] Per-envelope curve control (*.CRV, -8 to 8, log/linear/exp)
- [x] Simple percussive envelopes (Env.perc) with controllable attack and curve
- [x] Exponential pitch envelope scaling for proper octave behavior

### MAP Operator [Low] - COMPLETE
- [x] `MAP <val> <in_min> <in_max> <out_min> <out_max>` - Range mapping with clamping

### TOG Generator [Low] - COMPLETE
- [x] `TOG <a> <b>` - Toggle between two values on each trigger
- [x] State is per-script and per-line

### Global State Sync Command [Medium] - COMPLETE
- [x] `SYNC` - Reset all stateful elements to starting position
- [x] `SYNC.SEQ` - Partial reset for SEQ sequences
- [x] `SYNC.TOG` - Partial reset for TOG toggles
- [x] `SYNC.PAT` - Partial reset for pattern indices

### Auto-Increment Counters [Low] - COMPLETE
- [x] `N1`, `N2`, `N3`, `N4` - Auto-increment on each read
- [x] `N1.MIN <n>`, `N1.MAX <n>`, `N1.RST`

### Pattern Storage [Low] - COMPLETE
- [x] Increased from 4 to 6 pattern slots (PN accepts 0-5)

---

## Phase 2: Pattern Expansion [COMPLETE]

### Pattern Manipulation [Medium] - COMPLETE
- [x] P.PUSH, P.POP, P.INS, P.RM, P.REV, P.ROT, P.SHUF, P.SORT, P.RND
- [x] PN.* variants for explicit pattern selection

### Pattern Math [Low] - COMPLETE
- [x] P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE
- [x] PN.* variants for explicit pattern selection

### Pattern Queries [Low] - COMPLETE
- [x] P.MIN, P.MAX, P.SUM, P.AVG, P.FND
- [x] PN.* variants for explicit pattern selection

### Randomization System [Medium] - COMPLETE
- [x] RND.VOICE, RND.OSC, RND.FM
- [x] RND.MOD, RND.ENV
- [x] RND.FX, RND.FILT, RND.DLY, RND.VERB
- [x] RND.P, RND.PN, RND.PALL

---

## Phase 3: Musical Features [PARTIAL]

### Scale Quantization [Medium] - COMPLETE
- [x] `Q <note>` - Quantize note to current scale
- [x] `Q.SCALE <0-11>` - Set scale type (12 presets)
- [x] `Q.ROOT <0-11>` - Set root note
- [x] `Q.BIT <binary>` - Custom scale mask

### Mini Notation / Inline Sequencing [High] - PHASE 2 COMPLETE
- [x] `SEQ "x _ x _"` - Trigger pattern notation
- [x] `SEQ "200 400 300 _"` - Numeric value sequences
- [x] `SEQ "C3 E3 G3 C4"` - Note names
- [x] `*n` - Repeat token n times
- [x] `?` - Random trigger (50% chance)
- [x] `<a b>` - Toggle/Cycle (deterministic)
- [x] `{a b}` - Random Choice (unpredictable)

### Preset System [Medium] - COMPLETE
- [x] `PSET <script> <name>` - Load preset into script 1-8
- [x] `PSET.SAVE <script> <name>` - Save script as user preset
- [x] `PSET.DEL <name>` - Delete user preset
- [x] `PSETS` - List all presets
- [x] 22 factory presets (drums, bass, lead, percussion, FX)

### DAW / MIDI Clock Sync [High] - PARTIAL
- [x] `M.SYNC <0|1>` - Sync mode (0=internal, 1=MIDI clock)
- [x] `MIDI.IN` - List available MIDI input devices
- [x] `MIDI.IN <name>` - Connect to MIDI device for clock sync
- [x] Timing diagnostics: MIDI.DIAG, SC.DIAG

### Command Delay System [Medium] - COMPLETE
- [x] `DEL <ms>: <cmd>` - Execute command after delay
- [x] `DEL.X <count> <ms>: <cmd>` - Queue command N times at intervals
- [x] `DEL.R <count> <ms>: <cmd>` - Execute immediately, then repeat
- [x] `DEL.CLR` - Clear all pending delayed commands

---

## Phase 4: UI/Feedback [COMPLETE]

### Phase 4.1: Activity Indicators [Medium] - COMPLETE
- [x] Script and metro execution feedback with decay animations
- [x] Header shows script indicators (1-8, M, I) with color decay

### Phase 4.2: SEQ/TOG State Highlighting [Medium] - COMPLETE
- [x] Current SEQ step highlighted in foreground/success color
- [x] TOG active option highlighted
- [x] Nested alternation and random choice highlighting

### Phase 4.3: Variables Page [Medium] - COMPLETE
- [x] Dedicated page showing all variable state

### Phase 4.4: Parameter Activity Grid [Medium] - COMPLETE
- [x] 8x6 grid of 48 unicode icons representing synth parameters
- [x] Icons light up and decay when parameters change

### Phase 4.5: Audio Metering [High] - COMPLETE
- [x] Real-time amplitude display via bidirectional OSC
- [x] Unicode bargraph display in header
- [x] 15-band spectrum analyzer
- [x] CPU percentage display

### Phase 4.6: Notes Page [Medium] - COMPLETE
- [x] 8 fixed lines with same editing commands as scripts
- [x] Notes saved and loaded with scenes

### Phase 4.7: Conditional Execution Highlighting [Medium] - COMPLETE
- [x] IF/ELIF/ELSE/PROB/EV/SKIP highlighting
- [x] Segment-based highlighting

### Phase 4.8: Scope Page [Medium] - COMPLETE
- [x] 128 samples at 20Hz from SuperCollider
- [x] Multiple character rendering modes
- [x] SCOPE.TIME, SCOPE.CLR, SCOPE.MODE, SCOPE.UNI commands

### Global Search [Medium] - COMPLETE
- [x] Ctrl+F search mode
- [x] Two isolated scopes: Help search, Script search

---

## Phase 5: Polish & Refinements [MOSTLY COMPLETE]

### Config Persistence Audit [Medium] - COMPLETE
### Auto-Load Previous Scene [Low] - COMPLETE
### Clear REPL on Load [Low] - COMPLETE
### Alias Coverage Audit [Low] - COMPLETE
### Per-Element UI Toggles [Low] - COMPLETE
### Scene Name Header Display [Low] - COMPLETE
### BPM Header Display [Medium] - COMPLETE
### Global Text Audit [High] - COMPLETE
### REPL/DEBUG Level Audit [Medium] - COMPLETE
### Global Error Handling Audit [Medium] - COMPLETE
### Help Coverage Audit [Low] - COMPLETE
### Script Undo/Redo [Medium] - COMPLETE
### Line Duplicate Push Behavior [Low] - COMPLETE
### List Output Formatting [Low] - COMPLETE
### NR and ER Operators [Medium] - COMPLETE

### Incomplete:
- [ ] Dynamic Grid Layout [Medium]
- [ ] Command Arg Scaling Audit [Medium]
- [ ] State Highlight Timing Verification [Low]

---

## Phase 6: Release Preparation [COMPLETE]

### Terminal Compatibility [Medium] - PHASE 1 COMPLETE
- [x] Terminal capability detection at startup
- [x] 256-color theme fallback
- [x] COMPAT command
- [x] METER.ASCII command

### Release Build & Tag [High] - COMPLETE
### GitHub Release [Medium] - COMPLETE
### Homebrew Tap [High] - COMPLETE

### Direct scsynth Integration [High] - COMPLETE
- [x] SynthDef Pre-compilation
- [x] Direct scsynth Spawning
- [x] OSC Message Routing
- [x] Recording Without sclang
- [x] Audio Device Handling
- [x] Bundling & Distribution

**Implementation Summary:**
- Feature gate: `cargo build --features scsynth-direct`
- New modules: src/scsynth_direct.rs, src/audio_devices.rs
- Bundle script: scripts/bundle.sh
- Complete OSC message flow from Rust to scsynth and back
