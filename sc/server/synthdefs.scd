// SynthDef Definitions - Plaits, Spectrum, Scope
(
SynthDef(\monokit_plaits, {
    arg t_gate = 0,
        pitch = 131,
        detune = 0,
        engine = 0,
        harmonics = 0.5,
        timbre = 0.5,
        morph = 0.5,
        decay = 0.5,
        lpg = 0.5,
        plv = 8192,
        pav = 0;

    var freq = pitch * (2 ** (detune / 1200));

    var sig = MiPlaits.ar(
        pitch: freq.cpsmidi,
        engine: engine.clip(0, 15),
        harm: harmonics.clip(0, 1),
        timbre: timbre.clip(0, 1),
        morph: morph.clip(0, 1),
        trigger: t_gate,
        level: 0.8,
        lpg_colour: lpg.clip(0, 1),
        decay: decay.clip(0, 1)
    );

    var main = sig[0] * (plv / 16383.0);
    var aux = sig[1] * (pav / 16383.0);

    Out.ar(19, main);
    Out.ar(20, aux);
}).add;

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
}).add;

SynthDef(\monokit_scope, {
    arg scopeRate = 0.1,
        scopeGain = 1;
    var sigL, sigR, mono;
    var phase, trig;
    var buf = LocalBuf(128);

    sigL = InFeedback.ar(0);
    sigR = InFeedback.ar(1);
    mono = (sigL + sigR) * 0.5 * scopeGain;

    phase = Phasor.ar(0, scopeRate, 0, 128);
    BufWr.ar(mono, buf, phase);

    trig = Impulse.kr(20);
    SendReply.kr(trig, '/monokit/scope',
        BufRd.kr(1, buf, Array.series(128, 0, 1), 0, 1)
    );
}).add;
)
