{ |synthdefs_dir|
SynthDef(\monokit_spectrum, {
    var sigL, sigR, mono, bands, freqs, clips, data;

    sigL = InFeedback.ar(0);
    sigR = InFeedback.ar(1);
    mono = (sigL + sigR) * 0.5;

    freqs = [25, 40, 63, 100, 160, 250, 400, 630, 1000, 1600, 2500, 4000, 6300, 10000, 16000];

    bands = freqs.collect({ |freq|
        var filtered = BPF.ar(mono, freq, 0.5);
        Amplitude.kr(filtered, 0.01, 0.1)
    });

    clips = bands.collect({ |level|
        (level > 0.95).asInteger
    });

    data = bands ++ clips;

    SendReply.kr(Impulse.kr(20), '/monokit/spectrum', data);
}).writeDefFile(synthdefs_dir);

"  monokit_spectrum.scsyndef".postln;

SynthDef(\monokit_scope, {
    arg scopeRate = 0.1;
    var sigL, sigR, mono;
    var phase, trig;
    var buf = LocalBuf(128);

    sigL = InFeedback.ar(0);
    sigR = InFeedback.ar(1);
    mono = (sigL + sigR) * 0.5;

    phase = Phasor.ar(0, scopeRate, 0, 128);
    BufWr.ar(mono, buf, phase);

    trig = Impulse.kr(20);
    SendReply.kr(trig, '/monokit/scope',
        BufRd.kr(1, buf, Array.series(128, 0, 1), 0, 1)
    );
}).writeDefFile(synthdefs_dir);

"  monokit_scope.scsyndef".postln;

SynthDef(\monokit_recorder, { |bufnum|
    DiskOut.ar(bufnum, In.ar(0, 2));
}).writeDefFile(synthdefs_dir);

"  monokit_recorder.scsyndef".postln;
}.value;
