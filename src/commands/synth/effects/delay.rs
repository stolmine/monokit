use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_float_param, define_fx_mix_param, define_int_param, define_mode_param_with_names};

define_mode_param_with_names!(handle_d_mode, "dmode", 0, 2, "D.MODE", "RANGE 0-2", "DELAY MODE", "Failed to parse delay mode", &["BYPASS", "INSERT", "SEND"]);
define_mode_param_with_names!(handle_d_tail, "dtail", 0, 2, "D.TAIL", "RANGE 0-2", "DELAY TAIL", "Failed to parse delay tail mode", &["CUT", "RING", "FREEZE"]);
define_int_param!(handle_df, "df", 0, 16383, "DF", "DELAY FEEDBACK", "Failed to parse delay feedback");
define_float_param!(handle_dlp, "dlp", 100.0, 20000.0, "DLP", "DELAY DAMPING", "Failed to parse delay damping frequency", "HZ");
define_fx_mix_param!(handle_dw, "dw", delay_wet, "SET DELAY WET MIX TO {}");

fn calculate_delay_ms(raw: i32, sync: bool, metro_interval: u64) -> i32 {
    let t = raw as f64 / 16383.0;
    let curved_t = t.powf(0.7);

    if sync {
        let min_ms = (metro_interval / 16) as f64;
        let max_ms = (metro_interval * 4) as f64;
        let delay_ms = min_ms + curved_t * (max_ms - min_ms);
        delay_ms.min(4000.0).max(1.0) as i32
    } else {
        let delay_ms = 1.0 + curved_t * 1999.0;
        delay_ms as i32
    }
}

fn format_beat_division(delay_ms: i32, metro_interval: u64) -> String {
    // metro_interval is 1/4 beat (one tick), so 1 beat = metro_interval * 4
    let beat_ms = (metro_interval * 4) as f64;
    let beats = delay_ms as f64 / beat_ms;

    // Find closest musical division
    if beats >= 3.5 { "4".to_string() }
    else if beats >= 2.5 { "3".to_string() }
    else if beats >= 1.5 { "2".to_string() }
    else if beats >= 0.75 { "1".to_string() }
    else if beats >= 0.375 { "1/2".to_string() }
    else if beats >= 0.1875 { "1/4".to_string() }
    else if beats >= 0.09375 { "1/8".to_string() }
    else { "1/16".to_string() }
}

pub fn handle_dt<F>(
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
    metro_interval: u64,
    fx_mix_state: &mut FxMixState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        anyhow::bail!("DT requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse delay time")?
    };

    let clamped = value.clamp(0, 16383);
    fx_mix_state.delay_time_raw = clamped;

    let delay_ms = calculate_delay_ms(clamped, fx_mix_state.delay_sync, metro_interval);
    metro_tx.send(MetroCommand::SendParam("dt".to_string(), OscType::Int(delay_ms)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        if fx_mix_state.delay_sync {
            let div = format_beat_division(delay_ms, metro_interval);
            output(format!("SET DELAY TIME TO {} (~{} BEAT, {}MS)", clamped, div, delay_ms));
        } else {
            output(format!("SET DELAY TIME TO {} ({}MS)", clamped, delay_ms));
        }
    }
    Ok(())
}

pub fn handle_ds<F>(
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
    metro_interval: u64,
    fx_mix_state: &mut FxMixState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        anyhow::bail!("DS requires a value");
    }

    let value: i32 = if let Some((v, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        v as i32
    } else {
        parts[1].parse().context("Failed to parse delay sync")?
    };

    let clamped = value.clamp(0, 1);
    fx_mix_state.delay_sync = clamped != 0;

    let delay_ms = calculate_delay_ms(fx_mix_state.delay_time_raw, fx_mix_state.delay_sync, metro_interval);
    metro_tx.send(MetroCommand::SendParam("dt".to_string(), OscType::Int(delay_ms)))?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        let mode_name = if fx_mix_state.delay_sync { "SYNC" } else { "FREE" };
        output(format!("SET DELAY SYNC TO {} ({})", clamped, mode_name));
    }
    Ok(())
}

