# Tier 0 (TIER_SILENT) Commands

## Overview

**Tier 0** commands produce **absolutely no output** when executed successfully. They operate silently, only sending OSC parameters to the sound engine without any REPL confirmation.

## True Tier 0 Commands

The following **12 commands** are the ONLY commands in Monokit that produce zero output at debug level 0:

### Randomization Commands (RND.*)

All located in `/Users/why/repos/monokit/src/commands/randomization.rs`

1. **RND.VOICE** - Randomizes all voice parameters (oscillators + FM)
   - Parameters: pf, pw, mf, mw, fm, fb, fba, fbd
   - Line: 8-38

2. **RND.OSC** - Randomizes oscillator parameters only
   - Parameters: pf, pw, mf, mw
   - Line: 40-62

3. **RND.FM** - Randomizes FM parameters
   - Parameters: fm, fb, fba, fbd
   - Line: 64-86

4. **RND.MOD** - Randomizes modulation bus parameters
   - Parameters: mb, tk, mp, md, mt, ma, mm, me, mx, mba, mbd
   - Line: 88-114

5. **RND.ENV** - Randomizes envelope parameters
   - Parameters: ad, pd, fd, pa, fa, da, dd, fbd, fed
   - Line: 116-146

6. **RND.P** - Randomizes working pattern (current P.N pattern)
   - Fills pattern with random values 0-16383
   - Line: 148-190

7. **RND.PN** `<0-5>` - Randomizes specific explicit pattern
   - Fills specified pattern with random values 0-16383
   - Line: 192-253
   - **Note**: Produces TIER_ERRORS output on invalid pattern number

8. **RND.PALL** - Randomizes all 6 patterns
   - Fills patterns 0-5 with random values
   - Line: 255-299

9. **RND.FX** - Randomizes all effect parameters
   - Parameters: ct, cr, ca, cl, cm, dt, df, dlp, dw, rv, rh, rp, rw, lb, ls, lm, rgf, rgw, rgm, el, em, ef, eq, eh
   - Line: 301-373

10. **RND.FILT** - Randomizes filter parameters
    - Parameters: fc, fq, ft, fe, fed, fk, mc, mq
    - Line: 375-399

11. **RND.DLY** - Randomizes delay parameters
    - Parameters: dt, df, dlp, dw
    - Line: 401-425

12. **RND.VERB** - Randomizes reverb parameters
    - Parameters: rv, rh, rp, rw
    - Line: 427-451

13. **RND.PL** - Randomizes Plaits oscillator parameters
    - Parameters: plh (harmonics), plt (timbre), plm (morph), pld (decay), pll (lpg)
    - Line: 453-482

---

## Key Characteristics

### Implementation Pattern
All RND.* commands follow this pattern:
```rust
pub fn handle_rnd_xxx<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,      // Accepted but NEVER used
    mut output: F,        // Accepted but NEVER called
) -> Result<()>
where
    F: FnMut(String),
{
    // Generate random values
    let param = rng.gen_range(...);

    // Send OSC params silently
    metro_tx.send(MetroCommand::SendParam(...))?;

    // Return with NO output() call
    Ok(())
}
```

### Why These Are Tier 0
1. **No output() calls** - Unlike all other commands, these never call `output()`
2. **Silent operation** - They perform actions (randomizing parameters) without confirmation
3. **Debug level ignored** - They accept `debug_level` parameter but never check it
4. **Only errors surface** - Only RND.PN produces error output for validation failures

---

## What About Config Commands?

**Config commands are NOT Tier 0**, despite being listed as such in the initial classification.

### Config Commands That ALWAYS Produce Output

The following commands use `define_bool_toggle` and `define_enum_select` macros that **unconditionally call output()**:

- DEBUG, HEADER, CPU, BPM
- AUTOLOAD, LOAD.RST, LOAD.CLR
- METER.HDR, METER.GRID, METER.ASCII
- SPECTRUM, ACTIVITY, GRID, GRID.DEF, GRID.MODE
- HL.SEQ
- OUT.ERR, OUT.ESS, OUT.QRY, OUT.CFM
- SCRMBL, SCRMBL.MODE, SCRMBL.SPD, SCRMBL.CRV

### Why Config Commands Are NOT Tier 0

Looking at the macro implementations in `/Users/why/repos/monokit/src/commands/system/misc.rs:10-120`:

```rust
macro_rules! define_bool_toggle {
    ($fn_name:ident, $cmd_name:expr, $save_fn:path) => {
        pub fn $fn_name<F>(parts: &[&str], state: &mut bool, mut output: F)
        where F: FnMut(String),
        {
            if parts.len() == 1 {
                output(format!("{}: {}", $cmd_name, if *state { "ON" } else { "OFF" }));
                // ^^^^^^ ALWAYS outputs - no debug level check!
            } else {
                // ... setting logic also ALWAYS outputs
            }
        }
    };
}
```

**These macros always call `output()` regardless of debug level.**

To make config commands truly Tier 0, they would need to:
1. Use `ExecutionContext.should_output()` method, OR
2. Check `debug_level >= TIER_X` before calling output, OR
3. Be refactored to use the new OutputCategory system

---

## Summary

**Total TRUE Tier 0 Commands: 13**
- RND.VOICE, RND.OSC, RND.FM, RND.MOD, RND.ENV
- RND.P, RND.PN, RND.PALL
- RND.FX, RND.FILT, RND.DLY, RND.VERB, RND.PL

**All other commands produce output** at some debug tier, including:
- Config commands (always output)
- System commands (tier 2+)
- Synth parameters (tier 2-4)
- Pattern commands (tier 3-4)
- Core commands (tier 2-4)

---

## Usage Example

```
DEBUG 0          # Set to silent mode
RND.VOICE        # Randomizes voice - NO OUTPUT
PF 440           # Would produce output if debug > 0, silent at tier 0
DEBUG           # This outputs "DEBUG: 0" - config commands ALWAYS output
```

## Future Considerations

To properly support Tier 0 silent operation for config commands, consider:

1. **Refactor config macros** to accept `ExecutionContext` and use `should_output()`
2. **Add OutputCategory::Config** tier for configuration changes
3. **Make config commands respect debug level** like other commands do

This would allow true silent operation where even config changes produce no output.
