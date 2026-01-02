//! Control flow and randomization commands

use super::{ArgCount, CommandDef};

pub fn register_control(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    // Trigger and Control
    m.insert("TR", CommandDef::new("TR", None, ArgCount::None, "Trigger voice"));
    m.insert("PLTR", CommandDef::new("PLTR", None, ArgCount::None, "Trigger Plaits"));
    m.insert("RST", CommandDef::new("RST", None, ArgCount::None, "Reset system"));
    m.insert("BRK", CommandDef::new("BRK", None, ArgCount::None, "Break script execution"));
    m.insert("CLEAR", CommandDef::new("CLEAR", None, ArgCount::None, "Clear screen"));
    m.insert("CLR", CommandDef::new("CLR", None, ArgCount::None, "Clear screen"));

    // Script execution
    m.insert("SCRIPT", CommandDef::new("SCRIPT", None, ArgCount::AtLeast(1), "Execute script"));
    m.insert("$", CommandDef::new("$", Some("SCRIPT"), ArgCount::AtLeast(1), "Execute script"));

    // Math operations
    m.insert("ADD", CommandDef::new("ADD", None, ArgCount::AtLeast(2), "Add values"));
    m.insert("+", CommandDef::new("+", Some("ADD"), ArgCount::AtLeast(2), "Add values"));
    m.insert("SUB", CommandDef::new("SUB", None, ArgCount::AtLeast(2), "Subtract values"));
    m.insert("-", CommandDef::new("-", Some("SUB"), ArgCount::AtLeast(2), "Subtract values"));
    m.insert("MUL", CommandDef::new("MUL", None, ArgCount::AtLeast(2), "Multiply values"));
    m.insert("*", CommandDef::new("*", Some("MUL"), ArgCount::AtLeast(2), "Multiply values"));
    m.insert("DIV", CommandDef::new("DIV", None, ArgCount::AtLeast(2), "Divide values"));
    m.insert("/", CommandDef::new("/", Some("DIV"), ArgCount::AtLeast(2), "Divide values"));
    m.insert("MOD", CommandDef::new("MOD", None, ArgCount::AtLeast(2), "Modulo operation"));
    m.insert("%", CommandDef::new("%", Some("MOD"), ArgCount::AtLeast(2), "Modulo operation"));
    m.insert("MAP", CommandDef::new("MAP", None, ArgCount::AtLeast(5), "Map value to range"));
    m.insert("N", CommandDef::new("N", None, ArgCount::AtLeast(1), "Note to frequency"));

    // Random operations
    m.insert("RND", CommandDef::new("RND", None, ArgCount::AtLeast(1), "Random value"));
    m.insert("RRND", CommandDef::new("RRND", None, ArgCount::AtLeast(2), "Random range"));
    m.insert("TOSS", CommandDef::new("TOSS", None, ArgCount::None, "Random 0 or 1"));
    m.insert("EITH", CommandDef::new("EITH", None, ArgCount::AtLeast(2), "Either value"));
    m.insert("TOG", CommandDef::new("TOG", None, ArgCount::AtLeast(2), "Toggle between values"));

    // Randomization
    m.insert("RND.VOICE", CommandDef::new("RND.VOICE", None, ArgCount::None, "Randomize voice"));
    m.insert("RND.OSC", CommandDef::new("RND.OSC", None, ArgCount::None, "Randomize oscillators"));
    m.insert("RND.FM", CommandDef::new("RND.FM", None, ArgCount::None, "Randomize FM"));
    m.insert("RND.MOD", CommandDef::new("RND.MOD", None, ArgCount::None, "Randomize modulation"));
    m.insert("RND.ENV", CommandDef::new("RND.ENV", None, ArgCount::None, "Randomize envelopes"));
    m.insert("RND.P", CommandDef::new("RND.P", None, ArgCount::Custom, "Randomize working pattern"));
    m.insert("RND.PN", CommandDef::new("RND.PN", None, ArgCount::Custom, "Randomize pattern N"));
    m.insert("RND.PALL", CommandDef::new("RND.PALL", None, ArgCount::Custom, "Randomize all patterns"));
    m.insert("RND.PL", CommandDef::new("RND.PL", None, ArgCount::None, "Randomize Plaits"));
    m.insert("RND.FX", CommandDef::new("RND.FX", None, ArgCount::None, "Randomize effects"));
    m.insert("RND.FILT", CommandDef::new("RND.FILT", None, ArgCount::None, "Randomize filter"));
    m.insert("RND.DLY", CommandDef::new("RND.DLY", None, ArgCount::None, "Randomize delay"));
    m.insert("RND.VERB", CommandDef::new("RND.VERB", None, ArgCount::None, "Randomize reverb"));

    // Comparisons
    m.insert("EZ", CommandDef::new("EZ", None, ArgCount::AtLeast(1), "Equal to zero"));
    m.insert("NZ", CommandDef::new("NZ", None, ArgCount::AtLeast(1), "Not zero"));
    m.insert("GT", CommandDef::new("GT", None, ArgCount::AtLeast(2), "Greater than"));
    m.insert("LT", CommandDef::new("LT", None, ArgCount::AtLeast(2), "Less than"));
    m.insert("GTE", CommandDef::new("GTE", None, ArgCount::AtLeast(2), "Greater than or equal"));
    m.insert("LTE", CommandDef::new("LTE", None, ArgCount::AtLeast(2), "Less than or equal"));

    // Output
    m.insert("PRINT", CommandDef::new("PRINT", None, ArgCount::AtLeast(1), "Print value"));
}
