use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_br_len<F>(
    parts: &[&str],
    metro_interval: u64,
    br_len: &mut usize,
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
        output("BR.LEN: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse beat repeat length division")?
    };
    let clipped = value.clamp(0, 7) as usize;
    *br_len = clipped;
    metro_tx
        .send(MetroCommand::SendParam("br_len".to_string(), OscType::Int(clipped as i32)))
        .context("Failed to send param to metro thread")?;

    let loop_length_ms = match clipped {
        0 => metro_interval / 16,
        1 => metro_interval / 8,
        2 => metro_interval / 4,
        3 => metro_interval / 2,
        4 => metro_interval,
        5 => metro_interval * 2,
        6 => metro_interval * 4,
        7 => metro_interval * 8,
        _ => metro_interval / 4,
    };
    metro_tx
        .send(MetroCommand::SendParam("br_len_ms".to_string(), OscType::Float(loop_length_ms as f32)))
        .context("Failed to send br_len_ms to metro thread")?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET BEAT REPEAT LENGTH DIVISION TO {}", clipped));
    }
    Ok(())
}

pub fn handle_br_rev<F>(
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
        output("BR.REV: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse beat repeat reverse")?
    };
    if value != 0 && value != 1 {
        output("BR.REV: RANGE 0-1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("br_rev".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET BEAT REPEAT REVERSE TO {}", value));
    }
    Ok(())
}

pub fn handle_br_win<F>(
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
        output("BR.WIN: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse beat repeat window size")?
    };
    let clipped = value.clamp(1, 50);
    metro_tx
        .send(MetroCommand::SendParam("br_win".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET BEAT REPEAT CROSSFADE WINDOW TO {} MS", clipped));
    }
    Ok(())
}

pub fn handle_br_mix<F>(
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
        output("BR.MIX: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse beat repeat mix")?
    };
    let clipped = value.clamp(0, 16383);
    metro_tx
        .send(MetroCommand::SendParam("br_mix".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET BEAT REPEAT MIX TO {}", clipped));
    }
    Ok(())
}

