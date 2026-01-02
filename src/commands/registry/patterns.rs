//! Pattern manipulation commands (P.*, PN.*)

use super::{ArgCount, CommandDef};

pub fn register_patterns(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    // Working pattern (P.*) query commands
    m.insert("P.N", CommandDef::new("P.N", None, ArgCount::Range(0, 1), "Working pattern number"));
    m.insert("P.L", CommandDef::new("P.L", None, ArgCount::Range(0, 1), "Working pattern length"));
    m.insert("P.I", CommandDef::new("P.I", None, ArgCount::Range(0, 1), "Working pattern index"));

    // Working pattern navigation
    m.insert("P.HERE", CommandDef::new("P.HERE", None, ArgCount::None, "Current pattern value"));
    m.insert("P.NEXT", CommandDef::new("P.NEXT", None, ArgCount::None, "Advance pattern index"));
    m.insert("P.PREV", CommandDef::new("P.PREV", None, ArgCount::None, "Decrement pattern index"));

    // Working pattern modification
    m.insert("P.PUSH", CommandDef::new("P.PUSH", None, ArgCount::AtLeast(1), "Append value(s)"));
    m.insert("P.POP", CommandDef::new("P.POP", None, ArgCount::None, "Remove last value"));
    m.insert("P.INS", CommandDef::new("P.INS", None, ArgCount::AtLeast(1), "Insert value at index"));
    m.insert("P.RM", CommandDef::new("P.RM", None, ArgCount::AtLeast(1), "Remove value at index"));
    m.insert("P.ROT", CommandDef::new("P.ROT", None, ArgCount::AtLeast(1), "Rotate pattern"));
    m.insert("P.REV", CommandDef::new("P.REV", None, ArgCount::None, "Reverse pattern"));
    m.insert("P.SHUF", CommandDef::new("P.SHUF", None, ArgCount::None, "Shuffle pattern"));
    m.insert("P.SORT", CommandDef::new("P.SORT", None, ArgCount::None, "Sort pattern"));
    m.insert("P.RND", CommandDef::new("P.RND", None, ArgCount::Custom, "Randomize pattern"));

    // Working pattern math
    m.insert("P.ADD", CommandDef::new("P.ADD", None, ArgCount::AtLeast(1), "Add to all values"));
    m.insert("P.SUB", CommandDef::new("P.SUB", None, ArgCount::AtLeast(1), "Subtract from all"));
    m.insert("P.MUL", CommandDef::new("P.MUL", None, ArgCount::AtLeast(1), "Multiply all values"));
    m.insert("P.DIV", CommandDef::new("P.DIV", None, ArgCount::AtLeast(1), "Divide all values"));
    m.insert("P.MOD", CommandDef::new("P.MOD", None, ArgCount::AtLeast(1), "Modulo all values"));
    m.insert("P.SCALE", CommandDef::new("P.SCALE", None, ArgCount::AtLeast(2), "Scale to min/max"));

    // Working pattern queries
    m.insert("P.MIN", CommandDef::new("P.MIN", None, ArgCount::None, "Minimum value"));
    m.insert("P.MAX", CommandDef::new("P.MAX", None, ArgCount::None, "Maximum value"));
    m.insert("P.SUM", CommandDef::new("P.SUM", None, ArgCount::None, "Sum of values"));
    m.insert("P.AVG", CommandDef::new("P.AVG", None, ArgCount::None, "Average value"));
    m.insert("P.FND", CommandDef::new("P.FND", None, ArgCount::AtLeast(1), "Find value in pattern"));

    // Working pattern access
    m.insert("P", CommandDef::new("P", None, ArgCount::AtLeast(1), "Get/set pattern value"));

    // Numbered pattern (PN.*) operations
    m.insert("PN.L", CommandDef::new("PN.L", None, ArgCount::AtLeast(1), "Pattern N length"));
    m.insert("PN.I", CommandDef::new("PN.I", None, ArgCount::AtLeast(1), "Pattern N index"));

    // Numbered pattern navigation
    m.insert("PN.HERE", CommandDef::new("PN.HERE", None, ArgCount::AtLeast(1), "Pattern N current value"));
    m.insert("PN.NEXT", CommandDef::new("PN.NEXT", None, ArgCount::AtLeast(1), "Advance pattern N"));
    m.insert("PN.PREV", CommandDef::new("PN.PREV", None, ArgCount::AtLeast(1), "Decrement pattern N"));

    // Numbered pattern modification
    m.insert("PN.PUSH", CommandDef::new("PN.PUSH", None, ArgCount::AtLeast(2), "Append to pattern N"));
    m.insert("PN.POP", CommandDef::new("PN.POP", None, ArgCount::AtLeast(1), "Remove from pattern N"));
    m.insert("PN.INS", CommandDef::new("PN.INS", None, ArgCount::AtLeast(3), "Insert in pattern N"));
    m.insert("PN.RM", CommandDef::new("PN.RM", None, ArgCount::AtLeast(2), "Remove from pattern N"));
    m.insert("PN.ROT", CommandDef::new("PN.ROT", None, ArgCount::AtLeast(2), "Rotate pattern N"));
    m.insert("PN.REV", CommandDef::new("PN.REV", None, ArgCount::AtLeast(1), "Reverse pattern N"));
    m.insert("PN.SHUF", CommandDef::new("PN.SHUF", None, ArgCount::AtLeast(1), "Shuffle pattern N"));
    m.insert("PN.SORT", CommandDef::new("PN.SORT", None, ArgCount::AtLeast(1), "Sort pattern N"));
    m.insert("PN.RND", CommandDef::new("PN.RND", None, ArgCount::AtLeast(1), "Randomize pattern N"));

    // Numbered pattern math
    m.insert("PN.ADD", CommandDef::new("PN.ADD", None, ArgCount::AtLeast(2), "Add to pattern N"));
    m.insert("PN.SUB", CommandDef::new("PN.SUB", None, ArgCount::AtLeast(2), "Subtract from pattern N"));
    m.insert("PN.MUL", CommandDef::new("PN.MUL", None, ArgCount::AtLeast(2), "Multiply pattern N"));
    m.insert("PN.DIV", CommandDef::new("PN.DIV", None, ArgCount::AtLeast(2), "Divide pattern N"));
    m.insert("PN.MOD", CommandDef::new("PN.MOD", None, ArgCount::AtLeast(2), "Modulo pattern N"));
    m.insert("PN.SCALE", CommandDef::new("PN.SCALE", None, ArgCount::AtLeast(3), "Scale pattern N"));

    // Numbered pattern queries
    m.insert("PN.MIN", CommandDef::new("PN.MIN", None, ArgCount::AtLeast(1), "Pattern N min value"));
    m.insert("PN.MAX", CommandDef::new("PN.MAX", None, ArgCount::AtLeast(1), "Pattern N max value"));
    m.insert("PN.SUM", CommandDef::new("PN.SUM", None, ArgCount::AtLeast(1), "Pattern N sum"));
    m.insert("PN.AVG", CommandDef::new("PN.AVG", None, ArgCount::AtLeast(1), "Pattern N average"));
    m.insert("PN.FND", CommandDef::new("PN.FND", None, ArgCount::AtLeast(2), "Find in pattern N"));

    // Numbered pattern access
    m.insert("PN", CommandDef::new("PN", None, ArgCount::AtLeast(2), "Get/set pattern N value"));
}
