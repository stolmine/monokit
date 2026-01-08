// Monokit Server - Main Entry Point
// Loads modular server components
(
var basePath = PathName(thisProcess.nowExecutingPath).pathOnly;

// Initialize audio device configuration
this.executeFile(basePath +/+ "server/init.scd");

s.waitForBoot {
    // Load SynthDef definitions
    this.executeFile(basePath +/+ "server/synthdef_monokit.scd");
    this.executeFile(basePath +/+ "server/synthdefs.scd");

    s.sync;

    // Create synth instances
    this.executeFile(basePath +/+ "server/synth_instances.scd");

    // Setup OSC forwarding
    this.executeFile(basePath +/+ "server/osc_forwarding.scd");

    // Setup OSC command handlers
    this.executeFile(basePath +/+ "server/osc_handlers.scd");
};
)
