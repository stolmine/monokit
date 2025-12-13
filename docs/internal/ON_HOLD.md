# Monokit - Features On Hold

This document tracks features that have been deferred indefinitely due to technical constraints or architectural limitations.

---

## Phase 4: Modulation System

**Status:** ON HOLD INDEFINITELY - SuperCollider UGen complexity limits

**Issue:** SuperCollider SynthDef complexity limits prevent implementation of freely routable LFO destinations. Extensive testing (December 2025) showed:
- 2 LFOs × 5 destinations worked initially
- 3-4 LFOs × 7+ destinations hit UGen limits
- Architectural attempts (control bus separation, split routers) all failed
- The InRange.kr routing matrix creates too many UGens regardless of synth splitting

**Attempted Solutions:**
- Control bus architecture (LFO generation separate from main synth)
- Pre-computed destination routing in separate synth
- Split lfo_bank into lfo_gen + lfo_router
- Split router into two 8-destination synths (lfo_router_a + lfo_router_b)
- All approaches hit SC's internal optimization/complexity limits

**Conclusion:** Freely routable multi-LFO modulation is not feasible within SuperCollider's SynthDef architecture for a synth of monokit's complexity. Would require either:
- Fixed LFO→destination assignments (not user-routable)
- Significant reduction in main synth features
- Alternative audio engine (not SC)

---

### LFO System [High]
- [ ] 2-4 LFO units (L1, L2, L3, L4)
- [ ] `L1.RATE <hz>` - LFO frequency (0.01-100 Hz)
- [ ] `L1.WAVE <0-4>` - Waveform (sin, tri, saw, square, random)
- [ ] `L1.AMP <0-16383>` - Amplitude/depth
- [ ] `L1.PHASE <0-360>` - Phase offset
- [ ] `L1.SYNC <0|1>` - Sync to metro
- [ ] `L1.DEST <param>` - Set destination parameter
- [ ] `L1.AMT <0-16383>` - Modulation amount
- [ ] `L1.SLEW <ms>` - Slew/lag on LFO output
- [ ] `L1.QUANT <steps>` - Quantize LFO to N steps
- [ ] Multiple destinations per LFO (optional)
- [ ] SC implementation: New UGens, routing matrix, phase sync

### Aux Envelope System [High]
Flexible auxiliary envelope that can be routed to any synth parameter.
Same routing complexity issues as LFO system.

- [ ] `XENV.DEC <ms>` - Aux envelope decay time
- [ ] `XENV.ATK <ms>` - Aux envelope attack time
- [ ] `XENV.CRV <-8 to 8>` - Aux envelope curve
- [ ] `XENV.AMT <0-16383>` - Aux envelope amount
- [ ] `XENV.DEST <param>` - Set destination parameter (e.g., `XENV.DEST FC`)
- [ ] Multiple destinations support (optional)
- [ ] SC implementation: New envelope with routing matrix

### Extended Envelope Coverage [Medium]
Dedicated envelopes for synth/FX parameters currently lacking envelope control.
Blocked by same SC complexity constraints.

**Lo-Fi Effect:**
- [ ] `LOEV.DEC <ms>` - Lo-Fi envelope decay
- [ ] `LOEV.ATK <ms>` - Lo-Fi envelope attack
- [ ] `LOEV.CRV <-8 to 8>` - Lo-Fi envelope curve
- [ ] `LOEV.AMT <0-16383>` - Lo-Fi envelope amount (modulates LM mix)

**Other candidates (to be evaluated):**
- [ ] Ring mod envelope (RGM mix)
- [ ] Resonator envelope (RM mix)
- [ ] Delay envelope (DW wet)
- [ ] Reverb envelope (RW wet)

---

## MIDI Clock Output

**Status:** ON HOLD INDEFINITELY

- [ ] `M.SEND <0|1>` - Send MIDI clock out
- [ ] Send start/stop/continue messages

---

## Initial Window Sizing

**Status:** ON HOLD INDEFINITELY (UNENFORCEABLE)

Terminal window sizing cannot be reliably controlled or enforced across different terminal emulators.

- [ ] Document recommended terminal size (50x18 minimum)
- [ ] Detect current terminal size on startup
- [ ] Warn if terminal too small (< 50x18)
- [ ] Optional: Set terminal size via ANSI escape sequences (if supported)
- [ ] Add to README/docs as setup requirement
