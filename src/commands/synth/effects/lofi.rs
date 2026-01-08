use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::define_int_param;

define_int_param!(handle_lb, "lb", 1, 16, "LB", "LO-FI BIT DEPTH", "Failed to parse bit depth");
define_int_param!(handle_ls, "ls", 100, 48000, "LS", "LO-FI SAMPLE RATE", "Failed to parse sample rate");

pub fn handle_lm<F>(
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
    fx_mix_state: &mut FxMixState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        anyhow::bail!("LM requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse lo-fi mix")?
    };

    let clamped = value.clamp(0, 16383);
    fx_mix_state.lofi_mix = clamped;
    metro_tx.send(MetroCommand::SendParam("lm".to_string(), OscType::Int(clamped)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET LO-FI MIX TO {}", clamped));
    }
    Ok(())
}
