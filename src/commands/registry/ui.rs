//! UI, display, and configuration commands

use super::{ArgCount, CommandDef};

pub fn register_ui(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    // Theme and Display
    m.insert("THEME", CommandDef::new("THEME", None, ArgCount::Range(0, 1), "Color theme"));
    m.insert("DEBUG", CommandDef::new("DEBUG", None, ArgCount::Range(0, 1), "Debug level"));
    m.insert("HEADER", CommandDef::new("HEADER", None, ArgCount::Range(0, 1), "Header display level"));
    m.insert("TITLE", CommandDef::new("TITLE", None, ArgCount::Range(0, 1), "Title mode"));
    m.insert("TITLE.TIMER", CommandDef::new("TITLE.TIMER", None, ArgCount::Range(0, 2), "Title timer"));

    // Output Flags
    m.insert("OUT.ERR", CommandDef::new("OUT.ERR", None, ArgCount::Range(0, 1), "Output errors"));
    m.insert("OUT.ESS", CommandDef::new("OUT.ESS", None, ArgCount::Range(0, 1), "Output essential"));
    m.insert("OUT.QRY", CommandDef::new("OUT.QRY", None, ArgCount::Range(0, 1), "Output queries"));
    m.insert("OUT.CFM", CommandDef::new("OUT.CFM", None, ArgCount::Range(0, 1), "Output confirmations"));
    m.insert("REPL.DUMP", CommandDef::new("REPL.DUMP", None, ArgCount::Range(0, 1), "REPL dump mode"));

    // Meters
    m.insert("METER.HDR", CommandDef::new("METER.HDR", None, ArgCount::Range(0, 1), "Show meters in header"));
    m.insert("METER.GRID", CommandDef::new("METER.GRID", None, ArgCount::Range(0, 1), "Show meters in grid"));
    m.insert("METER.ASCII", CommandDef::new("METER.ASCII", None, ArgCount::Range(0, 1), "ASCII meters"));

    // Grid
    m.insert("GRID", CommandDef::new("GRID", None, ArgCount::Range(0, 1), "Grid visibility"));
    m.insert("GRID.DEF", CommandDef::new("GRID.DEF", None, ArgCount::Range(0, 1), "Grid default view"));
    m.insert("GRID.MODE", CommandDef::new("GRID.MODE", None, ArgCount::Range(0, 1), "Grid mode"));

    // Highlighting
    m.insert("HL.COND", CommandDef::new("HL.COND", None, ArgCount::Range(0, 1), "Highlight conditionals"));
    m.insert("HL.SEQ", CommandDef::new("HL.SEQ", None, ArgCount::Range(0, 1), "Highlight sequences"));

    // Activity
    m.insert("ACTIVITY", CommandDef::new("ACTIVITY", None, ArgCount::Range(0, 1), "Activity indicator"));
    m.insert("FLASH", CommandDef::new("FLASH", None, ArgCount::Range(0, 1), "Activity flash duration"));

    // Spectrum and Scope
    m.insert("SPECTRUM", CommandDef::new("SPECTRUM", None, ArgCount::Range(0, 1), "Spectrum analyzer"));
    m.insert("SCOPE.TIME", CommandDef::new("SCOPE.TIME", None, ArgCount::Custom, "Scope time window"));
    m.insert("SCOPE.CLR", CommandDef::new("SCOPE.CLR", None, ArgCount::Custom, "Scope color"));
    m.insert("SCOPE.MODE", CommandDef::new("SCOPE.MODE", None, ArgCount::Custom, "Scope mode"));
    m.insert("SCOPE.UNI", CommandDef::new("SCOPE.UNI", None, ArgCount::Custom, "Scope unipolar"));
    m.insert("SCOPE.GAIN", CommandDef::new("SCOPE.GAIN", Some("SCG"), ArgCount::Range(0, 1), "Scope input gain"));
    m.insert("SCG", CommandDef::new("SCG", None, ArgCount::Range(0, 1), "Scope input gain"));
    m.insert("SCOPE.RST", CommandDef::new("SCOPE.RST", Some("SCR"), ArgCount::None, "Reset scope settings"));
    m.insert("SCR", CommandDef::new("SCR", None, ArgCount::None, "Reset scope settings"));

    // Notes
    m.insert("NOTE", CommandDef::new("NOTE", None, ArgCount::AtLeast(1), "Add note"));
    m.insert("NOTE.CLR", CommandDef::new("NOTE.CLR", None, ArgCount::None, "Clear notes"));

    // Performance monitoring
    m.insert("CPU", CommandDef::new("CPU", None, ArgCount::Range(0, 1), "CPU meter"));
    m.insert("BPM", CommandDef::new("BPM", None, ArgCount::Range(0, 1), "BPM display"));

    // Scramble
    m.insert("SCRMBL", CommandDef::new("SCRMBL", None, ArgCount::Range(0, 1), "Scramble text"));
    m.insert("SCRMBL.MODE", CommandDef::new("SCRMBL.MODE", None, ArgCount::Range(0, 1), "Scramble mode"));
    m.insert("SCRMBL.SPD", CommandDef::new("SCRMBL.SPD", None, ArgCount::Range(0, 1), "Scramble speed"));
    m.insert("SCRMBL.CRV", CommandDef::new("SCRMBL.CRV", None, ArgCount::Range(0, 1), "Scramble curve"));

    // Confirmation
    m.insert("CFM.QUIT", CommandDef::new("CFM.QUIT", None, ArgCount::Range(0, 1), "Confirm quit if unsaved"));
    m.insert("CFM.SAVE", CommandDef::new("CFM.SAVE", None, ArgCount::Range(0, 1), "Confirm overwrite scene"));
}
