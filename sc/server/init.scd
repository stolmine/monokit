// Audio Device Configuration and Server Initialization
(
var outDev = "MONOKIT_AUDIO_OUT".getenv;

if(outDev.notNil && (outDev.size > 0), {
    s.options.outDevice = outDev;
    ("MONOKIT: Using output device: " ++ outDev).postln;
}, {
    "MONOKIT: Using default output device".postln;
});

s.options.numInputBusChannels = 0;

if(UGen.findRespondingMethodFor(\ar, \MiPlaits).isNil, {
    "WARNING: MiPlaits UGen not found! Plaits voice will not work.".postln;
    "Install mi-UGens from: https://github.com/v7b1/mi-UGens".postln;
});
)
