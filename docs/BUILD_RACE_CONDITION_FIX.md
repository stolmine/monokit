# Build Race Condition Fix

**Date:** December 12, 2025
**Issue:** `verified_build.sh` hanging on "Building bundle" step

---

## Problem

The build process would hang intermittently during SynthDef compilation in `bundle.sh`. Testing revealed:

- First `sclang` invocation: **Success** (exit 0)
- Second `sclang` invocation: **Timeout** (exit 124)
- Third `sclang` invocation: **Timeout** (exit 124)

Error message when hanging:
```
ERROR: Message 'rate' not understood.
Perhaps you misspelled 'rank', or meant to call 'rate' on another receiver?
RECEIVER: *
```

---

## Root Cause

**Race condition:** When `sclang` is invoked too quickly after a previous invocation, lingering processes or locked resources cause the compiler to fail with cryptic syntax errors.

The issue was **timing-dependent**:
- Adding a 2-second delay before `sclang`: **Success**
- Running immediately after previous run: **Failure**

---

## Solution

Modified `scripts/bundle.sh` to add cleanup before SynthDef compilation:

```bash
# Kill any lingering sclang/scsynth processes to avoid race conditions
pkill -9 sclang 2>/dev/null || true
pkill -9 scsynth 2>/dev/null || true
sleep 2
```

This ensures:
1. No stale `sclang` processes are running
2. No stale `scsynth` server processes are holding resources
3. A brief delay allows full cleanup before next invocation

---

## Verification

After the fix:
- `./scripts/bundle.sh` runs consistently without hanging
- `./scripts/verified_build.sh` completes all 8 steps successfully
- Multiple consecutive builds work reliably

---

## Files Modified

- `scripts/bundle.sh` (lines 47-50): Added process cleanup and delay

---

## Lessons Learned

SuperCollider's `sclang` interpreter can leave processes resident in memory even after script completion. When used in automated build scripts, explicit cleanup is necessary to prevent race conditions and cryptic compilation errors.

The error "Message 'rate' not understood. RECEIVER: *" was a red herring - not a syntax error in the SynthDef code, but a symptom of the interpreter being in an inconsistent state due to unclean shutdown.
