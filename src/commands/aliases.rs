use super::registry::COMMAND_REGISTRY;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static CANONICAL_TO_ALIAS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (name, def) in COMMAND_REGISTRY.iter() {
        if let Some(canonical) = def.canonical {
            m.insert(canonical, *name);
        }
    }
    m
});

pub fn resolve_alias(cmd: &str) -> String {
    CANONICAL_TO_ALIAS
        .get(cmd)
        .map(|&alias| alias.to_string())
        .unwrap_or_else(|| cmd.to_string())
}
