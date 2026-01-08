{ |synthdefs_dir|
SynthDef(\monokit_noise, {
    arg nw = 0,
        nv = 0,
        noiseBus = 18;

    var nwVal, nvSmooth, nvDynamic;
    var noiseRaw;

    nwVal = nw.clip(0, 2).round;
    nvDynamic = Lag.kr(nv, 0.0001);
    nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);

    noiseRaw = Select.ar(nwVal, [
        WhiteNoise.ar(1),
        PinkNoise.ar(1),
        BrownNoise.ar(1)
    ]);

    Out.ar(noiseBus, noiseRaw * nvSmooth);
}).writeDefFile(synthdefs_dir);

"  monokit_noise.scsyndef".postln;

SynthDef(\monokit_mod, {
    arg mf = 262,
        mw = 0,
        mv = 0,
        fb = 0,
        fba = 0,
        fbd = 10,
        nm = 0,
        mb = 0,
        mba = 0,
        mbd = 100,
        md = 0,
        t_gate = 0,
        env_atk = 1, env_crv = -4,
        fbev_atk = -1, fbev_crv = -100,
        slew_time = 0,
        slew_mf = -1, slew_fb = -1,
        modBus = 17,
        noiseBus = 18;

    var trig, fbEnv, modBusEnv;
    var modBusValue, fbAmount, fbAtk, fbCrv;
    var modulatorFreq, modOsc;
    var mfSmooth, fbSmooth;
    var fbaCtl, fbdDynamic, mbdDynamic;
    var nmSmooth, mvSmooth, mvDynamic;
    var noiseIn, noiseToMod;

    fbaCtl = Lag.kr(fba, 0);
    fbdDynamic = Lag.kr(fbd, 0);
    mbdDynamic = Lag.kr(mbd, 0);

    mfSmooth = Lag.kr(mf, Select.kr(slew_mf >= 0, [slew_time, slew_mf]));
    fbSmooth = Lag.kr(fb, Select.kr(slew_fb >= 0, [slew_time, slew_fb]));
    nmSmooth = Lag.kr(nm / 16383, 0.01);
    mvDynamic = Lag.kr(mv, 0.0003);
    mvSmooth = Lag.kr(mvDynamic / 16383, 0.011);

    fbAtk = Select.kr(fbev_atk >= 0, [env_atk, fbev_atk]) / 1000;
    fbCrv = Select.kr(fbev_crv > -100, [env_crv, fbev_crv]);

    trig = Trig1.kr(t_gate, 0.001);
    fbEnv = EnvGen.kr(Env.perc(fbAtk, fbdDynamic / 1000, 1, fbCrv), trig);
    modBusEnv = EnvGen.kr(Env.perc(0.001, mbdDynamic / 1000, 1, -4), trig);

    noiseIn = InFeedback.ar(noiseBus, 1);
    noiseToMod = noiseIn * nmSmooth * 1000;

    modBusValue = (mb / 16383) + (modBusEnv * mba / 16383);
    fbAmount = (fbSmooth / 16383) + (fbEnv * fbaCtl / 16383);

    modulatorFreq = mfSmooth + noiseToMod;

    modOsc = Select.ar(mw.clip(0, 3).round, [
        SinOsc.ar(modulatorFreq),
        LFTri.ar(modulatorFreq),
        LFSaw.ar(modulatorFreq),
        SinOscFB.ar(modulatorFreq, fbAmount * 2)
    ]);

    modulatorFreq = modulatorFreq * (1 + (modOsc * modBusValue * md));

    modOsc = Select.ar(mw.clip(0, 3).round, [
        SinOsc.ar(modulatorFreq),
        LFTri.ar(modulatorFreq),
        LFSaw.ar(modulatorFreq),
        SinOscFB.ar(modulatorFreq, fbAmount * 2)
    ]);

    Out.ar(modBus, modOsc);
}).writeDefFile(synthdefs_dir);

"  monokit_mod.scsyndef".postln;
}.value;
