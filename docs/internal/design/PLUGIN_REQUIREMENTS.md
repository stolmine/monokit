# Plugin Requirements for monokit Bundle

## Overview

Monokit requires specific SuperCollider UGen plugins to function correctly. This document lists all required plugins for bundled distribution.

## Required Plugins

### Core SuperCollider Plugins

These plugins come with the standard SuperCollider installation:

- **BinaryOpUGens.scx** - Binary operations (addition, multiplication, etc.)
- **UnaryOpUGens.scx** - Unary operations (negation, abs, etc.)
- **LID_UGens.scx** - Control rate utilities
- **IOUGens.scx** - Input/output operations
- **DelayUGens.scx** - Delay lines (CombC, DelayC)
- **FilterUGens.scx** - Basic filters (BLowShelf, BPeakEQ, BHiShelf, BPF)
- **DynNoiseUGens.scx** - Dynamic noise generators
- **NoiseUGens.scx** - Noise generators
- **PanUGens.scx** - Panning (Pan2)
- **TriggerUGens.scx** - Triggers and gates (Trig1)
- **OscUGens.scx** - Oscillators (SinOsc, LFTri, LFSaw, LFPulse, SinOscFB)
- **BufIOUGens.scx** - Buffer I/O (BufWr, BufRd, LocalBuf)
- **GrainUGens.scx** - Granular synthesis (PitchShift)
- **FFT_UGens.scx** - FFT operations (FreqShift)
- **PV_ThirdParty.scx** - Phase vocoder utilities
- **DemandUGens.scx** - Demand rate UGens
- **PhysicalModelingUGens.scx** - Physical modeling
- **MulAddUGens.scx** - Multiplication and addition optimizations

### Additional Core Plugins

- **ReverbUGens.scx** - Reverb effects (FreeVerb, FreeVerb2)
  - FreeVerb2 is a core SuperCollider UGen, NOT from sc3-plugins
  - Used in monokit SynthDef for reverb effect

- **LFUGens.scx** - LF oscillators (LFTri, LFSaw, LFPulse, etc.)
  - Essential for monokit's oscillator section

### sc3-plugins (Required)

These plugins come from the separate sc3-plugins package:

- **BlackrainUGens.scx** - Contains SVF (State Variable Filter)
  - SVF is used in monokit SynthDef for filter section (line 269-272)
  - Provides lowpass, highpass, bandpass, and notch filtering
  - **CRITICAL**: Without this plugin, monokit will fail to load
  - Also contains BMoog and IIRFilter

## Installation Instructions

### For Development

1. Install SuperCollider from https://supercollider.github.io/
2. Install sc3-plugins:
   - macOS Homebrew: `brew install sc3-plugins` (if available)
   - Manual: Download from https://github.com/supercollider/sc3-plugins
   - Place .scx files in:
     - macOS: `~/Library/Application Support/SuperCollider/Extensions/`
     - Linux: `~/.local/share/SuperCollider/Extensions/`

### For Bundle Creation

The `scripts/bundle.sh` script automatically searches for and copies required plugins from:
- `/Applications/SuperCollider.app/Contents/Resources/plugins/`
- `/usr/local/lib/SuperCollider/plugins/`
- `/opt/homebrew/lib/SuperCollider/plugins/`
- `~/Library/Application Support/SuperCollider/Extensions/`

If BlackrainUGens.scx (contains SVF) is not found, the script will warn you.

## Verifying Plugin Installation

To verify sc3-plugins are installed:

```bash
# macOS
ls -la "/Applications/SuperCollider.app/Contents/Resources/plugins/" | grep SVF
ls -la ~/Library/Application\ Support/SuperCollider/Extensions/ | grep SVF

# Check if plugins work
sclang -e "SVF.ar(SinOsc.ar(440), 1000, 0.5, 1, 0, 0, 0, 0); 0.exit;"
```

## UGen Usage in monokit

### Main SynthDef (\monokit)

**Oscillators:**
- SinOsc, LFTri, LFSaw, LFPulse - Basic waveforms
- SinOscFB - Feedback oscillator for modulation

**Filters:**
- SVF (sc3-plugins) - Multi-mode filter with LP/HP/BP/Notch
- BLowShelf, BPeakEQ, BHiShelf - 3-band EQ

**Effects:**
- CombC - Comb filter / resonator
- FreeVerb2 (sc3-plugins) - Stereo reverb
- Compander - Compression
- Limiter - Output limiting
- PitchShift - Granular pitch shifting
- FreqShift - Frequency shifting

**Buffers:**
- LocalBuf, BufWr, BufRd - Beat repeat buffer
- Phasor - Buffer indexing

**Utilities:**
- Pan2 - Stereo panning
- Lag - Parameter smoothing
- Select - Signal routing
- EnvGen, Env - Envelopes
- SendPeakRMS, SendReply - OSC data transmission

### Spectrum Analyzer (\monokit_spectrum)

- InFeedback - Audio monitoring
- BPF - Bandpass filtering for 15 frequency bands
- Amplitude - Level detection
- SendReply - Spectrum data transmission

### Oscilloscope (\monokit_scope)

- InFeedback - Audio monitoring
- LocalBuf, BufWr, BufRd - Waveform capture
- Phasor - Buffer scanning
- SendReply - Waveform data transmission

## Bundle Size Estimation

- scsynth binary: ~1.6 MB
- Core plugins: ~4-5 MB
- sc3-plugins (SVF + FreeVerb2): ~100-200 KB
- monokit binary: ~5 MB
- SynthDefs: ~35 KB
- **Total estimated: ~11-12 MB**

## License Considerations

- SuperCollider: GPL v3
- sc3-plugins: GPL v2/v3 (varies by plugin)
- monokit: Check project LICENSE

Bundling scsynth and plugins means monokit distribution inherits GPL requirements.

## Troubleshooting

**Error: "UGen 'SVF' not found"**
- sc3-plugins not installed, or BlackrainUGens.scx not in plugins directory
- SVF is part of BlackrainUGens, NOT a standalone SVF.scx file
- Solution: Install sc3-plugins and ensure BlackrainUGens.scx is copied to bundle

**Error: "UGen 'FreeVerb2' not found"**
- ReverbUGens.scx not in plugins directory
- FreeVerb2 is a CORE SuperCollider UGen (not sc3-plugins)
- Solution: Ensure ReverbUGens.scx is copied from SuperCollider.app

**Bundle script warnings about missing plugins**
- Non-critical: Core plugins may have alternate names
- Critical: BlackrainUGens.scx must be present for SVF filter
- Solution: Verify sc3-plugins installation at ~/Library/Application Support/SuperCollider/Extensions/SC3plugins/
