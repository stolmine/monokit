# Phase 6 Polish Implementation Plan

Detailed implementation plan for monokit Phase 6: Polish & Refinements.

**Total Estimated Effort:** 6-10 weeks (can be parallelized across multiple agents)

---

## Overview

Phase 6 covers final polish items before advanced DSP work (Phase 7) and distribution (Phase 8). Items are grouped into four categories:

1. **Config & UI Toggles** - Persistence and per-element controls
2. **Header Enhancements** - Scene name and BPM display
3. **Audits** - Alias, text, scaling, debug, help coverage
4. **Infrastructure** - Error handling, audio device selection, window sizing

---

## 1. CONFIG PERSISTENCE & UI TOGGLES

### 1.1 Config Persistence Audit [COMPLETE]

**Current State:**
- `DisplayConfig` in `src/config.rs` tracks: theme, header_level, load_rst
- Pattern: load full config → modify field → save entire config
- Save-on-change approach (immediate persistence)

**New Fields to Add:**

```rust
pub struct DisplayConfig {
    // Existing
    pub theme: String,
    pub header_level: u8,
    pub load_rst: bool,

    // New persistence
    pub debug_level: u8,                    // Default: 2
    pub show_cpu: bool,                     // Default: false
    pub show_meters: bool,                  // Default: true
    pub show_spectrum: bool,                // Default: true
    pub show_activity: bool,                // Default: true
    pub show_grid_view: bool,               // Default: false
    pub show_seq_highlight: bool,           // Default: true
    pub show_conditional_highlight: bool,   // Default: true
    pub grid_mode: u8,                      // Default: 1 (icons)
    pub limiter_enabled: bool,              // Default: true
    pub activity_hold_ms: u32,              // Default: 200
    pub title_mode: u8,                     // Default: 0 (MONOKIT)

    // Scope settings
    pub scope_timespan_ms: u32,             // Default: 30
    pub scope_color_mode: u8,               // Default: 0
    pub scope_display_mode: u8,             // Default: 0
    pub scope_unipolar: bool,               // Default: false
}
```

**Files to Modify:**
- `src/config.rs` - Add fields and persistence helpers (~80 lines)
- `src/main.rs` - Load preferences on startup (~10 lines)
- `src/app/mod.rs` - Initialize from config (~20 lines)

**Effort:** 2-3 hours

---

### 1.2 Per-Element UI Toggles [COMPLETE]

New commands for granular UI control:

| Command | Description | Default |
|---------|-------------|---------|
| `METER <0\|1>` | Toggle audio meters | 1 (on) |
| `SPECTRUM <0\|1>` | Toggle spectrum analyzer | 1 (on) |
| `ACTIVITY <0\|1>` | Toggle script activity indicators | 1 (on) |
| `GRID <0\|1>` | Toggle parameter activity grid | 0 (off) |
| `HL.SEQ <0\|1>` | Toggle SEQ/TOG state highlighting | 1 (on) |
| `HL.COND <0\|1>` | Toggle conditional highlighting (exists, add persistence) | 1 (on) |
| `GRID.MODE <0\|1>` | Toggle grid labels (0) vs icons (1) | 1 (icons) |

**Implementation per command:**
1. Add handler in `src/commands/system/misc.rs` (~30 lines each)
2. Wire in `src/commands/mod.rs` process_command (~3 lines each)
3. Update UI rendering to respect flag
4. Call persistence helper on change

**Files to Modify:**
- `src/commands/system/misc.rs` - Add 6 handlers (~180 lines)
- `src/commands/mod.rs` - Add match arms (~20 lines)
- `src/ui/header.rs` - Respect show_activity, show_meters (~40 lines)
- `src/ui/pages/live.rs` - Respect show_spectrum, grid_mode (~100 lines)
- `src/ui/pages/script.rs`, `metro.rs`, `init.rs` - Respect show_seq_highlight (~30 lines each)

**Effort:** 4-6 hours

---

## 2. HEADER ENHANCEMENTS

### 2.1 Scene Name Header Display [COMPLETE]

**Command:** `TITLE <0|1>`
- 0 = Show "MONOKIT" (default)
- 1 = Show current scene name or "[UNSAVED]"

**Implementation:**

1. **Track current scene** - Add to App struct:
   ```rust
   pub current_scene_name: Option<String>,
   pub title_mode: u8,
   ```

2. **Update on SAVE/LOAD** - Modify `src/commands/system/scene.rs`:
   ```rust
   // In handle_load success path:
   *current_scene_name = Some(name.clone());

   // In handle_save success path:
   *current_scene_name = Some(name.clone());
   ```

3. **Render dynamic title** - Modify `src/ui/header.rs`:
   ```rust
   let header_title = match app.title_mode {
       0 => " MONOKIT ".to_string(),
       1 => match &app.current_scene_name {
           Some(name) => format!(" {} ", truncate(name, 15)),
           None => " [UNSAVED] ".to_string(),
       },
       _ => " MONOKIT ".to_string(),
   };
   ```

**Files to Modify:**
- `src/config.rs` - Add title_mode field (~5 lines)
- `src/app/mod.rs` - Add current_scene_name field (~5 lines)
- `src/commands/system/scene.rs` - Update name on save/load (~20 lines)
- `src/commands/system/misc.rs` - Add TITLE handler (~30 lines)
- `src/ui/header.rs` - Dynamic title rendering (~20 lines)

**Effort:** 3-4 hours

---

### 2.2 BPM Header Display [COMPLETE]

**Automatic display** - No command needed, integrates with HEADER levels.

**Display Logic:**
- HEADER 0: No BPM
- HEADER 1-4: Show "BPM 120" in right border

**Calculation:** `bpm = 15000 / period_ms` (16th note → quarter note)

**Implementation:**

In `src/ui/header.rs` right-aligned section:
```rust
// Add BPM to title_parts (after REC, before CPU)
if app.header_level >= 1 {
    if let Ok(metro) = app.metro_state.try_lock() {
        let bpm = (15000.0 / metro.period_ms as f32).round() as u32;
        title_parts.push(Span::styled(
            format!("BPM {}", bpm),
            Style::default().fg(app.theme.secondary),
        ));
    }
}
```

**Files to Modify:**
- `src/ui/header.rs` - Add BPM display (~15 lines)

**Effort:** 1-2 hours

---

## 3. AUDITS

### 3.1 Alias Coverage Audit [COMPLETE]

**Current State:** 89 aliases in `src/commands/aliases.rs`

**Missing Aliases Identified:**

| Canonical | Proposed Alias | Status |
|-----------|---------------|--------|
| AENV.ATK | AA | Add |
| PENV.ATK | PAA | Add |
| FMEV.ATK | FAA | Add |
| DENV.ATK | DAA | Add |
| FBEV.ATK | FBAA | Add |
| FLEV.ATK | FLAA | Add |
| AENV.CRV | AC | Add |
| PENV.CRV | PC | Add |
| FBEV.CRV | FBAC | Add |
| FLEV.CRV | FLAC | Add |
| FMEV.CRV | - | CONFLICT (FC = Filter Cut) |
| DENV.CRV | - | CONFLICT (DC = Discontinuity) |

**Action Items:**
1. Add 10 non-conflicting aliases to HashMap
2. Document FC/DC conflicts in help system
3. Update help_content.rs with new aliases

**Files to Modify:**
- `src/commands/aliases.rs` - Add aliases (~15 lines)
- `src/ui/pages/help_content.rs` - Document new aliases (~20 lines)

**Effort:** 2-3 hours

---

### 3.2 Global Text Audit [COMPLETE]

**Constraint:** 50 columns × 18 rows minimum terminal

**Usable Content Width:** 46-48 characters
- Terminal: 50 columns total
- Borders: 2 characters (│ on each side)
- Padding: 1-2 characters from each border
- Math: 50 - 2 (borders) - 2 (padding) = 46 chars safe width

**Audit Scope:**
1. Help pages - Check all 11 categories for lines > 46 chars
2. Error messages - Standardize format and length
3. REPL history - Verify scrolling at 50-char width
4. Grid layouts - Verify centering at minimum width
5. Header/footer - Verify all elements fit

**Action Items:**
1. Write script to find long lines in help_content.rs
2. Test at exactly 50×18 terminal
3. Reformat any overflow text
4. Standardize error message format

**Effort:** 4-6 hours

---

### 3.3 Command Arg Scaling Audit

**Current Ranges:**
- 0-16383: ~40 parameters (FM, feedback, mix levels)
- Float 0.0-1.0: Volume
- Time-based 1-10000ms: Decay, attack
- Special: Ratios (1-20), curves (-8 to 8)

**Decision:** Keep 0-16383 as default (consistency, 14-bit MIDI compatibility)

**Action Items:**
1. Document rationale for each range in help system
2. Verify consistency within parameter categories
3. Consider MAP7/MAP14 operators if useful

**Effort:** 2-4 hours (mostly documentation)

---

### 3.4 REPL/DEBUG Level Audit

**Current Levels:**
- DEBUG 0: Errors only
- DEBUG 1: Errors + PRINT + metro info + state changes
- DEBUG 2: All params + expressions + pattern ops

**Future Enhancements (optional):**
- DEBUG 3: Full expression tracing, command timing
- Per-subsystem flags: DEBUG.MIDI, DEBUG.OSC, DEBUG.METRO

**Action Items:**
1. Document exactly what each level shows
2. Create DEBUG_LEVELS.md reference
3. Audit for any sensitive data leaks
4. Plan DEBUG 3 implementation (optional)

**Effort:** 3-5 hours

---

### 3.5 Help Coverage Audit [COMPLETE]

**Current State:** 11 HelpCategory pages, mostly complete

**Gaps Identified:**
1. HEADER command levels 0-4 - incomplete
2. CPU command - not documented
3. HL.COND toggle - not documented
4. FLASH command - not documented
5. CLEAR command - not documented
6. Navigation shortcuts (Alt+key) - incomplete
7. Expression evaluation syntax - not documented
8. Complete synth parameter reference - needs expansion

**Action Items:**
1. Cross-reference ROADMAP.md features with help pages
2. Add missing commands
3. Expand synth parameter reference
4. Add expression evaluation guide
5. Complete navigation documentation

**Files to Modify:**
- `src/ui/pages/help_content.rs` - Major additions (~200 lines)

**Effort:** 6-8 hours

---

## 4. INFRASTRUCTURE

### 4.1 Global Error Handling Audit

**Current State:**
- Uses `anyhow::Result<T>` for error propagation
- 44 `.unwrap()` calls (most are safe Mutex locks)
- Error messages vary in format and helpfulness

**Improvement Plan:**

1. **Standardize error format:**
   ```
   [ERROR] <CATEGORY>: <MESSAGE> [SUGGESTION]

   Categories: FILE_IO, MIDI, OSC, BOUNDS, SYNTAX, RANGE, NOT_FOUND
   ```

2. **Improve specific areas:**
   - Scene I/O: User-friendly messages with recovery suggestions
   - MIDI: Report when device not found with alternatives
   - OSC: Log connection failures
   - Patterns: Add bounds checking on all operations
   - Expressions: Detailed parse error context

3. **Create centralized error types:**
   ```rust
   // src/errors.rs (new file)
   pub enum MonokitError {
       FileNotFound { name: String, suggestion: String },
       MidiDeviceNotFound { name: String, available: Vec<String> },
       PatternIndexOutOfBounds { index: usize, max: usize },
       // etc.
   }
   ```

**Files to Modify/Create:**
- `src/errors.rs` - New centralized error types (~150 lines)
- `src/scene.rs` - Improve error messages (~50 lines)
- `src/midi.rs` - Add connection status tracking (~50 lines)
- `src/commands/patterns/common.rs` - Bounds checking (~100 lines)
- `src/eval/mod.rs` - Expression error context (~80 lines)

**Effort:** 1-2 weeks

---

### 4.2 Audio Device Selection

**New Commands:**
- `AUDIO.OUT` - List available output devices
- `AUDIO.OUT <device>` - Select output device (restarts audio)
- `AUDIO.IN` - List available input devices (future)
- `AUDIO.IN <device>` - Select input device (future)

**Implementation:**

1. **SuperCollider side** (`sc/monokit_server.scd`):
   ```supercollider
   OSCdef(\monokit_audio_devices, { |msg|
       ServerOptions.outDevices.do { |dev|
           Server.default.addr.sendMsg("/monokit/audio_devices", dev);
       };
   }, "/monokit/audio_devices");

   OSCdef(\monokit_set_output_device, { |msg|
       var device = msg[1];
       Server.default.options.outDevice_(device);
       Server.default.reboot;
   }, "/monokit/set_output_device");
   ```

2. **Rust side:**
   - Add `QueryAudioDevices`, `SelectAudioDevice(String)` to MetroCommand
   - Add handler in new `src/commands/system/audio.rs`
   - Persist selection to config.toml

3. **Config persistence:**
   ```rust
   pub struct AudioConfig {
       pub output_device: String,  // Default: "default"
       pub input_device: Option<String>,
   }
   ```

**Files to Create/Modify:**
- `src/commands/system/audio.rs` - New file (~200 lines)
- `src/types.rs` - Add MetroCommand variants (~5 lines)
- `src/metro.rs` - Add command handlers (~150 lines)
- `src/config.rs` - Add AudioConfig (~40 lines)
- `sc/monokit_server.scd` - Add OSC handlers (~60 lines)

**Effort:** 2-3 weeks

---

### 4.3 Initial Window Sizing

**Implementation:**

1. **Detect size on startup** (`src/main.rs`):
   ```rust
   let (width, height) = terminal::size().unwrap_or((80, 24));
   if width < 50 || height < 18 {
       println!("WARNING: Terminal size {}x{} is below minimum 50x18", width, height);
       println!("Resize and restart, or continue with reduced quality? (y/n)");
       // ... handle response
   }
   ```

2. **Documentation** - Add to README.md:
   - Minimum terminal size: 50×18
   - Recommended: 80×24 or larger
   - Platform-specific setup instructions

**Files to Modify:**
- `src/main.rs` - Add size check (~30 lines)
- `README.md` - Document requirements (~40 lines)

**Effort:** 3-5 hours

---

## Implementation Priority & Timeline

### Week 1-2: Foundation
| Item | Priority | Effort |
|------|----------|--------|
| Config Persistence | HIGH | 2-3 hrs |
| Per-Element UI Toggles | HIGH | 4-6 hrs |
| Alias Coverage Audit | HIGH | 2-3 hrs |
| Window Sizing | LOW | 3-5 hrs |

### Week 3-4: Header & Audits
| Item | Priority | Effort |
|------|----------|--------|
| Scene Name Display | MEDIUM | 3-4 hrs |
| BPM Display | MEDIUM | 1-2 hrs |
| Global Text Audit | HIGH | 4-6 hrs |
| Help Coverage Audit | MEDIUM | 6-8 hrs |

### Week 5-6: Debug & Scaling
| Item | Priority | Effort |
|------|----------|--------|
| REPL/DEBUG Audit | LOW | 3-5 hrs |
| Command Arg Scaling | LOW | 2-4 hrs |

### Week 7-10: Infrastructure
| Item | Priority | Effort |
|------|----------|--------|
| Error Handling Audit | HIGH | 1-2 wks |
| Audio Device Selection | HIGH | 2-3 wks |

---

## Parallel Agent Allocation

**Agent 1: Config & UI** (Week 1-2)
- Config persistence
- UI toggle commands
- Scene name display
- BPM display

**Agent 2: Audits** (Week 2-4)
- Alias coverage
- Global text audit
- Help coverage audit
- REPL/DEBUG audit
- Command arg scaling

**Agent 3: Infrastructure** (Week 3-10)
- Error handling audit
- Audio device selection
- Window sizing

---

## Success Criteria

- [✓] All new commands persist to config.toml
- [✓] UI toggles work independently and together
- [✓] Scene name displays correctly (truncation, [UNSAVED])
- [✓] BPM updates in real-time in header
- [✓] All aliases documented and working
- [✓] No text overflow at 50×18 terminal
- [✓] Help covers all Phase 1-5 features
- [ ] Error messages are user-friendly and actionable
- [ ] Audio device selection works on macOS/Linux/Windows
- [ ] Terminal size warning on startup
- [ ] All 411+ tests pass

---

## Files Summary

**New Files:**
- `src/commands/system/audio.rs` (~200 lines)
- `src/errors.rs` (~150 lines)
- `docs/DEBUG_LEVELS.md` (~80 lines)

**Major Modifications:**
- `src/config.rs` (~100 lines added)
- `src/commands/system/misc.rs` (~200 lines added)
- `src/ui/pages/help_content.rs` (~200 lines added)
- `src/ui/header.rs` (~50 lines modified)
- `sc/monokit_server.scd` (~60 lines added)

---

*Document created: December 2025*
*Status: In Progress*
*Last updated: December 2, 2025*
