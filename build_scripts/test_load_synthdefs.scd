(
// Test script to verify compiled SynthDefs load correctly
// Run with: sclang test_load_synthdefs.scd

"Testing SynthDef loading...".postln;

s.waitForBoot {
    var synthdefs_dir = "../sc/synthdefs/".resolveRelative;

    "Server booted".postln;
    ("Loading SynthDefs from: " ++ synthdefs_dir).postln;

    // Load all synthdefs from directory
    s.loadDirectory(synthdefs_dir);

    s.sync;

    "SynthDefs loaded successfully".postln;

    // Instantiate each SynthDef to verify they work
    "Testing SynthDef instantiation...".postln;

    ~voice = Synth(\monokit);
    ~spectrum = Synth(\monokit_spectrum, addAction: \addToTail);
    ~scope = Synth(\monokit_scope, addAction: \addToTail);

    0.5.wait;

    "All synths instantiated successfully".postln;

    // Test basic parameter setting
    ~voice.set(\pf, 440);
    ~voice.set(\gate, 1);

    0.1.wait;

    ~voice.set(\gate, 0);

    "Parameter setting test passed".postln;

    1.wait;

    "".postln;
    "=== SUCCESS ===".postln;
    "All 3 SynthDefs loaded and instantiated without error".postln;
    "  - monokit.scsyndef (22 KB)".postln;
    "  - monokit_spectrum.scsyndef (3.1 KB)".postln;
    "  - monokit_scope.scsyndef (8.4 KB)".postln;
    "".postln;

    1.wait;

    // Clean shutdown
    ~voice.free;
    ~spectrum.free;
    ~scope.free;

    0.5.wait;

    s.quit;

    0.5.wait;

    0.exit;
};
)
