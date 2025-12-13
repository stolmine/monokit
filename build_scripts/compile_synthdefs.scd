(
// SynthDef Pre-compilation Script
// Compiles monokit SynthDefs to binary .scsyndef files
// Run with: sclang -D compile_synthdefs.scd

// Output to sibling sc/synthdefs directory (relative to this script's parent)
var thisDir = thisProcess.nowExecutingPath.dirname;
var synthdefs_dir = thisDir +/+ "../sc/synthdefs/";
var hasMiPlaits;

// Ensure output directory exists
File.mkdir(synthdefs_dir);
("Output directory: " ++ synthdefs_dir).postln;

// Check if MiPlaits UGen is available
"Checking for MiPlaits...".postln;
hasMiPlaits = try {
    var result = MiPlaits.notNil;
    ("  MiPlaits class check result: " ++ result).postln;
    result;
} {
    |error|
    ("  MiPlaits check failed with error: " ++ error).postln;
    false;
};

("  hasMiPlaits = " ++ hasMiPlaits).postln;

if(hasMiPlaits.not, {
    "WARNING: MiPlaits UGen not found! Plaits voice will not work.".postln;
    "Install mi-UGens from: https://github.com/v7b1/mi-UGens".postln;
}, {
    "✓ MiPlaits UGen found and will be used".postln;
});

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
        mba = 0,      // ModBus envelope amount
        mbd = 100,    // ModBus envelope decay (ms)
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
        aenv_crv = -100, penv_crv = -100, fmev_crv = -100, denv_crv = -100, fbev_crv = -100, flev_crv = -100,
        nw = 0,       // Noise waveform (0=white, 1=pink, 2=brown)
        na = 1,       // Noise envelope attack (ms)
        nd = 100,     // Noise envelope decay (ms)
        nc = -4,      // Noise envelope curve
        ne = 16383,   // Noise envelope amount (0=bypass env)
        np = 0,       // Noise -> primary FM amount
        nm = 0,       // Noise -> modulator FM amount
        nv = 0,       // Noise output level
        ng = 1,       // Noise gate mode (0=drone, 1=gated)
        pv = 16383,   // Primary volume
        mv = 0;       // Modulator volume

    var trig, pitchEnv, ampEnv, fmEnv, dcEnv, fbEnv, filterEnv, modBusEnv;
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
    var adDynamic, pdDynamic, fdDynamic, ddDynamic, fbdDynamic, fedDynamic, mbdDynamic;
    var nwVal, naSmooth, ndSmooth, ncVal, neSmooth, npSmooth, nmSmooth, nvSmooth, ngVal, pvSmooth, mvSmooth;
    var nvDynamic, pvDynamic, mvDynamic;
    var noiseRaw, noiseEnv, noiseEnvTrig, noiseLevel, noiseShaped, noiseToPrimary, noiseToMod;
    var sources, scaledSources;

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

    // Noise parameters smoothing
    naSmooth = Lag.kr(na, 0.01);
    ndSmooth = Lag.kr(nd, 0.01);
    ncVal = nc.clip(-8, 8);
    neSmooth = Lag.kr(ne / 16383, 0.01);
    npSmooth = Lag.kr(np / 16383, 0.01);
    nmSmooth = Lag.kr(nm / 16383, 0.01);

    // Force dynamic evaluation of volume params (workaround for SC optimizer bug)
    // Use different lag times and interleaved operations to prevent optimizer conflation
    nvDynamic = Lag.kr(nv, 0.0001);
    nwVal = nw.clip(0, 2).round;      // Moved here to break up sequential pattern
    pvDynamic = Lag.kr(pv, 0.0002);
    ngVal = ng.clip(0, 1).round;      // Moved here to break up pattern again
    mvDynamic = Lag.kr(mv, 0.0003);

    // Compute smoothed volume values (different lag times prevent optimizer conflation)
    nvSmooth = Lag.kr(nvDynamic / 16383, 0.01);
    pvSmooth = Lag.kr(pvDynamic / 16383, 0.0105);
    mvSmooth = Lag.kr(mvDynamic / 16383, 0.011);

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

    // Force dynamic evaluation of decay params (workaround for SC optimizer bug)
    adDynamic = Lag.kr(ad, 0);
    pdDynamic = Lag.kr(pd, 0);
    fdDynamic = Lag.kr(fd, 0);
    ddDynamic = Lag.kr(dd, 0);
    fbdDynamic = Lag.kr(fbd, 0);
    fedDynamic = Lag.kr(fed, 0);
    mbdDynamic = Lag.kr(mbd, 0);

    trig = Trig1.kr(t_gate, 0.001);
    pitchEnv = EnvGen.kr(Env.perc(pAtk, pdDynamic / 1000, 1, pCrv), trig);
    ampEnv = EnvGen.kr(Env.perc(aAtk, adDynamic / 1000, 1, aCrv), trig);
    fmEnv = EnvGen.kr(Env.perc(fmAtk, fdDynamic / 1000, 1, fmCrv), trig);
    dcEnv = EnvGen.kr(Env.perc(dAtk, ddDynamic / 1000, 1, dCrv), trig);
    fbEnv = EnvGen.kr(Env.perc(fbAtk, fbdDynamic / 1000, 1, fbCrv), trig);
    filterEnv = EnvGen.kr(Env.perc(flAtk, fedDynamic / 1000, 1, flCrv), trig);
    modBusEnv = EnvGen.kr(Env.perc(0.001, mbdDynamic / 1000, 1, -4), trig);

    // Noise generator (white/pink/brown)
    noiseRaw = Select.ar(nwVal, [
        WhiteNoise.ar(1),
        PinkNoise.ar(1),
        BrownNoise.ar(1)
    ]);

    // Simplified noise - just volume controlled like oscillators
    // No envelope or gate modes - noise is always constant
    noiseShaped = noiseRaw;

    // Noise modulation amounts for oscillators (independent of NV)
    noiseToPrimary = noiseShaped * npSmooth * 1000;
    noiseToMod = noiseShaped * nmSmooth * 1000;

    // ModBus: base + (envelope * amount)
    modBus = (mb / 16383) + (modBusEnv * mba / 16383);
    trackingAmount = tk / 16383;
    fmIndex = fmSmooth / 16383;
    dcAmount = dcSmooth / 16383;
    mixAmount = mxSmooth / 16383;
    fbAmount = (fbSmooth / 16383) + (fbEnv * fbaCtl / 16383);

    // Base primary frequency (before modbus routing)
    primaryFreq = pfSmooth * (1 + (pitchEnv * paCtl));

    // Modulator frequency: base + tracking
    modulatorFreq = mfSmooth + (trackingAmount * mt * (primaryFreq - pfSmooth));

    // Apply noise modulation to oscillator frequencies (before oscillator generation)
    primaryFreq = primaryFreq + noiseToPrimary;
    modulatorFreq = modulatorFreq + noiseToMod;

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

    // Array-based source expansion (Option 3)
    // Force multichannel expansion to prevent optimizer conflation
    sources = [primaryOsc, modOsc, noiseShaped];

    // Apply volumes using separate array elements to force distinct UGen instances
    scaledSources = [
        sources[0] * pvSmooth,
        sources[1] * mvSmooth,
        sources[2] * nvSmooth
    ];

    // Mix all sources
    sig = Mix.ar(scaledSources);

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

    // Multi-Mode Filter (FT 0-13)
    // MC: audio-rate filter cutoff modulation (modOsc * modBus * toggle * range)
    filterCutoff = fcSmooth + ((fkSmooth / 16383) * primaryFreq) + (filterEnv * (feSmooth / 16383) * 5000) + (modOsc * modBus * mf_f * 5000);
    filterCutoff = filterCutoff.clip(20, 20000);
    // MQ: audio-rate filter resonance modulation (base resonance calculation per filter type)
    sig = Select.ar(ft.clip(0, 13).round, [
        // 0: SVF LP
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBus * mf_q * 0.5)).clip(0.1, 1.0),
            1, 0, 0, 0, 0),
        // 1: SVF HP
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBus * mf_q * 0.5)).clip(0.1, 1.0),
            0, 1, 0, 0, 0),
        // 2: SVF BP
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBus * mf_q * 0.5)).clip(0.1, 1.0),
            0, 0, 1, 0, 0),
        // 3: SVF Notch
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBus * mf_q * 0.5)).clip(0.1, 1.0),
            0, 0, 0, 1, 0),
        // 4: MoogFF (Moog ladder LP)
        MoogFF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 4.0) + (modOsc * modBus * mf_q * 2.0)).clip(0, 4)),
        // 5: RLPF (resonant LP 12dB)
        RLPF.ar(sig, filterCutoff,
            ((1.0 - (fqSmooth / 16383) * 0.95) - (modOsc * modBus * mf_q * 0.5)).clip(0.05, 1.0)),
        // 6: RHPF (resonant HP 12dB)
        RHPF.ar(sig, filterCutoff,
            ((1.0 - (fqSmooth / 16383) * 0.95) - (modOsc * modBus * mf_q * 0.5)).clip(0.05, 1.0)),
        // 7: DFM1 LP (diode filter LP)
        DFM1.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 1.1) + (modOsc * modBus * mf_q * 0.5)).clip(0, 1.1),
            1.0, 0.0),
        // 8: DFM1 HP (diode filter HP)
        DFM1.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 1.1) + (modOsc * modBus * mf_q * 0.5)).clip(0, 1.1),
            0.0, 1.0),
        // 9: BMoog LP (24dB saturating LP)
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBus * mf_q * 0.5)).clip(0, 0.95),
            1, 0),
        // 10: BMoog HP (24dB saturating HP)
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBus * mf_q * 0.5)).clip(0, 0.95),
            0, 1),
        // 11: BMoog BP (24dB saturating BP)
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBus * mf_q * 0.5)).clip(0, 0.95),
            2, 0),
        // 12: Latch-SC LP (switched-capacitor style LP)
        // Clock rate: filterCutoff / 8 gives aliasing at low freqs, cleaner at high freqs
        // Clamped to 100-20000 Hz to stay within reasonable audio range
        SVF.ar(Latch.ar(sig, Impulse.ar((filterCutoff / 8).clip(100, 20000))), filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBus * mf_q * 0.5)).clip(0.1, 1.0),
            1, 0, 0, 0, 0),
        // 13: Latch-SC HP (switched-capacitor style HP)
        SVF.ar(Latch.ar(sig, Impulse.ar((filterCutoff / 8).clip(100, 20000))), filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBus * mf_q * 0.5)).clip(0.1, 1.0),
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

// MULTI-SYNTH ARCHITECTURE - 4 separate SynthDefs for bug fix
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

    Out.ar(modBus, modOsc * mvSmooth);
}).writeDefFile(synthdefs_dir);

"  monokit_mod.scsyndef".postln;

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

// Conditional Plaits SynthDef - only use MiPlaits if available
if(hasMiPlaits, {
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
    }).writeDefFile(synthdefs_dir);
    "  monokit_plaits.scsyndef (with MiPlaits)".postln;
}, {
    SynthDef(\monokit_plaits, {
        arg t_gate = 0, pitch = 131, detune = 0, engine = 0, harmonics = 0.5,
            timbre = 0.5, morph = 0.5, decay = 0.5, lpg = 0.5, plv = 8192, pav = 0;
        Out.ar(19, Silent.ar);
        Out.ar(20, Silent.ar);
    }).writeDefFile(synthdefs_dir);
    "  monokit_plaits.scsyndef (STUB - MiPlaits not available)".postln;
});

SynthDef(\monokit_main, {
    // Defaults match RST command values
    arg pf = 131,     // Primary freq (C3) - needed for tracking
        mf = 262,     // Mod freq (C4) - needed for tracking
        dc = 0,
        dm = 0,
        mb = 0,
        mba = 0,
        mbd = 100,
        ma = 0,
        mc = 0,
        mq = 0,
        ad = 100,
        dd = 10,
        da = 0,
        fc = 10000,   // Filter wide open
        fq = 0,
        ft = 0,
        fe = 0,
        fed = 100,
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
        dmode = 2,    // Delay send mode by default
        dtail = 1,    // Ring tail behavior
        rmode = 2,    // Reverb send mode by default
        rtail = 1,
        lb = 16,      // Lo-fi bit depth (1-16)
        ls = 48000,   // Lo-fi sample rate (100-48000)
        lm = 0,       // Lo-fi mix (0-16383)
        rgf = 440,    // Ring mod frequency (20-2000)
        rgw = 0,      // Ring mod waveform (0-3)
        rgm = 0,      // Ring mod mix (0-16383)
        ct = 8192,    // Compressor threshold (0-16383)
        cr = 1,       // Compressor ratio (1:1 = off)
        ca = 10,      // Compressor attack ms (1-500)
        cl = 100,     // Compressor release ms (10-2000)
        cm = 0,       // Compressor makeup gain (0-16383)
        el = 0,       // EQ low shelf dB (-24 to +24)
        em = 0,       // EQ mid peak dB (-24 to +24)
        ef = 1000,    // EQ mid frequency Hz (200-8000)
        eq = 1,       // EQ mid Q (0.1-10)
        eh = 0,       // EQ high shelf dB (-24 to +24)
        pn = 0,       // Pan position (-16383 to +16383)
        br_len = 2,   // Beat repeat loop division (0-7)
        br_len_ms = 250, // Beat repeat loop length in ms
        br_rev = 0,   // Beat repeat reverse (0-1)
        br_win = 5,   // Beat repeat crossfade window ms (1-50)
        br_mix = 0,   // Beat repeat mix (0-16383)
        ps_mode = 0,  // Pitch shift mode (0=granular, 1=freq)
        ps_semi = 0,  // Pitch shift semitones (-24 to +24)
        ps_grain = 20, // Pitch shift grain size ms (5-100)
        ps_mix = 0,   // Pitch shift mix (0-16383)
        ps_targ = 0,  // Pitch shift target (0=main, 1=repeat)
        limit = 1,    // Limiter enabled (0-1)
        vca_mode = 1, // VCA mode (0=drone, 1=gate)
        volume = 1,
        t_gate = 0,  // TrigControl - automatically resets after each control block
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
        plaitsAuxBus = 20;

    // All var declarations at top
    var trig, ampEnv, dcEnv, filterEnv, modBusEnv;
    var modBusValue, dcAmount;
    var primaryFreq, modOsc;
    var sig, amp, discontinuity;
    var primaryIn, modIn, noiseIn, plaitsMainIn, plaitsAuxIn;
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
    var fcSmooth, dcSmooth, fqSmooth, fkSmooth, feSmooth;
    var rfSmooth, rmSmooth, dtSmooth, dfSmooth, dwSmooth, rvSmooth, rwSmooth, volumeSmooth, panSmooth;
    var lbSmooth, lsSmooth, lmSmooth, rgfSmooth, rgmSmooth, ctSmooth, cmSmooth;
    var elSmooth, emSmooth, ehSmooth, efSmooth;
    var daCtl;
    var aAtk, aCrv, dAtk, dCrv, flAtk, flCrv;
    var adDynamic, ddDynamic, fedDynamic, mbdDynamic;

    // Capture envelope amounts
    daCtl = Lag.kr(da, 0);

    // Smoothed parameters using Lag.kr with per-parameter slew time overrides
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

    // Resolve per-envelope attack times (ms -> seconds, with global fallback)
    aAtk = Select.kr(aenv_atk >= 0, [env_atk, aenv_atk]) / 1000;
    dAtk = Select.kr(denv_atk >= 0, [env_atk, denv_atk]) / 1000;
    flAtk = Select.kr(flev_atk >= 0, [env_atk, flev_atk]) / 1000;

    // Resolve per-envelope curves (with global fallback)
    aCrv = Select.kr(aenv_crv > -100, [env_crv, aenv_crv]);
    dCrv = Select.kr(denv_crv > -100, [env_crv, denv_crv]);
    flCrv = Select.kr(flev_crv > -100, [env_crv, flev_crv]);

    // Force dynamic evaluation of decay params
    adDynamic = Lag.kr(ad, 0);
    ddDynamic = Lag.kr(dd, 0);
    fedDynamic = Lag.kr(fed, 0);
    mbdDynamic = Lag.kr(mbd, 0);

    // Trigger and envelopes
    trig = Trig1.kr(t_gate, 0.001);
    ampEnv = EnvGen.kr(Env.perc(aAtk, adDynamic / 1000, 1, aCrv), trig);
    dcEnv = EnvGen.kr(Env.perc(dAtk, ddDynamic / 1000, 1, dCrv), trig);
    filterEnv = EnvGen.kr(Env.perc(flAtk, fedDynamic / 1000, 1, flCrv), trig);
    modBusEnv = EnvGen.kr(Env.perc(0.001, mbdDynamic / 1000, 1, -4), trig);

    // Read audio from the 5 source busses
    primaryIn = InFeedback.ar(primaryBus, 1);
    modIn = InFeedback.ar(modBus, 1);
    noiseIn = InFeedback.ar(noiseBus, 1);
    plaitsMainIn = InFeedback.ar(plaitsMainBus, 1);
    plaitsAuxIn = InFeedback.ar(plaitsAuxBus, 1);

    // Use modIn for modulation routing
    modOsc = modIn;

    // Scaled parameters
    modBusValue = (mb / 16383) + (modBusEnv * mba / 16383);
    dcAmount = dcSmooth / 16383;

    // Primary frequency tracking for filter key tracking
    primaryFreq = pf;

    // Mix oscillator sources (pre-filter)
    // NOTE: Plaits mixed in post-VCA to preserve its own envelope
    sig = primaryIn + modIn + noiseIn;

    // Discontinuity: base + (envelope * amount)
    discontinuity = dcAmount + (dcEnv * daCtl / 16383);
    sig = sig + (modOsc * discontinuity);
    sig = Select.ar(dm.clip(0, 6).round, [
        sig.fold2(1),           // 0: fold
        sig.tanh,               // 1: tanh
        sig.softclip,           // 2: softclip
        sig.clip2(1),           // 3: hard clip
        (sig.abs.pow(0.5) * sig.sign),  // 4: asymmetric (sqrt saturation)
        sig.abs,                // 5: full-wave rectify
        (sig * 8).round / 8     // 6: bitcrush waveshape
    ]);

    // Lo-Fi processor
    lofiBits = lbSmooth.clip(1, 16);
    lofiRate = lsSmooth.clip(100, 48000);
    lofiMix = lmSmooth / 16383;
    lofi = (sig * (2.pow(lofiBits - 1))).round / (2.pow(lofiBits - 1));
    lofi = Latch.ar(lofi, Impulse.ar(lofiRate));
    sig = (sig * (1 - lofiMix)) + (lofi * lofiMix);

    // Multi-Mode Filter (FT 0-13)
    // MF_F: audio-rate filter cutoff modulation (modOsc * modBusValue * mf_f * range)
    filterCutoff = fcSmooth + ((fkSmooth / 16383) * primaryFreq) + (filterEnv * (feSmooth / 16383) * 5000) + (modOsc * modBusValue * mf_f * 5000);
    filterCutoff = filterCutoff.clip(20, 20000);
    // MF_Q: audio-rate filter resonance modulation
    sig = Select.ar(ft.clip(0, 13).round, [
        // 0: SVF LP
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            1, 0, 0, 0, 0),
        // 1: SVF HP
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 1, 0, 0, 0),
        // 2: SVF BP
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 0, 1, 0, 0),
        // 3: SVF Notch
        SVF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 0, 0, 1, 0),
        // 4: MoogFF (Moog ladder LP)
        MoogFF.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 4.0) + (modOsc * modBusValue * mf_q * 2.0)).clip(0, 4)),
        // 5: RLPF (resonant LP 12dB)
        RLPF.ar(sig, filterCutoff,
            ((1.0 - (fqSmooth / 16383) * 0.95) - (modOsc * modBusValue * mf_q * 0.5)).clip(0.05, 1.0)),
        // 6: RHPF (resonant HP 12dB)
        RHPF.ar(sig, filterCutoff,
            ((1.0 - (fqSmooth / 16383) * 0.95) - (modOsc * modBusValue * mf_q * 0.5)).clip(0.05, 1.0)),
        // 7: DFM1 LP (diode filter LP)
        DFM1.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 1.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 1.1),
            1.0, 0.0),
        // 8: DFM1 HP (diode filter HP)
        DFM1.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 1.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 1.1),
            0.0, 1.0),
        // 9: BMoog LP (24dB saturating LP)
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 0.95),
            1, 0),
        // 10: BMoog HP (24dB saturating HP)
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 0.95),
            0, 1),
        // 11: BMoog BP (24dB saturating BP)
        BMoog.ar(sig, filterCutoff,
            (((fqSmooth / 16383) * 0.95) + (modOsc * modBusValue * mf_q * 0.5)).clip(0, 0.95),
            2, 0),
        // 12: Latch-SC LP (switched-capacitor style LP)
        SVF.ar(Latch.ar(sig, Impulse.ar((filterCutoff / 8).clip(100, 20000))), filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            1, 0, 0, 0, 0),
        // 13: Latch-SC HP (switched-capacitor style HP)
        SVF.ar(Latch.ar(sig, Impulse.ar((filterCutoff / 8).clip(100, 20000))), filterCutoff,
            (((fqSmooth / 16383) * 0.9 + 0.1) + (modOsc * modBusValue * mf_q * 0.5)).clip(0.1, 1.0),
            0, 1, 0, 0, 0)
    ]);

    // Ring modulator
    ringModOsc = Select.ar(rgw.clip(0, 3).round, [
        SinOsc.ar(rgfSmooth),
        LFTri.ar(rgfSmooth),
        LFSaw.ar(rgfSmooth),
        LFPulse.ar(rgfSmooth, 0, 0.5, 2, -1)  // Square wave
    ]);
    ringModMix = rgmSmooth / 16383;
    sig = (sig * (1 - ringModMix)) + ((sig * ringModOsc) * ringModMix);

    // Comb Resonator (Karplus-Strong)
    resonatorFreq = rfSmooth + ((rk / 16383) * (primaryFreq - rfSmooth));
    resonatorFreq = resonatorFreq.clip(20, 5000);
    resonatorDelay = (1 / resonatorFreq).clip(0.0002, 0.05);
    resonatorDecay = (rd / 1000).clip(0.01, 5.0);
    resonated = CombC.ar(sig, 0.05, resonatorDelay, resonatorDecay);
    resonatorMix = rmSmooth / 16383;
    sig = (sig * (1 - resonatorMix)) + (resonated * resonatorMix);

    // Amplitude: base + (modOsc * modBusValue * routing)
    // MA: audio-rate amplitude modulation (tremolo)
    // VCA mode: 0=drone (always 1.0), 1=gated (uses ampEnv)
    amp = Select.kr(vca_mode.clip(0, 1), [1.0, ampEnv]) * volumeSmooth * (1 + (modOsc * modBusValue * ma * 0.5));
    sig = sig * amp;

    // Mix Plaits post-VCA (bypasses envelope gating, preserves Plaits internal envelope)
    sig = sig + plaitsMainIn + plaitsAuxIn;

    // Compressor
    compThresh = (ctSmooth / 16383).clip(0.001, 1);
    compRatio = 1 / cr.clip(1, 20);
    sig = Compander.ar(sig, sig,
        thresh: compThresh,
        slopeBelow: 1,
        slopeAbove: compRatio,
        clampTime: ca / 1000,
        relaxTime: cl / 1000
    );
    sig = sig * (1 + (cmSmooth / 16383 * 2));  // Makeup gain (up to 3x)

    // Pan control
    panPos = (panSmooth / 16383).clip(-1, 1);
    #sigL, sigR = Pan2.ar(sig, panPos);

    // Beat Repeat Buffer Processing
    brActive = Lag.kr(br_mix > 0, 0.01);
    brLoopLen = br_len_ms / 1000;
    brBufSize = (brLoopLen * SampleRate.ir).ceil.max(4410);

    // Separate mono buffers for L and R to avoid channel mismatch
    beatRepeatBufL = LocalBuf(brBufSize, 1);
    beatRepeatBufR = LocalBuf(brBufSize, 1);

    // Write phasor stops advancing when beat repeat is active (freeze buffer)
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

    // Read from separate L and R buffers
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

    // Pitch Shift Processing
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

    // Stereo Delay with Feedback Filtering and Routing Modes
    dmodeSmooth = Lag.kr(dmode, 0.02);
    delayWetSmooth = Lag.kr(dwSmooth / 16383, 0.01);
    delayTime = (dtSmooth / 1000).clip(0.001, 2.0);
    delayFeedback = dfSmooth / 16383;

    // Tail behavior: CUT=gate feedback, RING=normal, FREEZE=infinite
    dfeedbackMult = Select.kr(dtail.clip(0, 2).round, [
        (dwSmooth > 100),    // CUT: feedback stops when wet low
        1,             // RING: normal feedback
        0.999          // FREEZE: infinite sustain
    ]);

    // Input gating: in SEND mode, wet controls input level
    // In RING/FREEZE modes, input still gated by wet (only output differs)
    delayInput = Select.ar(dmodeSmooth.clip(0, 2).round, [
        sigL,                      // BYPASS: unused
        sigL,                      // INSERT: full input
        sigL * delayWetSmooth      // SEND: wet controls input
    ]);

    delayedL = CombC.ar(LPF.ar(delayInput, dlp), 2.0, delayTime, delayFeedback * 3 * dfeedbackMult);
    delayedR = CombC.ar(LPF.ar(Select.ar(dmodeSmooth.clip(0, 2).round, [sigR, sigR, sigR * delayWetSmooth]), dlp), 2.0, delayTime * 1.02, delayFeedback * 3 * dfeedbackMult);

    // Output mixing based on mode and tail behavior
    // - BYPASS: dry only
    // - INSERT: wet/dry crossfade
    // - SEND: dry + delay (level depends on tail mode)
    //   - CUT: delay output gated by wet (goes silent when wet=0)
    //   - RING/FREEZE: delay output at FULL level (tails continue)
    sigL = Select.ar(dmodeSmooth.clip(0, 2).round, [
        sigL,                                                              // BYPASS
        (sigL * (1 - delayWetSmooth)) + (delayedL * delayWetSmooth),       // INSERT: wet/dry mix
        sigL + (delayedL * Select.kr(dtail.clip(0, 2).round, [delayWetSmooth, 1, 1]))  // SEND: CUT=wet, RING/FREEZE=full
    ]);

    sigR = Select.ar(dmodeSmooth.clip(0, 2).round, [
        sigR,
        (sigR * (1 - delayWetSmooth)) + (delayedR * delayWetSmooth),
        sigR + (delayedR * Select.kr(dtail.clip(0, 2).round, [delayWetSmooth, 1, 1]))
    ]);

    // 3-Band EQ (post-delay, pre-reverb)
    sigL = BLowShelf.ar(sigL, 200, 1, elSmooth.clip(-24, 24));
    sigL = BPeakEQ.ar(sigL, efSmooth.clip(200, 8000), (1/eq.clip(0.1, 10)), emSmooth.clip(-24, 24));
    sigL = BHiShelf.ar(sigL, 4000, 1, ehSmooth.clip(-24, 24));
    sigR = BLowShelf.ar(sigR, 200, 1, elSmooth.clip(-24, 24));
    sigR = BPeakEQ.ar(sigR, efSmooth.clip(200, 8000), (1/eq.clip(0.1, 10)), emSmooth.clip(-24, 24));
    sigR = BHiShelf.ar(sigR, 4000, 1, ehSmooth.clip(-24, 24));

    // Plate Reverb with Routing Modes
    rmodeSmooth = Lag.kr(rmode, 0.02);
    reverbWetSmooth = Lag.kr(rwSmooth / 16383, 0.01);

    // Room size: rv controls decay (0-1), FREEZE uses max
    rroomSize = Select.kr(rtail.clip(0, 2).round, [
        (rvSmooth / 16383).clip(0, 1),   // CUT: normal
        (rvSmooth / 16383).clip(0, 1),   // RING: normal
        1.0                         // FREEZE: maximum decay
    ]);

    // Input gating: SEND mode uses wet as send level
    reverbInput = Select.ar(rmodeSmooth.clip(0, 2).round, [
        DC.ar(0),                  // BYPASS: no input
        sigL,                      // INSERT: full input
        sigL * reverbWetSmooth     // SEND: wet controls send level
    ]);

    #reverbedL, reverbedR = FreeVerb2.ar(
        DelayC.ar(reverbInput, 0.1, rp / 1000),
        DelayC.ar(Select.ar(rmodeSmooth.clip(0, 2).round, [DC.ar(0), sigR, sigR * reverbWetSmooth]), 0.1, rp / 1000),
        mix: 1,
        room: rroomSize,
        damp: (rh / 16383).clip(0, 1)
    );

    // Output level control:
    // - BYPASS: dry only
    // - INSERT: wet/dry crossfade
    // - SEND + CUT: reverb gated by wet (silent when wet=0)
    // - SEND + RING/FREEZE: reverb at FULL level (tails continue)
    sigL = Select.ar(rmodeSmooth.clip(0, 2).round, [
        sigL,                                                              // BYPASS
        (sigL * (1 - reverbWetSmooth)) + (reverbedL * reverbWetSmooth),    // INSERT
        sigL + (reverbedL * Select.kr(rtail.clip(0, 2).round, [reverbWetSmooth, 1, 1]))  // SEND
    ]);

    sigR = Select.ar(rmodeSmooth.clip(0, 2).round, [
        sigR,
        (sigR * (1 - reverbWetSmooth)) + (reverbedR * reverbWetSmooth),
        sigR + (reverbedR * Select.kr(rtail.clip(0, 2).round, [reverbWetSmooth, 1, 1]))
    ]);

    // Limiter at end of chain (before output)
    #sigL, sigR = Select.ar(limit.clip(0, 1).round, [
        [sigL, sigR],
        Limiter.ar([sigL, sigR], 1.0, 0.01)
    ]);

    // DC blocking
    sigL = LeakDC.ar(sigL);
    sigR = LeakDC.ar(sigR);

    // Send peak/RMS at 20Hz to Rust meter thread
    SendPeakRMS.kr(sigL, 20, 3, '/monokit/meter', 1);
    SendPeakRMS.kr(sigR, 20, 3, '/monokit/meter', 2);

    Out.ar(0, [sigL, sigR]);
}).writeDefFile(synthdefs_dir);

"  monokit_main.scsyndef".postln;

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
("SynthDefs compiled successfully to " ++ synthdefs_dir).postln;
"  - monokit.scsyndef (monolithic - backward compatibility)".postln;
"  - monokit_noise.scsyndef (noise generator)".postln;
"  - monokit_mod.scsyndef (modulator oscillator)".postln;
"  - monokit_primary.scsyndef (primary oscillator)".postln;
"  - monokit_plaits.scsyndef (Plaits voice)".postln;
"  - monokit_main.scsyndef (main signal path)".postln;
"  - monokit_spectrum.scsyndef (spectrum analyzer)".postln;
"  - monokit_scope.scsyndef (oscilloscope)".postln;
"  - monokit_recorder.scsyndef (audio recorder)".postln;
"NOTE: Runtime uses 5-synth architecture (noise, mod, primary, plaits, main)".postln;
0.exit;
)
