(
s.waitForBoot {
    SynthDef(\monokit, {
        arg freq = 200,
            pitchDecay = 0.01,
            pitchAmt = 4,
            ampDecay = 0.1,
            volume = 1,
            gate = 0;

        var trig = Trig1.kr(gate, 0.001);
        var pitchEnv = EnvGen.kr(Env.perc(0.001, pitchDecay, 1, -4), trig);
        var ampEnv = EnvGen.kr(Env.perc(0.001, ampDecay, 1, -4), trig);

        var pitch = freq * (1 + (pitchEnv * pitchAmt));
        var sig = SinOsc.ar(pitch);
        var amp = ampEnv * volume;

        Out.ar(0, Pan2.ar(sig * amp, 0));
    }).add;

    s.sync;

    ~voice = Synth(\monokit);

    OSCdef(\monokit_trigger, { |msg|
        ~voice.set(\gate, 1);
        SystemClock.sched(0.01, {
            ~voice.set(\gate, 0);
            nil;
        });
    }, '/monokit/trigger');

    OSCdef(\monokit_volume, { |msg|
        var vol = msg[1].asFloat.clip(0, 1);
        ~voice.set(\volume, vol);
    }, '/monokit/volume');

    "monokit server ready".postln;
};
)
