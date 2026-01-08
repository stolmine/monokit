use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_bool_param, define_float_param, define_int_param, define_int_param_ms, define_mode_param_with_names};

define_mode_param_with_names!(handle_d_mode, "dmode", 0, 2, "D.MODE", "RANGE 0-2", "DELAY MODE", "Failed to parse delay mode", &["BYPASS", "INSERT", "SEND"]);
define_mode_param_with_names!(handle_d_tail, "dtail", 0, 2, "D.TAIL", "RANGE 0-2", "DELAY TAIL", "Failed to parse delay tail mode", &["CUT", "RING", "FREEZE"]);
define_int_param_ms!(handle_dt, "dt", 1, 2000, "DT", "DELAY TIME", "Failed to parse delay time");
define_int_param!(handle_df, "df", 0, 16383, "DF", "DELAY FEEDBACK", "Failed to parse delay feedback");
define_float_param!(handle_dlp, "dlp", 100.0, 20000.0, "DLP", "DELAY DAMPING", "Failed to parse delay damping frequency", "HZ");
define_bool_param!(handle_ds, "ds", "DS", "DELAY SYNC", "Failed to parse delay sync");

pub fn handle_dw<F>(
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
        anyhow::bail!("DW requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse delay wet mix")?
    };

    let clamped = value.clamp(0, 16383);
    fx_mix_state.delay_wet = clamped;
    metro_tx.send(MetroCommand::SendParam("dw".to_string(), OscType::Int(clamped)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET DELAY WET MIX TO {}", clamped));
    }
    Ok(())
}
