// Synth Instance Creation
(
~noise = Synth(\monokit_noise, target: s, addAction: \addToHead);
~mod = Synth(\monokit_mod, target: s, addAction: \addToHead);
~primary = Synth(\monokit_primary, target: s, addAction: \addToHead);
~plaits = Synth(\monokit_plaits, target: s, addAction: \addToHead);
~main = Synth(\monokit_main, target: s, addAction: \addToTail);
~spectrum = Synth(\monokit_spectrum, addAction: \addToTail);
~scope = Synth(\monokit_scope, addAction: \addToTail);
)
