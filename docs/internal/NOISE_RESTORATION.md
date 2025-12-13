# Noise Implementation Restoration

**Date:** December 12, 2025
**Action:** Restored to commit 89d035b (working multi-synth architecture)

---

## What Was Restored

Reverted SynthDef files to commit `89d035b` which contains:

- **Working multi-synth architecture:** 4 source synths (noise, mod, primary, main)
- **Simple noise implementation:** Basic noise generation without envelope/gate
- **Fixed NV/PV/MV isolation:** Volume parameters work independently

---

## Simple Noise Synth (Restored)

```supercollider
SynthDef(\monokit_noise, {
    arg nw = 0,      // Noise waveform (0=white, 1=pink, 2=brown)
        nv = 0,      // Noise volume (0-16383)
        noiseBus = 18;

    var noiseRaw = Select.ar(nw, [WhiteNoise, PinkNoise, BrownNoise]);
    Out.ar(noiseBus, noiseRaw * (nv / 16383));
});
```

**Parameters:**
- `NW`: Noise waveform selection
- `NV`: Noise volume (0-16383 range)

**No envelope, no gate, no FM routing** - just basic noise generation.

---

## What Was Removed

The attempted noise envelope/gate implementation included:
- Envelope parameters (NA, ND, NC, NE)
- Gate control (NG)
- FM routing to oscillators (NP, NM)
- Pre-gate/post-gate signal flow separation

This implementation had issues:
- NV defaulting to 0 made noise completely silent
- Complex signal routing introduced bugs
- FM routing independence wasn't working correctly

---

## Current State

**Files restored:**
- `build_scripts/compile_synthdefs.scd`
- `sc/monokit_server.scd`

**Files kept (unrelated to noise):**
- `scripts/bundle.sh` (race condition fix)
- `docs/ROADMAP.md` (progress tracking)
- `docs/documentation_index.md` (index updates)
- `src/metro.rs`, `src/types.rs` (other features)

**Build verified:**
- All 7 synthdefs compile correctly
- Bundle builds successfully
- Noise synth: 505 bytes (simple implementation)

---

## Next Steps

The noise source is functional but basic. Future enhancements could include:
- Envelope shaping (NA, ND, NC, NE, NG)
- FM routing to oscillators (NP, NM)
- Better default values
- Clearer signal flow documentation

For now, noise works as a simple source controlled by NW (waveform) and NV (volume).
