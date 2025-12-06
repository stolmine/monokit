# Build Scripts

## SynthDef Pre-compilation (Phase 1)

This directory contains scripts for pre-compiling monokit's SuperCollider SynthDefs to binary `.scsyndef` format for direct scsynth integration.

### Scripts

**compile_synthdefs.scd**
- Compiles all 3 SynthDefs to binary format
- Output: `../sc/synthdefs/*.scsyndef`
- Run: `sclang -D compile_synthdefs.scd`

**test_load_synthdefs.scd**
- Tests loading compiled SynthDefs
- Verifies instantiation and params
- Run: `sclang test_load_synthdefs.scd`

**verify_phase1.sh**
- Full verification of Phase 1
- Checks all success criteria
- Run: `./verify_phase1.sh`

### Output Files

Generated in `../sc/synthdefs/`:
- `monokit.scsyndef` (~23 KB)
- `monokit_spectrum.scsyndef` (~3 KB)
- `monokit_scope.scsyndef` (~9 KB)

### Status

Phase 1: COMPLETE
- All 3 SynthDefs compile successfully
- Files are valid and loadable
- Headless compilation works (sclang -D)
