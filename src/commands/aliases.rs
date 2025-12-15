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
    m.insert("MODF.CUT", "MFF");
    m.insert("MODF.RES", "MFQ");

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
    m.insert("COMP.MIX", "CRMIX");

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
    m.insert("NOISE.PRI", "NP");
    m.insert("NOISE.MOD", "NM");
    m.insert("NOISE.VOL", "NV");

    // Source levels
    m.insert("PRI.VOL", "PV");
    m.insert("MOD.VOL", "MV");

    // Plaits
    m.insert("PL.FREQ", "PLF");
    m.insert("PL.HARM", "PLH");
    m.insert("PL.TIMB", "PLT");
    m.insert("PL.ENG", "PLE");
    m.insert("PL.MORPH", "PLM");
    m.insert("PL.DEC", "PLD");
    m.insert("PL.LPG", "PLL");

    // Pitch Shift
    m.insert("PS.MODE", "PSM");
    m.insert("PS.SEMI", "PSS");
    m.insert("PS.GRAIN", "PSG");
    m.insert("PS.MIX", "PSX");
    m.insert("PS.TARG", "PST");

    // Beat Repeat
    m.insert("BR.LEN", "BRL");
    m.insert("BR.REV", "BRR");
    m.insert("BR.WIN", "BRW");
    m.insert("BR.MIX", "BRX");

    // MiClouds Granular Effect
    m.insert("CL.TRIG", "CLTR");
    m.insert("CL.PITCH", "CLP");
    m.insert("CL.POS", "CLO");
    m.insert("CL.SIZE", "CLS");
    m.insert("CL.DENS", "CLD");
    m.insert("CL.TEX", "CLT");
    m.insert("CL.WET", "CLW");
    m.insert("CL.GAIN", "CLG");
    m.insert("CL.SPREAD", "CLSP");
    m.insert("CL.RVB", "CLRV");
    m.insert("CL.FB", "CLF");
    m.insert("CL.FREEZE", "CLFZ");
    m.insert("CL.MODE", "CLM");
    m.insert("CL.LOFI", "CLLO");

    // Page navigation
    m.insert("PG", "PAGE");

    m
});

use std::fs::OpenOptions;
use std::io::Write;

fn log_alias_debug(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/monokit_commands.log")
    {
        let _ = writeln!(file, "{}", msg);
    }
}

pub fn resolve_alias(cmd: &str) -> String {
    // DEBUG: Log CL command alias resolution
    if cmd.starts_with("CL") {
        log_alias_debug(&format!("[DEBUG] [ALIAS-1] resolve_alias called with cmd='{}'", cmd));
        let result = CANONICAL_TO_ALIAS
            .get(cmd)
            .map(|&alias| {
                log_alias_debug(&format!("[DEBUG] [ALIAS-2] Found in map: '{}' -> '{}'", cmd, alias));
                alias.to_string()
            })
            .unwrap_or_else(|| {
                log_alias_debug(&format!("[DEBUG] [ALIAS-3] NOT found in map, returning unchanged: '{}'", cmd));
                cmd.to_string()
            });
        log_alias_debug(&format!("[DEBUG] [ALIAS-4] resolve_alias returning: '{}'", result));
        result
    } else {
        CANONICAL_TO_ALIAS
            .get(cmd)
            .map(|&alias| alias.to_string())
            .unwrap_or_else(|| cmd.to_string())
    }
}
