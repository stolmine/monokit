use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_float_param, define_int_param, define_int_param_ms};

define_float_param!(handle_rf, "rf", 20.0, 5000.0, "RF", "RESONATOR FREQUENCY", "Failed to parse resonator frequency value", "HZ");
define_int_param_ms!(handle_rd, "rd", 10, 5000, "RD", "RESONATOR DECAY", "Failed to parse resonator decay time");
define_int_param!(handle_rk, "rk", 0, 16383, "RK", "RESONATOR KEY TRACKING", "Failed to parse resonator key tracking amount");

pub fn handle_rm<F>(
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
        anyhow::bail!("RM requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse resonator mix amount")?
    };

    let clamped = value.clamp(0, 16383);
    fx_mix_state.reso_mix = clamped;
    metro_tx.send(MetroCommand::SendParam("rm".to_string(), OscType::Int(clamped)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET RESONATOR MIX TO {}", clamped));
    }
    Ok(())
}
