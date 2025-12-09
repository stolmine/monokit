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

**9. SYNC Command [Medium]**
- `SYNC` resets all stateful elements to starting position:
  - SEQ sequences → first element
  - TOG toggles → first value
  - EITH/`<>`/`{}` states cleared
  - EV/SKIP counters → 0
  - Pattern indices (P.I) → 0
  - N1-N4 counters → MIN values
- Consider partial variants: `SYNC.SEQ`, `SYNC.TOG`, `SYNC.PAT`
- Use cases: live performance resets, song structure sync points

**10. Auto-Load Previous Scene [Low]**
- `AUTOLOAD <0|1>` - Enable/disable auto-load on startup
- Track last loaded scene name in config.toml
- On startup, if enabled, run LOAD with saved scene name
- Handle missing scene gracefully

---

### Phase 3: Foundation Work (2-4 weeks)

**11. Script Validation Overhaul [High]**
- Reject invalid commands on script line entry (before save)
- Validate all argument counts and types
- Validate expression syntax (balanced parens, valid operators)
- Validate SEQ notation syntax (quotes, brackets, valid tokens)
- Validate pattern references (0-5 range)
- Validate script references (1-8, M, I)
- Clear, specific error messages for each failure type

**12. File Size/DRY Audit [Medium]**
- Ensure all files within agent readable limits (~500 lines)
- Create `CommandContext` struct to bundle shared parameters
- Create category-aware output helper
- Audit synth param macros for shared patterns

---

### Deferred Items

- Script undo/redo [Medium] - Nice but not critical
- Dynamic grid layout [Medium] - Polish item
- Line duplicate push behavior [Low] - Minor UX improvement
- NR/ER operators [Medium] - Nice-to-have Teletype features
- DSP bugs (FMEV envelope) - Needs deep SynthDef investigation

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
9. SYNC command      → High value, medium effort
10. Auto-load scene  → Quick win
11. Validation       → Larger investment, enables future work
12. DRY audit        → Maintainability foundation
```

## Rationale

1. **Bug fixes first** - Restore trust in existing features
2. **SYNC is high-value** - Useful for live performance, contained scope
3. **Auto-load is quick** - Simple QoL improvement
4. **Validation enables future** - Catch errors early, better UX
5. **DRY audit pays dividends** - Easier maintenance long-term
