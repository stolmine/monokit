# Monokit Server Modularization

The monokit_server.scd file has been split into modular components for better maintainability.

## File Structure

```
sc/
├── monokit_server.scd          (25 lines)  - Main entry point
└── server/
    ├── init.scd                (18 lines)  - Audio device initialization
    ├── synthdef_monokit.scd    (576 lines) - Main synthesis engine
    ├── synthdefs.scd           (79 lines)  - Plaits, spectrum, scope SynthDefs
    ├── synth_instances.scd     (10 lines)  - Synth instantiation
    ├── osc_forwarding.scd      (39 lines)  - Meter/spectrum/scope/CPU forwarding
    └── osc_handlers.scd        (231 lines) - All OSC command handlers
```

## Module Responsibilities

### monokit_server.scd
Main entry point that loads all modules in correct order using `executeFile()`.

### server/init.scd
- Audio device configuration from environment variable
- Input channel disabling
- MiPlaits UGen availability check

### server/synthdef_monokit.scd
The complete main synthesis engine (\monokit SynthDef):
- Parameter declarations and smoothing
- Envelope generators
- Noise source
- Oscillators (primary + modulator)
- FM synthesis
- Filters (13 types)
- Effects (compressor, EQ, delay, reverb, ring mod, resonator, lofi, beat repeat, pitch shift)
- Metering outputs

### server/synthdefs.scd
Additional SynthDef definitions:
- \monokit_plaits - MiPlaits integration
- \monokit_spectrum - 15-band spectrum analyzer
- \monokit_scope - Waveform capture

### server/synth_instances.scd
Creates all synth instances in correct order with proper addActions.

### server/osc_forwarding.scd
Forwards internal OSC messages to Rust app on port 57121:
- Meter data (peak/RMS per channel)
- Spectrum analyzer data
- Oscilloscope waveform data
- Compressor metering
- CPU usage (2Hz)

### server/osc_handlers.scd
All OSC command handlers from Rust:
- /monokit/trigger - Gate/trigger events
- /monokit/volume - Master volume
- /monokit/param - Parameter routing to synths
- /monokit/scope/rate - Scope timespan control
- /monokit/rec - Recording start/stop/path
- /monokit/slew - Global and per-parameter slew
- /monokit/diag - Timing diagnostics
- /monokit/audio/out/query - Audio device enumeration

## Benefits

1. **Maintainability**: Each module has clear responsibility
2. **Readability**: All files under 750 lines (largest is 576)
3. **Agent-friendly**: Files can be read in single pass
4. **Preserves behavior**: Exact same functionality as monolithic file
5. **Load order**: Clear dependency chain via main file

## Original File

The original 1,084-line monokit_server.scd has been backed up and replaced with the modular version.
