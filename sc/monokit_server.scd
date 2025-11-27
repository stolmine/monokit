(
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
            ad = 0.1,
            pd = 0.01,
            fd = 0.01,
            pa = 4,
            volume = 1,
            gate = 0;

        // All var declarations at top
        var trig, pitchEnv, ampEnv, fmEnv;
        var modBus, trackingAmount, fmIndex, dcAmount;
        var primaryFreq, modulatorFreq, modOsc;
        var fmAmount, primaryOsc, sig, amp;

        // Trigger and envelopes
        trig = Trig1.kr(gate, 0.001);
        pitchEnv = EnvGen.kr(Env.perc(0.001, pd, 1, -4), trig);
        ampEnv = EnvGen.kr(Env.perc(0.001, ad, 1, -4), trig);
        fmEnv = EnvGen.kr(Env.perc(0.001, fd, 1, -4), trig);

        // Scaled parameters
        modBus = mb / 16383;
        trackingAmount = tk / 16383;
        fmIndex = fm / 16383;
        dcAmount = dc / 16383;

        // Primary frequency with pitch envelope
        primaryFreq = pf * (1 + (pitchEnv * pa));
        primaryFreq = Select.kr(mp > 0.5, [primaryFreq, primaryFreq * (1 + (modBus * 4))]);

        // Modulator frequency with tracking
        modulatorFreq = Select.kr(mt > 0.5, [mf, mf + (trackingAmount * (primaryFreq - pf))]);
        modulatorFreq = modulatorFreq * (1 + (modBus * md));

        // Modulator oscillator
        modOsc = Select.ar(mw.clip(0, 2).round, [
            SinOsc.ar(modulatorFreq),
            LFTri.ar(modulatorFreq),
            LFSaw.ar(modulatorFreq)
        ]);

        // Primary oscillator with FM
        fmAmount = fmIndex * fmEnv * 1000;
        primaryOsc = Select.ar(pw.clip(0, 2).round, [
            SinOsc.ar(primaryFreq + (modOsc * fmAmount)),
            LFTri.ar(primaryFreq + (modOsc * fmAmount)),
            LFSaw.ar(primaryFreq + (modOsc * fmAmount))
        ]);

        // Mix and discontinuity
        sig = primaryOsc + (modOsc * dcAmount);
        sig = Select.ar(dm.clip(0, 2).round, [
            sig.fold2(1),
            sig.tanh,
            sig.softclip
        ]);

        // Amplitude
        amp = ampEnv * volume;
        amp = Select.kr(ma > 0.5, [amp, amp * (1 + (modBus * 0.5))]);

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
