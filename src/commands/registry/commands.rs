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

    m
});
