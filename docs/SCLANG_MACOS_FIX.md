# SuperCollider sclang macOS Execution Fix

## Problem Summary

When running `sclang` from the command line on macOS to compile SynthDefs, the script files are not executed. Instead, sclang starts in interactive mode and hangs, showing only:

```
*** Welcome to SuperCollider 3.14.1. *** For help type cmd-d.
```

The script never runs, and no SynthDefs are compiled.

## Root Cause

### macOS-Specific sclang Issue

There is a **known macOS-specific issue** with running `sclang` from the command line:

> **On macOS, you need to cd into the SuperCollider.app directory before executing sclang, in order to avoid an issue with Cocoa.**

**Source:** [GitHub Issue #4552 - macos: provide script wrapper for sclang executable](https://github.com/supercollider/supercollider/issues/4552)

### Why This Happens

When you run:
```bash
/Applications/SuperCollider.app/Contents/MacOS/sclang script.scd
```

From any other working directory, sclang's Cocoa framework initialization fails to properly execute the script file. This affects:
- Script execution from command line
- Automated builds
- CI/CD pipelines
- Any headless sclang usage

## Solution

### Wrapper Script Approach

Create a wrapper script that changes to the correct directory before executing sclang:

```bash
#!/bin/bash
# sclang-wrapper.sh
cd /Applications/SuperCollider.app/Contents/MacOS
exec ./sclang "$@"
```

### Implementation for Monokit

Update build scripts to either:
1. Use the wrapper
2. `cd` into the SC directory before calling sclang
3. Use absolute paths with proper working directory

## Applied Fix

### 1. Updated `scripts/bundle.sh`

**Before:**
```bash
"${SCLANG}" "${REPO_ROOT}/build_scripts/compile_synthdefs.scd"
```

**After:**
```bash
cd /Applications/SuperCollider.app/Contents/MacOS
./sclang "${REPO_ROOT}/build_scripts/compile_synthdefs.scd"
cd "${REPO_ROOT}"
```

### 2. Updated `scripts/verified_build.sh`

**Before:**
```bash
timeout 30 /Applications/SuperCollider.app/Contents/MacOS/sclang build_scripts/compile_synthdefs.scd
```

**After:**
```bash
(cd /Applications/SuperCollider.app/Contents/MacOS && \
 timeout 30 ./sclang "${OLDPWD}/build_scripts/compile_synthdefs.scd")
```

## Related Issues

### mi-UGens on Apple Silicon

The research also uncovered potential compatibility issues with mi-UGens on ARM64:

1. **Architecture:** Some mi-UGens components (like MiRipples) don't build on ARM
2. **Latest Version:** v0.0.6 (April 2024) from [v7b1/mi-UGens releases](https://github.com/v7b1/mi-UGens/releases)
3. **Compatibility:** SuperCollider 3.14 is compatible with sc3-plugins 3.13+ as the plugin interface wasn't changed
4. **Solution:** May need to recompile mi-UGens from source for Apple Silicon

**Sources:**
- [mi-UGens GitHub Repository](https://github.com/v7b1/mi-UGens)
- [SuperCollider 3.14 Downloads](https://supercollider.github.io/downloads.html)
- [Building SC on Mac M1](https://scsynth.org/t/building-supercollider-and-plugins-on-mac-m1/4626)

### MiPlaits Detection Issue

The check `UGen.findRespondingMethodFor(\ar, \MiPlaits)` returns `nil` even when MiPlaits class exists.

**Workaround:** Use direct class existence check:
```supercollider
hasMiPlaits = try { MiPlaits.notNil } { false };
```

This is more reliable than method lookup on macOS with certain UGen installations.

## Testing the Fix

### Verify sclang Execution

```bash
# Test that script actually runs
cd /Applications/SuperCollider.app/Contents/MacOS
./sclang /path/to/test.scd
```

The script should execute and output messages, not just hang in interactive mode.

### Verify SynthDef Compilation

```bash
# From monokit root directory
./scripts/verified_build.sh
```

Should complete successfully with all 8 SynthDefs compiled.

## Additional Resources

### Command-Line sclang Issues
- [sclang "command not found" on macOS](https://scsynth.org/t/command-not-found-when-running-sclang-from-command-line/3006)
- [Running sclang scripts from terminal](https://sc-users.bham.ac.narkive.com/QDwWVaMs/sclang-running-scd-files-from-terminal)
- [sclang hangs by default](https://github.com/supercollider/supercollider/issues/3393)
- [Non-interactive scripts hang on compilation failures](https://github.com/supercollider/supercollider/issues/5218)

### mi-UGens Resources
- [mi-UGens Installation (Tidal Cycles)](https://tidalcycles.org/docs/reference/mi-ugens-installation/)
- [mi-UGens Discussion (Tidal Club)](https://club.tidalcycles.org/t/mutable-instruments-ugens/2730)
- [Building mi-UGens from source](https://github.com/v7b1/mi-UGens/blob/master/build.sh)

### SuperCollider Documentation
- [Understanding Errors in SuperCollider](https://doc.sccode.org/Guides/Understanding-Errors.html)
- [First Steps with SuperCollider](https://doc.sccode.org/Tutorials/Getting-Started/02-First-Steps.html)
- [UGen Class Documentation](https://doc.sccode.org/Classes/UGen.html)

## Summary

The core issue preventing SynthDef compilation was **not** related to MiPlaits or the conditional compilation logic, but rather a fundamental macOS-specific problem with how `sclang` must be invoked from the command line.

By ensuring sclang runs from its own directory (`/Applications/SuperCollider.app/Contents/MacOS`), the scripts now execute properly and SynthDefs compile successfully.
