{ |synthdefs_dir, hasMiPlaits|
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
}
