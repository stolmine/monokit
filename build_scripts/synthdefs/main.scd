{ |synthdefs_dir|
SynthDef(\monokit_main, {
    arg pf = 131,
        mf = 262,
        mv = 0,
        dc = 0,
        dm = 0,
        mb = 0,
        mba = 0,
        ma = 0,
        mc = 0,
        mq = 0,
        da = 0,
        fc = 10000,
        fq = 0,
        ft = 0,
        fe = 0,
        fk = 0,
        mf_f = 0,
        mf_q = 0,
        rf = 440,
        rd = 500,
        rm = 0,
        rk = 0,
        dt = 250,
        df = 0,
        dlp = 5000,
        dw = 0,
        ds = 0,
        rv = 0,
        rp = 0,
        rh = 8000,
        rw = 0,
        dmode = 2,
        dtail = 1,
        rmode = 2,
        rtail = 1,
        lb = 16,
        ls = 48000,
        lm = 0,
        rgf = 440,
        rgw = 0,
        rgm = 0,
        ct = 8192,
        cr = 1,
        ca = 10,
        cl = 100,
        cm = 0,
        cr_mix = 16383,
        cr_auto = 1,
        el = 0,
        em = 0,
        ef = 1000,
        eq = 1,
        eh = 0,
        pn = 0,
        br_len = 2,
        br_len_ms = 250,
        br_rev = 0,
        br_win = 5,
        br_mix = 0,
        ps_mode = 0,
        ps_semi = 0,
        ps_grain = 20,
        ps_mix = 0,
        ps_targ = 0,
        t_cl_trig = 0,
        cl_pitch = 8192,
        cl_pos = 8192,
        cl_size = 8192,
        cl_dens = 8192,
        cl_tex = 8192,
        cl_wet = 0,
        cl_gain = 8192,
        cl_spread = 8192,
        cl_rvb = 0,
        cl_fb = 0,
        cl_freeze = 0,
        cl_mode = 0,
        cl_lofi = 0,
        limit = 1,
        vca_mode = 1,
        volume = 1,
        t_gate = 0,
        slew_time = 0,
        slew_fc = -1, slew_dc = -1, slew_fq = -1,
        slew_fk = -1, slew_fe = -1, slew_rf = -1, slew_rm = -1,
        slew_dt = -1, slew_df = -1, slew_dw = -1, slew_rv = -1,
        slew_rw = -1, slew_volume = -1, slew_pn = -1, slew_lb = -1,
        slew_ls = -1, slew_lm = -1, slew_rgf = -1, slew_rgm = -1,
        slew_ct = -1, slew_cm = -1, slew_el = -1, slew_em = -1,
        slew_eh = -1, slew_ef = -1,
        env_atk = 1, env_crv = -4,
        aenv_atk = -1, denv_atk = -1, flev_atk = -1,
        aenv_crv = -100, denv_crv = -100, flev_crv = -100,
        primaryBus = 16,
        modBus = 17,
        noiseBus = 18,
        plaitsMainBus = 19,
        plaitsAuxBus = 20,
        samplerBus = 21,
        vol_osc = 16383,
        vol_pla = 16383,
        vol_nos = 16383,
        vol_smp = 16383,
        pan_osc = 0,
        pan_pla = 0,
        pan_nos = 0,
        pan_smp = 0,
        mute_osc = 0,
        mute_pla = 0,
        mute_nos = 0,
        mute_smp = 0;

    var trig, ampEnv, dcEnv, filterEnv, modBusEnv;
    var modBusValue, dcAmount, mvSmooth;
    var primaryFreq, modOsc;
    var sig, amp, discontinuity;
    var primaryIn, modIn, noiseIn, plaitsMainIn, plaitsAuxIn, samplerIn;
    var oscVol, plaVol, nosVol, smpVol, oscPan, plaPan, nosPan, smpPan;
    var oscMuted, plaMuted, nosMuted, smpMuted;
    var primaryL, primaryR, noiseL, noiseR, plaitsMainL, plaitsMainR, plaitsAuxL, plaitsAuxR;
    var samplerL, samplerR;
    var filterCutoff, filterQ;
    var resonatorFreq, resonatorDelay, resonatorDecay, resonatorMix, resonated;
    var delayTime, delayFeedback, delayWet, sigL, sigR, delayedL, delayedR;
    var reverbDecay, reverbDamping, reverbWet, reverbedL, reverbedR;
    var dmodeSmooth, delayWetSmooth, dfeedbackMult, delayInput, delayBypass;
    var rmodeSmooth, reverbWetSmooth, rinputGate, rroomSize, reverbInput;
    var lofi, lofiBits, lofiRate, lofiMix;
    var ringMod, ringModOsc, ringModMix;
    var drySig, wetSig, compressed, compThresh, compRatio, compMixAmt, autoMakeup;
    var eqSig;
    var panPos;
    var beatRepeatBufL, beatRepeatBufR, brBufSize, brWritePos, brReadPos, brLoopLen;
    var brReadRate, brRepeatedL, brRepeatedR, brMix, brWindowSize, brCrossfade;
    var brActive;
    var psRatio, psGrainSize, psShifted, psMix;
    var psInputL, psInputR;
    var cloudsProcessedL, cloudsProcessedR, cloudsActive, cloudsWet;
    var compInputLevel, compOutputLevel, compFinalLevel, gainReductionDb;
    var fcSmooth, dcSmooth, fqSmooth, fkSmooth, feSmooth;
    var rfSmooth, rmSmooth, dtSmooth, dfSmooth, dwSmooth, rvSmooth, rwSmooth, volumeSmooth, panSmooth;
    var lbSmooth, lsSmooth, lmSmooth, rgfSmooth, rgmSmooth, ctSmooth, cmSmooth;
    var elSmooth, emSmooth, ehSmooth, efSmooth;
    var daCtl;
    var aAtk, aCrv, dAtk, dCrv, flAtk, flCrv;
    var adDynamic, ddDynamic, fedDynamic, mbdDynamic;

    daCtl = Lag.kr(da, 0);

    fcSmooth = Lag.kr(fc, Select.kr(slew_fc >= 0, [slew_time, slew_fc]));
    dcSmooth = Lag.kr(dc, Select.kr(slew_dc >= 0, [slew_time, slew_dc]));
    fqSmooth = Lag.kr(fq, Select.kr(slew_fq >= 0, [slew_time, slew_fq]));
    fkSmooth = Lag.kr(fk, Select.kr(slew_fk >= 0, [slew_time, slew_fk]));
    feSmooth = Lag.kr(fe, Select.kr(slew_fe >= 0, [slew_time, slew_fe]));
    rfSmooth = Lag.kr(rf, Select.kr(slew_rf >= 0, [slew_time, slew_rf]));
    rmSmooth = Lag.kr(rm, Select.kr(slew_rm >= 0, [slew_time, slew_rm]));
    dtSmooth = Lag.kr(dt, Select.kr(slew_dt >= 0, [slew_time, slew_dt]));
    dfSmooth = Lag.kr(df, Select.kr(slew_df >= 0, [slew_time, slew_df]));
    dwSmooth = Lag.kr(dw, Select.kr(slew_dw >= 0, [slew_time, slew_dw]));
    rvSmooth = Lag.kr(rv, Select.kr(slew_rv >= 0, [slew_time, slew_rv]));
    rwSmooth = Lag.kr(rw, Select.kr(slew_rw >= 0, [slew_time, slew_rw]));
    volumeSmooth = Lag.kr(volume, Select.kr(slew_volume >= 0, [slew_time, slew_volume]));
    panSmooth = Lag.kr(pn, Select.kr(slew_pn >= 0, [slew_time, slew_pn]));
    lbSmooth = Lag.kr(lb, Select.kr(slew_lb >= 0, [slew_time, slew_lb]));
    lsSmooth = Lag.kr(ls, Select.kr(slew_ls >= 0, [slew_time, slew_ls]));
    lmSmooth = Lag.kr(lm, Select.kr(slew_lm >= 0, [slew_time, slew_lm]));
    rgfSmooth = Lag.kr(rgf, Select.kr(slew_rgf >= 0, [slew_time, slew_rgf]));
    rgmSmooth = Lag.kr(rgm, Select.kr(slew_rgm >= 0, [slew_time, slew_rgm]));
    ctSmooth = Lag.kr(ct, Select.kr(slew_ct >= 0, [slew_time, slew_ct]));
    cmSmooth = Lag.kr(cm, Select.kr(slew_cm >= 0, [slew_time, slew_cm]));
    elSmooth = Lag.kr(el, Select.kr(slew_el >= 0, [slew_time, slew_el]));
    emSmooth = Lag.kr(em, Select.kr(slew_em >= 0, [slew_time, slew_em]));
    ehSmooth = Lag.kr(eh, Select.kr(slew_eh >= 0, [slew_time, slew_eh]));
    efSmooth = Lag.kr(ef, Select.kr(slew_ef >= 0, [slew_time, slew_ef]));

    aAtk = Select.kr(aenv_atk >= 0, [env_atk, aenv_atk]) / 1000;
    dAtk = Select.kr(denv_atk >= 0, [env_atk, denv_atk]) / 1000;
    flAtk = Select.kr(flev_atk >= 0, [env_atk, flev_atk]) / 1000;

    aCrv = Select.kr(aenv_crv > -100, [env_crv, aenv_crv]);
    dCrv = Select.kr(denv_crv > -100, [env_crv, denv_crv]);
    flCrv = Select.kr(flev_crv > -100, [env_crv, flev_crv]);

    adDynamic = Lag.kr(\ad.kr(100), 0.001);
    ddDynamic = Lag.kr(\dd.kr(10), 0.001);
    fedDynamic = Lag.kr(\fed.kr(100), 0.001);
    mbdDynamic = Lag.kr(\mbd.kr(100), 0.001);

    trig = Trig1.kr(t_gate, 0.001);
    ampEnv = EnvGen.kr(Env.perc(aAtk, adDynamic / 1000, 1, aCrv), trig);
    dcEnv = EnvGen.kr(Env.perc(dAtk, ddDynamic / 1000, 1, dCrv), trig);
    filterEnv = EnvGen.kr(Env.perc(flAtk, fedDynamic / 1000, 1, flCrv), trig);
    modBusEnv = EnvGen.kr(Env.perc(0.001, mbdDynamic / 1000, 1, -4), trig);

    primaryIn = InFeedback.ar(primaryBus, 1);
    modIn = InFeedback.ar(modBus, 1);
    noiseIn = InFeedback.ar(noiseBus, 1);
    plaitsMainIn = InFeedback.ar(plaitsMainBus, 1);
    plaitsAuxIn = InFeedback.ar(plaitsAuxBus, 1);
    samplerIn = InFeedback.ar(samplerBus, 1);

    modOsc = modIn;

    modBusValue = (mb / 16383) + (modBusEnv * mba / 16383);
    dcAmount = dcSmooth / 16383;
    mvSmooth = mv / 16383;

    primaryFreq = pf;

    oscVol = (vol_osc / 16383) * (1 - mute_osc);
    plaVol = (vol_pla / 16383) * (1 - mute_pla);
    nosVol = (vol_nos / 16383) * (1 - mute_nos);
    smpVol = (vol_smp / 16383) * (1 - mute_smp);
    oscPan = (pan_osc / 8192).clip(-1, 1);
    plaPan = (pan_pla / 8192).clip(-1, 1);
    nosPan = (pan_nos / 8192).clip(-1, 1);
    smpPan = (pan_smp / 8192).clip(-1, 1);

    sig = (primaryIn * oscVol) + (modIn * mvSmooth * oscVol);

    discontinuity = dcAmount + (dcEnv * daCtl / 16383);
    SendReply.kr(Changed.kr(dc), '/dc_changed', [dc, dm, daCtl, discontinuity]);
    sig = sig + (modOsc * discontinuity);
    sig = Select.ar(dm.clip(0, 6).round, [
        sig.fold2(1),
        sig.tanh,
        sig.softclip,
        sig.clip2(1),
        (sig.abs.pow(0.5) * sig.sign),
        sig.abs,
        (sig * 8).round / 8
    ]);

    lofiBits = lbSmooth.clip(1, 16);
    lofiRate = lsSmooth.clip(100, 48000);
    lofiMix = lmSmooth / 16383;
    lofi = (sig * (2.pow(lofiBits - 1))).round / (2.pow(lofiBits - 1));
    lofi = Latch.ar(lofi, Impulse.ar(lofiRate));
    sig = (sig * (1 - lofiMix)) + (lofi * lofiMix);

    filterCutoff = fcSmooth + ((fkSmooth / 16383) * primaryFreq) + (filterEnv * (feSmooth / 16383) * 5000) + (modOsc * modBusValue * mf_f * 5000);
    filterCutoff = filterCutoff.clip(20, 20000);
    sig = Select.ar(ft.clip(0, 13).round, [
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            1, 0, 0, 0, 0),
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 1, 0, 0, 0),
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 0, 1, 0, 0),
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 0, 0, 1, 0),
        MoogFF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 4.0) + (modOsc * modBusValue * mf_q * 2.0)).clip(0, 4)),
        RLPF.ar(sig, filterCutoff,
            ((1.0 - (fqSmooth / 16383) * 0.95) - (modOsc * modBusValue * mf_q * 0.5)).clip(0.05, 1.0)),
        RHPF.ar(sig, filterCutoff,
            ((1.0 - (fqSmooth / 16383) * 0.95) - (modOsc * modBusValue * mf_q * 0.5)).clip(0.05, 1.0)),
        DFM1.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 1.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 1.1),
            1.0, 0.0),
        DFM1.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 1.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 1.1),
            0.0, 1.0),
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 0.95),
            1, 0),
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 0.95),
            0, 1),
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 0.95),
            2, 0),
        SVF.ar(Latch.ar(sig, Impulse.ar((filterCutoff / 8).clip(100, 20000))), filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            1, 0, 0, 0, 0),
        SVF.ar(Latch.ar(sig, Impulse.ar((filterCutoff / 8).clip(100, 20000))), filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 1, 0, 0, 0)
    ]);

    ringModOsc = Select.ar(rgw.clip(0, 3).round, [
        SinOsc.ar(rgfSmooth),
        LFTri.ar(rgfSmooth),
        LFSaw.ar(rgfSmooth),
        LFPulse.ar(rgfSmooth, 0, 0.5, 2, -1)
    ]);
    ringModMix = rgmSmooth / 16383;
    sig = (sig * (1 - ringModMix)) + ((sig * ringModOsc) * ringModMix);

    resonatorFreq = rfSmooth + ((rk / 16383) * (primaryFreq - rfSmooth));
    resonatorFreq = resonatorFreq.clip(20, 5000);
    resonatorDelay = (1 / resonatorFreq).clip(0.0002, 0.05);
    resonatorDecay = (rd / 1000).clip(0.01, 5.0);
    resonated = CombC.ar(sig, 0.05, resonatorDelay, resonatorDecay);
    resonatorMix = rmSmooth / 16383;
    sig = (sig * (1 - resonatorMix)) + (resonated * resonatorMix);

    amp = Select.kr(vca_mode.clip(0, 1), [1.0, ampEnv]) * volumeSmooth * (1 + (modOsc * modBusValue * ma * 0.5));
    sig = sig * amp;

    #primaryL, primaryR = Pan2.ar(sig, oscPan);

    #noiseL, noiseR = Pan2.ar(noiseIn * nosVol, nosPan);

    #plaitsMainL, plaitsMainR = Pan2.ar(plaitsMainIn * plaVol, plaPan);
    #plaitsAuxL, plaitsAuxR = Pan2.ar(plaitsAuxIn * plaVol, plaPan);

    #samplerL, samplerR = Pan2.ar(samplerIn * smpVol, smpPan);

    SendPeakRMS.kr(primaryL, 20, 3, '/monokit/voice_osc', 1);
    SendPeakRMS.kr(primaryR, 20, 3, '/monokit/voice_osc', 2);
    SendPeakRMS.kr(plaitsMainL + plaitsAuxL, 20, 3, '/monokit/voice_pla', 1);
    SendPeakRMS.kr(plaitsMainR + plaitsAuxR, 20, 3, '/monokit/voice_pla', 2);
    SendPeakRMS.kr(noiseL, 20, 3, '/monokit/voice_nos', 1);
    SendPeakRMS.kr(noiseR, 20, 3, '/monokit/voice_nos', 2);
    SendPeakRMS.kr(samplerL, 20, 3, '/monokit/voice_smp', 1);
    SendPeakRMS.kr(samplerR, 20, 3, '/monokit/voice_smp', 2);

    sigL = (primaryL + noiseL + plaitsMainL + plaitsAuxL + samplerL) * 0.5;
    sigR = (primaryR + noiseR + plaitsMainR + plaitsAuxR + samplerR) * 0.5;

    compThresh = (ctSmooth / 16383).clip(0.001, 1);
    compRatio = 1 / cr.clip(1, 20);

    drySig = (sigL + sigR) * 0.5;
    wetSig = Compander.ar([sigL, sigR], drySig,
        thresh: compThresh,
        slopeBelow: 1,
        slopeAbove: compRatio,
        clampTime: ca / 1000,
        relaxTime: cl / 1000
    );

    compInputLevel = Amplitude.kr(drySig, 0.01, 0.1);
    compOutputLevel = Amplitude.kr((wetSig[0] + wetSig[1]) * 0.5, 0.01, 0.1);
    gainReductionDb = (compOutputLevel / compInputLevel.max(0.0001)).max(0.0001).ampdb.max(-40).min(0);

    autoMakeup = Select.kr(cr_auto, [
        1 + (cmSmooth / 16383 * 2),
        Lag.kr(gainReductionDb.max(-40), 0.3).neg.dbamp.clip(1, 4)
    ]);
    wetSig = wetSig * autoMakeup;

    compFinalLevel = Amplitude.kr((wetSig[0] + wetSig[1]) * 0.5, 0.01, 0.1);

    compMixAmt = Lag.kr(cr_mix / 16383, 0.01);
    sigL = XFade2.ar(sigL, wetSig[0], compMixAmt * 2 - 1);
    sigR = XFade2.ar(sigR, wetSig[1], compMixAmt * 2 - 1);

    SendReply.kr(Impulse.kr(20), '/monokit/comp', [compInputLevel, compFinalLevel, gainReductionDb]);

    panPos = (panSmooth / 16383).clip(-1, 1);
    #sigL, sigR = Balance2.ar(sigL, sigR, panPos);

    cloudsWet = Lag.kr(cl_wet / 16383, 0.01);
    cloudsActive = cloudsWet > 0;

    #cloudsProcessedL, cloudsProcessedR = MiClouds.ar(
        [sigL, sigR],
        pit: (cl_pitch / 16383).clip(0, 1),
        pos: (cl_pos / 16383).clip(0, 1),
        size: (cl_size / 16383).clip(0, 1),
        dens: (cl_dens / 16383).clip(0, 1),
        tex: (cl_tex / 16383).clip(0, 1),
        drywet: cloudsWet,
        in_gain: (cl_gain / 16383).clip(0, 2),
        spread: (cl_spread / 16383).clip(0, 1),
        rvb: (cl_rvb / 16383).clip(0, 1),
        fb: (cl_fb / 16383).clip(0, 1),
        freeze: cl_freeze.clip(0, 1),
        mode: cl_mode.clip(0, 3).round,
        lofi: (cl_lofi / 16383).clip(0, 1),
        trig: t_cl_trig
    );

    sigL = Select.ar(cloudsActive, [sigL, cloudsProcessedL * 3]);
    sigR = Select.ar(cloudsActive, [sigR, cloudsProcessedR * 3]);

    brActive = Lag.kr(br_mix > 0, 0.01);
    brLoopLen = br_len_ms / 1000;
    brBufSize = (brLoopLen * SampleRate.ir).ceil.max(4410);

    beatRepeatBufL = LocalBuf(brBufSize, 1);
    beatRepeatBufR = LocalBuf(brBufSize, 1);

    brWritePos = Phasor.ar(0, 1 - brActive, 0, brBufSize);
    BufWr.ar(sigL, beatRepeatBufL, brWritePos, 0);
    BufWr.ar(sigR, beatRepeatBufR, brWritePos, 0);

    brReadRate = Select.kr(br_rev.clip(0, 1), [1, -1]);
    brReadPos = Phasor.ar(
        brActive,
        brReadRate,
        0,
        brBufSize,
        Select.kr(br_rev.clip(0, 1), [0, brBufSize])
    );

    brWindowSize = (br_win / 1000 * SampleRate.ir).clip(1, brBufSize * 0.5);
    brCrossfade = 1 - (
        min(brReadPos, brBufSize - brReadPos) / brWindowSize
    ).clip(0, 1).sqrt;

    brRepeatedL = BufRd.ar(1, beatRepeatBufL, brReadPos, 0, 4);
    brRepeatedR = BufRd.ar(1, beatRepeatBufR, brReadPos, 0, 4);

    brMix = (br_mix / 16383).clip(0, 1);
    sigL = Select.ar(brActive, [
        sigL,
        (sigL * (1 - brMix)) + (brRepeatedL * brMix)
    ]);
    sigR = Select.ar(brActive, [
        sigR,
        (sigR * (1 - brMix)) + (brRepeatedR * brMix)
    ]);

    psInputL = Select.ar(ps_targ.clip(0, 1), [
        sigL,
        brRepeatedL * brMix
    ]);
    psInputR = Select.ar(ps_targ.clip(0, 1), [
        sigR,
        brRepeatedR * brMix
    ]);

    psRatio = (ps_semi / 12).midiratio;
    psGrainSize = (ps_grain / 1000).clip(0.005, 0.1);

    psShifted = Select.ar(ps_mode.clip(0, 1), [
        [
            PitchShift.ar(
                psInputL,
                psGrainSize,
                psRatio,
                0,
                0.1
            ),
            PitchShift.ar(
                psInputR,
                psGrainSize,
                psRatio,
                0,
                0.1
            )
        ],
        [
            FreqShift.ar(psInputL, ps_semi * 100),
            FreqShift.ar(psInputR, ps_semi * 100)
        ]
    ]);

    psMix = (ps_mix / 16383).clip(0, 1);
    sigL = Select.ar(ps_targ.clip(0, 1), [
        (sigL * (1 - psMix)) + (psShifted[0] * psMix),
        sigL + (psShifted[0] * psMix)
    ]);
    sigR = Select.ar(ps_targ.clip(0, 1), [
        (sigR * (1 - psMix)) + (psShifted[1] * psMix),
        sigR + (psShifted[1] * psMix)
    ]);

    dmodeSmooth = Lag.kr(dmode, 0.02);
    delayWetSmooth = Lag.kr(dwSmooth / 16383, 0.01);
    delayTime = (dtSmooth / 1000).clip(0.001, 2.0);
    delayFeedback = dfSmooth / 16383;

    dfeedbackMult = Select.kr(dtail.clip(0, 2).round, [
        (dwSmooth > 100),
        1,
        0.999
    ]);

    delayInput = Select.ar(dmodeSmooth.clip(0, 2).round, [
        sigL,
        sigL,
        sigL * delayWetSmooth
    ]);

    delayedL = CombC.ar(LPF.ar(delayInput, dlp), 2.0, delayTime, delayFeedback * 3 * dfeedbackMult);
    delayedR = CombC.ar(LPF.ar(Select.ar(dmodeSmooth.clip(0, 2).round, [sigR, sigR, sigR * delayWetSmooth]), dlp), 2.0, delayTime * 1.02, delayFeedback * 3 * dfeedbackMult);

    sigL = Select.ar(dmodeSmooth.clip(0, 2).round, [
        sigL,
        (sigL * (1 - delayWetSmooth)) + (delayedL * delayWetSmooth),
        sigL + (delayedL * Select.kr(dtail.clip(0, 2).round, [delayWetSmooth, 1, 1]))
    ]);

    sigR = Select.ar(dmodeSmooth.clip(0, 2).round, [
        sigR,
        (sigR * (1 - delayWetSmooth)) + (delayedR * delayWetSmooth),
        sigR + (delayedR * Select.kr(dtail.clip(0, 2).round, [delayWetSmooth, 1, 1]))
    ]);

    sigL = BLowShelf.ar(sigL, 200, 1, elSmooth.clip(-24, 24));
    sigL = BPeakEQ.ar(sigL, efSmooth.clip(200, 8000), (1/eq.clip(0.1, 10)), emSmooth.clip(-24, 24));
    sigL = BHiShelf.ar(sigL, 4000, 1, ehSmooth.clip(-24, 24));
    sigR = BLowShelf.ar(sigR, 200, 1, elSmooth.clip(-24, 24));
    sigR = BPeakEQ.ar(sigR, efSmooth.clip(200, 8000), (1/eq.clip(0.1, 10)), emSmooth.clip(-24, 24));
    sigR = BHiShelf.ar(sigR, 4000, 1, ehSmooth.clip(-24, 24));

    rmodeSmooth = Lag.kr(rmode, 0.02);
    reverbWetSmooth = Lag.kr(rwSmooth / 16383, 0.01);

    rroomSize = Select.kr(rtail.clip(0, 2).round, [
        (rvSmooth / 16383).clip(0, 1),
        (rvSmooth / 16383).clip(0, 1),
        1.0
    ]);

    reverbInput = Select.ar(rmodeSmooth.clip(0, 2).round, [
        DC.ar(0),
        sigL,
        sigL * reverbWetSmooth
    ]);

    #reverbedL, reverbedR = FreeVerb2.ar(
        DelayC.ar(reverbInput, 0.1, rp / 1000),
        DelayC.ar(Select.ar(rmodeSmooth.clip(0, 2).round, [DC.ar(0), sigR, sigR * reverbWetSmooth]), 0.1, rp / 1000),
        mix: 1,
        room: rroomSize,
        damp: (rh / 16383).clip(0, 1)
    );

    sigL = Select.ar(rmodeSmooth.clip(0, 2).round, [
        sigL,
        (sigL * (1 - reverbWetSmooth)) + (reverbedL * reverbWetSmooth),
        sigL + (reverbedL * Select.kr(rtail.clip(0, 2).round, [reverbWetSmooth, 1, 1]))
    ]);

    sigR = Select.ar(rmodeSmooth.clip(0, 2).round, [
        sigR,
        (sigR * (1 - reverbWetSmooth)) + (reverbedR * reverbWetSmooth),
        sigR + (reverbedR * Select.kr(rtail.clip(0, 2).round, [reverbWetSmooth, 1, 1]))
    ]);

    #sigL, sigR = Select.ar(limit.clip(0, 1).round, [
        [sigL, sigR],
        Limiter.ar([sigL, sigR], 1.0, 0.01)
    ]);

    sigL = LeakDC.ar(sigL);
    sigR = LeakDC.ar(sigR);

    SendPeakRMS.kr(sigL, 20, 3, '/monokit/meter', 1);
    SendPeakRMS.kr(sigR, 20, 3, '/monokit/meter', 2);

    Out.ar(0, [sigL, sigR]);
}).writeDefFile(synthdefs_dir);

"  monokit_main.scsyndef".postln;
}.value;
