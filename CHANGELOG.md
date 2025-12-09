# Changelog

## v0.3.1 (December 2025)

### Bug Fixes

**Beat Repeat**
- Fixed beat repeat not working in bundle mode
- Synced SynthDef between compile_synthdefs.scd and monokit_server.scd
- Beat repeat activation now derives from br_mix > 0

**Config Command Output**
- All config/query commands now output regardless of DEBUG level
- Fixed: THEME, HEADER, CPU, GRID, METER.*, SCOPE.*, OUT.*, SCRMBL.*
- Fixed: LOAD.RST, LOAD.CLR, AUTOLOAD, VCA, LIMIT
- Fixed: TITLE, TITLE.TIMER, BPM, SCENES, PSETS
- Fixed: REC, REC.STOP, REC.PATH recording commands
- Fixed: SAVE, LOAD, DELETE scene commands

---

## v0.3.0 (December 2025)

### New Features

**SYNC Command**
- `SYNC` resets all stateful elements to starting position
- SEQ sequences reset to first element
- TOG toggles reset to first value
- EITH and inline random choice states cleared
- EV/SKIP counters reset to 0
- Pattern indices (P.I) reset to 0
- N1-N4 counters reset to MIN values
- Partial variants: `SYNC.SEQ`, `SYNC.TOG`, `SYNC.PAT`

**AUTOLOAD Setting**
- `AUTOLOAD 0/1` enables auto-load of last scene on startup
- Last loaded scene tracked in config.toml
- Missing scenes handled gracefully

**Script Validation Overhaul**
- Invalid commands now rejected at script entry time, before save
- Paste validation added (previously bypassed validation)
- Expression syntax validation without evaluation
- Control flow validation for loops, conditionals, DEL commands
- SEQ pattern content validation (bracket balancing, valid tokens)
- Reference range validation (patterns 0-5, scripts 1-8/M/I)
- Extra tokens after expressions now rejected
- Clear error messages for all validation failures

**Pattern Operations as Expressions**
- 32 pattern operations now return values when used in expressions
- P.PUSH, P.POP, P.REV, P.SHUF, P.SORT, P.ROT with return values
- P.ADD, P.SUB, P.MUL, P.DIV, P.MOD, P.SCALE with return values
- P.INS, P.RM, P.CLR, P.RND with return values
- All PN.* variants support pattern number as first argument
- Enables Teletype-style patterns like `A P.PUSH 42`

### Bug Fixes

**Envelope Parameter Scaling**
- FA (FM envelope amount) now accepts 0-16383 range (was 0-16)
- DA (Disc envelope amount) now accepts 0-16383 range (was 0-16)
- Root cause: SynthDef divided by 16383 but Rust limited input range

**SynthDef/RST Default Alignment**
- RST command now matches SynthDef default values
- pf: 200 → 131 (C3)
- mf: 50 → 262 (C4)
- pa: 4 → 0 (no pitch envelope by default)
- fc: 1000 → 10000 (filter wide open)
- cr: 4 → 1 (compressor off by default)

**Nested IF in Loops**
- Fixed `L 1 6: IF GT I 2: IF LT I 5: PRINT I` giving "UNKNOWN COMMAND: IF"
- Changed colon splitting from find(':') to rfind(':')
- Added recursive nested conditional handling

**SEQ Quote Parsing**
- Fixed `A SEQ "A B C D"` giving "FAILED TO PARSE VALUE"
- Added quote-respecting whitespace splitting
- Variable assignment now handles quoted strings correctly

**TOG Zero Parsing**
- Fixed display corruption when TOG uses zero as argument
- Key format mismatch between state storage and highlighting resolved

**Beat Repeat Activation**
- Removed BR.ACT command
- Beat repeat now activates automatically when BR.MIX > 0
- Fixes stickiness bug where BR wouldn't turn off reliably

### CLI Enhancements

- `--dry-run --run <scene>` runs without SuperCollider/audio
- Enables headless testing of command logic
- Batch mode auto-starts metro after loading scene

### Test Coverage

- 9 new validation test scenes
- 583 unit tests passing
- Comprehensive coverage for pattern operations, expressions, control flow

---

## v0.2.0 (December 2025)

- scsynth-direct integration (bundled binary distribution)
- Recording via DiskOut UGen
- Audio device handling improvements
- sc3-plugins extraction path fix

## v0.1.1 (December 2025)

- Initial Homebrew tap release
- Bundle signing and distribution fixes

## v0.1.0 (December 2025)

- Initial release
- Core voice with 77 parameters
- Pattern system (6 patterns x 64 steps)
- SEQ mini-notation
- Scale quantization
- MIDI clock sync
- Preset system
- Full effects chain
