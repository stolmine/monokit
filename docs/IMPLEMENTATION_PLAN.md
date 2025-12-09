# Implementation Plan

## Current Focus (December 2025)

### Phase 1: Bug Fixes (1-2 days)

**1. TOG Zero Parsing Bug [COMPLETE]**
- Issue: `DC TOG 2000 0` displays as `DC TOG 2000 000 0` on script line
- Root cause: Key format mismatch between state storage and highlighting
- Fix: Changed key format from `"cmd_<idx>_DC_TOG_2000_0"` to `"<idx>_TOG_2000_0"`
- Status: Fixed in src/commands/core/random_ops.rs

**2. IF/ELSE/ELIF Scope Logic [COMPLETE]**
- Investigated: ELSE does NOT cut off downstream logic processing
- Scope is SCRIPT-LOCAL (persists across lines within same script)
- Multi-line IF/ELIF/ELSE chains work correctly
- Semicolon-separated conditionals work correctly
- Edge case: "orphan" ELSE fires if prior IF was false (by design)
- Help system updated with scope documentation
- Test scenes: test_conditionals.json, test_conditionals_edge.json

**3. Boolean Operator Verification [COMPLETE]**
- User reported: IF EZ not firing when expected
- Tested all boolean ops: EZ, NZ, EQ, NE, GT, LT, GTE, LTE
- All operators work correctly in IF conditions
- Works with variables, literals, and nested expressions (SUB, ADD, MUL)
- Test scene: test_boolean_ops.json (all tests pass)
- Conclusion: Operators work correctly; user issue likely variable state

**4. Loops + Stateful Ops Testing [COMPLETE]**
- Tested TOG, EITH, SEQ, booleans in loops - all work correctly
- Test scene: test_loops_stateful.json

**5. Nested IF in Loops Bug [COMPLETE]**
- Issue: `L 1 6: IF GT I 2: IF LT I 5: PRINT I` → "UNKNOWN COMMAND: IF"
- Root cause: process_sub_command uses find(':') which only finds first colon
- Fix: Changed find(':') to rfind(':') to split at rightmost colon
- Added recursive nested conditional handling
- Status: Fixed in src/app/script_exec/control_flow.rs

**6. SEQ Quote Parsing Bug [COMPLETE]**
- Issue: `A SEQ "A B C D"` → "FAILED TO PARSE VALUE"
- Root cause: split_whitespace() fragments quoted strings
- Fix: Added split_whitespace_respecting_quotes() utility function
- Variable assignment now respects quoted strings
- Status: Fixed in src/utils.rs, src/commands/mod.rs, src/app/script_exec/mod.rs

**7. CLI Enhancements [COMPLETE]**
- `--dry-run --run <scene>` runs without SuperCollider/audio
- Metro thread skips OSC socket creation in dry-run mode
- Enables headless testing of command logic
- Batch mode auto-starts metro: SetActive(true) after loading scene
- Status: Fixed in src/main.rs, src/metro.rs

**8. State Highlight Timing Verification [Low]**
- Audit SEQ, TOG, EITH, `<>`, `{}` highlight timing
- Verify highlights show current state vs previous state
- Test in metro context and nested contexts
- Fix any timing inconsistencies found

---

### Phase 2: High-Value Features (1-2 weeks)

**9. SYNC Command [COMPLETE]**
- `SYNC` resets all stateful elements to starting position:
  - SEQ sequences → first element
  - TOG toggles → first value
  - EITH/`<>`/`{}` states cleared
  - EV/SKIP counters → 0
  - Pattern indices (P.I) → 0
  - N1-N4 counters → MIN values
- Partial variants implemented: `SYNC.SEQ`, `SYNC.TOG`, `SYNC.PAT`
- Status: Implemented in src/commands/core/sync.rs

**10. Auto-Load Previous Scene [COMPLETE]**
- `AUTOLOAD <0|1>` - Enable/disable auto-load on startup
- Track last loaded scene name in config.toml
- On startup, if enabled, run LOAD with saved scene name
- Handle missing scene gracefully
- Status: Implemented in config.rs, commands/system/misc.rs, commands/system/scene.rs, main.rs

---

### Phase 3: Foundation Work (2-4 weeks)

**11. Script Validation Overhaul [COMPLETE]**
- Reject invalid commands on script line entry (before save)
- Validate all argument counts and types
- Validate expression syntax (balanced parens, valid operators)
- Validate SEQ notation syntax (quotes, brackets, valid tokens)
- Validate pattern references (0-5 range)
- Validate script references (1-8, M, I)
- Clear, specific error messages for each failure type
- Status: All 6 phases complete (paste bypass, pattern ops, expressions, control flow, SEQ, references)

**12. File Size/DRY Audit [Medium]**
- Ensure all files within agent readable limits (~500 lines)
- Create `CommandContext` struct to bundle shared parameters
- Create category-aware output helper
- Audit synth param macros for shared patterns

---

**9. Envelope Parameter Scaling Bug [COMPLETE]**
- Issue: FA, DA envelopes had no audible effect
- Root cause: Rust limited FA/DA to 0-16 but SynthDef divided by 16383
- Fix: Changed FA, DA from float 0-16 to int 0-16383
- Status: Fixed in src/commands/synth/envelopes/fm.rs, disc.rs

**10. SynthDef/RST Default Mismatch [COMPLETE]**
- Issue: SynthDef defaults differed from RST values (fc=1000 was filtering FM)
- Fix: Aligned SynthDef defaults with RST values
- Changes: pf=131, mf=262, pa=0, fc=10000, cr=1, dmode=2, rmode=2
- Status: Fixed in sc/monokit_server.scd, build_scripts/compile_synthdefs.scd

---

### Deferred Items

- Dynamic grid layout [Medium] - Polish item
- NR/ER operators [Medium] - Nice-to-have Teletype features

---

## P1 Features (December 2025)

### 1. Script Undo/Redo [Medium Complexity]

**Current State:**
- Scripts stored in `ScriptStorage` (src/types.rs lines 275-306)
- Each script has 8 lines as `[String; 8]`
- Editing in src/app/input.rs: `save_line()`, `duplicate_line()`, `delete_entire_line()`, `cut_line()`, `paste_line()`

**Data Structure:**
```rust
pub struct UndoEntry {
    pub script_index: usize,
    pub line_index: usize,
    pub previous_content: String,
    pub new_content: String,
}

// Add to App struct:
pub undo_stack: Vec<UndoEntry>,
pub redo_stack: Vec<UndoEntry>,
```

**Scope:** Per-session, per-script history. Clear stacks when changing scripts/pages.

**Undoable Operations:**
- `save_line()` - save before/after state
- `duplicate_line()` - save overwritten line
- `delete_entire_line()` - save deleted content
- `paste_line()` - save overwritten line

**Files to Modify:**
| File | Changes |
|------|---------|
| src/app/mod.rs | Add UndoEntry struct and stacks to App |
| src/app/input.rs | Record state before each edit operation |
| src/ui/mod.rs | Add Ctrl+Z/Ctrl+Y key handlers (~line 490) |
| src/ui/pages/help_content.rs | Document undo/redo keybindings |

**Implementation Steps:**
1. Add `UndoEntry` struct and stacks to App
2. Create `push_undo()` helper that clears redo stack
3. Modify each editing function to record state before mutation
4. Add `undo()` and `redo()` methods to App
5. Add key bindings in ui/mod.rs

---

### 2. Line Duplicate Push Behavior [Low Complexity]

**Current Behavior (src/app/input.rs lines 145-157):**
```rust
script.lines[selected + 1] = line_content;  // OVERWRITES next line
```
Duplicating line N copies to N+1, **overwriting** whatever was there.

**Requested Behavior:**
Shift lines N+1 through 7 down (line 7 lost if non-empty), insert duplicate at N+1.

**Implementation:**
```rust
pub fn duplicate_line(&mut self) {
    if let Some(script_idx) = self.current_script_index() {
        if let Some(selected) = self.selected_line {
            if selected < 7 {
                let script = self.scripts.get_script(script_idx);
                let line_content = script.lines[selected].clone();

                // Push lines down from bottom to selected+1
                let script = self.scripts.get_script_mut(script_idx);
                for i in (selected + 2..=7).rev() {
                    script.lines[i] = script.lines[i - 1].clone();
                }
                script.lines[selected + 1] = line_content;
                self.selected_line = Some(selected + 1);
            }
        }
    }
}
```

**Files to Modify:**
| File | Changes |
|------|---------|
| src/app/input.rs | Modify `duplicate_line()` and `duplicate_notes_line()` |
| src/ui/pages/help_content.rs | Update help text |

---

### 3. Version Display [Low Complexity]

**Compile-time version access:**
```rust
const VERSION: &str = env!("CARGO_PKG_VERSION");
```

**Startup message (src/main.rs ~line 156):**
```rust
println!("MONOKIT v{} - Starting...", env!("CARGO_PKG_VERSION"));
```

**VERSION command (src/commands/mod.rs):**
```rust
"VERSION" | "VER" => {
    output(format!("MONOKIT v{}", env!("CARGO_PKG_VERSION")));
}
```

**Help page header (src/ui/pages/help.rs ~line 76):**
```rust
let title = format!(
    " HELP: {} ({}/{}) - v{} ",
    category.name, help_page + 1, HELP_CATEGORIES.len(),
    env!("CARGO_PKG_VERSION")
);
```

**Files to Modify:**
| File | Changes |
|------|---------|
| src/main.rs | Add version to startup messages |
| src/commands/mod.rs | Add VERSION/VER command handler |
| src/ui/pages/help.rs | Add version to help page title |
| src/ui/pages/help_content.rs | Document VERSION command |

---

### Priority Order

1. **Version Display** - COMPLETE
2. **Line Duplicate Push** - COMPLETE
3. **Script Undo/Redo** - Most complex, requires careful state tracking

---

## NR and ER Operators [Medium Complexity]

Teletype-style rhythm generators that return 0 or 1 for use in conditionals.

### Operator Definitions

**ER fill length step** - Euclidean Rhythm
- `fill`: Number of beats to distribute (1-32)
- `length`: Length of the pattern (1-32)
- `step`: Current step index (any value, will be modulo'd)
- Returns: 0 or 1

**NR prime mask factor step** - Numeric Repeater
- `prime`: Prime pattern index (0-31)
- `mask`: Mask variation (0-3)
- `factor`: Variation factor (0-16)
- `step`: Current step (0-15)
- Returns: 0 or 1

### Usage Pattern
```
IF NE ER 13 16 A 0: TR    ; Trigger if euclidean step is active
IF ER 5 8 I: N ON 60      ; In a loop, play note on euclidean hits
```

### Implementation

**1. Euclidean Algorithm (src/eval/rhythm.rs - new file)**
```rust
/// Compute euclidean rhythm - returns true if step `i` should trigger
/// for a pattern with `fill` beats distributed over `length` steps
pub fn euclidean(fill: i16, length: i16, step: i16) -> i16 {
    if length <= 0 || fill <= 0 {
        return 0;
    }
    let fill = fill.min(length) as i32;
    let length = length as i32;
    let step = ((step as i32) % length + length) % length; // Handle negative steps

    // Bjorklund's algorithm check: ((step * fill) % length) < fill
    if (step * fill) % length < fill { 1 } else { 0 }
}
```

**2. NR Prime Patterns (precomputed lookup table)**
```rust
/// 32 prime rhythms (16-bit patterns)
const NR_PRIMES: [u16; 32] = [
    0x8888, 0xAAAA, 0x9249, 0xA4A4, // etc - derived from Teletype
    // ... fill with actual prime patterns
];

pub fn numeric_repeater(prime: i16, mask: i16, factor: i16, step: i16) -> i16 {
    let prime_idx = (prime as usize).clamp(0, 31);
    let pattern = NR_PRIMES[prime_idx];
    let masked = match mask.clamp(0, 3) {
        0 => pattern,
        1 => pattern & 0xFF00,
        2 => pattern & 0x00FF,
        3 => pattern ^ (factor as u16 * 0x1111),
        _ => pattern,
    };
    let step_idx = ((step as usize) % 16);
    if (masked >> (15 - step_idx)) & 1 == 1 { 1 } else { 0 }
}
```

**3. Register in eval system (src/eval/logic.rs)**
```rust
"ER" => {
    // ER fill length step
    if let Some((fill, f_consumed)) = eval_expr_fn(...) {
        if let Some((length, l_consumed)) = eval_expr_fn(...) {
            if let Some((step, s_consumed)) = eval_expr_fn(...) {
                let result = rhythm::euclidean(fill, length, step);
                return Some((result, 1 + f_consumed + l_consumed + s_consumed));
            }
        }
    }
    None
}

"NR" => {
    // NR prime mask factor step
    // Similar pattern with 4 arguments
}
```

**4. Add to validation (src/commands/validate.rs)**
- ER requires 3 arguments
- NR requires 4 arguments

### Files to Modify/Create
| File | Changes |
|------|---------|
| src/eval/rhythm.rs | NEW - euclidean() and numeric_repeater() functions |
| src/eval/mod.rs | Add `pub mod rhythm;` |
| src/eval/logic.rs | Add ER and NR match arms |
| src/commands/validate.rs | Add ER/NR validation |
| src/ui/pages/help_content.rs | Document ER and NR |

### Complexity Assessment
- **ER**: Low - simple mathematical formula
- **NR**: Medium - need to research/port prime patterns from Teletype source

### Research Needed
- Exact NR prime patterns from Teletype firmware
- Verify ER algorithm matches Teletype behavior

Sources:
- [Teletype Euclidean Rhythm Discussion](https://llllllll.co/t/a-euclidean-rhythm-operator-for-the-teletype/2344)
- [Sam Doshi's Teletype Euclidean Tutorial](https://samdoshi.com/post/2016/03/teletype-euclidean/)

### Estimated Effort

| Feature | Complexity | Files | Est. Lines |
|---------|------------|-------|------------|
| Version Display | Low | 4 | ~20 |
| Line Duplicate Push | Low | 2 | ~15 |
| Script Undo/Redo | Medium | 4 | ~100 |

---

## Order of Operations

```
1. TOG zero bug      → COMPLETE
2. IF/ELSE scope     → COMPLETE (documented, working as designed)
3. Boolean ops       → COMPLETE (all operators verified working)
4. Loops + stateful  → COMPLETE (tested, bugs identified)
5. Nested IF bug     → COMPLETE (rfind colon splitting)
6. SEQ var assign    → COMPLETE (quote-respecting split)
7. CLI enhancements  → COMPLETE (dry-run, batch auto-start)
8. Highlight timing  → Quick audit
9. Envelope scaling  → COMPLETE (FA/DA now 0-16383)
10. Default mismatch → COMPLETE (SynthDef/RST aligned)
11. SYNC command     → COMPLETE
12. Auto-load scene  → COMPLETE
13. Validation       → COMPLETE (all 6 phases)
14. DRY audit        → Maintainability foundation
```

## Rationale

1. **Bug fixes first** - Restore trust in existing features
2. **SYNC is high-value** - Useful for live performance, contained scope
3. **Auto-load is quick** - Simple QoL improvement
4. **Validation enables future** - Catch errors early, better UX
5. **DRY audit pays dividends** - Easier maintenance long-term
