//! Variable assignment commands (A-K, T, J)

use super::{ArgCount, CommandDef};

pub fn register_variables(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    m.insert("A", CommandDef::new("A", None, ArgCount::Custom, "Variable A (expression)").with_special_validation());
    m.insert("B", CommandDef::new("B", None, ArgCount::Custom, "Variable B (expression)").with_special_validation());
    m.insert("C", CommandDef::new("C", None, ArgCount::Custom, "Variable C (expression)").with_special_validation());
    m.insert("D", CommandDef::new("D", None, ArgCount::Custom, "Variable D (expression)").with_special_validation());
    m.insert("I", CommandDef::new("I", None, ArgCount::Custom, "Variable I (expression)").with_special_validation());
    m.insert("X", CommandDef::new("X", None, ArgCount::Custom, "Variable X (expression)").with_special_validation());
    m.insert("Y", CommandDef::new("Y", None, ArgCount::Custom, "Variable Y (expression)").with_special_validation());
    m.insert("Z", CommandDef::new("Z", None, ArgCount::Custom, "Variable Z (expression)").with_special_validation());
    m.insert("T", CommandDef::new("T", None, ArgCount::Custom, "Variable T (expression)").with_special_validation());
    m.insert("J", CommandDef::new("J", None, ArgCount::Custom, "Variable J (expression)").with_special_validation());
    m.insert("K", CommandDef::new("K", None, ArgCount::Custom, "Variable K (expression)").with_special_validation());
}
