# CL Command Debug Test Instructions

## Overview

Comprehensive debug logging has been added to trace CL.TRIG/CLTR command execution. All debug output writes to `/tmp/monokit_commands.log`.

## Debug Logging Points

The following debug markers trace command execution:

- `[DEBUG] [1]` - Command parsing (after uppercase)
- `[DEBUG] [ALIAS-1 to ALIAS-4]` - Alias resolution in aliases.rs
- `[DEBUG] [2]` - After alias resolution
- `[DEBUG] [3]` - Entering match statement
- `[DEBUG] [4]` - Pattern matched successfully
- `[DEBUG] [5-8]` - Dispatch execution
- `[DEBUG] [HANDLER-1 to HANDLER-6]` - Handler internal execution
- `[DEBUG] [X]` - Unknown command (match failed)

## Before Testing

1. **Rebuild with debug logging:**
   ```bash
   cargo build --release
   ./scripts/bundle.sh
   ```

2. **Clean the log file:**
   ```bash
   rm -f /tmp/monokit_commands.log
   ```

3. **Verify bundle timestamp:**
   ```bash
   ls -lT dist/bundle/monokit-dev-aarch64-apple-darwin/monokit
   ```

## Test Procedure

### Test 1: CLTR (alias form)

1. Run monokit:
   ```bash
   ./dist/bundle/monokit-dev-aarch64-apple-darwin/monokit
   ```

2. Type:
   ```
   OUT.CFM 1
   CLTR
   ```

3. Quit (Ctrl+Q)

4. Check log:
   ```bash
   cat /tmp/monokit_commands.log | grep DEBUG
   ```

### Test 2: CL.TRIG (canonical form)

1. Clean log: `rm -f /tmp/monokit_commands.log`
2. Run monokit
3. Type: `OUT.CFM 1` then `CL.TRIG`
4. Quit and check log

### Test 3: CLW 8192 (parameter command)

1. Clean log: `rm -f /tmp/monokit_commands.log`
2. Run monokit
3. Type: `OUT.CFM 1` then `CLW 8192`
4. Quit and check log

## Expected Output (Success)

```
CMD: OUT.CFM 1
[DEBUG] [1] Command parsing: original_cmd='CLTR' (from input 'CLTR')
[DEBUG] [ALIAS-1] resolve_alias called with cmd='CLTR'
[DEBUG] [ALIAS-3] NOT found in map, returning unchanged: 'CLTR'
[DEBUG] [ALIAS-4] resolve_alias returning: 'CLTR'
[DEBUG] [2] After alias resolution: original='CLTR' resolved='CLTR' match=false
[DEBUG] [3] Entering match statement: cmd.as_str()='CLTR'
[DEBUG] [4] MATCHED CL.TRIG/CLTR pattern! cmd='CLTR'
[DEBUG] [5] Calling handle_cl_trig...
[DEBUG] [HANDLER-1] handle_cl_trig ENTERED
[DEBUG] [HANDLER-2] Sending OSC: param='t_cl_trig' value=1
[DEBUG] [HANDLER-3] OSC sent successfully
[DEBUG] [HANDLER-4] Generating output message...
[DEBUG] [HANDLER-5] Output callback: msg='GRAINS TRIGGERED'
[DEBUG] [HANDLER-6] handle_cl_trig EXITING, output.len()=1
[DEBUG] [6] handle_cl_trig returned, output_vec.len()=1
[DEBUG] [7] Outputting message: 'GRAINS TRIGGERED'
[DEBUG] [8] CL.TRIG dispatch complete
CMD: CLTR → DISPATCHED
```

## Expected Output (Failure - Unknown Command)

```
[DEBUG] [1] Command parsing: original_cmd='CLTR' (from input 'CLTR')
[DEBUG] [ALIAS-1] resolve_alias called with cmd='CLTR'
[DEBUG] [ALIAS-3] NOT found in map, returning unchanged: 'CLTR'
[DEBUG] [ALIAS-4] resolve_alias returning: 'CLTR'
[DEBUG] [2] After alias resolution: original='CLTR' resolved='CLTR' match=false
[DEBUG] [3] Entering match statement: cmd.as_str()='CLTR'
[DEBUG] [X] UNKNOWN COMMAND HIT! cmd='CLTR' original_cmd='CLTR' input='CLTR'
[DEBUG] [X] cmd bytes: [67, 76, 84, 82]
CMD: CLTR → UNKNOWN
```

**Key indicator:** Missing `[DEBUG] [4]` means the match pattern failed!

## Using Test Scene

Alternatively, use the test scene:

```bash
./dist/bundle/monokit-dev-aarch64-apple-darwin/monokit --run test_cl_commands
```

Check scripts 2 and 3 which test CLTR.

## Quick One-Liner Test

```bash
rm -f /tmp/monokit_commands.log && \
(echo "OUT.CFM 1"; echo "CLTR"; sleep 1; echo "QUIT") | \
timeout 5 ./dist/bundle/monokit-dev-aarch64-apple-darwin/monokit >/dev/null 2>&1 && \
echo "=== DEBUG LOG ===" && \
cat /tmp/monokit_commands.log | grep -E "DEBUG|CLTR"
```

## Interpreting Results

### If you see [DEBUG] [4]:
✅ Pattern matched! The command is being dispatched correctly. Check handler execution.

### If you see [DEBUG] [X]:
❌ Pattern didn't match. The command is hitting the unknown/default case.

Possible causes:
- Command string has unexpected characters (check byte array)
- Match pattern not in binary (rebuild issue)
- Code path not reaching match statement

### If you only see [ALIAS-1] through [ALIAS-4]:
⚠️  Alias resolution is running but mod.rs debug isn't.

Possible causes:
- Binary is stale (from before mod.rs changes)
- `cmd.starts_with("CL")` condition not matching
- Force rebuild needed

## Troubleshooting

### No debug output in log:
```bash
# Verify binary has debug strings
strings ./target/release/monokit | grep "\[DEBUG\] \[3\]"

# Force clean rebuild
cargo clean
cargo build --release
./scripts/bundle.sh
```

### Only partial debug output:
Check which debug markers are present:
```bash
cat /tmp/monokit_commands.log | grep -o "\[DEBUG\] \[[^]]*\]" | sort -u
```

Should show: [1], [2], [3], [4] or [X], [5-8], [ALIAS-1 through 4], [HANDLER-1 through 6]

## Files Modified

Debug logging added to:
- `src/commands/mod.rs` - Command dispatch pipeline
- `src/commands/aliases.rs` - Alias resolution
- `src/commands/synth/effects/clouds.rs` - CL.TRIG handler

All write to `/tmp/monokit_commands.log` using the existing `log_command()` infrastructure.

## Next Steps After Testing

Share the complete `/tmp/monokit_commands.log` output. The debug trace will show exactly where and why the command is failing.
