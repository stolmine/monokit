//! Synthesis parameter commands

use super::{ArgCount, CommandDef};

pub fn register_synth(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    // Primary Oscillator
    m.insert("PF", CommandDef::new("PF", Some("POSC.FREQ"), ArgCount::AtLeast(1), "Primary osc frequency"));
    m.insert("PW", CommandDef::new("PW", Some("POSC.WAVE"), ArgCount::AtLeast(1), "Primary osc waveform"));
    m.insert("PV", CommandDef::new("PV", Some("PRI.VOL"), ArgCount::AtLeast(1), "Primary osc volume"));

    // Modulation Oscillator
    m.insert("MF", CommandDef::new("MF", Some("MOSC.FREQ"), ArgCount::AtLeast(1), "Mod osc frequency"));
    m.insert("MW", CommandDef::new("MW", Some("MOSC.WAVE"), ArgCount::AtLeast(1), "Mod osc waveform"));
    m.insert("MV", CommandDef::new("MV", Some("MOD.VOL"), ArgCount::AtLeast(1), "Mod osc volume"));
    m.insert("FB", CommandDef::new("FB", Some("MOSC.FB"), ArgCount::AtLeast(1), "Mod osc feedback"));
    m.insert("FBA", CommandDef::new("FBA", Some("MOSC.FBA"), ArgCount::AtLeast(1), "Mod osc feedback amt"));

    // Discontinuity
    m.insert("DC", CommandDef::new("DC", Some("DISC.AMT"), ArgCount::AtLeast(1), "Discontinuity amount"));
    m.insert("DM", CommandDef::new("DM", Some("DISC.MODE"), ArgCount::AtLeast(1), "Discontinuity mode"));

    // Modulation Bus
    m.insert("TK", CommandDef::new("TK", Some("MBUS.TRK"), ArgCount::AtLeast(1), "Modulation bus track"));
    m.insert("MB", CommandDef::new("MB", Some("MBUS.AMT"), ArgCount::AtLeast(1), "Modulation bus amount"));
    m.insert("MBA", CommandDef::new("MBA", Some("MBEV.AMT"), ArgCount::AtLeast(1), "Mod bus envelope amt"));
    m.insert("MBD", CommandDef::new("MBD", Some("MBEV.DEC"), ArgCount::AtLeast(1), "Mod bus envelope dec"));

    // Routing
    m.insert("FM", CommandDef::new("FM", Some("MBUS.FM"), ArgCount::AtLeast(1), "FM routing"));
    m.insert("MX", CommandDef::new("MX", Some("MBUS.MIX"), ArgCount::AtLeast(1), "Mix routing"));
    m.insert("MM", CommandDef::new("MM", Some("MBUS.MMX"), ArgCount::AtLeast(1), "Mod mix routing"));
    m.insert("ME", CommandDef::new("ME", Some("MBUS.EMX"), ArgCount::AtLeast(1), "Effect mix routing"));
    m.insert("MP", CommandDef::new("MP", Some("ROUT.MP"), ArgCount::AtLeast(1), "Route to pitch"));
    m.insert("MD", CommandDef::new("MD", Some("ROUT.MD"), ArgCount::AtLeast(1), "Route to discontinuity"));
    m.insert("MT", CommandDef::new("MT", Some("ROUT.MT"), ArgCount::AtLeast(1), "Route to timbre"));
    m.insert("MA", CommandDef::new("MA", Some("ROUT.MA"), ArgCount::AtLeast(1), "Route to amplitude"));
    m.insert("MC", CommandDef::new("MC", Some("ROUT.MC"), ArgCount::AtLeast(1), "Route to filter cutoff"));
    m.insert("MQ", CommandDef::new("MQ", Some("ROUT.MQ"), ArgCount::AtLeast(1), "Route to filter Q"));

    // Envelopes - Global Controls
    m.insert("ENV.ATK", CommandDef::new("ENV.ATK", None, ArgCount::Exactly(1), "Global envelope attack"));
    m.insert("ENV.DEC", CommandDef::new("ENV.DEC", None, ArgCount::Exactly(1), "Global envelope decay"));
    m.insert("ENV.CRV", CommandDef::new("ENV.CRV", None, ArgCount::Exactly(1), "Global envelope curve"));
    m.insert("ENV.MODE", CommandDef::new("ENV.MODE", None, ArgCount::Exactly(1), "Global envelope mode"));

    // Envelopes - Decay
    m.insert("AD", CommandDef::new("AD", Some("AENV.DEC"), ArgCount::AtLeast(1), "Amplitude env decay"));
    m.insert("PD", CommandDef::new("PD", Some("PENV.DEC"), ArgCount::AtLeast(1), "Pitch env decay"));
    m.insert("FD", CommandDef::new("FD", Some("FMEV.DEC"), ArgCount::AtLeast(1), "FM env decay"));
    m.insert("DD", CommandDef::new("DD", Some("DENV.DEC"), ArgCount::AtLeast(1), "Discontinuity env decay"));
    m.insert("AENV.DEC", CommandDef::new("AENV.DEC", None, ArgCount::AtLeast(1), "Amplitude env decay"));
    m.insert("PENV.DEC", CommandDef::new("PENV.DEC", None, ArgCount::AtLeast(1), "Pitch env decay"));
    m.insert("FMEV.DEC", CommandDef::new("FMEV.DEC", None, ArgCount::AtLeast(1), "FM env decay"));
    m.insert("DENV.DEC", CommandDef::new("DENV.DEC", None, ArgCount::AtLeast(1), "Discontinuity env decay"));
    m.insert("FBD", CommandDef::new("FBD", Some("FBEV.DEC"), ArgCount::AtLeast(1), "FB env decay"));
    m.insert("FBEV.DEC", CommandDef::new("FBEV.DEC", None, ArgCount::AtLeast(1), "FB envelope decay"));
    m.insert("FED", CommandDef::new("FED", Some("FLEV.DEC"), ArgCount::AtLeast(1), "Filter env decay"));
    m.insert("FLEV.DEC", CommandDef::new("FLEV.DEC", None, ArgCount::AtLeast(1), "Filter env decay"));

    // Envelopes - Amount
    m.insert("PA", CommandDef::new("PA", Some("PENV.AMT"), ArgCount::AtLeast(1), "Pitch env amount"));
    m.insert("FA", CommandDef::new("FA", Some("FMEV.AMT"), ArgCount::AtLeast(1), "FM env amount"));
    m.insert("DA", CommandDef::new("DA", Some("DENV.AMT"), ArgCount::AtLeast(1), "Discontinuity env amt"));
    m.insert("PENV.AMT", CommandDef::new("PENV.AMT", None, ArgCount::AtLeast(1), "Pitch env amount"));
    m.insert("FMEV.AMT", CommandDef::new("FMEV.AMT", None, ArgCount::AtLeast(1), "FM env amount"));
    m.insert("DENV.AMT", CommandDef::new("DENV.AMT", None, ArgCount::AtLeast(1), "Discontinuity env amt"));

    // Envelopes - Attack
    m.insert("AA", CommandDef::new("AA", Some("AENV.ATK"), ArgCount::Exactly(1), "Amplitude env attack"));
    m.insert("PAA", CommandDef::new("PAA", Some("PENV.ATK"), ArgCount::Exactly(1), "Pitch env attack"));
    m.insert("FAA", CommandDef::new("FAA", Some("FMEV.ATK"), ArgCount::Exactly(1), "FM env attack"));
    m.insert("DAA", CommandDef::new("DAA", Some("DENV.ATK"), ArgCount::Exactly(1), "Discontinuity env attack"));
    m.insert("FBAA", CommandDef::new("FBAA", Some("FBEV.ATK"), ArgCount::Exactly(1), "FB env attack"));
    m.insert("FLAA", CommandDef::new("FLAA", Some("FLEV.ATK"), ArgCount::Exactly(1), "Filter env attack"));
    m.insert("AENV.ATK", CommandDef::new("AENV.ATK", None, ArgCount::Exactly(1), "Amplitude env attack"));
    m.insert("PENV.ATK", CommandDef::new("PENV.ATK", None, ArgCount::Exactly(1), "Pitch env attack"));
    m.insert("FMEV.ATK", CommandDef::new("FMEV.ATK", None, ArgCount::Exactly(1), "FM env attack"));
    m.insert("DENV.ATK", CommandDef::new("DENV.ATK", None, ArgCount::Exactly(1), "Discontinuity env attack"));
    m.insert("FBEV.ATK", CommandDef::new("FBEV.ATK", None, ArgCount::Exactly(1), "FB env attack"));
    m.insert("FLEV.ATK", CommandDef::new("FLEV.ATK", None, ArgCount::Exactly(1), "Filter env attack"));

    // Envelopes - Curve
    m.insert("AC", CommandDef::new("AC", Some("AENV.CRV"), ArgCount::Exactly(1), "Amplitude env curve"));
    m.insert("PC", CommandDef::new("PC", Some("PENV.CRV"), ArgCount::Exactly(1), "Pitch env curve"));
    m.insert("FBC", CommandDef::new("FBC", Some("FBEV.CRV"), ArgCount::Exactly(1), "FB env curve"));
    m.insert("FLC", CommandDef::new("FLC", Some("FLEV.CRV"), ArgCount::Exactly(1), "Filter env curve"));
    m.insert("AENV.CRV", CommandDef::new("AENV.CRV", None, ArgCount::Exactly(1), "Amplitude env curve"));
    m.insert("PENV.CRV", CommandDef::new("PENV.CRV", None, ArgCount::Exactly(1), "Pitch env curve"));
    m.insert("FMEV.CRV", CommandDef::new("FMEV.CRV", None, ArgCount::Exactly(1), "FM env curve"));
    m.insert("DENV.CRV", CommandDef::new("DENV.CRV", None, ArgCount::Exactly(1), "Discontinuity env curve"));
    m.insert("FBEV.CRV", CommandDef::new("FBEV.CRV", None, ArgCount::Exactly(1), "FB env curve"));
    m.insert("FLEV.CRV", CommandDef::new("FLEV.CRV", None, ArgCount::Exactly(1), "Filter env curve"));

    // Envelopes - Unvalidated (has handler but no validation)
    m.insert("FBEV.AMT", CommandDef::new("FBEV.AMT", None, ArgCount::AtLeast(1), "FB envelope amount"));
    m.insert("FLEV.AMT", CommandDef::new("FLEV.AMT", None, ArgCount::AtLeast(1), "Filter env amount"));
    m.insert("FE", CommandDef::new("FE", Some("FLEV.AMT"), ArgCount::AtLeast(1), "Filter envelope amt"));

    // Envelopes - Gate
    m.insert("GATE", CommandDef::new("GATE", None, ArgCount::Exactly(1), "Trigger all gates"));
    m.insert("AENV.GATE", CommandDef::new("AENV.GATE", None, ArgCount::Exactly(1), "Amplitude env gate"));
    m.insert("PENV.GATE", CommandDef::new("PENV.GATE", None, ArgCount::Exactly(1), "Pitch env gate"));
    m.insert("FMEV.GATE", CommandDef::new("FMEV.GATE", None, ArgCount::Exactly(1), "FM env gate"));
    m.insert("DENV.GATE", CommandDef::new("DENV.GATE", None, ArgCount::Exactly(1), "Discontinuity env gate"));
    m.insert("FBEV.GATE", CommandDef::new("FBEV.GATE", None, ArgCount::Exactly(1), "FB env gate"));
    m.insert("FLEV.GATE", CommandDef::new("FLEV.GATE", None, ArgCount::Exactly(1), "Filter env gate"));

    // Noise
    m.insert("NW", CommandDef::new("NW", Some("NOISE.WAV"), ArgCount::AtLeast(1), "Noise waveform"));
    m.insert("NV", CommandDef::new("NV", Some("NOISE.VOL"), ArgCount::AtLeast(1), "Noise volume"));
    m.insert("NP", CommandDef::new("NP", Some("NOISE.PRI"), ArgCount::AtLeast(1), "Noise priority"));
    m.insert("NM", CommandDef::new("NM", Some("NOISE.MOD"), ArgCount::AtLeast(1), "Noise modulation"));

    // Plaits
    m.insert("PLV", CommandDef::new("PLV", None, ArgCount::AtLeast(1), "Plaits voice level"));
    m.insert("PAV", CommandDef::new("PAV", None, ArgCount::AtLeast(1), "Plaits aux level"));
    m.insert("PLE", CommandDef::new("PLE", Some("PL.ENG"), ArgCount::AtLeast(1), "Plaits engine"));
    m.insert("PLF", CommandDef::new("PLF", Some("PL.FREQ"), ArgCount::AtLeast(1), "Plaits frequency"));
    m.insert("PLH", CommandDef::new("PLH", Some("PL.HARM"), ArgCount::AtLeast(1), "Plaits harmonics"));
    m.insert("PLT", CommandDef::new("PLT", Some("PL.TIMB"), ArgCount::AtLeast(1), "Plaits timbre"));
    m.insert("PLM", CommandDef::new("PLM", Some("PL.MORPH"), ArgCount::AtLeast(1), "Plaits morph"));
    m.insert("PLD", CommandDef::new("PLD", Some("PL.DEC"), ArgCount::AtLeast(1), "Plaits decay"));
    m.insert("PLL", CommandDef::new("PLL", Some("PL.LPG"), ArgCount::AtLeast(1), "Plaits LPG"));
    m.insert("PL.ENG", CommandDef::new("PL.ENG", None, ArgCount::AtLeast(1), "Plaits engine"));
    m.insert("PL.FREQ", CommandDef::new("PL.FREQ", None, ArgCount::AtLeast(1), "Plaits frequency"));
    m.insert("PL.HARM", CommandDef::new("PL.HARM", None, ArgCount::AtLeast(1), "Plaits harmonics"));
    m.insert("PL.TIMB", CommandDef::new("PL.TIMB", None, ArgCount::AtLeast(1), "Plaits timbre"));
    m.insert("PL.MORPH", CommandDef::new("PL.MORPH", None, ArgCount::AtLeast(1), "Plaits morph"));
    m.insert("PL.DEC", CommandDef::new("PL.DEC", None, ArgCount::AtLeast(1), "Plaits decay"));
    m.insert("PL.LPG", CommandDef::new("PL.LPG", None, ArgCount::AtLeast(1), "Plaits LPG"));

    // Filter
    m.insert("FC", CommandDef::new("FC", Some("FILT.CUT"), ArgCount::AtLeast(1), "Filter cutoff"));
    m.insert("FQ", CommandDef::new("FQ", Some("FILT.RES"), ArgCount::AtLeast(1), "Filter resonance"));
    m.insert("FT", CommandDef::new("FT", Some("FILT.TYP"), ArgCount::AtLeast(1), "Filter type"));
    m.insert("FK", CommandDef::new("FK", Some("FILT.KEY"), ArgCount::AtLeast(1), "Filter key tracking"));
    m.insert("MFF", CommandDef::new("MFF", Some("MODF.CUT"), ArgCount::AtLeast(1), "Mod filter cutoff"));
    m.insert("MFQ", CommandDef::new("MFQ", Some("MODF.RES"), ArgCount::AtLeast(1), "Mod filter resonance"));

    // Resonator
    m.insert("RF", CommandDef::new("RF", Some("RESO.FRQ"), ArgCount::AtLeast(1), "Resonator frequency"));
    m.insert("RD", CommandDef::new("RD", Some("RESO.DEC"), ArgCount::AtLeast(1), "Resonator decay"));
    m.insert("RM", CommandDef::new("RM", Some("RESO.MIX"), ArgCount::AtLeast(1), "Resonator mix"));
    m.insert("RK", CommandDef::new("RK", Some("RESO.KEY"), ArgCount::AtLeast(1), "Resonator key track"));

    // Output
    m.insert("VOL", CommandDef::new("VOL", Some("OUT.VOL"), ArgCount::Exactly(1), "Output volume"));
    m.insert("PAN", CommandDef::new("PAN", Some("OUT.PAN"), ArgCount::AtLeast(1), "Pan position"));
    m.insert("VCA", CommandDef::new("VCA", None, ArgCount::Range(0, 1), "VCA mode"));

    // Slew
    m.insert("SLEW", CommandDef::new("SLEW", None, ArgCount::AtLeast(2), "Slew single parameter"));
    m.insert("SLEW.ALL", CommandDef::new("SLEW.ALL", None, ArgCount::AtLeast(1), "Slew all parameters"));

    // Mixer - Volume (0-16383)
    m.insert("VOL.OSC", CommandDef::new("VOL.OSC", None, ArgCount::AtLeast(1), "Complex osc volume"));
    m.insert("VO", CommandDef::new("VO", Some("VOL.OSC"), ArgCount::AtLeast(1), "Complex osc volume"));
    m.insert("VOL.PLA", CommandDef::new("VOL.PLA", None, ArgCount::AtLeast(1), "Plaits volume"));
    m.insert("VP", CommandDef::new("VP", Some("VOL.PLA"), ArgCount::AtLeast(1), "Plaits volume"));
    m.insert("VOL.NOS", CommandDef::new("VOL.NOS", None, ArgCount::AtLeast(1), "Noise volume"));
    m.insert("VN", CommandDef::new("VN", Some("VOL.NOS"), ArgCount::AtLeast(1), "Noise volume"));
    m.insert("VOL.SMP", CommandDef::new("VOL.SMP", None, ArgCount::AtLeast(1), "Sampler volume"));
    m.insert("VS", CommandDef::new("VS", Some("VOL.SMP"), ArgCount::AtLeast(1), "Sampler volume"));

    // Mixer - Pan (-8192 to 8191, center=0)
    m.insert("PAN.OSC", CommandDef::new("PAN.OSC", None, ArgCount::AtLeast(1), "Complex osc pan"));
    m.insert("PO", CommandDef::new("PO", Some("PAN.OSC"), ArgCount::AtLeast(1), "Complex osc pan"));
    m.insert("PAN.PLA", CommandDef::new("PAN.PLA", None, ArgCount::AtLeast(1), "Plaits pan"));
    m.insert("PP", CommandDef::new("PP", Some("PAN.PLA"), ArgCount::AtLeast(1), "Plaits pan"));
    m.insert("PAN.NOS", CommandDef::new("PAN.NOS", None, ArgCount::AtLeast(1), "Noise pan"));
    m.insert("PNN", CommandDef::new("PNN", Some("PAN.NOS"), ArgCount::AtLeast(1), "Noise pan"));
    m.insert("PAN.SMP", CommandDef::new("PAN.SMP", None, ArgCount::AtLeast(1), "Sampler pan"));
    m.insert("PS", CommandDef::new("PS", Some("PAN.SMP"), ArgCount::AtLeast(1), "Sampler pan"));

    // Mixer - Mute (0/1)
    // Note: Using MPL for MUTE.PLA instead of MP to avoid conflict with existing MP (Route to pitch)
    m.insert("MUTE.OSC", CommandDef::new("MUTE.OSC", None, ArgCount::AtLeast(1), "Mute complex osc"));
    m.insert("MO", CommandDef::new("MO", Some("MUTE.OSC"), ArgCount::AtLeast(1), "Mute complex osc"));
    m.insert("MUTE.PLA", CommandDef::new("MUTE.PLA", None, ArgCount::AtLeast(1), "Mute Plaits"));
    m.insert("MPL", CommandDef::new("MPL", Some("MUTE.PLA"), ArgCount::AtLeast(1), "Mute Plaits"));
    m.insert("MUTE.NOS", CommandDef::new("MUTE.NOS", None, ArgCount::AtLeast(1), "Mute noise"));
    m.insert("MN", CommandDef::new("MN", Some("MUTE.NOS"), ArgCount::AtLeast(1), "Mute noise"));
    m.insert("MUTE.SMP", CommandDef::new("MUTE.SMP", None, ArgCount::AtLeast(1), "Mute sampler"));
    m.insert("MS", CommandDef::new("MS", Some("MUTE.SMP"), ArgCount::AtLeast(1), "Mute sampler"));

    // Sampler - Loading & Triggering
    m.insert("KIT", CommandDef::new("KIT", None, ArgCount::AtLeast(1), "Load sample kit or file"));
    m.insert("STR", CommandDef::new("STR", None, ArgCount::AtLeast(0), "Trigger sample slot"));

    // Sampler - Pitch Parameters
    m.insert("S.RATE", CommandDef::new("S.RATE", None, ArgCount::AtLeast(1), "Sample playback rate"));
    m.insert("SR", CommandDef::new("SR", Some("S.RATE"), ArgCount::AtLeast(1), "Sample playback rate"));
    m.insert("S.PITCH", CommandDef::new("S.PITCH", None, ArgCount::AtLeast(1), "Sample pitch (semitones)"));
    m.insert("SPT", CommandDef::new("SPT", Some("S.PITCH"), ArgCount::AtLeast(1), "Sample pitch (semitones)"));
    m.insert("S.FINE", CommandDef::new("S.FINE", None, ArgCount::AtLeast(1), "Sample fine pitch (cents)"));
    m.insert("SFN", CommandDef::new("SFN", Some("S.FINE"), ArgCount::AtLeast(1), "Sample fine pitch (cents)"));

    // Sampler - Playback Parameters
    m.insert("S.DIR", CommandDef::new("S.DIR", None, ArgCount::AtLeast(1), "Sample direction (0=fwd, 1=rev)"));
    m.insert("SD", CommandDef::new("SD", Some("S.DIR"), ArgCount::AtLeast(1), "Sample direction (0=fwd, 1=rev)"));
    m.insert("S.LOOP", CommandDef::new("S.LOOP", None, ArgCount::AtLeast(1), "Sample loop mode"));
    m.insert("SL", CommandDef::new("SL", Some("S.LOOP"), ArgCount::AtLeast(1), "Sample loop mode"));
    m.insert("S.START", CommandDef::new("S.START", None, ArgCount::AtLeast(1), "Sample start offset"));
    m.insert("SST", CommandDef::new("SST", Some("S.START"), ArgCount::AtLeast(1), "Sample start offset"));
    m.insert("S.LEN", CommandDef::new("S.LEN", None, ArgCount::AtLeast(1), "Sample loop length"));
    m.insert("SLE", CommandDef::new("SLE", Some("S.LEN"), ArgCount::AtLeast(1), "Sample loop length"));

    // Sampler - Envelope Parameters
    m.insert("S.ATK", CommandDef::new("S.ATK", None, ArgCount::AtLeast(1), "Sample attack time"));
    m.insert("SA", CommandDef::new("SA", Some("S.ATK"), ArgCount::AtLeast(1), "Sample attack time"));
    m.insert("S.DEC", CommandDef::new("S.DEC", None, ArgCount::AtLeast(1), "Sample decay time"));
    m.insert("SDC", CommandDef::new("SDC", Some("S.DEC"), ArgCount::AtLeast(1), "Sample decay time"));
    m.insert("S.REL", CommandDef::new("S.REL", None, ArgCount::AtLeast(1), "Sample release time"));
    m.insert("SRE", CommandDef::new("SRE", Some("S.REL"), ArgCount::AtLeast(1), "Sample release time"));
    m.insert("S.SUST", CommandDef::new("S.SUST", None, ArgCount::AtLeast(1), "Sample sustain mode"));
    m.insert("SSU", CommandDef::new("SSU", Some("S.SUST"), ArgCount::AtLeast(1), "Sample sustain mode"));

    // Sampler - Output Parameters
    m.insert("S.VOL", CommandDef::new("S.VOL", None, ArgCount::AtLeast(1), "Sample volume"));
    m.insert("SV", CommandDef::new("SV", Some("S.VOL"), ArgCount::AtLeast(1), "Sample volume"));
    m.insert("S.PAN", CommandDef::new("S.PAN", None, ArgCount::AtLeast(1), "Sample pan position"));
    m.insert("SP", CommandDef::new("SP", Some("S.PAN"), ArgCount::AtLeast(1), "Sample pan position"));
    m.insert("S.FX", CommandDef::new("S.FX", None, ArgCount::AtLeast(1), "Sample FX routing"));
    m.insert("SFX", CommandDef::new("SFX", Some("S.FX"), ArgCount::AtLeast(1), "Sample FX routing"));

    // Sampler - Modulation Parameters
    m.insert("S.RATEMOD", CommandDef::new("S.RATEMOD", None, ArgCount::AtLeast(1), "Sample rate modulation"));
    m.insert("SRM", CommandDef::new("SRM", Some("S.RATEMOD"), ArgCount::AtLeast(1), "Sample rate modulation"));
    m.insert("S.PITCHMOD", CommandDef::new("S.PITCHMOD", None, ArgCount::AtLeast(1), "Sample pitch modulation"));
    m.insert("SPM", CommandDef::new("SPM", Some("S.PITCHMOD"), ArgCount::AtLeast(1), "Sample pitch modulation"));

    // Sampler FX - Filter (DFM1)
    m.insert("SF.CUT", CommandDef::new("SF.CUT", None, ArgCount::AtLeast(1), "Sampler filter cutoff"));
    m.insert("SFC", CommandDef::new("SFC", Some("SF.CUT"), ArgCount::AtLeast(1), "Sampler filter cutoff"));
    m.insert("SF.RES", CommandDef::new("SF.RES", None, ArgCount::AtLeast(1), "Sampler filter resonance"));
    m.insert("SFQ", CommandDef::new("SFQ", Some("SF.RES"), ArgCount::AtLeast(1), "Sampler filter resonance"));
    m.insert("SF.TYPE", CommandDef::new("SF.TYPE", None, ArgCount::AtLeast(1), "Sampler filter type"));
    m.insert("SFT", CommandDef::new("SFT", Some("SF.TYPE"), ArgCount::AtLeast(1), "Sampler filter type"));

    // Sampler FX - Decimator
    m.insert("SF.BITS", CommandDef::new("SF.BITS", None, ArgCount::AtLeast(1), "Sampler bit depth"));
    m.insert("SFB", CommandDef::new("SFB", Some("SF.BITS"), ArgCount::AtLeast(1), "Sampler bit depth"));
    m.insert("SF.RATE", CommandDef::new("SF.RATE", None, ArgCount::AtLeast(1), "Sampler rate reduction"));
    m.insert("SFR", CommandDef::new("SFR", Some("SF.RATE"), ArgCount::AtLeast(1), "Sampler rate reduction"));
    m.insert("SF.DECI", CommandDef::new("SF.DECI", None, ArgCount::AtLeast(1), "Sampler decimator mix"));
    m.insert("SFD", CommandDef::new("SFD", Some("SF.DECI"), ArgCount::AtLeast(1), "Sampler decimator mix"));

    // Sampler FX - Disintegrator
    m.insert("SF.PROB", CommandDef::new("SF.PROB", None, ArgCount::AtLeast(1), "Sampler glitch probability"));
    m.insert("SFP", CommandDef::new("SFP", Some("SF.PROB"), ArgCount::AtLeast(1), "Sampler glitch probability"));
    m.insert("SF.MULT", CommandDef::new("SF.MULT", None, ArgCount::AtLeast(1), "Sampler glitch multiplier"));
    m.insert("SFM", CommandDef::new("SFM", Some("SF.MULT"), ArgCount::AtLeast(1), "Sampler glitch multiplier"));
    m.insert("SF.GLIT", CommandDef::new("SF.GLIT", None, ArgCount::AtLeast(1), "Sampler disintegrator mix"));
    m.insert("SFG", CommandDef::new("SFG", Some("SF.GLIT"), ArgCount::AtLeast(1), "Sampler disintegrator mix"));
}
