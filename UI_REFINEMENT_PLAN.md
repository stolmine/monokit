# Monokit UI Refinement Plan

## Vision

Transform the Monokit interface from a functional TUI into a visually responsive instrument display inspired by:
- **Teenage Engineering KO II** - Grid-based activity indicators with decay animations
- **Monome Teletype** - Variable monitoring, terse information density
- **Hardware sequencers** - Real-time feedback on what's happening

## Design Principles

1. **Immediate feedback** - Every action should have visual confirmation
2. **Decay aesthetics** - Activity fades rather than disappearing instantly
3. **Information density** - Show state without overwhelming
4. **Terminal-native** - Unicode characters, theme colors, no external dependencies

---

## Phase 1: Activity Indicators (Script/Metro Highlighting)

**Goal:** Visual pulse when scripts execute and metro ticks

### 1.1 Icon Decay System

**Characters for decay levels:**
```
Level 0 (peak):    █  ●  ◆
Level 1 (bright):  ▓  ◉  ◇
Level 2 (mid):     ▒  ○
Level 3 (dim):     ░  ·
Level 4 (off):     ·  ·
```

**Decay timing:** 500ms total (100ms per level)

### 1.2 Data Structure

```rust
// In src/types.rs or src/app/mod.rs

#[derive(Clone, Default)]
pub struct ActivityState {
    pub scripts: [Option<Instant>; 10],  // Scripts 1-8 + M + I
    pub metro_tick: Option<Instant>,
    pub trigger: Option<Instant>,
}

impl ActivityState {
    pub fn mark_script(&mut self, index: usize) {
        if index < 10 {
            self.scripts[index] = Some(Instant::now());
        }
    }

    pub fn mark_metro(&mut self) {
        self.metro_tick = Some(Instant::now());
    }

    pub fn mark_trigger(&mut self) {
        self.trigger = Some(Instant::now());
    }

    /// Returns decay level 0-4 (0=peak, 4=off)
    pub fn decay_level(timestamp: Option<Instant>, now: Instant) -> u8 {
        match timestamp {
            None => 4,
            Some(t) => {
                let elapsed = now.duration_since(t).as_millis() as u64;
                match elapsed {
                    0..=100 => 0,
                    101..=200 => 1,
                    201..=350 => 2,
                    351..=500 => 3,
                    _ => 4,
                }
            }
        }
    }
}
```

### 1.3 Header Integration

Modify `src/ui/header.rs` to show activity:

```
Current:
┌──────────────────────────────────────────────────────────┐
│ [LIVE] 1 2 3 4 5 6 7 8 M I P ?                     ● REC │
└──────────────────────────────────────────────────────────┘

With activity (M script just fired, script 1 decaying):
┌──────────────────────────────────────────────────────────┐
│ [LIVE] 1▓ 2 3 4 5 6 7 8 M█ I P ?                   ● REC │
└──────────────────────────────────────────────────────────┘
```

### 1.4 Implementation Steps

- [ ] Add `ActivityState` to `App` struct
- [ ] Mark script activity in `execute_script()`
- [ ] Mark metro activity in metro event handler
- [ ] Mark trigger activity in TR command handler
- [ ] Update header rendering to show decay icons
- [ ] Add decay character selection based on level and theme

### 1.5 Files to Modify

- `src/types.rs` - Add `ActivityState` struct
- `src/app/mod.rs` - Add `activity: ActivityState` field
- `src/app/script_exec/mod.rs` - Mark script execution
- `src/commands/system/misc.rs` - Mark TR trigger
- `src/ui/header.rs` - Render activity indicators

---

## Phase 2: SEQ/TOG State Highlighting

**Goal:** Show current position in stateful operators within script display

### 2.1 Visual Design

```
Before:
  SEQ "C3 E3 G3 C4"
  TOG 10 20

After (current step highlighted):
  SEQ "C3 E3 >G3< C4"
  TOG 10 >20<
```

**Bracket markers:** `>value<` for current/next item
**Color:** Theme accent color for highlighted value

### 2.2 State Already Tracked

State lives in `PatternStorage.toggle_state: HashMap<String, usize>`:
- SEQ key: `seq_{script_index}_{pattern_string}`
- TOG key: `{script_index}_TOG_{arg1}_{arg2}`

### 2.3 Rendering Approach

Create `src/ui/state_highlight.rs`:

```rust
pub struct HighlightedSegment {
    pub text: String,
    pub is_current: bool,
}

/// Parse a line and identify SEQ/TOG operators with their current state
pub fn highlight_stateful_operators(
    line: &str,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
) -> Vec<HighlightedSegment> {
    // 1. Find SEQ "..." patterns
    // 2. Look up current index in toggle_state
    // 3. Insert bracket markers around current token
    // 4. Find TOG a b patterns
    // 5. Look up state and mark active option
    // Return segments for rendering with Spans
}
```

### 2.4 Implementation Steps

- [ ] Create `src/ui/state_highlight.rs` module
- [ ] Implement SEQ pattern detection and token extraction
- [ ] Implement TOG expression detection
- [ ] Look up state from `toggle_state` HashMap
- [ ] Generate highlighted segments with bracket markers
- [ ] Integrate into `src/ui/pages/script.rs` rendering
- [ ] Apply theme accent color to highlighted segments
- [ ] Handle edge cases: empty state, quoted strings, multiple per line

### 2.5 Challenges

- **Long patterns:** `SEQ "C3 E3 G3 C4 D4 E4..."` - only bracket current, don't clutter
- **Nested quotes:** Respect string boundaries when parsing
- **First render:** State may not exist until first execution (show index 0)
- **Multiple operators:** Each gets independent highlighting

### 2.6 Files to Create/Modify

- `src/ui/state_highlight.rs` - NEW: highlighting logic
- `src/ui/pages/script.rs` - Integrate state-aware rendering
- `src/ui/pages/mod.rs` - Export new module

---

## Phase 3: Variables Page

**Goal:** Dedicated page showing all variable state (like Teletype monitor)

### 3.1 Layout Design

```
┌─────────────────────────────────────────────────────────────────────┐
│ VARIABLES                                                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│ GLOBAL                                                              │
│   A:      0    B:      0    C:      0    D:      0                 │
│   X:      0    Y:      0    Z:      0    T:      0                 │
│                                                                     │
│ COUNTERS                                                            │
│   N1:     0 [0..16]     N2:     0 [0..16]                          │
│   N3:     0 [0..16]     N4:     0 [0..16]                          │
│                                                                     │
│ LOCAL (per-script J/K)                                              │
│   S1: J:    0  K:    0      S5: J:    0  K:    0                   │
│   S2: J:    0  K:    0      S6: J:    0  K:    0                   │
│   S3: J:    0  K:    0      S7: J:    0  K:    0                   │
│   S4: J:    0  K:    0      S8: J:    0  K:    0                   │
│   M:  J:    0  K:    0      I:  J:    0  K:    0                   │
│                                                                     │
│ LOOP: I = 0                                                         │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Implementation Steps

- [ ] Add `Variables` variant to `Page` enum in `src/types.rs`
- [ ] Create `src/ui/pages/variables.rs`
- [ ] Implement `render_variables_page()` function
- [ ] Add page routing in `src/ui/mod.rs`
- [ ] Add keybinding (suggest: `Alt+V` or `F13` if available, or reassign)
- [ ] Update header to show VAR page option
- [ ] Optional: highlight recently-changed values with accent color

### 3.3 Change Highlighting (Optional Enhancement)

Track last-changed variable for visual feedback:

```rust
pub struct App {
    // ... existing ...
    pub last_changed_var: Option<(String, Instant)>,
}
```

Highlight with accent color, fade after 1 second.

### 3.4 Files to Create/Modify

- `src/types.rs` - Add `Page::Variables`
- `src/ui/pages/variables.rs` - NEW: page renderer
- `src/ui/pages/mod.rs` - Export new module
- `src/ui/mod.rs` - Route to new page
- `src/ui/header.rs` - Show V in page list
- `src/app/input.rs` - Add keybinding

---

## Phase 4: Activity Grid (Parameter Categories)

**Goal:** KO II-style grid showing which parameter categories are being modified

### 4.1 Visual Design

```
┌─ ACTIVITY ─────────────────────────────────────┐
│ OSC:█  FM:▓  ENV:░  FLT:·  FX:·  MOD:·  OUT:█ │
└────────────────────────────────────────────────┘
```

### 4.2 Parameter Categories

| Category | Parameters |
|----------|------------|
| OSC | PF, PW, MF, MW |
| FM | FM, FB, FBA, FBD |
| ENV | AD, PD, FD, DD, PA, FA, DA, ATK, CRV |
| FLT | FC, FQ, FT, FE, FED, FK |
| FX | DT, DF, DW, RV, RW, LB, LS, etc. |
| MOD | MB, TK, MP, MD, MT, MA, MX |
| OUT | VOL, PAN |

### 4.3 Data Structure

```rust
#[derive(Clone, Default)]
pub struct ParamActivity {
    pub osc: Option<Instant>,
    pub fm: Option<Instant>,
    pub env: Option<Instant>,
    pub flt: Option<Instant>,
    pub fx: Option<Instant>,
    pub modulation: Option<Instant>,
    pub output: Option<Instant>,
}

impl ParamActivity {
    pub fn mark_param(&mut self, param_name: &str) {
        match param_name.to_uppercase().as_str() {
            "PF" | "PW" | "MF" | "MW" => self.osc = Some(Instant::now()),
            "FM" | "FB" | "FBA" | "FBD" => self.fm = Some(Instant::now()),
            // ... etc
        }
    }
}
```

### 4.4 Implementation Steps

- [ ] Define parameter-to-category mapping
- [ ] Add `ParamActivity` to `App` struct
- [ ] Mark category activity when parameters change (in OSC send path)
- [ ] Create activity bar rendering component
- [ ] Integrate into header or dedicated status area
- [ ] Use same decay timing as script indicators

### 4.5 Placement Options

**Option A:** Extended header (3rd line)
```
│ [LIVE] 1 2 3 4 5 6 7 8 M I P ?                     ● REC │
│ OSC:█ FM:▓ ENV:░ FLT:· FX:· MOD:· OUT:█           120BPM │
```

**Option B:** Footer status line
```
│ > command input here                                     │
│ OSC:█ FM:▓ ENV:░ FLT:· FX:· MOD:· OUT:█  [?] HELP       │
```

**Option C:** Live page only (dedicated activity section)

### 4.6 Files to Modify

- `src/types.rs` - Add `ParamActivity` struct
- `src/app/mod.rs` - Add to App
- `src/commands/synth/*.rs` - Mark param activity on change
- `src/ui/header.rs` or `src/ui/footer.rs` - Render activity bar

---

## Phase 5: Audio Metering

**Goal:** Real-time amplitude and CPU display

### 5.1 Architecture

```
┌─────────┐  OSC commands  ┌──────────────┐
│  Main   │───────────────>│ SuperCollider│
│  Thread │                │   (57120)    │
└─────────┘                └──────────────┘
                                 │
                                 │ OSC replies (meter data)
                                 v
┌─────────┐                ┌──────────────┐
│ Meter   │<───────────────│  UDP Socket  │
│ Thread  │    receive     │   (57121)    │
└─────────┘                └──────────────┘
     │
     │ mpsc channel
     v
┌─────────┐
│   App   │──> UI rendering
└─────────┘
```

### 5.2 SuperCollider Changes

Add to `sc/monokit_server.scd` after final `Out.ar`:

```supercollider
// Amplitude metering - send peak/RMS to Rust
SendPeakRMS.kr(sigL, 20, 3, '/monokit/meter', replyID: 1001);
SendPeakRMS.kr(sigR, 20, 3, '/monokit/meter', replyID: 1002);
```

### 5.3 Rust Receiver

New file `src/meter.rs`:

```rust
pub struct MeterData {
    pub peak_l: f32,
    pub peak_r: f32,
    pub rms_l: f32,
    pub rms_r: f32,
    pub timestamp: Instant,
}

pub struct CpuData {
    pub avg_cpu: f32,
    pub peak_cpu: f32,
}

pub fn meter_thread(event_tx: Sender<MetroEvent>) {
    let socket = UdpSocket::bind("127.0.0.1:57121").unwrap();
    // ... receive loop, parse OSC, send MeterData via channel
}
```

### 5.4 Display

```
Unicode bargraph characters: ▁ ▂ ▃ ▄ ▅ ▆ ▇ █

Display in header or footer:
  L:▅▆▇█▇▅▃▂  R:▄▅▆▇▆▄▃▂  CPU: 12%
```

### 5.5 Implementation Steps

- [ ] Add `SendPeakRMS` to SuperCollider SynthDef
- [ ] Configure SC to send replies to port 57121
- [ ] Create `src/meter.rs` with receiver thread
- [ ] Add `MeterData` and `CpuData` types
- [ ] Add `MetroEvent::MeterUpdate` variant
- [ ] Spawn meter thread in main.rs
- [ ] Add meter state to App struct
- [ ] Create meter rendering component (bargraph)
- [ ] Integrate into UI (header/footer/live page)
- [ ] Add CPU polling (SC `/status` command)

### 5.6 Files to Create/Modify

- `sc/monokit_server.scd` - Add SendPeakRMS
- `src/meter.rs` - NEW: receiver thread
- `src/types.rs` - Add meter types and events
- `src/main.rs` - Spawn meter thread
- `src/app/mod.rs` - Add meter state
- `src/ui/header.rs` or dedicated component - Render meters

---

## Implementation Order

| Phase | Feature | Complexity | Dependencies | Priority |
|-------|---------|------------|--------------|----------|
| 1 | Activity Indicators | Low | None | **Start here** |
| 2 | SEQ/TOG Highlighting | Medium | Phase 1 patterns | High |
| 3 | Variables Page | Medium | None | Medium |
| 4 | Parameter Activity Grid | Medium | Phase 1 patterns | Medium |
| 5 | Audio Metering | High | SC changes, new thread | Lower |

**Recommended approach:** Complete Phase 1 first - it establishes the decay animation pattern and activity tracking infrastructure that Phases 2 and 4 will reuse.

---

## Theme Integration

All activity indicators should use theme colors:

```rust
// Decay level to color mapping
fn decay_color(level: u8, theme: &Theme) -> Color {
    match level {
        0 => theme.accent,      // Peak brightness
        1 => theme.success,     // Bright
        2 => theme.foreground,  // Normal
        3 => theme.secondary,   // Dim
        _ => theme.background,  // Off (invisible)
    }
}
```

---

## Testing Strategy

### Visual Testing
- Verify decay animations are smooth (not flickering)
- Check all themes for readable contrast
- Test at different terminal sizes
- Verify Unicode characters render correctly

### Functional Testing
- Script execution triggers correct indicator
- Metro tick pulses M indicator
- TR command pulses trigger indicator
- State highlighting matches actual SEQ/TOG position
- Variable page shows correct values

### Performance Testing
- Ensure activity tracking doesn't impact metro timing
- Meter thread shouldn't block main thread
- UI refresh rate handles 20Hz meter updates

---

## Future Enhancements (Post-MVP)

- **Waveform preview:** Mini oscilloscope on Live page
- **Pattern visualization:** Current step highlighted in Pattern page
- **Command history activity:** Show last N executed commands
- **Recording indicator animation:** Pulsing red dot
- **BPM tap tempo visual:** Flash on tap input

---

## References

- Teenage Engineering KO II display research: `research_ko_ii_display_style.md`
- Existing UI architecture: `src/ui/`
- Theme system: `src/theme.rs`
- Activity state patterns: Based on existing `last_error` timeout pattern
