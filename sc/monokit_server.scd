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
            fb = 0,
            fba = 0,
            fbd = 10,
            fc = 1000,
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
            dmode = 1,
            dtail = 1,
            rmode = 1,
            rtail = 1,
            lb = 16,      // Lo-fi bit depth (1-16)
            ls = 48000,   // Lo-fi sample rate (100-48000)
            lm = 0,       // Lo-fi mix (0-16383)
            rgf = 440,    // Ring mod frequency (20-2000)
            rgw = 0,      // Ring mod waveform (0-3)
            rgm = 0,      // Ring mod mix (0-16383)
            ct = 8192,    // Compressor threshold (0-16383)
            cr = 4,       // Compressor ratio (1-20)
            ca = 10,      // Compressor attack ms (1-500)
            cl = 100,     // Compressor release ms (10-2000)
            cm = 0,       // Compressor makeup gain (0-16383)
            el = 0,       // EQ low shelf dB (-24 to +24)
            em = 0,       // EQ mid peak dB (-24 to +24)
            ef = 1000,    // EQ mid frequency Hz (200-8000)
            eq = 1,       // EQ mid Q (0.1-10)
            eh = 0,       // EQ high shelf dB (-24 to +24)
            pn = 0,       // Pan position (-16383 to +16383)
            br_act = 0,   // Beat repeat active (0-1)
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
            volume = 1,
            gate = 0,
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

        // All var declarations at top
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
        // Envelope amount controls - captured early to ensure consistent reading
        var paCtl, faCtl, daCtl, fbaCtl;
        var aAtk, aCrv, pAtk, pCrv, fmAtk, fmCrv, dAtk, dCrv, fbAtk, fbCrv, flAtk, flCrv;

        // Capture envelope amounts - use Lag.kr(0) to force proper signal evaluation
        paCtl = Lag.kr(pa, 0);
        faCtl = Lag.kr(fa, 0);
        daCtl = Lag.kr(da, 0);
        fbaCtl = Lag.kr(fba, 0);

        // Smoothed parameters using Lag.kr with per-parameter slew time overrides
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

        // Resolve per-envelope attack times (ms -> seconds, with global fallback)
        aAtk = Select.kr(aenv_atk >= 0, [env_atk, aenv_atk]) / 1000;
        pAtk = Select.kr(penv_atk >= 0, [env_atk, penv_atk]) / 1000;
        fmAtk = Select.kr(fmev_atk >= 0, [env_atk, fmev_atk]) / 1000;
        dAtk = Select.kr(denv_atk >= 0, [env_atk, denv_atk]) / 1000;
        fbAtk = Select.kr(fbev_atk >= 0, [env_atk, fbev_atk]) / 1000;
        flAtk = Select.kr(flev_atk >= 0, [env_atk, flev_atk]) / 1000;

        // Resolve per-envelope curves (with global fallback)
        aCrv = Select.kr(aenv_crv > -100, [env_crv, aenv_crv]);
        pCrv = Select.kr(penv_crv > -100, [env_crv, penv_crv]);
        fmCrv = Select.kr(fmev_crv > -100, [env_crv, fmev_crv]);
        dCrv = Select.kr(denv_crv > -100, [env_crv, denv_crv]);
        fbCrv = Select.kr(fbev_crv > -100, [env_crv, fbev_crv]);
        flCrv = Select.kr(flev_crv > -100, [env_crv, flev_crv]);

        // Envelope model: output = base + (envelope * amount)
        // Exception: ampEnv is multiplicative (0→1 gating)

        // Trigger and envelopes
        trig = Trig1.kr(gate, 0.001);
        pitchEnv = EnvGen.kr(Env.perc(pAtk, pd / 1000, 1, pCrv), trig);
        ampEnv = EnvGen.kr(Env.perc(aAtk, ad / 1000, 1, aCrv), trig);
        fmEnv = EnvGen.kr(Env.perc(fmAtk, fd / 1000, 1, fmCrv), trig);
        dcEnv = EnvGen.kr(Env.perc(dAtk, dd / 1000, 1, dCrv), trig);
        fbEnv = EnvGen.kr(Env.perc(fbAtk, fbd / 1000, 1, fbCrv), trig);
        filterEnv = EnvGen.kr(Env.perc(flAtk, fed / 1000, 1, flCrv), trig);

        // Scaled parameters
        modBus = mb / 16383;
        trackingAmount = tk / 16383;
        fmIndex = fmSmooth / 16383;
        dcAmount = dcSmooth / 16383;
        mixAmount = mxSmooth / 16383;
        fbAmount = (fbSmooth / 16383) + (fbEnv * fbaCtl / 16383);  // base + (envelope * amount)

        // Primary frequency: base + (pitchEnv * amount) + (modBus * routing)
        primaryFreq = pfSmooth * (1 + (pitchEnv * paCtl) + (modBus * mp * 4));

        // Modulator frequency: base + (tracking) + (modBus * routing)
        modulatorFreq = mfSmooth + (trackingAmount * mt * (pfSmooth * (1 + (pitchEnv * paCtl)) - pfSmooth));
        modulatorFreq = modulatorFreq * (1 + (modBus * md));

        // Modulator oscillator (mw: 0=sine, 1=tri, 2=saw, 3=feedback)
        modOsc = Select.ar(mw.clip(0, 3).round, [
            SinOsc.ar(modulatorFreq),
            LFTri.ar(modulatorFreq),
            LFSaw.ar(modulatorFreq),
            SinOscFB.ar(modulatorFreq, fbAmount * 2)  // *2 scales 0-1 to useful 0-2 range
        ]);

        // Primary oscillator with FM: base + (envelope * amount)
        fmAmount = (fmIndex + (fmEnv * faCtl / 16383)) * 1000;
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

        // SVF Multi-Mode Filter
        filterCutoff = fcSmooth + ((fkSmooth / 16383) * primaryFreq) + (filterEnv * (feSmooth / 16383) * 5000) + (modBus * mf_f * 5000);
        filterCutoff = filterCutoff.clip(20, 20000);
        filterQ = ((fqSmooth / 16383) * 0.9 + 0.1);
        sig = Select.ar(ft.clip(0, 3).round, [
            SVF.ar(sig, filterCutoff, filterQ, 1, 0, 0, 0, 0),
            SVF.ar(sig, filterCutoff, filterQ, 0, 1, 0, 0, 0),
            SVF.ar(sig, filterCutoff, filterQ, 0, 0, 1, 0, 0),
            SVF.ar(sig, filterCutoff, filterQ, 0, 0, 0, 1, 0)
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

        // Amplitude: base + (modBus * routing)
        amp = ampEnv * volumeSmooth * (1 + (modBus * ma * 0.5));
        sig = sig * amp;

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
        brActive = Lag.kr(br_act.clip(0, 1), 0.01);
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

        // Send peak/RMS at 20Hz to Rust meter thread
        SendPeakRMS.kr(sigL, 20, 3, '/monokit/meter', 1);
        SendPeakRMS.kr(sigR, 20, 3, '/monokit/meter', 2);

        Out.ar(0, [sigL, sigR]);
    }).add;

    SynthDef(\monokit_spectrum, {
        var sigL, sigR, mono, bands, freqs, clips, data;

        sigL = InFeedback.ar(0);
        sigR = InFeedback.ar(1);
        mono = (sigL + sigR) * 0.5;

        // 15 logarithmically-spaced band center frequencies (20Hz - 20kHz)
        freqs = [25, 40, 63, 100, 160, 250, 400, 630, 1000, 1600, 2500, 4000, 6300, 10000, 16000];

        bands = freqs.collect({ |freq|
            // BPF rq parameter = 1/Q, use rq=0.5 for moderate bandwidth
            var filtered = BPF.ar(mono, freq, 0.5);
            // Amplitude follower with fast attack, moderate release
            Amplitude.kr(filtered, 0.01, 0.1)
        });

        // Detect clipping per band (1 if level > 0.95, 0 otherwise)
        clips = bands.collect({ |level|
            (level > 0.95).asInteger
        });

        // Combine bands and clips into single array: [band0, band1, ..., band14, clip0, clip1, ..., clip14]
        data = bands ++ clips;

        SendReply.kr(Impulse.kr(20), '/monokit/spectrum', data);
    }).add;

    s.sync;

    ~voice = Synth(\monokit);
    ~spectrum = Synth(\monokit_spectrum, addAction: \addToTail);

    // Forward meter data from SendPeakRMS to Rust app on port 57121
    // SendPeakRMS msg format: [cmdName, nodeID, replyID, peak, rms]
    // replyID is our channel (1=L, 2=R)
    ~meterAddr = NetAddr("127.0.0.1", 57121);
    OSCdef(\monokit_meter_fwd, { |msg|
        var channel = msg[2];  // replyID = channel (1 or 2)
        var peak = msg[3];
        var rms = msg[4];
        ~meterAddr.sendMsg('/monokit/meter', channel, peak, rms);
    }, '/monokit/meter');

    // Forward spectrum data to Rust app on port 57121
    // SendReply msg format: [cmdName, nodeID, replyID, ...values]
    // With 15 bands + 15 clips, values are at indices 3-32 (30 values total)
    ~spectrumAddr = NetAddr("127.0.0.1", 57121);
    OSCdef(\monokit_spectrum_fwd, { |msg|
        var data = msg.copyRange(3, 32);
        ~spectrumAddr.sendMsg('/monokit/spectrum', *data);
    }, '/monokit/spectrum');

    // CPU status sender (2Hz)
    ~cpuRoutine = Routine({
        inf.do {
            ~meterAddr.sendMsg('/monokit/cpu', s.avgCPU, s.peakCPU);
            0.5.wait;
        };
    }).play;

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

    // Recording support
    ~recPath = nil;  // Custom path prefix (set by REC.PATH)
    ~recIndex = 0;   // Counter for sequential recordings

    OSCdef(\monokit_rec, { |msg|
        var path, dir;
        if(s.isRecording.not, {
            // msg[1] is the directory from Rust (current working dir)
            dir = if(msg[1].notNil, { msg[1].asString }, { "." });

            if(~recPath.notNil, {
                // Use custom prefix with index
                path = ~recPath ++ "_" ++ ~recIndex ++ ".wav";
                ~recIndex = ~recIndex + 1;
            }, {
                // Auto-generate timestamped filename in specified dir
                path = dir +/+ "monokit_" ++ Date.getDate.stamp ++ ".wav";
            });

            s.recHeaderFormat = "wav";
            s.recSampleFormat = "int24";
            s.record(path);
            ("REC: " ++ path).postln;
        }, {
            "Already recording".postln;
        });
    }, '/monokit/rec');

    OSCdef(\monokit_rec_stop, { |msg|
        if(s.isRecording, {
            s.stopRecording;
            "Recording stopped".postln;
        }, {
            "Not recording".postln;
        });
    }, '/monokit/rec/stop');

    OSCdef(\monokit_rec_path, { |msg|
        var pathPrefix = msg[1].asString;
        ~recPath = pathPrefix;
        ~recIndex = 0;
        ("Recording path prefix: " ++ pathPrefix).postln;
    }, '/monokit/rec/path');

    OSCdef(\monokit_slew, { |msg|
        var slewMs = msg[1].asFloat.clip(0, 10000);
        var slewSec = slewMs / 1000;
        ~voice.set(\slew_time, slewSec);
        ("SLEW TIME SET TO" + slewMs + "MS").postln;
    }, '/monokit/slew');

    OSCdef(\monokit_slew_param, { |msg|
        var paramName = msg[1].asString;
        var slewTime = msg[2].asFloat.clip(0, 10);
        var slewArgName = ("slew_" ++ paramName).asSymbol;
        ~voice.set(slewArgName, slewTime);
        ("SLEW" + paramName.toUpper + "SET TO" + (slewTime * 1000) + "MS").postln;
    }, '/monokit/slew/param');

    // Timing diagnostics
    ~diagEnabled = false;
    ~diagTimestamps = [];
    ~diagMaxSamples = 1000;
    ~triggerCount = 0;

    // Modify trigger to record timestamps when diagnostics enabled
    OSCdef(\monokit_trigger).free;
    OSCdef(\monokit_trigger, { |msg|
        if(~diagEnabled, {
            ~triggerCount = ~triggerCount + 1;
            ~diagTimestamps = ~diagTimestamps.add(Main.elapsedTime);
            if(~diagTimestamps.size > ~diagMaxSamples, {
                ~diagTimestamps = ~diagTimestamps.drop(1);
            });
        });
        // Set gate high immediately
        ~voice.set(\gate, 1);
        // Schedule gate low after 10ms using language-side clock
        SystemClock.sched(0.01, {
            ~voice.set(\gate, 0);
            nil;
        });
    }, '/monokit/trigger');

    OSCdef(\monokit_diag, { |msg|
        var enable = msg[1].asInteger;
        ~diagEnabled = enable > 0;
        if(~diagEnabled, {
            ~diagTimestamps = [];
            ~triggerCount = 0;
            "SC TIMING DIAGNOSTICS ENABLED".postln;
        }, {
            "SC TIMING DIAGNOSTICS DISABLED".postln;
        });
    }, '/monokit/diag');

    OSCdef(\monokit_diag_report, { |msg|
        var intervals, oddIntervals, evenIntervals, avgOdd, avgEven, swingRatio;
        var file, reportPath, minInterval, maxInterval, avgInterval, stdDev;
        var sum, sumSq, variance;

        ("SC DIAG: " ++ ~triggerCount ++ " triggers received, " ++ ~diagTimestamps.size ++ " timestamps recorded").postln;

        if(~diagTimestamps.size < 2, {
            "SC DIAG: Insufficient data (need at least 2 triggers)".postln;
        }, {
            intervals = (~diagTimestamps.size - 1).collect({ |i|
                (~diagTimestamps[i + 1] - ~diagTimestamps[i]) * 1000;
            });

            oddIntervals = [];
            evenIntervals = [];
            intervals.do({ |val, idx|
                if(idx.odd, {
                    oddIntervals = oddIntervals.add(val);
                }, {
                    evenIntervals = evenIntervals.add(val);
                });
            });

            avgInterval = intervals.mean;
            minInterval = intervals.minItem;
            maxInterval = intervals.maxItem;

            sum = intervals.sum;
            sumSq = intervals.collect({ |x| x * x }).sum;
            variance = (sumSq / intervals.size) - (avgInterval * avgInterval);
            stdDev = variance.sqrt;

            avgOdd = if(oddIntervals.size > 0, { oddIntervals.mean }, { 0 });
            avgEven = if(evenIntervals.size > 0, { evenIntervals.mean }, { 0 });
            swingRatio = if(avgEven > 0, { avgOdd / avgEven }, { 1 });

            reportPath = "/Users/why/repos/monokit/sc_timing_report.txt";
            file = File.open(reportPath, "w");

            file.write("=== SuperCollider Timing Report ===\n");
            file.write("Total triggers received: " ++ ~triggerCount ++ "\n");
            file.write("Total timestamps recorded: " ++ ~diagTimestamps.size ++ "\n");
            file.write("Total intervals: " ++ intervals.size ++ "\n\n");
            file.write("Interval Statistics (ms):\n");
            file.write("  Mean: " ++ avgInterval.round(0.001) ++ "\n");
            file.write("  Min: " ++ minInterval.round(0.001) ++ "\n");
            file.write("  Max: " ++ maxInterval.round(0.001) ++ "\n");
            file.write("  Std Dev: " ++ stdDev.round(0.001) ++ "\n");
            file.write("  Jitter: " ++ (maxInterval - minInterval).round(0.001) ++ "\n\n");
            file.write("Swing Analysis:\n");
            file.write("  Even intervals avg: " ++ avgEven.round(0.001) ++ " ms\n");
            file.write("  Odd intervals avg: " ++ avgOdd.round(0.001) ++ " ms\n");
            file.write("  Ratio (odd/even): " ++ swingRatio.round(0.0001) ++ "\n");
            if((swingRatio - 1.0).abs > 0.02, {
                file.write("  >>> SWING DETECTED AT SC LEVEL! <<<\n");
            });
            file.write("\nFirst 16 intervals (ms):\n");
            intervals.keep(16).do({ |val, idx|
                file.write("  " ++ val.round(0.01) ++ "\n");
            });
            file.write("==========================================\n");

            file.close;

            "SC DIAG: Report written to sc_timing_report.txt".postln;
            ("SC DIAG: Mean=" ++ avgInterval.round(0.01) ++ "ms, Swing ratio=" ++ swingRatio.round(0.0001)).postln;
        });
    }, '/monokit/diag/report');

    "monokit server ready".postln;
};
)
