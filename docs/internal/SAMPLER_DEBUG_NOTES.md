# Sampler Debug Session Notes

**Date:** 2026-01-03
**Status:** Both issues unresolved

## Issues
1. **No sound from sampler** - kits load, other engines work, but STR produces no audio
2. **Path resolution requires absolute paths** - relative paths and library lookup don't work

---

## What We've Tried

### Sound Issue
- [x] Verified `monokit_sampler` SynthDef exists and is compiled
- [x] Verified synth is instantiated in scsynth_direct.rs (node 1005)
- [x] Verified `route_param_to_node()` routes `s_bufnum`, `t_gate_sampler` to SAMPLER_NODE_ID
- [x] Added `samplerBus` parameter to monokit_main creation (was missing)
- [x] Verified monokit_main reads from bus 21 and mixes `samplerIn`
- [x] Buffer loading sends `/b_allocRead` OSC correctly
- [x] STR sends correct buffer ID from loaded slots

**Still not working** - signal path appears complete but no audio

### Path Resolution Issue
- [x] Added `~` tilde expansion (lines 18-33 in sampler.rs)
- [x] Fixed canonicalization logic (was silently dropping valid paths)
- [x] Simplified to direct `library_path.join(path_str).exists()` check
- [x] Library path: `~/.config/monokit/samples/`

**Still not working** - only absolute paths work

---

## Key Files Changed
- `/Users/why/repos/monokit/src/scsynth_direct.rs` - added SAMPLER_BUS import and param
- `/Users/why/repos/monokit/src/commands/synth/sampler.rs` - path resolution, buffer loading
- `/Users/why/repos/monokit/build_scripts/compile_synthdefs.scd` - monokit_main bus 21, t_gate_sampler
- `/Users/why/repos/monokit/src/types.rs` - param routing

---

## Next Steps to Investigate

### Sound
1. Add debug logging to verify OSC messages actually reach scsynth
2. Check if buffers are actually allocated (scsynth may reject `/b_allocRead`)
3. Verify buffer IDs match between load and trigger
4. Check if sampler envelope is gating audio (s_atk, s_rel values)

### Path Resolution
1. Add debug output in `resolve_sample_path()` to see which step fails
2. Verify `dirs::home_dir()` returns correct value at runtime
3. Check if library directory exists: `~/.config/monokit/samples/`
4. Test with explicit debug prints for each resolution step

---

## Architecture Summary

### Buffer Loading Flow
1. KIT command → `handle_kit()` in sampler.rs
2. Calls `send_buffer_alloc_read(buffer_id, file_path)`
3. Sends `/b_allocRead <buffer_id> <path>` to scsynth (port 57110)
4. Buffer IDs start at SAMPLER_BUFFER_BASE (100)

### Trigger Flow
1. STR command → `handle_str()` in sampler.rs
2. Gets buffer_id from `sampler_state.slots[slot].buffer_id`
3. Sends `s_bufnum` param via `MetroCommand::SendParam`
4. Sends `t_gate_sampler` trigger via `MetroCommand::SendParam`
5. metro.rs `create_param_message()` routes to node via `route_param_to_node()`
6. For scsynth-direct: sends `/n_set <node_id> <param> <value>`

### Signal Path
1. monokit_sampler outputs to bus 21
2. monokit_main reads bus 21 via `InFeedback.ar(samplerBus, 1)`
3. Mixed into signal: `sig = sig + plaitsMainIn + plaitsAuxIn + samplerIn`

### Path Resolution Order (design spec)
1. Absolute path → use directly
2. Relative to library → prepend `~/.config/monokit/samples/`
3. Search library → recursive search by filename
4. Current directory → use as-is

---

## Design Doc Reference
`/Users/why/repos/monokit/docs/SAMPLER_DESIGN.md` - full sampler specification
