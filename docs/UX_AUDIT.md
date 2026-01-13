# UX Audit Report

Comprehensive audit of monokit's user experience: error messages, help system, and behavioral consistency.

---

## Executive Summary

| Area | Grade | Critical Issues |
|------|-------|-----------------|
| Error Messages | B- | REC commands silent, 40% missing ERROR prefix |
| Help System | C+ | 50+ commands missing, no cross-references |
| Behavioral Consistency | C | No query standard, toggle syntax varies |

---

## 1. Error Messages & Feedback

### 1.1 Silent Commands (CRITICAL)

These commands give no feedback on success:

| Command | Issue | Fix |
|---------|-------|-----|
| `REC` | No confirmation recording started | Add "RECORDING STARTED" |
| `REC.STOP` | No confirmation recording stopped | Add "RECORDING STOPPED: path" |

User has no idea if recording worked until audio appears in UI.

### 1.2 Missing ERROR Prefix (~40% of errors)

**Inconsistent:**
```
Q.ROOT REQUIRES A VALUE           # Missing "ERROR:"
RND REQUIRES A MAX VALUE          # Missing "ERROR:"
TOG REQUIRES AT LEAST TWO VALUES  # Missing "ERROR:"
```

**Correct:**
```
ERROR: SAVE REQUIRES A SCENE NAME
ERROR: SCENE 'foo' NOT FOUND
ERROR: PATTERN NUMBER MUST BE 0-5
```

### 1.3 Range Error Format Inconsistency

| Style | Example | Clarity |
|-------|---------|---------|
| Descriptive | "MUST BE 0-5" | Good |
| Terse | "RANGE 0-16383" | Poor - no context |
| With unit | "RANGE 20-20000 HZ" | Good |

**Recommendation:** Standardize to "CMD: MUST BE X-Y [UNIT]"

### 1.4 Debug Format Leaks

Some errors expose Rust debug format:
```
FAILED TO LOAD CONFIG: {:?}  # User sees Debug representation
ERROR: {:?}                   # Unhelpful
```

### 1.5 Good Examples to Follow

```rust
"ERROR: VOL REQUIRES A VALUE (0.0-1.0)"     // Shows range
"ERROR: CANNOT DELETE FACTORY PRESET 'x'"   // Explains WHY
"ERROR: TITLE TAKES 0 (MONOKIT) OR 1 (SCENE)" // Shows options
```

---

## 2. Help System Discoverability

### 2.1 Commands Missing from Help (~50+)

**Aliases not documented:**
- Beat repeat: `BRL, BRR, BRW, BRX`
- Clouds: `CLDS, CLF, CLFZ, CLG, CLLO, CLM, CLPS, CLPT, CLRV, CLSP, CLSZ, CLTR, CLTX, CLW`
- Mute shortcuts: `MN, MO, MPL, MS`
- Sampler: `SFCM, SFQM, KL`
- EQ: `EQ.HF, EQ.LF`

**Commands not documented:**
- `AENV.GATE, PENV.GATE, DENV.GATE, FBEV.GATE, FLEV.GATE, FMEV.GATE`
- `AUDIO` (base command)
- `GATE`
- `MIDI` (base command)

**Impact:** User types alias → "UNKNOWN COMMAND" → no guidance to canonical form.

### 2.2 Organization Issues

**Related commands scattered:**
- Compressor params in FILTER & EFFECTS
- `SYNC.PAT` in CONTROL FLOW (not PATTERNS)
- Pattern randomization (`RND.P`) in VARIABLES (not PATTERNS)

**Missing cross-references:**
- `MODF.CUT` doesn't mention `MBUS.AMT`
- Filter section doesn't link to modulation section
- No "See Also" notes anywhere

### 2.3 Missing Learning Content

| Missing | Impact |
|---------|--------|
| "Getting Started" section | New users don't know where to begin |
| Signal flow diagram | Users can't visualize audio path |
| Common workflows | No pattern-based teaching |
| Troubleshooting | "I set PF but hear nothing" - no help |
| Keyboard quick reference | Keys scattered across pages |

### 2.4 Inconsistent Examples

**Well-documented:**
```
MAP <V> <I1> <I2> <O1> <O2>
MAP VAL IN RANGE -> OUT
EX: PF MAP A 0 127 200 2000
```

**Under-documented:**
```
SLEW <P> <MS>
PER-PARAM SLEW MS
(No example)
```

**Missing entirely:**
- SRINGS commands have no usage examples
- Many sampler commands lack examples

### 2.5 No Search/Index

- No command index page
- No alphabetical listing
- Must scroll through all 15 pages to find a command
- `CTRL+F` searches visible text only

---

## 3. Behavioral Consistency

### 3.1 Query Syntax (CRITICAL)

**No standard for "give me current value":**

| Command | No Args Behavior |
|---------|------------------|
| `A` | Returns "A = 100" ✓ |
| `M` | Returns interval ✓ |
| `M.SYNC` | Returns mode ✓ |
| `MUTE` | Lists all states ✓ |
| `LOAD.RST` | Returns mode ✓ |
| `FC` | ERROR ✗ |
| `M.BPM` | ERROR ✗ |
| `Q.ROOT` | ERROR ✗ |
| `SCOPE.TIME` | ERROR ✗ |

**User expectation:** If `M` returns interval, `FC` should return cutoff.

### 3.2 Toggle Syntax Inconsistency

| Command | Required? | Toggle? | Query? |
|---------|-----------|---------|--------|
| `M.ACT` | Yes | No | No |
| `MUTE` | No | Yes | Yes |
| `CPU` | No | Yes | No |
| `BPM` | No | Yes | No |
| `VCA` | Yes | No | No |
| `M.SYNC` | Yes | No | Yes |

**User expectation:** All boolean commands should behave the same.

### 3.3 Direct Forms (MUTE only)

```
MUTE.1    # Works - toggles script 1
MUTE.M    # Works - toggles metro
MUTE.I    # Works - toggles init
```

No other command has this pattern:
```
M.ACT.1   # Doesn't exist
DEBUG.1   # Doesn't exist
```

**User confusion:** Why only MUTE has direct forms?

### 3.4 Silent Side Effects

**LOAD.RST modifies LOAD behavior invisibly:**

```
LOAD.RST 1        # Set flag (no feedback about what this does)
LOAD mysound      # Silently runs RST before loading!
```

User doesn't see that LOAD will reset all parameters.

### 3.5 Return Value Format Inconsistency

| Command | Returns |
|---------|---------|
| `A` | "A = 100" (formatted) |
| `M` | "METRO INTERVAL: 500MS" (formatted) |
| `MUTE` | Multi-line state list |
| `P.HERE` | Raw value (usable in expression) |

No consistent pattern.

### 3.6 Boolean Feedback Inconsistency

| Command | Off | On |
|---------|-----|-----|
| `M.ACT 0` | "METRO DEACTIVATED" | "METRO ACTIVATED" |
| `CPU 0` | (silent) | (silent) |
| `BPM 0` | (silent) | (silent) |
| `MUTE 1 0` | "SCRIPT 1: ACTIVE" | "SCRIPT 1: MUTED" |

---

## 4. Priority Fixes

### P0 - Critical (User Confusion)

1. **Add REC/REC.STOP feedback** - Users can't tell if recording started
2. **Standardize query support** - Either all params support query or document which do
3. **Document LOAD.RST side effect** - Warn that LOAD behavior changes

### P1 - High (Discoverability)

4. **Add missing aliases to help** - 50+ commands undocumented
5. **Add ERROR prefix consistently** - 40% of errors missing it
6. **Add cross-references in help** - Related commands should link

### P2 - Medium (Polish)

7. **Standardize toggle behavior** - All booleans should work same way
8. **Add examples to all commands** - Many lack usage examples
9. **Create "Getting Started" section** - New user onboarding
10. **Add command index page** - Quick reference

### P3 - Low (Nice to Have)

11. **Add signal flow diagram** - Visual learning aid
12. **Standardize error format** - "CMD: MUST BE X-Y [UNIT]"
13. **Add troubleshooting section** - Common problems
14. **Fix debug format leaks** - Replace `{:?}` with user messages

---

## 5. Proposed Standards

### 5.1 Error Message Format

```
ERROR: <COMMAND>: <DESCRIPTION>
ERROR: FC: MUST BE 20-20000 HZ
ERROR: MUTE: SCRIPT NUMBER MUST BE 1-8
```

### 5.2 Query Behavior

**Option A (Recommended):** All parameters support query
```
FC          # Returns "FC: 1000 HZ"
AD          # Returns "AD: 200 MS"
```

**Option B:** No parameters support query (current for most)
- Document clearly that query is not supported
- Only variables (A, B, C...) support query

### 5.3 Toggle Behavior

All boolean parameters should accept:
```
CMD         # Query current state (if query supported)
CMD 0       # Set to off
CMD 1       # Set to on
```

Remove toggle-without-value for consistency.

### 5.4 Confirmation Messages

All state-changing commands should confirm:
```
REC         # "RECORDING STARTED"
REC.STOP    # "RECORDING STOPPED: /path/to/file.wav"
SAVE foo    # "SAVED SCENE: FOO"
LOAD foo    # "LOADED SCENE: FOO" (+ note if RST ran)
```

---

## 6. Files to Modify

### Error Messages
- `src/commands/system/recording.rs` - Add REC feedback
- `src/commands/core/scale.rs` - Add ERROR prefix
- `src/commands/core/random_ops.rs` - Add ERROR prefix
- `src/commands/synth/mixer.rs` - Improve range errors

### Help System
- `src/ui/pages/help_content.rs` - Add missing commands
- `src/ui/pages/help_effects.rs` - Add Clouds aliases
- `src/ui/pages/help_synth.rs` - Add sampler aliases
- `src/ui/pages/help_system.rs` - Add cross-references

### Behavioral Consistency
- `src/commands/synth/*.rs` - Consider adding query support
- `src/commands/system/config.rs` - Standardize toggle behavior
- `src/commands/system/scene.rs` - Add LOAD.RST warning to LOAD output
