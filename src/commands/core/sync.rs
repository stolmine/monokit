use crate::types::{Counters, PatternStorage, TIER_CONFIRMS};

pub fn handle_sync<F>(
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    ev_counters: &mut [[u32; 8]; 10],
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    patterns.toggle_state.clear();

    for pattern in &mut patterns.patterns {
        pattern.index = 0;
    }

    for counter_idx in 0..4 {
        counters.values[counter_idx] = counters.min[counter_idx];
    }

    for script in ev_counters.iter_mut() {
        for line in script.iter_mut() {
            *line = 0;
        }
    }

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("SYNC: ALL STATE RESET".to_string());
    }
}

pub fn handle_sync_seq<F>(
    patterns: &mut PatternStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    patterns.toggle_state.retain(|key, _| {
        !key.starts_with("seq_") && !key.starts_with("seq_alt_") && !key.starts_with("seq_rnd_")
    });

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("SYNC.SEQ: SEQ STATE RESET".to_string());
    }
}

pub fn handle_sync_tog<F>(
    patterns: &mut PatternStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    patterns.toggle_state.retain(|key, _| {
        !key.contains("_TOG_") && !key.contains("_EITH_")
    });

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("SYNC.TOG: TOG/EITH STATE RESET".to_string());
    }
}

pub fn handle_sync_pat<F>(
    patterns: &mut PatternStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    for pattern in &mut patterns.patterns {
        pattern.index = 0;
    }

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("SYNC.PAT: PATTERN INDICES RESET".to_string());
    }
}
