//! Effects and FX parameter commands

use super::{ArgCount, CommandDef};

pub fn register_effects(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    // Delay
    m.insert("DT", CommandDef::new("DT", Some("DLY.TIME"), ArgCount::AtLeast(1), "Delay time"));
    m.insert("DF", CommandDef::new("DF", Some("DLY.FB"), ArgCount::AtLeast(1), "Delay feedback"));
    m.insert("DLP", CommandDef::new("DLP", Some("DLY.LP"), ArgCount::AtLeast(1), "Delay lowpass"));
    m.insert("DW", CommandDef::new("DW", Some("DLY.WET"), ArgCount::AtLeast(1), "Delay wet"));
    m.insert("DS", CommandDef::new("DS", Some("DLY.SYN"), ArgCount::AtLeast(1), "Delay sync"));
    m.insert("D.MODE", CommandDef::new("D.MODE", Some("DLY.MODE"), ArgCount::AtLeast(1), "Delay mode (0-2)"));
    m.insert("D.TAIL", CommandDef::new("D.TAIL", Some("DLY.TAIL"), ArgCount::AtLeast(1), "Delay tail (0-2)"));

    // Reverb
    m.insert("RV", CommandDef::new("RV", Some("REV.DEC"), ArgCount::AtLeast(1), "Reverb decay"));
    m.insert("RP", CommandDef::new("RP", Some("REV.PRE"), ArgCount::AtLeast(1), "Reverb predelay"));
    m.insert("RH", CommandDef::new("RH", Some("REV.DMP"), ArgCount::AtLeast(1), "Reverb damping"));
    m.insert("RW", CommandDef::new("RW", Some("REV.WET"), ArgCount::AtLeast(1), "Reverb wet"));
    m.insert("R.MODE", CommandDef::new("R.MODE", Some("REV.MODE"), ArgCount::AtLeast(1), "Reverb mode (0-2)"));
    m.insert("R.TAIL", CommandDef::new("R.TAIL", Some("REV.TAIL"), ArgCount::AtLeast(1), "Reverb tail (0-2)"));

    // Lo-Fi
    m.insert("LB", CommandDef::new("LB", Some("LOFI.BIT"), ArgCount::AtLeast(1), "Lo-fi bit depth"));
    m.insert("LS", CommandDef::new("LS", Some("LOFI.SMP"), ArgCount::AtLeast(1), "Lo-fi sample rate"));
    m.insert("LM", CommandDef::new("LM", Some("LOFI.MIX"), ArgCount::AtLeast(1), "Lo-fi mix"));

    // Ring Modulator
    m.insert("RGF", CommandDef::new("RGF", Some("RING.FRQ"), ArgCount::AtLeast(1), "Ring mod frequency"));
    m.insert("RGW", CommandDef::new("RGW", Some("RING.WAV"), ArgCount::AtLeast(1), "Ring mod waveform"));
    m.insert("RGM", CommandDef::new("RGM", Some("RING.MIX"), ArgCount::AtLeast(1), "Ring mod mix"));

    // Compressor
    m.insert("CT", CommandDef::new("CT", Some("COMP.THR"), ArgCount::AtLeast(1), "Compressor threshold"));
    m.insert("CR", CommandDef::new("CR", Some("COMP.RAT"), ArgCount::AtLeast(1), "Compressor ratio"));
    m.insert("CA", CommandDef::new("CA", Some("COMP.ATK"), ArgCount::AtLeast(1), "Compressor attack"));
    m.insert("CL", CommandDef::new("CL", Some("COMP.REL"), ArgCount::AtLeast(1), "Compressor release"));
    m.insert("CM", CommandDef::new("CM", Some("COMP.MKP"), ArgCount::AtLeast(1), "Compressor makeup"));
    m.insert("CR.MIX", CommandDef::new("CR.MIX", Some("COMP.MIX"), ArgCount::AtLeast(1), "Compressor mix"));
    m.insert("CRMIX", CommandDef::new("CRMIX", Some("COMP.MIX"), ArgCount::AtLeast(1), "Compressor mix"));
    m.insert("CR.AUTO", CommandDef::new("CR.AUTO", Some("CRA"), ArgCount::Range(0, 1), "Compressor auto-makeup"));
    m.insert("CRA", CommandDef::new("CRA", Some("CR.AUTO"), ArgCount::Range(0, 1), "Compressor auto-makeup"));

    // EQ
    m.insert("EL", CommandDef::new("EL", Some("EQ.LOW"), ArgCount::AtLeast(1), "EQ low band"));
    m.insert("ELF", CommandDef::new("ELF", Some("EQ.LF"), ArgCount::AtLeast(1), "EQ low shelf freq"));
    m.insert("EM", CommandDef::new("EM", Some("EQ.MID"), ArgCount::AtLeast(1), "EQ mid band"));
    m.insert("EH", CommandDef::new("EH", Some("EQ.HI"), ArgCount::AtLeast(1), "EQ high band"));
    m.insert("EHF", CommandDef::new("EHF", Some("EQ.HF"), ArgCount::AtLeast(1), "EQ high shelf freq"));
    m.insert("EF", CommandDef::new("EF", Some("EQ.FRQ"), ArgCount::AtLeast(1), "EQ mid frequency"));
    m.insert("EQ", CommandDef::new("EQ", None, ArgCount::AtLeast(1), "EQ mid Q"));

    // Beat Repeat
    m.insert("BRL", CommandDef::new("BRL", Some("BR.LEN"), ArgCount::AtLeast(1), "Beat repeat length"));
    m.insert("BRR", CommandDef::new("BRR", Some("BR.REV"), ArgCount::AtLeast(1), "Beat repeat reverse"));
    m.insert("BRW", CommandDef::new("BRW", Some("BR.WIN"), ArgCount::AtLeast(1), "Beat repeat window"));
    m.insert("BRX", CommandDef::new("BRX", Some("BR.MIX"), ArgCount::AtLeast(1), "Beat repeat mix"));
    m.insert("BR.LEN", CommandDef::new("BR.LEN", None, ArgCount::AtLeast(1), "Beat repeat length"));
    m.insert("BR.REV", CommandDef::new("BR.REV", None, ArgCount::AtLeast(1), "Beat repeat reverse"));
    m.insert("BR.WIN", CommandDef::new("BR.WIN", None, ArgCount::AtLeast(1), "Beat repeat window"));
    m.insert("BR.MIX", CommandDef::new("BR.MIX", None, ArgCount::AtLeast(1), "Beat repeat mix"));

    // Pitch Shift
    m.insert("PSM", CommandDef::new("PSM", Some("PS.MODE"), ArgCount::AtLeast(1), "Pitch shift mode"));
    m.insert("PSS", CommandDef::new("PSS", Some("PS.SEMI"), ArgCount::AtLeast(1), "Pitch shift semitones"));
    m.insert("PSG", CommandDef::new("PSG", Some("PS.GRAIN"), ArgCount::AtLeast(1), "Pitch shift grain size"));
    m.insert("PSX", CommandDef::new("PSX", Some("PS.MIX"), ArgCount::AtLeast(1), "Pitch shift mix"));
    m.insert("PST", CommandDef::new("PST", Some("PS.TARG"), ArgCount::AtLeast(1), "Pitch shift target"));
    m.insert("PS.MODE", CommandDef::new("PS.MODE", None, ArgCount::AtLeast(1), "Pitch shift mode"));
    m.insert("PS.SEMI", CommandDef::new("PS.SEMI", None, ArgCount::AtLeast(1), "Pitch shift semitones"));
    m.insert("PS.GRAIN", CommandDef::new("PS.GRAIN", None, ArgCount::AtLeast(1), "Pitch shift grain size"));
    m.insert("PS.MIX", CommandDef::new("PS.MIX", None, ArgCount::AtLeast(1), "Pitch shift mix"));
    m.insert("PS.TARG", CommandDef::new("PS.TARG", None, ArgCount::AtLeast(1), "Pitch shift target"));

    // Clouds Granular
    m.insert("CLTR", CommandDef::new("CLTR", Some("CL.TRIG"), ArgCount::None, "Trigger clouds"));
    m.insert("CLP", CommandDef::new("CLP", None, ArgCount::AtLeast(1), "Clouds pitch"));
    m.insert("CLPT", CommandDef::new("CLPT", Some("CL.PITCH"), ArgCount::AtLeast(1), "Clouds pitch"));
    m.insert("CLO", CommandDef::new("CLO", None, ArgCount::AtLeast(1), "Clouds position"));
    m.insert("CLPS", CommandDef::new("CLPS", Some("CL.POS"), ArgCount::AtLeast(1), "Clouds position"));
    m.insert("CLS", CommandDef::new("CLS", None, ArgCount::AtLeast(1), "Clouds size"));
    m.insert("CLSZ", CommandDef::new("CLSZ", Some("CL.SIZE"), ArgCount::AtLeast(1), "Clouds size"));
    m.insert("CLD", CommandDef::new("CLD", None, ArgCount::AtLeast(1), "Clouds density"));
    m.insert("CLDS", CommandDef::new("CLDS", Some("CL.DENS"), ArgCount::AtLeast(1), "Clouds density"));
    m.insert("CLT", CommandDef::new("CLT", None, ArgCount::AtLeast(1), "Clouds texture"));
    m.insert("CLTX", CommandDef::new("CLTX", Some("CL.TEX"), ArgCount::AtLeast(1), "Clouds texture"));
    m.insert("CLW", CommandDef::new("CLW", Some("CL.WET"), ArgCount::AtLeast(1), "Clouds wet"));
    m.insert("CLG", CommandDef::new("CLG", Some("CL.GAIN"), ArgCount::AtLeast(1), "Clouds gain"));
    m.insert("CLSP", CommandDef::new("CLSP", Some("CL.SPREAD"), ArgCount::AtLeast(1), "Clouds spread"));
    m.insert("CLRV", CommandDef::new("CLRV", Some("CL.RVB"), ArgCount::AtLeast(1), "Clouds reverb"));
    m.insert("CLF", CommandDef::new("CLF", Some("CL.FB"), ArgCount::AtLeast(1), "Clouds feedback"));
    m.insert("CLFZ", CommandDef::new("CLFZ", Some("CL.FREEZE"), ArgCount::AtLeast(1), "Clouds freeze"));
    m.insert("CLM", CommandDef::new("CLM", Some("CL.MODE"), ArgCount::AtLeast(1), "Clouds mode"));
    m.insert("CLLO", CommandDef::new("CLLO", Some("CL.LOFI"), ArgCount::AtLeast(1), "Clouds lo-fi"));
    m.insert("CL.TRIG", CommandDef::new("CL.TRIG", None, ArgCount::None, "Trigger clouds"));
    m.insert("CL.PITCH", CommandDef::new("CL.PITCH", None, ArgCount::AtLeast(1), "Clouds pitch"));
    m.insert("CL.POS", CommandDef::new("CL.POS", None, ArgCount::AtLeast(1), "Clouds position"));
    m.insert("CL.SIZE", CommandDef::new("CL.SIZE", None, ArgCount::AtLeast(1), "Clouds size"));
    m.insert("CL.DENS", CommandDef::new("CL.DENS", None, ArgCount::AtLeast(1), "Clouds density"));
    m.insert("CL.TEX", CommandDef::new("CL.TEX", None, ArgCount::AtLeast(1), "Clouds texture"));
    m.insert("CL.WET", CommandDef::new("CL.WET", None, ArgCount::AtLeast(1), "Clouds wet"));
    m.insert("CL.GAIN", CommandDef::new("CL.GAIN", None, ArgCount::AtLeast(1), "Clouds gain"));
    m.insert("CL.SPREAD", CommandDef::new("CL.SPREAD", None, ArgCount::AtLeast(1), "Clouds spread"));
    m.insert("CL.RVB", CommandDef::new("CL.RVB", None, ArgCount::AtLeast(1), "Clouds reverb"));
    m.insert("CL.FB", CommandDef::new("CL.FB", None, ArgCount::AtLeast(1), "Clouds feedback"));
    m.insert("CL.FREEZE", CommandDef::new("CL.FREEZE", None, ArgCount::AtLeast(1), "Clouds freeze"));
    m.insert("CL.MODE", CommandDef::new("CL.MODE", None, ArgCount::AtLeast(1), "Clouds mode"));
    m.insert("CL.LOFI", CommandDef::new("CL.LOFI", None, ArgCount::AtLeast(1), "Clouds lo-fi"));
}
