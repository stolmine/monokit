{ |synthdefs_dir|
SynthDef(\monokit_primary, {
    arg pf = 131,
        pw = 0,
        pv = 16383,
        fm = 0,
        fa = 0,
        fd = 10,
        pa = 0,
        pd = 10,
        np = 0,
        tk = 0,
        mt = 0,
        mp = 0,
        mb = 0,
        mba = 0,
        mbd = 100,
        t_gate = 0,
        env_atk = 1, env_crv = -4,
        penv_atk = -1, penv_crv = -100,
        fmev_atk = -1, fmev_crv = -100,
        slew_time = 0,
        slew_pf = -1, slew_fm = -1,
        primaryBus = 16,
        modBus = 17,
        noiseBus = 18;

    var trig, pitchEnv, fmEnv, modBusEnv;
    var modBusValue, trackingAmount, fmIndex, fmAmount;
    var primaryFreq, primaryOsc;
    var pfSmooth, fmSmooth;
    var paCtl, faCtl, pdDynamic, fdDynamic, mbdDynamic;
    var pAtk, pCrv, fmAtk, fmCrv;
    var npSmooth, pvSmooth, pvDynamic;
    var noiseIn, noiseToPrimary;
    var modIn;

    paCtl = Lag.kr(pa, 0);
    faCtl = Lag.kr(fa, 0);
    pdDynamic = Lag.kr(pd, 0);
    fdDynamic = Lag.kr(fd, 0);
    mbdDynamic = Lag.kr(mbd, 0);

    pfSmooth = Lag.kr(pf, Select.kr(slew_pf >= 0, [slew_time, slew_pf]));
    fmSmooth = Lag.kr(fm, Select.kr(slew_fm >= 0, [slew_time, slew_fm]));
    npSmooth = Lag.kr(np / 16383, 0.01);
    pvDynamic = Lag.kr(pv, 0.0002);
    pvSmooth = Lag.kr(pvDynamic / 16383, 0.0105);

    pAtk = Select.kr(penv_atk >= 0, [env_atk, penv_atk]) / 1000;
    pCrv = Select.kr(penv_crv > -100, [env_crv, penv_crv]);
    fmAtk = Select.kr(fmev_atk >= 0, [env_atk, fmev_atk]) / 1000;
    fmCrv = Select.kr(fmev_crv > -100, [env_crv, fmev_crv]);

    trig = Trig1.kr(t_gate, 0.001);
    pitchEnv = EnvGen.kr(Env.perc(pAtk, pdDynamic / 1000, 1, pCrv), trig);
    fmEnv = EnvGen.kr(Env.perc(fmAtk, fdDynamic / 1000, 1, fmCrv), trig);
    modBusEnv = EnvGen.kr(Env.perc(0.001, mbdDynamic / 1000, 1, -4), trig);

    noiseIn = InFeedback.ar(noiseBus, 1);
    noiseToPrimary = noiseIn * npSmooth * 1000;

    modIn = InFeedback.ar(modBus, 1);

    modBusValue = (mb / 16383) + (modBusEnv * mba / 16383);
    trackingAmount = tk / 16383;
    fmIndex = fmSmooth / 16383;

    primaryFreq = pfSmooth * (1 + (pitchEnv * paCtl));
    primaryFreq = primaryFreq + noiseToPrimary;
    primaryFreq = primaryFreq * (1 + (modIn * modBusValue * mp * 4));

    fmAmount = (fmIndex + (fmEnv * faCtl / 16383)) * 1000;
    primaryOsc = Select.ar(pw.clip(0, 2).round, [
        SinOsc.ar(primaryFreq + (modIn * fmAmount)),
        LFTri.ar(primaryFreq + (modIn * fmAmount)),
        LFSaw.ar(primaryFreq + (modIn * fmAmount))
    ]);

    Out.ar(primaryBus, primaryOsc * pvSmooth);
}).writeDefFile(synthdefs_dir);

"  monokit_primary.scsyndef".postln;
}.value;
