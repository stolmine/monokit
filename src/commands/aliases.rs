use once_cell::sync::Lazy;
use std::collections::HashMap;

static CANONICAL_TO_ALIAS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // NOTE: FMEV.CRV and DENV.CRV have no short aliases
    // FC = Filter Cutoff (FILT.CUT), DC = Discontinuity Amount (DISC.AMT)

    m.insert("POSC.FREQ", "PF");
    m.insert("POSC.WAVE", "PW");

    m.insert("MOSC.FREQ", "MF");
    m.insert("MOSC.WAVE", "MW");
    m.insert("MOSC.FB", "FB");
    m.insert("MOSC.FBA", "FBA");

    m.insert("DISC.AMT", "DC");
    m.insert("DISC.MODE", "DM");

    m.insert("FILT.CUT", "FC");
    m.insert("FILT.RES", "FQ");
    m.insert("FILT.TYP", "FT");
    m.insert("FILT.KEY", "FK");
    m.insert("FLEV.AMT", "FE");

    m.insert("RESO.FRQ", "RF");
    m.insert("RESO.DEC", "RD");
    m.insert("RESO.MIX", "RM");
    m.insert("RESO.KEY", "RK");

    m.insert("DLY.TIME", "DT");
    m.insert("DLY.FB", "DF");
    m.insert("DLY.WET", "DW");
    m.insert("DLY.LP", "DLP");
    m.insert("DLY.SYN", "DS");
    m.insert("DLY.MODE", "D.MODE");
    m.insert("DLY.TAIL", "D.TAIL");

    m.insert("REV.DEC", "RV");
    m.insert("REV.PRE", "RP");
    m.insert("REV.DMP", "RH");
    m.insert("REV.WET", "RW");
    m.insert("REV.MODE", "R.MODE");
    m.insert("REV.TAIL", "R.TAIL");

    m.insert("LOFI.BIT", "LB");
    m.insert("LOFI.SMP", "LS");
    m.insert("LOFI.MIX", "LM");

    m.insert("RING.FRQ", "RGF");
    m.insert("RING.WAV", "RGW");
    m.insert("RING.MIX", "RGM");

    m.insert("COMP.THR", "CT");
    m.insert("COMP.RAT", "CR");
    m.insert("COMP.ATK", "CA");
    m.insert("COMP.REL", "CL");
    m.insert("COMP.MKP", "CM");

    m.insert("EQ.LOW", "EL");
    m.insert("EQ.MID", "EM");
    m.insert("EQ.HI", "EH");
    m.insert("EQ.FRQ", "EF");

    m.insert("MBUS.AMT", "MB");
    m.insert("MBEV.AMT", "MBA");
    m.insert("MBEV.DEC", "MBD");
    m.insert("MBUS.TRK", "TK");
    m.insert("MBUS.FM", "FM");
    m.insert("MBUS.MIX", "MX");
    m.insert("MBUS.MMX", "MM");
    m.insert("MBUS.EMX", "ME");

    m.insert("ROUT.MP", "MP");
    m.insert("ROUT.MD", "MD");
    m.insert("ROUT.MT", "MT");
    m.insert("ROUT.MA", "MA");
    m.insert("ROUT.MC", "MC");
    m.insert("ROUT.MQ", "MQ");

    m.insert("OUT.VOL", "VOL");
    m.insert("OUT.PAN", "PAN");

    m.insert("AENV.DEC", "AD");
    m.insert("PENV.DEC", "PD");
    m.insert("FMEV.DEC", "FD");
    m.insert("DENV.DEC", "DD");
    m.insert("FBEV.DEC", "FBD");
    m.insert("FLEV.DEC", "FED");

    m.insert("PENV.AMT", "PA");
    m.insert("FMEV.AMT", "FA");
    m.insert("DENV.AMT", "DA");

    // Envelope ATK short forms (canonical → alias)
    m.insert("AENV.ATK", "AA");
    m.insert("PENV.ATK", "PAA");
    m.insert("FMEV.ATK", "FAA");
    m.insert("DENV.ATK", "DAA");
    m.insert("FBEV.ATK", "FBAA");
    m.insert("FLEV.ATK", "FLAA");

    // Envelope CRV short forms (canonical → alias)
    m.insert("AENV.CRV", "AC");
    m.insert("PENV.CRV", "PC");
    m.insert("FBEV.CRV", "FBC");
    m.insert("FLEV.CRV", "FLC");
    // Note: FMEV.CRV and DENV.CRV have no aliases (FC=FILT.CUT, DC=DISC.AMT)

    // Noise controls
    m.insert("NOISE.WAV", "NW");
    m.insert("NOEV.ATK", "NA");
    m.insert("NOEV.DEC", "ND");
    m.insert("NOEV.CRV", "NC");
    m.insert("NOISE.ENV", "NE");
    m.insert("NOISE.PRI", "NP");
    m.insert("NOISE.MOD", "NM");
    m.insert("NOISE.VOL", "NV");
    m.insert("NOISE.GATE", "NG");

    // Source levels
    m.insert("PRI.VOL", "PV");
    m.insert("MOD.VOL", "MV");

    m
});

pub fn resolve_alias(cmd: &str) -> String {
    CANONICAL_TO_ALIAS
        .get(cmd)
        .map(|&alias| alias.to_string())
        .unwrap_or_else(|| cmd.to_string())
}
