use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;
use super::super::common::{define_plaits_param, define_int_param};

define_plaits_param!(handle_pl_harm, "harmonics", "PL.HARM", "HARMONICS", "Failed to parse harmonics value");
define_plaits_param!(handle_pl_timb, "timbre", "PL.TIMB", "TIMBRE", "Failed to parse timbre value");
define_plaits_param!(handle_pl_morph, "morph", "PL.MORPH", "MORPH", "Failed to parse morph value");
define_plaits_param!(handle_pl_dec, "decay", "PL.DEC", "DECAY", "Failed to parse decay value");
define_plaits_param!(handle_pl_lpg, "lpg", "PL.LPG", "LPG", "Failed to parse LPG value");
define_int_param!(handle_plv, "plv", 0, 16383, "PLV", "PLAITS VOLUME", "Failed to parse Plaits volume");
define_int_param!(handle_pav, "pav", 0, 16383, "PAV", "PLAITS AUX VOLUME", "Failed to parse Plaits aux volume");

pub fn handle_pl_freq<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PL.FREQ REQUIRES HZ VALUE (20-20000)".to_string());
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse frequency value")?
    };
    if value < 20.0 || value > 20000.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output("ERROR: PLAITS PITCH MUST BE BETWEEN 20 AND 20000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pitch".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET PLAITS PITCH TO {} HZ", value));
    }
    Ok(())
}
