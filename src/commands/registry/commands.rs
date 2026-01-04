//! Command registry definitions

use super::{ArgCount, CommandDef};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// The central command registry - single source of truth
pub static COMMAND_REGISTRY: Lazy<HashMap<&'static str, CommandDef>> = Lazy::new(|| {
    let mut m = HashMap::new();

    super::variables::register_variables(&mut m);
    super::counters::register_counters(&mut m);
    super::patterns::register_patterns(&mut m);
    super::synth::register_synth(&mut m);
    super::effects::register_effects(&mut m);
    super::system::register_system(&mut m);
    super::control::register_control(&mut m);
    super::ui::register_ui(&mut m);

    // Register canonical forms as aliases to themselves
    // This allows commands to be looked up by either short (PF) or long (POSC.FREQ) form
    let entries: Vec<(&'static str, CommandDef)> = m.iter().map(|(k, v)| (*k, v.clone())).collect();
    for (_, def) in entries {
        if let Some(canonical) = def.canonical {
            // Create a new CommandDef for the canonical form with canonical=None
            // to avoid creating a self-referencing entry in CANONICAL_TO_ALIAS
            let canonical_def = CommandDef {
                name: canonical,
                canonical: None,
                args: def.args.clone(),
                help: def.help,
                special_validation: def.special_validation,
            };
            // Only insert if not already present (avoid overwriting explicit entries)
            m.entry(canonical).or_insert(canonical_def);
        }
    }

    m
});
