(
// SynthDef Pre-compilation Script
// Compiles monokit SynthDefs to binary .scsyndef files
// Run with: sclang -D compile_synthdefs.scd

// Output to sibling sc/synthdefs directory (relative to this script's parent)
var thisDir = thisProcess.nowExecutingPath.dirname;
var synthdefs_dir = thisDir +/+ "../sc/synthdefs/";
var hasMiPlaits, hasMiRings;

// Ensure output directory exists
File.mkdir(synthdefs_dir);
("Output directory: " ++ synthdefs_dir).postln;

// Check if MiPlaits UGen is available
"Checking for MiPlaits...".postln;
hasMiPlaits = try {
    var result = MiPlaits.notNil;
    ("  MiPlaits class check result: " ++ result).postln;
    result;
} {
    |error|
    ("  MiPlaits check failed with error: " ++ error).postln;
    false;
};

("  hasMiPlaits = " ++ hasMiPlaits).postln;

if(hasMiPlaits.not, {
    "WARNING: MiPlaits UGen not found! Plaits voice will not work.".postln;
    "Install mi-UGens from: https://github.com/v7b1/mi-UGens".postln;
}, {
    "✓ MiPlaits UGen found and will be used".postln;
});

"Compiling SynthDefs to binary format...".postln;

// MULTI-SYNTH ARCHITECTURE - 5 separate SynthDefs
File.readAllString(thisDir +/+ "synthdefs/noise_mod.scd").interpret.value(synthdefs_dir);

// Primary oscillator
File.readAllString(thisDir +/+ "synthdefs/primary.scd").interpret.value(synthdefs_dir);

// Conditional Plaits SynthDef - only use MiPlaits if available
File.readAllString(thisDir +/+ "synthdefs/plaits.scd").interpret.value(synthdefs_dir, hasMiPlaits);

// Main signal path
File.readAllString(thisDir +/+ "synthdefs/main.scd").interpret.value(synthdefs_dir);

// Visualization synthdefs
File.readAllString(thisDir +/+ "synthdefs/visualization.scd").interpret.value(synthdefs_dir);

// Sampler synthdef (with MiRings check)
hasMiRings = try { MiRings.notNil } { false };
if(hasMiRings.not, {
    "WARNING: MiRings UGen not found! Sampler resonator will not work.".postln;
});
File.readAllString(thisDir +/+ "synthdefs/sampler.scd").interpret.value(synthdefs_dir, hasMiRings);

"".postln;
"Compiled SynthDefs:".postln;
"  - monokit_noise.scsyndef (noise generator)".postln;
"  - monokit_mod.scsyndef (modulator oscillator)".postln;
"  - monokit_primary.scsyndef (primary oscillator)".postln;
"  - monokit_plaits.scsyndef (Plaits voice)".postln;
"  - monokit_main.scsyndef (main signal path)".postln;
"  - monokit_spectrum.scsyndef (spectrum analyzer)".postln;
"  - monokit_scope.scsyndef (oscilloscope)".postln;
"  - monokit_recorder.scsyndef (audio recorder)".postln;
"  - monokit_sampler.scsyndef (sample playback)".postln;
{ 0.exit }.defer(0.1);  // Deferred exit to allow postln to complete
)
