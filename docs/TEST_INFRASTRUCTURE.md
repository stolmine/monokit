# DRY Test Infrastructure with TestContext

## Location
`/Users/why/repos/monokit/src/tests/common.rs`

## What It Does
`TestContext` is a struct that encapsulates ALL parameters needed by `process_command()`. Instead of manually setting up 20+ parameters in every test, tests now use:

```rust
let mut ctx = TestContext::new();
ctx.run("SOME_COMMAND").unwrap();
assert_eq!(ctx.outputs[0], "EXPECTED OUTPUT");
```

## Key Components

### TestContext struct contains:
- All channel/state parameters (metro_tx, metro_interval, sync_mode, etc.)
- All data stores (variables, patterns, counters, scripts, scale)
- All UI state (theme, debug_level, show_cpu, limiter_enabled, notes)
- Output collection (`outputs: Vec<String>`)

### Methods:
- `new()` - Creates context with sensible defaults (debug_level=0)
- `run(&mut self, input: &str)` - Executes command via `process_command()`
- `clear_outputs(&mut self)` - Clears output vector between commands
- `with_debug_level(mut self, level: u8) -> Self` - Builder for setting debug level

## When Adding a New Parameter to process_command

1. Add field to `TestContext` struct in `common.rs`
2. Initialize it in `Default` impl
3. Pass it in the `run()` method's call to `process_command()`

**That's it!** No need to update `debug_level.rs`, `tog_integration_test.rs`, or any other test file.

## Example: Adding a hypothetical `foo_enabled: bool` parameter

```rust
// In TestContext struct:
pub foo_enabled: bool,

// In Default impl:
foo_enabled: false,

// In run() method, add to process_command call:
&mut self.foo_enabled,
```

## Files Using TestContext
- `src/tests/debug_level.rs` - 7 tests
- `src/tests/tog_integration_test.rs` - 2 tests

## Files Still Using Old Infrastructure
Other test files (variable_tests.rs, pattern_ops/, etc.) use `test_setup!` macro which only provides variables/patterns/counters/scripts/scale - they test individual handler functions, not `process_command` directly.
