# Phase 5.2: SEQ/TOG State Highlighting

## Status: COMPLETE (December 2025)

## Goal

Show current position in stateful operators (SEQ, TOG) within script display. When viewing a script page, highlight which value is currently active.

**Visual Design:**
- SEQ: `SEQ "C3 E3 G3 C4"` - current step shown in highlight color (no brackets)
- TOG: `TOG 10 20` - active value shown in highlight color (no brackets)
- Highlighting via **text color only**, no bracket markers

**Color Strategy:**
- **Non-selected line:** Current value in `foreground` color, rest in `secondary`
- **Selected line (on highlight_bg):** Current value in `success` (green), rest in `highlight_fg`

This provides good contrast across themes:
- Dark theme: green (80, 255, 80) on white highlight_bg
- Light theme: green (0, 160, 0) on black highlight_bg

## State Storage

Both SEQ and TOG states are stored in `PatternStorage.toggle_state: HashMap<String, usize>`:

**SEQ Key Format** (from `src/eval/seq.rs:384`):
```rust
let key = format!("seq_{}_{}", script_index, pattern);
// Example: "seq_0_C3 E3 G3 C4" for script 0
```

**SEQ Alternation Key** (from `src/eval/seq.rs:301`):
```rust
let alt_key = format!("seq_alt_{}_{}_{}", script_index, pattern, step_index);
```

**TOG Key Format** (from `src/eval/logic.rs:68`):
```rust
let key = format!("{}_{}", script_index, parts[start_idx..next_idx].join("_"));
// Example: "0_TOG_10_20" for script 0
```

## Files to Create/Modify

### NEW: `src/ui/state_highlight.rs`

Core module with:

```rust
use std::collections::HashMap;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};

/// A segment of text with highlighting information
#[derive(Debug, Clone)]
pub struct HighlightedSegment {
    pub text: String,
    pub is_highlighted: bool,
}

/// Result of parsing a line for stateful operators
#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<HighlightedSegment>,
}

impl HighlightedLine {
    /// Convert to ratatui Spans with appropriate styling
    /// - normal_color: color for non-highlighted text
    /// - highlight_color: color for current SEQ/TOG value
    pub fn to_spans(&self, normal_color: Color, highlight_color: Color) -> Vec<Span<'static>> {
        self.segments.iter().map(|seg| {
            if seg.is_highlighted {
                Span::styled(seg.text.clone(), Style::default().fg(highlight_color))
            } else {
                Span::styled(seg.text.clone(), Style::default().fg(normal_color))
            }
        }).collect()
    }
}

/// Main entry point - parse line and highlight stateful operators
pub fn highlight_stateful_operators(
    line: &str,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
) -> HighlightedLine
```

### MODIFY: `src/ui/pages/script.rs`

Integrate highlighting into script page rendering:

```rust
use super::state_highlight::highlight_stateful_operators;

// In render_script_page(), for each non-empty line:
let highlighted = highlight_stateful_operators(
    line_content,
    script_index,
    &app.patterns.toggle_state,
);

// Color strategy:
// - Non-selected: foreground for highlight, secondary for normal
// - Selected: success (green) for highlight, highlight_fg for normal
let (normal_color, highlight_color) = if is_selected {
    (app.theme.highlight_fg, app.theme.success)
} else {
    (app.theme.secondary, app.theme.foreground)
};

let spans = highlighted.to_spans(normal_color, highlight_color);
```

### MODIFY: `src/ui/pages/metro.rs`

Same highlighting logic for metro page (script_index = 8)

### MODIFY: `src/ui/pages/init.rs`

Same highlighting logic for init page (script_index = 9)

### MODIFY: `src/ui/pages/mod.rs`

Export the new module:
```rust
pub mod state_highlight;
```

## Algorithm

### SEQ Highlighting

1. Scan line for `SEQ ` followed by quoted string
2. Extract pattern string (handle `"` and `'` quotes)
3. Build state key: `format!("seq_{}_{}", script_index, pattern)`
4. Look up current index from `toggle_state` (default 0)
5. Parse pattern tokens to identify current step
6. Mark current token as highlighted (color change only, no brackets)
7. Return segments with is_highlighted flags

**Pattern Parsing Considerations:**
- Split by whitespace
- Handle special tokens: `_`, `.`, `x`, `X`, `?`
- Handle note names: `C3`, `Eb4`, `F#2`
- Handle numbers: `100`, `-12`
- Handle alternation: `<C3 E3>` (whole group = one step)
- Handle random choice: `{C3 E3}` (whole group = one step)
- Handle repeats: `C3*4` (expands to 4 steps)

### TOG Highlighting

1. Scan line for `TOG `
2. Extract the two value tokens following TOG
3. Build state key: `format!("{}_{}", script_index, "TOG_<val1>_<val2>")`
4. Look up current state (default 0)
5. Active value = state % 2 (0 = first, 1 = second)
6. Mark active token as highlighted (color change only, no brackets)

### Multiple Operators Per Line

Process left-to-right, building segments incrementally:
1. Find first operator occurrence
2. Highlight it, add segments
3. Continue from after that operator
4. Repeat until end of line

## Edge Cases

| Case | Handling |
|------|----------|
| No state exists yet | Show index 0 (first value) highlighted |
| Empty pattern | Return line unchanged |
| Nested expressions in TOG | Parse expression boundaries |
| Alternation inside SEQ | Show current option within group |
| Repeat syntax `C3*4` | Track expanded step count vs display position |

## Implementation Summary

All steps completed:

1. ✓ Created `src/ui/state_highlight.rs` with structs and main function signature
2. ✓ Implemented SEQ token parsing (reference `src/eval/seq.rs`)
3. ✓ Implemented TOG detection and state lookup (reference `src/eval/logic.rs:68`)
4. ✓ Handled multiple operators per line
5. ✓ Updated `src/ui/pages/script.rs` to use highlighting
6. ✓ Updated `src/ui/pages/metro.rs` (script_index = 8)
7. ✓ Updated `src/ui/pages/init.rs` (script_index = 9)
8. ✓ Added unit tests

## Additional Features Implemented

- **SEQ validation:** Reject invalid syntax (`SEQ"..."` and `SEQ "...`)
- **Random choice state tracking:** Track `{a b}` selections with `seq_rnd_` keys
- **Nested alternation highlighting:** Show active option in `<a b>` groups
- **Multi-operator support:** Correctly handle multiple SEQ/TOG on same line

## Files Created/Modified

- `src/ui/state_highlight.rs` - NEW: highlighting logic with unit tests
- `src/ui/mod.rs` - Export new module
- `src/ui/pages/script.rs` - Integrate state-aware rendering
- `src/ui/pages/metro.rs` - Integrate state-aware rendering
- `src/ui/pages/init.rs` - Integrate state-aware rendering

## Reference Files

- `src/eval/seq.rs` - SEQ pattern parsing, state key format
- `src/eval/logic.rs:68` - TOG state key format
- `src/types.rs` - `PatternStorage.toggle_state` HashMap
- `src/ui/pages/script.rs` - Current script rendering

## Testing

**Unit Tests:**
- SEQ highlighting at different positions
- TOG highlighting for both states
- Multiple operators per line
- Edge cases (empty, missing state, repeats)

**Manual Testing:**
- Create scripts with SEQ patterns
- Step through metro, verify highlighting advances
- Test with dark and light themes
