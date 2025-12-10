(
// SynthDef Pre-compilation Script
// Compiles monokit SynthDefs to binary .scsyndef files
// Run with: sclang -D compile_synthdefs.scd

var synthdefs_dir = "../sc/synthdefs/";

// Ensure output directory exists
File.mkdir(synthdefs_dir);

"Compiling SynthDefs to binary format...".postln;

// Defaults match RST command values
SynthDef(\monokit, {
    arg pf = 131,     // Primary freq (C3)
        pw = 0,
        mf = 262,     // Mod freq (C4)
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
        pa = 0,       // No pitch envelope by default
        fa = 0,
        da = 0,
        mx = 0,
        mm = 0,
        me = 0,
        fb = 0,
        fba = 0,
        fbd = 10,
        fc = 10000,   // Filter wide open
        fq = 0,
        ft = 0,
        fe = 0,
        fed = 100,
        fk = 0,
        mf_f = 0,
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
        dmode = 2,    // Delay send mode by default
        dtail = 1,    // Ring tail behavior
        rmode = 2,    // Reverb send mode by default
        rtail = 1,
        lb = 16,
        ls = 48000,
        lm = 0,
        rgf = 440,
        rgw = 0,
        rgm = 0,
        ct = 8192,
        cr = 1,       // Compressor ratio (1:1 = off)
        ca = 10,
        cl = 100,
        cm = 0,
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
        limit = 1,
        vca_mode = 1,
        volume = 1,
        t_gate = 0,  // TrigControl - automatically resets after each control block
        slew_time = 0,
        slew_pf = -1, slew_mf = -1, slew_fc = -1, slew_fm = -1,
        slew_mx = -1, slew_dc = -1, slew_fb = -1, slew_fq = -1,
        slew_fk = -1, slew_fe = -1, slew_rf = -1, slew_rm = -1,
        slew_dt = -1, slew_df = -1, slew_dw = -1, slew_rv = -1,
        slew_rw = -1, slew_volume = -1, slew_pn = -1, slew_lb = -1,
        slew_ls = -1, slew_lm = -1, slew_rgf = -1, slew_rgm = -1,
        slew_ct = -1, slew_cm = -1, slew_el = -1, slew_em = -1,
        slew_eh = -1, slew_ef = -1,
        env_atk = 1, env_crv = -4,
        aenv_atk = -1, penv_atk = -1, fmev_atk = -1, denv_atk = -1, fbev_atk = -1, flev_atk = -1,
        aenv_crv = -100, penv_crv = -100, fmev_crv = -100, denv_crv = -100, fbev_crv = -100, flev_crv = -100;

    var trig, pitchEnv, ampEnv, fmEnv, dcEnv, fbEnv, filterEnv;
    var modBus, trackingAmount, fmIndex, dcAmount, mixAmount, fbAmount;
    var primaryFreq, modulatorFreq, modOsc;
    var fmAmount, primaryOsc, sig, amp, discontinuity;
    var filterCutoff, filterQ;
    var resonatorFreq, resonatorDelay, resonatorDecay, resonatorMix, resonated;
    var delayTime, delayFeedback, delayWet, sigL, sigR, delayedL, delayedR;
    var reverbDecay, reverbDamping, reverbWet, reverbedL, reverbedR;
    var dmodeSmooth, delayWetSmooth, dfeedbackMult, delayInput, delayBypass;
    var rmodeSmooth, reverbWetSmooth, rinputGate, rroomSize, reverbInput;
    var lofi, lofiBits, lofiRate, lofiMix;
    var ringMod, ringModOsc, ringModMix;
    var compressed, compThresh, compRatio;
    var eqSig;
    var panPos;
    var beatRepeatBufL, beatRepeatBufR, brBufSize, brWritePos, brReadPos, brLoopLen;
    var brReadRate, brRepeatedL, brRepeatedR, brMix, brWindowSize, brCrossfade;
    var brActive;
    var psRatio, psGrainSize, psShifted, psMix;
    var psInputL, psInputR;
    var pfSmooth, mfSmooth, fcSmooth, fmSmooth, mxSmooth, dcSmooth, fbSmooth, fqSmooth, fkSmooth, feSmooth;
    var rfSmooth, rmSmooth, dtSmooth, dfSmooth, dwSmooth, rvSmooth, rwSmooth, volumeSmooth, panSmooth;
    var lbSmooth, lsSmooth, lmSmooth, rgfSmooth, rgmSmooth, ctSmooth, cmSmooth;
    var elSmooth, emSmooth, ehSmooth, efSmooth;
    var paCtl, faCtl, daCtl, fbaCtl;
    var aAtk, aCrv, pAtk, pCrv, fmAtk, fmCrv, dAtk, dCrv, fbAtk, fbCrv, flAtk, flCrv;

    paCtl = Lag.kr(pa, 0);
    faCtl = Lag.kr(fa, 0);
    daCtl = Lag.kr(da, 0);
    fbaCtl = Lag.kr(fba, 0);

    pfSmooth = Lag.kr(pf, Select.kr(slew_pf >= 0, [slew_time, slew_pf]));
    mfSmooth = Lag.kr(mf, Select.kr(slew_mf >= 0, [slew_time, slew_mf]));
    fcSmooth = Lag.kr(fc, Select.kr(slew_fc >= 0, [slew_time, slew_fc]));
    fmSmooth = Lag.kr(fm, Select.kr(slew_fm >= 0, [slew_time, slew_fm]));
    mxSmooth = Lag.kr(mx, Select.kr(slew_mx >= 0, [slew_time, slew_mx]));
    dcSmooth = Lag.kr(dc, Select.kr(slew_dc >= 0, [slew_time, slew_dc]));
    fbSmooth = Lag.kr(fb, Select.kr(slew_fb >= 0, [slew_time, slew_fb]));
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
    pAtk = Select.kr(penv_atk >= 0, [env_atk, penv_atk]) / 1000;
    fmAtk = Select.kr(fmev_atk >= 0, [env_atk, fmev_atk]) / 1000;
    dAtk = Select.kr(denv_atk >= 0, [env_atk, denv_atk]) / 1000;
    fbAtk = Select.kr(fbev_atk >= 0, [env_atk, fbev_atk]) / 1000;
    flAtk = Select.kr(flev_atk >= 0, [env_atk, flev_atk]) / 1000;

    aCrv = Select.kr(aenv_crv > -100, [env_crv, aenv_crv]);
    pCrv = Select.kr(penv_crv > -100, [env_crv, penv_crv]);
    fmCrv = Select.kr(fmev_crv > -100, [env_crv, fmev_crv]);
    dCrv = Select.kr(denv_crv > -100, [env_crv, denv_crv]);
    fbCrv = Select.kr(fbev_crv > -100, [env_crv, fbev_crv]);
    flCrv = Select.kr(flev_crv > -100, [env_crv, flev_crv]);

    trig = Trig1.kr(t_gate, 0.001);
    pitchEnv = EnvGen.kr(Env.perc(pAtk, pd / 1000, 1, pCrv), trig);
    ampEnv = EnvGen.kr(Env.perc(aAtk, ad / 1000, 1, aCrv), trig);
    fmEnv = EnvGen.kr(Env.perc(fmAtk, fd / 1000, 1, fmCrv), trig);
    dcEnv = EnvGen.kr(Env.perc(dAtk, dd / 1000, 1, dCrv), trig);
    fbEnv = EnvGen.kr(Env.perc(fbAtk, fbd / 1000, 1, fbCrv), trig);
    filterEnv = EnvGen.kr(Env.perc(flAtk, fed / 1000, 1, flCrv), trig);

    modBus = mb / 16383;
    trackingAmount = tk / 16383;
    fmIndex = fmSmooth / 16383;
    dcAmount = dcSmooth / 16383;
    mixAmount = mxSmooth / 16383;
    fbAmount = (fbSmooth / 16383) + (fbEnv * fbaCtl / 16383);

    // Base primary frequency (before modbus routing)
    primaryFreq = pfSmooth * (1 + (pitchEnv * paCtl));

    // Modulator frequency: base + tracking
    modulatorFreq = mfSmooth + (trackingAmount * mt * (primaryFreq - pfSmooth));

    modOsc = Select.ar(mw.clip(0, 3).round, [
        SinOsc.ar(modulatorFreq),
        LFTri.ar(modulatorFreq),
        LFSaw.ar(modulatorFreq),
        SinOscFB.ar(modulatorFreq, fbAmount * 2)
    ]);

    // Apply modbus routing to primary frequency (MP: vibrato)
    primaryFreq = primaryFreq * (1 + (modOsc * modBus * mp * 4));

    // Apply modbus routing to modulator frequency (MD: mod freq modulation)
    modulatorFreq = modulatorFreq * (1 + (modOsc * modBus * md));

    fmAmount = (fmIndex + (fmEnv * faCtl / 16383)) * 1000;
    primaryOsc = Select.ar(pw.clip(0, 2).round, [
        SinOsc.ar(primaryFreq + (modOsc * fmAmount)),
        LFTri.ar(primaryFreq + (modOsc * fmAmount)),
        LFSaw.ar(primaryFreq + (modOsc * fmAmount))
    ]);

    // MM: audio-rate mix modulation
    sig = (primaryOsc * (1 - (mixAmount + (modOsc * modBus * mm) + (ampEnv * me)).clip(0, 1)))
        + (modOsc * (mixAmount + (modOsc * modBus * mm) + (ampEnv * me)).clip(0, 1));

    discontinuity = dcAmount + (dcEnv * daCtl / 16383);
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

    // MF.F: audio-rate filter cutoff modulation
    filterCutoff = fcSmooth + ((fkSmooth / 16383) * primaryFreq) + (filterEnv * (feSmooth / 16383) * 5000) + (modOsc * modBus * mf_f * 5000);
    filterCutoff = filterCutoff.clip(20, 20000);
    filterQ = ((fqSmooth / 16383) * 0.9 + 0.1);
    sig = Select.ar(ft.clip(0, 3).round, [
        SVF.ar(sig, filterCutoff, filterQ, 1, 0, 0, 0, 0),
        SVF.ar(sig, filterCutoff, filterQ, 0, 1, 0, 0, 0),
        SVF.ar(sig, filterCutoff, filterQ, 0, 0, 1, 0, 0),
        SVF.ar(sig, filterCutoff, filterQ, 0, 0, 0, 1, 0)
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

    // MA: audio-rate amplitude modulation (tremolo)
    amp = Select.kr(vca_mode.clip(0, 1), [1.0, ampEnv]) * volumeSmooth * (1 + (modOsc * modBus * ma * 0.5));
    sig = sig * amp;

    compThresh = (ctSmooth / 16383).clip(0.001, 1);
    compRatio = 1 / cr.clip(1, 20);
    sig = Compander.ar(sig, sig,
        thresh: compThresh,
        slopeBelow: 1,
        slopeAbove: compRatio,
        clampTime: ca / 1000,
        relaxTime: cl / 1000
    );
    sig = sig * (1 + (cmSmooth / 16383 * 2));

    panPos = (panSmooth / 16383).clip(-1, 1);
    #sigL, sigR = Pan2.ar(sig, panPos);

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

"  monokit.scsyndef".postln;

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

"".postln;
"SynthDefs compiled successfully to " ++ synthdefs_dir.postln;
0.exit;
)
