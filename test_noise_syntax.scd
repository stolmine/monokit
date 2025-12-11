(
// Test noise source syntax
{
    var nw = 0, na = 1, nd = 100, nc = -4, ne = 16383;
    var np = 0, nm = 0, nv = 0, pv = 16383, mv = 0;
    var t_gate = Impulse.kr(1);
    var trig, noiseRaw, noiseEnv, noiseShaped, noiseToPrimary, noiseToMod;
    var nwVal, naSmooth, ndSmooth, ncVal, neSmooth, npSmooth, nmSmooth, nvSmooth, pvSmooth, mvSmooth;
    var primaryOsc, modOsc, sig, totalLevel, gainComp;

    // Smoothing
    nwVal = nw.clip(0, 2).round;
    naSmooth = Lag.kr(na, 0.01);
    ndSmooth = Lag.kr(nd, 0.01);
    ncVal = nc.clip(-8, 8);
    neSmooth = Lag.kr(ne / 16383, 0.01);
    npSmooth = Lag.kr(np / 16383, 0.01);
    nmSmooth = Lag.kr(nm / 16383, 0.01);
    nvSmooth = Lag.kr(nv / 16383, 0.01);
    pvSmooth = Lag.kr(pv / 16383, 0.01);
    mvSmooth = Lag.kr(mv / 16383, 0.01);

    // Trigger
    trig = Trig1.kr(t_gate, 0.001);

    // Noise generator
    noiseRaw = Select.ar(nwVal, [
        WhiteNoise.ar(1),
        PinkNoise.ar(1),
        BrownNoise.ar(1)
    ]);

    // Noise envelope
    noiseEnv = EnvGen.kr(
        Env.perc(naSmooth / 1000, ndSmooth / 1000, 1, ncVal),
        trig
    );

    // Shaped noise
    noiseShaped = noiseRaw * ((1 - neSmooth) + (neSmooth * noiseEnv));

    // Noise modulation
    noiseToPrimary = noiseShaped * npSmooth * 1000;
    noiseToMod = noiseShaped * nmSmooth * 1000;

    // Test oscillators
    primaryOsc = SinOsc.ar(440 + noiseToPrimary);
    modOsc = SinOsc.ar(880 + noiseToMod);

    // Mix sources
    totalLevel = (pvSmooth + mvSmooth + nvSmooth).max(0.001);
    gainComp = (1 / totalLevel.sqrt).clip(0.5, 1.5);
    sig = (primaryOsc * pvSmooth) + (modOsc * mvSmooth) + (noiseShaped * nvSmooth);
    sig = sig * gainComp;

    sig ! 2
}.play;
)
