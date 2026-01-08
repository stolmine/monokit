use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_float_param, define_int_param};

define_float_param!(handle_rgf, "rgf", 20.0, 2000.0, "RGF", "RING MOD FREQUENCY", "Failed to parse ring mod frequency", "HZ");
define_int_param!(handle_rgw, "rgw", 0, 3, "RGW", "RING MOD WAVEFORM", "Failed to parse ring mod waveform");

pub fn handle_rgm<F>(
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
        anyhow::bail!("RGM requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse ring mod mix")?
    };

    let clamped = value.clamp(0, 16383);
    fx_mix_state.ring_mix = clamped;
    metro_tx.send(MetroCommand::SendParam("rgm".to_string(), OscType::Int(clamped)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET RING MOD MIX TO {}", clamped));
    }
    Ok(())
}
