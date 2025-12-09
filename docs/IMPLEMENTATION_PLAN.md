# Implementation Plan

## Current Focus (December 2025)

### Phase 1: Bug Fixes (1-2 days)

**1. TOG Zero Parsing Bug [Low]**
- Issue: `DC TOG 2000 0` displays as `DC TOG 2000 000 0` on script line
- Input field shows correct version but script display adds extra zeros
- Investigate TOG state serialization/display logic
- Fix rendering to match input exactly

**2. IF/ELSE/ELIF Scope Logic [Medium]**
- Investigate: Does ELSE cut off downstream logic processing?
- Define clear scope for IF/ELIF/ELSE pairs (line-local vs script-local)
- Test multi-line conditional chains
- Test conditionals with semicolons
- Document expected behavior in help system

**3. State Highlight Timing Verification [Low]**
- Audit SEQ, TOG, EITH, `<>`, `{}` highlight timing
- Verify highlights show current state vs previous state
- Test in metro context and nested contexts
- Fix any timing inconsistencies found

---

### Phase 2: High-Value Features (1-2 weeks)

**4. SYNC Command [Medium]**
- `SYNC` resets all stateful elements to starting position:
  - SEQ sequences → first element
  - TOG toggles → first value
  - EITH/`<>`/`{}` states cleared
  - EV/SKIP counters → 0
  - Pattern indices (P.I) → 0
  - N1-N4 counters → MIN values
- Consider partial variants: `SYNC.SEQ`, `SYNC.TOG`, `SYNC.PAT`
- Use cases: live performance resets, song structure sync points

**5. Auto-Load Previous Scene [Low]**
- `AUTOLOAD <0|1>` - Enable/disable auto-load on startup
- Track last loaded scene name in config.toml
- On startup, if enabled, run LOAD with saved scene name
- Handle missing scene gracefully

---

### Phase 3: Foundation Work (2-4 weeks)

**6. Script Validation Overhaul [High]**
- Reject invalid commands on script line entry (before save)
- Validate all argument counts and types
- Validate expression syntax (balanced parens, valid operators)
- Validate SEQ notation syntax (quotes, brackets, valid tokens)
- Validate pattern references (0-5 range)
- Validate script references (1-8, M, I)
- Clear, specific error messages for each failure type

**7. File Size/DRY Audit [Medium]**
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
1. TOG zero bug      → Quick fix, stop the annoyance
2. IF/ELSE scope     → Investigate, fix or document
3. Highlight timing  → Quick audit
4. SYNC command      → High value, medium effort
5. Auto-load scene   → Quick win
6. Validation        → Larger investment, enables future work
7. DRY audit         → Maintainability foundation
```

## Rationale

1. **Bug fixes first** - Restore trust in existing features
2. **SYNC is high-value** - Useful for live performance, contained scope
3. **Auto-load is quick** - Simple QoL improvement
4. **Validation enables future** - Catch errors early, better UX
5. **DRY audit pays dividends** - Easier maintenance long-term
