//! Counter commands (N1-N4, N*.MAX/MIN/RST)

use super::{ArgCount, CommandDef};

pub fn register_counters(m: &mut std::collections::HashMap<&'static str, CommandDef>) {
    m.insert("N1", CommandDef::new("N1", None, ArgCount::None, "Counter 1 (query)"));
    m.insert("N2", CommandDef::new("N2", None, ArgCount::None, "Counter 2 (query)"));
    m.insert("N3", CommandDef::new("N3", None, ArgCount::None, "Counter 3 (query)"));
    m.insert("N4", CommandDef::new("N4", None, ArgCount::None, "Counter 4 (query)"));

    m.insert("N1.RST", CommandDef::new("N1.RST", None, ArgCount::None, "Reset counter 1"));
    m.insert("N2.RST", CommandDef::new("N2.RST", None, ArgCount::None, "Reset counter 2"));
    m.insert("N3.RST", CommandDef::new("N3.RST", None, ArgCount::None, "Reset counter 3"));
    m.insert("N4.RST", CommandDef::new("N4.RST", None, ArgCount::None, "Reset counter 4"));

    m.insert("N1.MAX", CommandDef::new("N1.MAX", None, ArgCount::Range(0, 1), "Counter 1 max"));
    m.insert("N2.MAX", CommandDef::new("N2.MAX", None, ArgCount::Range(0, 1), "Counter 2 max"));
    m.insert("N3.MAX", CommandDef::new("N3.MAX", None, ArgCount::Range(0, 1), "Counter 3 max"));
    m.insert("N4.MAX", CommandDef::new("N4.MAX", None, ArgCount::Range(0, 1), "Counter 4 max"));

    m.insert("N1.MIN", CommandDef::new("N1.MIN", None, ArgCount::Range(0, 1), "Counter 1 min"));
    m.insert("N2.MIN", CommandDef::new("N2.MIN", None, ArgCount::Range(0, 1), "Counter 2 min"));
    m.insert("N3.MIN", CommandDef::new("N3.MIN", None, ArgCount::Range(0, 1), "Counter 3 min"));
    m.insert("N4.MIN", CommandDef::new("N4.MIN", None, ArgCount::Range(0, 1), "Counter 4 min"));
}
