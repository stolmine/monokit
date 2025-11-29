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
            volume = 1,
            gate = 0,
            slew_time = 0;

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

        // Smoothed parameters using Lag.kr
        pfSmooth = Lag.kr(pf, slew_time);
        mfSmooth = Lag.kr(mf, slew_time);
        fcSmooth = Lag.kr(fc, slew_time);
        fmSmooth = Lag.kr(fm, slew_time);
        mxSmooth = Lag.kr(mx, slew_time);
        dcSmooth = Lag.kr(dc, slew_time);
        fbSmooth = Lag.kr(fb, slew_time);
        fqSmooth = Lag.kr(fq, slew_time);
        fkSmooth = Lag.kr(fk, slew_time);
        feSmooth = Lag.kr(fe, slew_time);
        rfSmooth = Lag.kr(rf, slew_time);
        rmSmooth = Lag.kr(rm, slew_time);
        dtSmooth = Lag.kr(dt, slew_time);
        dfSmooth = Lag.kr(df, slew_time);
        dwSmooth = Lag.kr(dw, slew_time);
        rvSmooth = Lag.kr(rv, slew_time);
        rwSmooth = Lag.kr(rw, slew_time);
        volumeSmooth = Lag.kr(volume, slew_time);
        panSmooth = Lag.kr(pn, slew_time);
        lbSmooth = Lag.kr(lb, slew_time);
        lsSmooth = Lag.kr(ls, slew_time);
        lmSmooth = Lag.kr(lm, slew_time);
        rgfSmooth = Lag.kr(rgf, slew_time);
        rgmSmooth = Lag.kr(rgm, slew_time);
        ctSmooth = Lag.kr(ct, slew_time);
        cmSmooth = Lag.kr(cm, slew_time);
        elSmooth = Lag.kr(el, slew_time);
        emSmooth = Lag.kr(em, slew_time);
        ehSmooth = Lag.kr(eh, slew_time);
        efSmooth = Lag.kr(ef, slew_time);

        // Envelope model: output = base + (envelope * amount)
        // Exception: ampEnv is multiplicative (0→1 gating)

        // Trigger and envelopes
        trig = Trig1.kr(gate, 0.001);
        pitchEnv = EnvGen.kr(Env.perc(0.001, pd / 1000, 1, -4), trig);
        ampEnv = EnvGen.kr(Env.perc(0.001, ad / 1000, 1, -4), trig);
        fmEnv = EnvGen.kr(Env.perc(0.001, fd / 1000, 1, -4), trig);
        dcEnv = EnvGen.kr(Env.perc(0.001, dd / 1000, 1, -4), trig);
        fbEnv = EnvGen.kr(Env.perc(0.001, fbd / 1000, 1, -4), trig);
        filterEnv = EnvGen.kr(Env.perc(0.001, fed / 1000, 1, -4), trig);

        // Scaled parameters
        modBus = mb / 16383;
        trackingAmount = tk / 16383;
        fmIndex = fmSmooth / 16383;
        dcAmount = dcSmooth / 16383;
        mixAmount = mxSmooth / 16383;
        fbAmount = (fbSmooth / 16383) + (fbEnv * fba / 16383);  // base + (envelope * amount)

        // Primary frequency: base + (pitchEnv * amount) + (modBus * routing)
        primaryFreq = pfSmooth * (1 + (pitchEnv * pa) + (modBus * mp * 4));

        // Modulator frequency: base + (tracking) + (modBus * routing)
        modulatorFreq = mfSmooth + (trackingAmount * mt * (pfSmooth * (1 + (pitchEnv * pa)) - pfSmooth));
        modulatorFreq = modulatorFreq * (1 + (modBus * md));

        // Modulator oscillator (mw: 0=sine, 1=tri, 2=saw, 3=feedback)
        modOsc = Select.ar(mw.clip(0, 3).round, [
            SinOsc.ar(modulatorFreq),
            LFTri.ar(modulatorFreq),
            LFSaw.ar(modulatorFreq),
            SinOscFB.ar(modulatorFreq, fbAmount * 2)  // *2 scales 0-1 to useful 0-2 range
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

        Out.ar(0, [sigL, sigR]);
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

    "monokit server ready".postln;
};
)
