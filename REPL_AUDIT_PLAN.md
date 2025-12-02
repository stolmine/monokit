# REPL Output Audit Plan

## Executive Summary

Audit of all REPL output in monokit codebase. Goal: tiered verbosity system with consistent messaging, 46-char compliance, and complete coverage at highest tier.

---

## Current State

### Output Infrastructure
- **Field:** `App::output: Vec<String>` (max 100 items)
- **Method:** `add_output(&mut self, msg: String)`
- **Pattern:** Commands use `output()` closure callback

### Current DEBUG Levels
| Level | Name | Behavior |
|-------|------|----------|
| 0 | SILENT | Errors only |
| 1 | IMPORTANT | Metro status, PRINT commands |
| 2 | VERBOSE | All parameter confirmations (default) |

### Statistics
- **Total output calls:** 537
- **debug_level checks:** 78 (only 15% of outputs gated)
- **Error messages:** 202+ (never suppressed)
- **46-char violations:** 21 messages

---

## Proposed Verbosity System

### Global Tier (DEBUG 0-5)
Sets baseline verbosity. Each tier includes all lower tiers.

| Tier | Name | Content |
|------|------|---------|
| **0** | SILENT | Nothing (use category overrides) |
| **1** | ERRORS | All errors |
| **2** | ESSENTIAL | + State changes (scene, metro, rec) |
| **3** | QUERIES | + Value reads |
| **4** | CONFIRMS | + Set confirmations |
| **5** | VERBOSE | + Detailed feedback |

### Category Overrides (Independent Toggles)
Enable specific categories regardless of tier level.

| Command | Category | Effect |
|---------|----------|--------|
| `OUT.ERR <0\|1>` | Errors | Show all error messages |
| `OUT.ESS <0\|1>` | Essential | Show state change messages |
| `OUT.QRY <0\|1>` | Queries | Show value read responses |
| `OUT.CFM <0\|1>` | Confirms | Show set confirmations |

### How They Interact

**Tier sets baseline, overrides add categories:**
```
DEBUG 0              → Silent (nothing)
DEBUG 0; OUT.ERR 1   → Errors only
DEBUG 0; OUT.QRY 1   → Queries only (no errors!)
DEBUG 2; OUT.CFM 1   → Essential + Confirms (skip queries)
DEBUG 5              → Everything (overrides ignored)
```

**Logic:** `show = (tier >= category_tier) OR (category_override == 1)`

### Example Configurations

**Minimal feedback (errors + state changes):**
```
DEBUG 2
```

**Query-focused (see values, hide set confirms):**
```
DEBUG 3
```

**Terse but complete (errors + confirms, skip fluff):**
```
DEBUG 1; OUT.CFM 1
```

**Performance mode (silent except errors):**
```
DEBUG 1
```

**Debug everything:**
```
DEBUG 5
```

### Message Examples by Category

**ERRORS (Tier 1):**
```
ERROR: DIVISION BY ZERO
ERROR: PF REQUIRES A VALUE
ERROR: PATTERN INDEX OUT OF BOUNDS
```

**ESSENTIAL (Tier 2):**
```
LOADED SCENE: drums
METRO ACTIVATED
RECORDING STARTED
PRESET LOADED: 808-kick
```

**QUERIES (Tier 3):**
```
A = 127
P.HERE = 64
M = 125MS
FC = 2000
```

**CONFIRMS (Tier 4):**
```
SET A TO 127
SET PF TO 440
ADDED 10 TO PATTERN 0
DELAYED: TR
```

**VERBOSE (Tier 5 only):**
```
RANDOMIZED VOICE
SLEW PF OVER 200MS
SET SCALE ROOT TO C
FLASH HOLD: 200MS
```

---

## Output Categorization

### TIER 0: SILENT (Critical Errors Only)
| Source | Message Pattern | Current Char |
|--------|-----------------|--------------|
| Math | `ERROR: DIVISION BY ZERO` | 24 |
| Math | `ERROR: MODULO BY ZERO` | 22 |
| Scene | `ERROR: SCENE '{name}' NOT FOUND` | 28+ |
| OSC | `ERROR: OSC CONNECTION FAILED` | 28 |

### TIER 1: ERRORS (All Errors)
| Source | Message Pattern | Current Char | Compliant |
|--------|-----------------|--------------|-----------|
| Variables | `ERROR: {VAR} REQUIRES SCRIPT CONTEXT` | 38 | ✓ |
| Validate | `ERROR: {CMD} REQUIRES {N} ARGUMENTS` | ~35 | ✓ |
| Patterns | `ERROR: PATTERN INDEX OUT OF BOUNDS` | 36 | ✓ |
| Scale | `ERROR: Q.SCALE MUST BE 0-11` | 28 | ✓ |
| Volume | `ERROR: VOL MUST BE 0.0-1.0` | 28 | ✓ |
| Patterns | `ERROR: PN.ROT REQUIRES 2 ARGS` | 30 | ✓ |

**Violations to fix:**
| Message | Current | Fix |
|---------|---------|-----|
| `PN.ROT requires at least 2 arguments (pattern number and rotation amount)` | 75 | `ERROR: PN.ROT NEEDS PAT AND AMT` |
| `ERROR: Q.SCALE PRESET MUST BE BETWEEN 0 AND 11` | 47 | `ERROR: Q.SCALE MUST BE 0-11` |
| `ERROR: Q.BIT REQUIRES A VALID BINARY STRING` | 45 | `ERROR: Q.BIT NEEDS BINARY STR` |

### TIER 2: ESSENTIAL (State Changes)
| Source | Message Pattern | Char |
|--------|-----------------|------|
| Scene | `SAVED SCENE: {name}` | 14+ |
| Scene | `LOADED SCENE: {name}` | 15+ |
| Scene | `DELETED SCENE: {name}` | 16+ |
| Metro | `METRO ACTIVATED` | 15 |
| Metro | `METRO DEACTIVATED` | 17 |
| Metro | `SET METRO TO {bpm} BPM` | 22 |
| Record | `RECORDING STARTED` | 17 |
| Record | `RECORDING STOPPED` | 17 |
| Preset | `LOADED PRESET: {name}` | 16+ |
| Reset | `RESET TO DEFAULTS` | 17 |
| DEL | `DELAY QUEUE CLEARED` | 19 |

### TIER 3: QUERIES (Value Reads)
| Source | Message Pattern | Char |
|--------|-----------------|------|
| Variables | `A = {value}` | 8-12 |
| Variables | `B = {value}` | 8-12 |
| Counters | `N1 = {value}` | 9-13 |
| Patterns | `P.N = {0-5}` | 10 |
| Patterns | `P.L = {1-64}` | 10 |
| Patterns | `P.I = {0-63}` | 10 |
| Patterns | `P.HERE = {value}` | 14 |
| Patterns | `PN.MIN = {value}` | 14 |
| Metro | `M = {ms}MS` | 10-15 |
| Synth | `{PARAM} = {value}` | varies |
| Config | `DEBUG LEVEL: {0-2}` | 16 |
| Config | `HEADER LEVEL: {0-4}` | 18 |

### TIER 4: CONFIRMATIONS (Value Sets)
| Source | Message Pattern | Char |
|--------|-----------------|------|
| Variables | `SET A TO {value}` | 14-18 |
| Counters | `N1.MAX SET TO {val}` | 18-22 |
| Counters | `N1.RST TO {min}` | 14-18 |
| Patterns | `SET P.N TO {0-5}` | 16 |
| Patterns | `SET P {idx} TO {val}` | 18-22 |
| Patterns | `ADDED {val} TO PAT {n}` | 22 |
| Synth | `SET PF TO {hz}` | 14-20 |
| Synth | `SET FC TO {hz}` | 14-20 |
| Effects | `SET DT TO {ms}MS` | 16-20 |
| DEL | `DELAYED: {command}` | 11+ |
| Note | `NOTE ADDED TO LINE {n}` | 22 |
| Note | `NOTES CLEARED` | 13 |
| PRINT | `{literal or result}` | varies |

### TIER 5: VERBOSE (Detailed Feedback)
| Source | Message Pattern | Char |
|--------|-----------------|------|
| Randomize | `RANDOMIZED VOICE` | 16 |
| Randomize | `RANDOMIZED OSCILLATORS` | 22 |
| Randomize | `RANDOMIZED FM` | 13 |
| Randomize | `RANDOMIZED FILTERS` | 18 |
| Slew | `SLEW {PARAM} OVER {ms}MS` | 22-28 |
| Scale | `SET SCALE ROOT TO {note}` | 24 |
| Scale | `SET SCALE TO {name}` | 20 |
| Config | `FLASH HOLD: {ms}MS` | 18 |
| Config | `SCOPE.TIME: {ms}MS` | 18 |
| Config | `SPECTRUM: ON` | 12 |
| Config | `ACTIVITY: OFF` | 13 |
| Theme | `SWITCHED TO {name} THEME` | 24 |

---

## Issues Found

### 1. Inconsistent Debug Gating
| Category | Gated? | Should Be |
|----------|--------|-----------|
| Errors | Never | Tier 0-1 |
| Pattern queries | Never | Tier 3 |
| Pattern manipulation | Never | Tier 4 |
| Synth params | >= 2 | Tier 4 |
| Metro status | >= 1 | Tier 2 |
| Randomization | >= 2 | Tier 5 |

**Pattern commands output at all levels** - should respect tiers.

### 2. Character Limit Violations (21 total)
| File | Message | Chars |
|------|---------|-------|
| validate.rs | Various pattern arg errors | 47-75 |
| scale.rs | Q.SCALE bounds error | 47 |
| scale.rs | Q.BIT validation error | 45 |
| patterns/common.rs | Manipulation messages | 45-60 |

### 3. Inconsistent Output Channels
| Channel | Used For | Issue |
|---------|----------|-------|
| `output()` | REPL display | Correct |
| `eprintln!()` | Meter/MIDI errors | NOT visible in REPL |

Meter and MIDI errors go to stderr, invisible to user.

### 4. Format Inconsistencies
| Pattern | Examples | Recommendation |
|---------|----------|----------------|
| `ERROR: X` | Most errors | Keep |
| `SET X TO Y` | Synth params | Keep |
| `X = Y` | Queries | Keep |
| `SAVED SCENE: X` | Scene ops | Keep |
| `{result}` | Math ops (no context) | Add context? |

Math operations (ADD, SUB, etc.) just output the number with no context.

---

## Implementation Plan

### Phase 1: Define Tier Constants and Output Categories
```rust
// src/types.rs
pub const TIER_SILENT: u8 = 0;      // Nothing
pub const TIER_ERRORS: u8 = 1;      // All errors
pub const TIER_ESSENTIAL: u8 = 2;   // State changes
pub const TIER_QUERIES: u8 = 3;     // Value reads
pub const TIER_CONFIRMS: u8 = 4;    // Value sets
pub const TIER_VERBOSE: u8 = 5;     // Everything

#[derive(Clone, Copy, PartialEq)]
pub enum OutputCategory {
    Error,      // Tier 1
    Essential,  // Tier 2
    Query,      // Tier 3
    Confirm,    // Tier 4
    Verbose,    // Tier 5
}
```

### Phase 2: Add Category Overrides to Config
```rust
// src/config.rs - DisplayConfig
pub out_err: bool,   // Override: show errors
pub out_ess: bool,   // Override: show essential
pub out_qry: bool,   // Override: show queries
pub out_cfm: bool,   // Override: show confirms

// Defaults: all false (tier controls everything)
```

### Phase 3: Create Output Helper
```rust
// src/app/mod.rs
impl App {
    pub fn should_output(&self, category: OutputCategory) -> bool {
        let tier = match category {
            OutputCategory::Error => TIER_ERRORS,
            OutputCategory::Essential => TIER_ESSENTIAL,
            OutputCategory::Query => TIER_QUERIES,
            OutputCategory::Confirm => TIER_CONFIRMS,
            OutputCategory::Verbose => TIER_VERBOSE,
        };

        // Tier check OR category override
        self.debug_level >= tier || match category {
            OutputCategory::Error => self.out_err,
            OutputCategory::Essential => self.out_ess,
            OutputCategory::Query => self.out_qry,
            OutputCategory::Confirm => self.out_cfm,
            OutputCategory::Verbose => false, // No override
        }
    }
}
```

### Phase 4: Update DEBUG Command
- Change range from 0-2 to 0-5
- Update help text with tier descriptions
- Keep backward compatibility (map old 0→1, 1→2, 2→4)

### Phase 5: Add Category Override Commands
```
OUT.ERR <0|1>  - Toggle error output override
OUT.ESS <0|1>  - Toggle essential output override
OUT.QRY <0|1>  - Toggle query output override
OUT.CFM <0|1>  - Toggle confirm output override
```
All persist to config.toml.

### Phase 6: Fix Character Violations
- Audit and rewrite 21 messages exceeding 46 chars
- Use abbreviations: DEF, ARG, PAT, VAL, REQ, etc.

### Phase 7: Standardize Output Gating
- Add category tags to all output calls
- Update pattern commands to use `should_output()`
- Update math operations to use `should_output()`
- Route meter/MIDI errors through REPL with Error category

### Phase 8: Message Consistency Pass
- Standardize error prefix: `ERROR: `
- Standardize query format: `{NAME} = {VALUE}`
- Standardize set format: `SET {NAME} TO {VALUE}`
- Standardize action format: `{VERB}ED {OBJECT}`

### Phase 9: Add Missing Coverage
- Math operation context: `ADD: 150` instead of `150`
- Expression errors in scripts
- Thread communication failures

---

## Migration Strategy

### Backward Compatibility Mapping
| Old Level | Old Name | New Level | New Name |
|-----------|----------|-----------|----------|
| 0 | SILENT | 1 | ERRORS |
| 1 | IMPORTANT | 2 | ESSENTIAL |
| 2 | VERBOSE | 4 | CONFIRMS |

New levels 0 (SILENT), 3 (QUERIES), 5 (VERBOSE) are additions.

### Config Migration
On startup, if `debug_level <= 2`, map to new values automatically.

---

## Testing Checklist

- [ ] Each tier shows only appropriate messages
- [ ] All error messages ≤ 46 chars
- [ ] Pattern commands respect tier settings
- [ ] Math commands show context at appropriate tier
- [ ] Meter/MIDI errors visible in REPL
- [ ] Query responses consistent format
- [ ] Set confirmations consistent format
- [ ] Backward compatibility preserved
- [ ] Config persistence works with new tiers

---

## Files to Modify

| File | Changes |
|------|---------|
| `src/types.rs` | Add tier constants, OutputCategory enum |
| `src/app/mod.rs` | Add should_output(), category override fields |
| `src/config.rs` | Add out_err/ess/qry/cfm fields, migration |
| `src/commands/system/misc.rs` | DEBUG 0-5, OUT.* commands |
| `src/commands/patterns/*.rs` | Add category tags to outputs |
| `src/commands/core/math_ops.rs` | Add output context, category tags |
| `src/commands/core/variables.rs` | Add Query/Confirm categories |
| `src/commands/synth/*.rs` | Add category tags |
| `src/commands/validate.rs` | Fix long messages |
| `src/commands/core/scale.rs` | Fix long messages |
| `src/meter.rs` | Route errors to REPL via callback |
| `src/midi.rs` | Route errors to REPL via callback |
| `src/ui/pages/help_content.rs` | Document tiers + OUT.* commands |

## New Commands Summary

| Command | Description | Persists |
|---------|-------------|----------|
| `DEBUG <0-5>` | Set global verbosity tier | ✓ |
| `OUT.ERR <0\|1>` | Override: show all errors | ✓ |
| `OUT.ESS <0\|1>` | Override: show essential msgs | ✓ |
| `OUT.QRY <0\|1>` | Override: show query responses | ✓ |
| `OUT.CFM <0\|1>` | Override: show confirmations | ✓ |
