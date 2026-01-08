use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_int_param, define_int_param_ms, define_mode_param_with_names};

define_mode_param_with_names!(handle_r_mode, "rmode", 0, 2, "R.MODE", "RANGE 0-2", "REVERB MODE", "Failed to parse reverb mode", &["BYPASS", "INSERT", "SEND"]);
define_mode_param_with_names!(handle_r_tail, "rtail", 0, 2, "R.TAIL", "RANGE 0-2", "REVERB TAIL", "Failed to parse reverb tail mode", &["CUT", "RING", "FREEZE"]);
define_int_param!(handle_rv, "rv", 0, 16383, "RV", "REVERB DECAY", "Failed to parse reverb decay");
define_int_param_ms!(handle_rp, "rp", 0, 100, "RP", "REVERB PRE-DELAY", "Failed to parse reverb pre-delay");
define_int_param!(handle_rh, "rh", 0, 16383, "RH", "REVERB DAMPING", "Failed to parse reverb damping");

pub fn handle_rw<F>(
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
        anyhow::bail!("RW requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse reverb wet mix")?
    };

    let clamped = value.clamp(0, 16383);
    fx_mix_state.reverb_wet = clamped;
    metro_tx.send(MetroCommand::SendParam("rw".to_string(), OscType::Int(clamped)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET REVERB WET MIX TO {}", clamped));
    }
    Ok(())
}
