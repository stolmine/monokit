# MONOKIT COMMAND OUTPUT PATTERNS - DETAILED REFERENCE

## PATTERN EXAMPLES

### NEW PATTERN: ctx.output(OutputCategory::X, message, &mut output)

**File**: src/commands/core/variables.rs
```rust
// Line 16-24
ctx.output(OutputCategory::Query, format!("{} = {}", $var_name, ctx.variables.$var_field), &mut output);
// ...
ctx.output(OutputCategory::Confirm, format!("SET {} TO {}", $var_name, value), &mut output);
```

**Characteristics**:
- Uses ExecutionContext method: ctx.output()
- Takes OutputCategory enum (Query, Confirm, Error)
- Message always generated and passed
- Debug level filtering happens inside ctx.output()
- Modern, centralized approach

---

### OLD PATTERN: if debug_level >= TIER_X { output(...) }

**File**: src/commands/core/counters.rs
```rust
// Line 19-21
if debug_level >= TIER_QUERIES || out_qry {
    output(format!("{}", current));
}

// Line 33-35
if debug_level >= TIER_CONFIRMS || out_cfm {
    output(format!("RESET {} TO {}", $counter_name, counters.min[$counter_idx]));
}

// Line 65-71
if debug_level >= TIER_ERRORS || out_err {
    output(format!(
        "ERROR: MAX ({}) MUST BE >= MIN ({})",
        value,
        counters.min[$counter_idx]
    ));
}
```

**Characteristics**:
- Direct debug_level comparison in handler
- Conditional output based on tier threshold
- Uses TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS constants
- Legacy, distributed approach
- Often combined with explicit boolean flags (out_qry, out_cfm, out_err)

---

### MAGIC NUMBER PATTERN: debug_level >= 2

**File**: src/commands/core/scale.rs
```rust
// Line 92-94
if debug_level >= 2 {
    output(format!("SET SCALE ROOT TO {}", value));
}

// Line 139-141
if debug_level >= 2 {
    output(format!("SET SCALE PRESET TO {}", preset));
}
```

**Characteristics**:
- Hardcoded numeric comparison (2)
- No semantic meaning from constant
- Problematic for maintenance
- Found in: scale.rs, gate.rs, output.rs, plaits/params.rs
- Should be replaced with named TIER constant

---

### MIXED PATTERN: Macros combining both approaches

**File**: src/commands/patterns/macros.rs
```rust
// Line 13-16 (ctx.output approach)
ctx.output($crate::types::OutputCategory::Error, 
    format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name), 
    &mut output);

// Line 33-34 (debug_level approach in helper call)
let pat = $crate::commands::patterns::common::parse_pattern_num(
    parts, 1, ctx.variables, ctx.patterns, ctx.counters, 
    ctx.scripts, ctx.script_index, ctx.scale, &mut output, 
    *ctx.debug_level, *ctx.out_err)?;
```

**Characteristics**:
- Macro-generated code uses both patterns
- Some parts modern (ctx.output)
- Some parts legacy (debug_level checks in helper functions)
- Transition state - partially migrated

---

## TIER CONSTANTS MAPPING

From `src/types/`:
```rust
TIER_ERRORS = 0      // Always show errors
TIER_QUERIES = 1     // Show query responses
TIER_CONFIRMS = 2    // Show confirmations
TIER_ESSENTIAL = 3   // Essential messages
```

### Usage by File

| File | Constants | Pattern | Lines |
|------|-----------|---------|-------|
| counters.rs | TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS | >= | 19,33,65,76,111 |
| scheduling.rs | TIER_ESSENTIAL, TIER_CONFIRMS | >= | 74,94,191,297 |
| sync.rs | TIER_CONFIRMS | >= | 29,46,63,80 |
| randomization.rs | TIER_CONFIRMS, TIER_QUERIES, TIER_ERRORS, TIER_ESSENTIAL | >= | 36,63,90,121,170,219 |
| beat_repeat.rs | TIER_CONFIRMS | >= | 57,97 |
| synth/common.rs | TIER_CONFIRMS, TIER_ERRORS, TIER_ESSENTIAL | >= | 64,129 |
| preset.rs | TIER_CONFIRMS | >= | Multiple |
| scene.rs | TIER_CONFIRMS | >= | Multiple |

---

## OUTPUT CALLBACK TYPES

### Direct Output Closure: FnMut(String)

Used in OLD pattern:
```rust
pub fn handle_n1<F>(counters: &mut Counters, debug_level: u8, out_qry: bool, mut output: F)
where
    F: FnMut(String),
{
    // ...
    if debug_level >= TIER_QUERIES || out_qry {
        output(format!("{}", current));  // Closure called directly
    }
}
```

### Context Output Method: ctx.output()

Used in NEW pattern:
```rust
pub fn handle_variable_a<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) where
    F: FnMut(String),
{
    // ...
    ctx.output(OutputCategory::Query, format!("{} = {}", "A", value), &mut output);
    // Context handles tier checking internally
}
```

---

## HANDLER SIGNATURES BY PATTERN

### NEW PATTERN (ExecutionContext)
```rust
pub fn handle_variable_a<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,      // ← Context provided
    mut output: F,
) where
    F: FnMut(String),
```

### OLD PATTERN (Individual Parameters)
```rust
pub fn handle_n1<F>(
    counters: &mut Counters,
    debug_level: u8,                 // ← Explicit debug_level
    out_qry: bool,                   // ← Explicit output flags
    mut output: F,
) where
    F: FnMut(String),
```

### SYNTH PATTERN (Metro TX)
```rust
pub fn handle_br_len<F>(
    parts: &[&str],
    metro_interval: u64,
    br_len: &mut usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,                 // ← Still explicit
    scale: &ScaleState,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
```

---

## DISPATCHER CALLS (src/commands/mod.rs)

Line 189+ shows how handlers are called. Pattern shown:

```rust
// OLD PATTERN CALL (most handlers)
counters::handle_n1(&mut counters, *debug_level, *out_qry, &mut output);

// NEW PATTERN CALL (variables, patterns, metro)
variables::handle_variable_a(&mut parts, ctx, &mut output);

// MIXED PATTERN CALLS
patterns::working::handle_p_add(&mut parts, ctx, &mut output)?;
```

---

## COMMAND FILES WITH NO OUTPUT

These 38 files execute silently without user feedback:

**Synth Parameters**:
- oscillator.rs (voice/modulation parameters)
- filter.rs (filter configuration)
- noise.rs (noise generator)
- modulation.rs (LFO/modulation setup)
- resonator.rs (resonator mode)
- source_levels.rs (individual source levels)
- discontinuity.rs (discontinuity settings)

**Effects (10 files)**:
- compressor.rs
- delay.rs
- eq.rs
- lofi.rs
- reverb.rs
- ring_mod.rs

**Envelopes (6 files)**:
- amp.rs (amplitude envelope)
- disc.rs (discontinuity envelope)
- feedback.rs (feedback envelope)
- filter.rs (filter envelope)
- fm.rs (FM envelope)
- pitch.rs (pitch envelope)

**System (3 files)**:
- audio.rs (audio configuration)
- midi.rs (MIDI setup)
- sc.rs (SuperCollider link)

**Utilities (3 files)**:
- validate.rs (validation logic)
- validate_expr.rs (expression validation)
- aliases.rs (command aliases)

**Infrastructure (2 files)**:
- context.rs (ExecutionContext definition)
- common.rs (pattern utilities)

---

## MIGRATION ROADMAP

### PHASE 1: Core Migrations (HIGH Priority - 14 uses)
1. sync.rs (8 uses) - Simple TIER_CONFIRMS pattern
2. scale.rs (6 uses) - Replace magic number with TIER constant

### PHASE 2: System Commands (HIGH Priority - 40 uses)
3. randomization.rs (25 uses) - Large, mixed tiers
4. misc.rs (24 uses) - Heavy refactor needed

### PHASE 3: Scheduling & Counting (HIGH Priority - 18 uses)
5. counters.rs (9 uses) - Nested macro definitions
6. scheduling.rs (9 uses) - Multiple handler functions

### PHASE 4: Gate & Output (HIGH Priority - 19 uses)
7. gate.rs (14 uses) - Replace magic numbers
8. synth/output.rs (5 uses) - Replace magic numbers

### PHASE 5: Synth Commands (MEDIUM Priority - 42 uses)
9. synth/common.rs (14 uses) - Macro-based parameter handlers
10. pitch_shift.rs (10 uses) - Effect parameter commands
11. beat_repeat.rs (8 uses) - Effect specific handlers
12. synth/envelopes/common.rs (4 uses) - Envelope parameters
13. plaits/params.rs (2 uses) - Magic number replacement
14. clouds.rs (1 use) - Finish migration

### PHASE 6: Patterns (MEDIUM Priority - 59 uses)
15. patterns/macros.rs (56 uses) - Refactor macro-generated code
16. patterns/common.rs (3 uses) - Helper function migration

### PHASE 7: System Finalization (HIGH Priority - 16 uses)
17. preset.rs (8 uses)
18. scene.rs (8 uses)

### PHASE 8: Slew & Edge Cases (LOW Priority - 4 uses)
19. slew.rs (4 uses)

---

## VALIDATION CHECKLIST FOR MIGRATION

When migrating a command from OLD to NEW pattern:

- [ ] Update function signature to accept `ctx: &mut ExecutionContext`
- [ ] Remove explicit `debug_level: u8` parameter
- [ ] Remove explicit `out_err`, `out_qry`, `out_cfm`, `out_ess` parameters
- [ ] Replace `if debug_level >= TIER_X` with `ctx.output(OutputCategory::X, ...)`
- [ ] Update dispatcher call in mod.rs to pass `ctx` instead of individual fields
- [ ] Add `ctx.` prefix to output calls
- [ ] Specify OutputCategory: Error, Query, Confirm, or Essential
- [ ] Ensure message is formatted before ctx.output call
- [ ] Test that debug levels still work correctly
- [ ] Remove now-unused TIER_ imports (if file-specific)

