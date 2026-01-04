use super::registry::COMMAND_REGISTRY;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Maps alias -> canonical (e.g., "SR" -> "S.RATE")
static ALIAS_TO_CANONICAL: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (name, def) in COMMAND_REGISTRY.iter() {
        if let Some(canonical) = def.canonical {
            // name is the alias (e.g., "SR"), canonical is the long form (e.g., "S.RATE")
            m.insert(*name, canonical);
        }
    }
    m
});

// Maps canonical -> alias (e.g., "S.RATE" -> "SR")
static CANONICAL_TO_ALIAS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (name, def) in COMMAND_REGISTRY.iter() {
        if let Some(canonical) = def.canonical {
            // canonical is the long form, name is the alias/short form
            m.insert(canonical, *name);
        }
    }
    m
});

/// Resolves an alias to its canonical form (e.g., "SR" -> "S.RATE")
/// Used for command dispatch
/// Returns the input unchanged if it's not an alias
pub fn resolve_alias(cmd: &str) -> String {
    ALIAS_TO_CANONICAL
        .get(cmd)
        .map(|&canonical| canonical.to_string())
        .unwrap_or_else(|| cmd.to_string())
}

/// Resolves a canonical form to its short alias (e.g., "REV.WET" -> "RW")
/// Used for OSC param names that expect short forms
/// Returns the input unchanged if it's not a canonical form with an alias
pub fn resolve_to_short(cmd: &str) -> String {
    CANONICAL_TO_ALIAS
        .get(cmd)
        .map(|&alias| alias.to_string())
        .unwrap_or_else(|| cmd.to_string())
}
