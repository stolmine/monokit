{ |synthdefs_dir|
SynthDef(\monokit_sampler, {
    arg out = 21,
        s_bufnum = 0,
        t_gate_sampler = 0,
        s_rate = 8192,
        s_pitch = 0,
        s_fine = 0,
        s_direction = 0,
        s_loop = 0,
        s_startFrame = 0,
        s_endFrame = -1,
        s_atk = 0,
        s_dec = 8192,
        s_rel = 1000,
        s_sust = 0,
        s_volume = 16383,
        sf_cut = 16383,
        sf_res = 0,
        sf_type = 0,
        sf_bits = 0,
        sf_rate = 16383,
        sf_deci = 0,
        sf_prob = 0,
        sf_mult = 8192,
        sf_glit = 0,
        s_ratemod = 0,
        s_pitchmod = 0,
        sf_cutmod = 0,
        sf_resmod = 0,
        modBus = 17;

    var trig, trigAr, gate;
    var rateBase, pitchRatio, fineRatio, finalRate;
    var bufFrames, actualEndFrame, playStart, playEnd;
    var phasorRate, phase;
    var phasorAtEnd, oneShotMute;
    var sig, env;
    var atkTime, decTime, relTime, sustLevel;
    var volScale;
    var normPos;
    var filterCutoff, filterRes;
    var decimated, deciBits, deciRate, deciMix;
    var glitchTrig, glitchRate, glitched, glitchMix;
    var modIn;

    trig = Trig1.kr(t_gate_sampler, 0.001);
    trigAr = T2A.ar(trig);
    gate = Latch.kr(s_sust, trig) + ((1 - s_sust) * trig);

    modIn = InFeedback.ar(modBus, 1);

    rateBase = (s_rate / 8192) * (1 + (modIn * (s_ratemod / 16383) * 2));
    // Pitch mod: base pitch (semitones) + modulation (±12 semitones at full)
    pitchRatio = ((s_pitch / 12) + (modIn * (s_pitchmod / 16383) * 12)).midiratio;
    fineRatio = (s_fine / 1200).midiratio;
    finalRate = rateBase * pitchRatio * fineRatio;
    finalRate = Select.kr(s_direction, [finalRate, finalRate.neg]);

    bufFrames = BufFrames.kr(s_bufnum);
    actualEndFrame = Select.kr(s_endFrame < 0, [s_endFrame, bufFrames]);
    playStart = s_startFrame.min(bufFrames - 1);
    playEnd = actualEndFrame.min(bufFrames);

    phasorRate = finalRate * BufRateScale.kr(s_bufnum);
    phase = Phasor.ar(
        trig: trig,
        rate: phasorRate,
        start: playStart,
        end: playEnd,
        resetPos: Select.kr(s_direction, [playStart, playEnd - 1])
    );

    phasorAtEnd = (phase >= (playEnd - 1));
    oneShotMute = Select.ar(s_loop, [
        SetResetFF.ar(phasorAtEnd, trigAr),
        DC.ar(0)
    ]);

    sig = BufRd.ar(1, s_bufnum, phase, 1, 4);

    sig = sig * (1 - oneShotMute);

    atkTime = (s_atk / 16383) * 2.0;
    decTime = (s_dec / 16383) * 2.0;
    relTime = (s_rel / 1000.0).min(5.0);
    sustLevel = Select.kr(s_sust > 0, [0, 1]);

    env = EnvGen.kr(
        Env.adsr(atkTime, decTime, sustLevel, relTime),
        gate,
        doneAction: 0
    );

    volScale = s_volume / 16383;
    sig = sig * env * volScale;

    // Filter activates when base params OR mod amounts are set (.sign clamps to 0/1)
    sig = Select.ar(((sf_cut < 16383) + (sf_res > 0) + (sf_cutmod > 0) + (sf_resmod > 0)).sign, [
        sig,
        {
            filterCutoff = ((sf_cut / 16383) + (modIn * (sf_cutmod / 16383))).clip(0, 1).linexp(0, 1, 20, 20000);
            // Res mod: larger range (0-0.9) for audible effect, rq goes 1.0 down to ~0.15
            filterRes = ((sf_res / 16383) + (modIn * (sf_resmod / 16383))).clip(0, 0.9);
            Select.ar(sf_type.clip(0, 2).round, [
                RLPF.ar(sig, filterCutoff, (1.0 - (filterRes * 0.95)).clip(0.05, 1.0)),
                RHPF.ar(sig, filterCutoff, (1.0 - (filterRes * 0.95)).clip(0.05, 1.0)),
                BPF.ar(sig, filterCutoff, (1.0 - (filterRes * 0.95)).clip(0.05, 1.0))
            ])
        }.value
    ]);

    sig = Select.ar(sf_deci > 0, [
        sig,
        {
            deciBits = sf_bits.clip(1, 16);
            deciRate = (sf_rate / 16383).linexp(0, 1, 100, 48000);
            deciMix = sf_deci / 16383;
            decimated = (sig * (2.pow(deciBits - 1))).round / (2.pow(deciBits - 1));
            decimated = Latch.ar(decimated, Impulse.ar(deciRate));
            (sig * (1 - deciMix)) + (decimated * deciMix)
        }.value
    ]);

    sig = Select.ar(sf_glit > 0, [
        sig,
        {
            glitchRate = (sf_mult / 8192).clip(0.1, 2.0);
            glitchTrig = Dust.ar((sf_prob / 16383) * 100);
            glitched = Latch.ar(sig, glitchTrig + Impulse.ar(glitchRate * 10));
            glitchMix = sf_glit / 16383;
            (sig * (1 - glitchMix)) + (glitched * glitchMix)
        }.value
    ]);

    Out.ar(out, sig);

    normPos = ((phase - playStart) / (playEnd - playStart)).max(0).min(1);
    SendReply.kr(Impulse.kr(20), '/monokit/sampler/position', normPos);
}).writeDefFile(synthdefs_dir);

"  monokit_sampler.scsyndef".postln;
}
