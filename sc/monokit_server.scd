(
// Disable input to avoid sample rate mismatch issues
s.options.numInputBusChannels = 0;

s.waitForBoot {
    SynthDef(\monokit, {
        arg pf = 200,
            pw = 0,
            mf = 50,
            mw = 0,
            dc = 0,
            dm = 0,
            tk = 0,
            mb = 0,
            mp = 0,
            md = 0,
            mt = 0,
            ma = 0,
            fm = 0,
            ad = 100,
            pd = 10,
            fd = 10,
            dd = 10,
            pa = 4,
            fa = 0,
            da = 0,
            mx = 0,
            mm = 0,
            me = 0,
            volume = 1,
            gate = 0;

        // All var declarations at top
        var trig, pitchEnv, ampEnv, fmEnv, dcEnv;
        var modBus, trackingAmount, fmIndex, dcAmount, mixAmount;
        var primaryFreq, modulatorFreq, modOsc;
        var fmAmount, primaryOsc, sig, amp, discontinuity;

        // Envelope model: output = base + (envelope * amount)
        // Exception: ampEnv is multiplicative (0→1 gating)

        // Trigger and envelopes
        trig = Trig1.kr(gate, 0.001);
        pitchEnv = EnvGen.kr(Env.perc(0.001, pd / 1000, 1, -4), trig);
        ampEnv = EnvGen.kr(Env.perc(0.001, ad / 1000, 1, -4), trig);
        fmEnv = EnvGen.kr(Env.perc(0.001, fd / 1000, 1, -4), trig);
        dcEnv = EnvGen.kr(Env.perc(0.001, dd / 1000, 1, -4), trig);

        // Scaled parameters
        modBus = mb / 16383;
        trackingAmount = tk / 16383;
        fmIndex = fm / 16383;
        dcAmount = dc / 16383;
        mixAmount = mx / 16383;

        // Primary frequency: base + (pitchEnv * amount) + (modBus * routing)
        primaryFreq = pf * (1 + (pitchEnv * pa) + (modBus * mp * 4));

        // Modulator frequency: base + (tracking) + (modBus * routing)
        modulatorFreq = mf + (trackingAmount * mt * (pf * (1 + (pitchEnv * pa)) - pf));
        modulatorFreq = modulatorFreq * (1 + (modBus * md));

        // Modulator oscillator
        modOsc = Select.ar(mw.clip(0, 2).round, [
            SinOsc.ar(modulatorFreq),
            LFTri.ar(modulatorFreq),
            LFSaw.ar(modulatorFreq)
        ]);

        // Primary oscillator with FM: base + (envelope * amount)
        fmAmount = (fmIndex + (fmEnv * fa / 16383)) * 1000;
        primaryOsc = Select.ar(pw.clip(0, 2).round, [
            SinOsc.ar(primaryFreq + (modOsc * fmAmount)),
            LFTri.ar(primaryFreq + (modOsc * fmAmount)),
            LFSaw.ar(primaryFreq + (modOsc * fmAmount))
        ]);

        // Secondary oscillator mix: base + (modBus * routing) + (ampEnv * env amount)
        // Note: mix envelope (me) uses ampEnv for gating behavior
        sig = (primaryOsc * (1 - (mixAmount + (modBus * mm) + (ampEnv * me)).clip(0, 1)))
            + (modOsc * (mixAmount + (modBus * mm) + (ampEnv * me)).clip(0, 1));

        // Discontinuity: base + (envelope * amount)
        discontinuity = dcAmount + (dcEnv * da / 16383);
        sig = sig + (modOsc * discontinuity);
        sig = Select.ar(dm.clip(0, 2).round, [
            sig.fold2(1),
            sig.tanh,
            sig.softclip
        ]);

        // Amplitude: base + (modBus * routing)
        amp = ampEnv * volume * (1 + (modBus * ma * 0.5));

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

    OSCdef(\monokit_param, { |msg|
        var paramName = msg[1].asSymbol;
        var paramValue = msg[2].asFloat;
        ~voice.set(paramName, paramValue);
    }, '/monokit/param');

    "monokit server ready".postln;
};
)
