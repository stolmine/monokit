use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_ps_mode<F>(
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
        output("PS.MODE: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift mode")?
    };
    if value != 0 && value != 1 {
        output("PS.MODE: RANGE 0-1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ps_mode".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        let mode_name = match value {
            0 => "GRANULAR",
            1 => "FREQ_SHIFT",
            _ => "UNKNOWN",
        };
        output(format!("PS.MODE: {}", mode_name));
    }
    Ok(())
}

pub fn handle_ps_semi<F>(
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
        output("PS.SEMI: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift semitones")?
    };
    let clipped = value.clamp(-24, 24);
    metro_tx
        .send(MetroCommand::SendParam("ps_semi".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("PS.SEMI: {} SEMITONES", clipped));
    }
    Ok(())
}

pub fn handle_ps_grain<F>(
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
        output("PS.GRAIN: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift grain size")?
    };
    let clipped = value.clamp(5, 100);
    metro_tx
        .send(MetroCommand::SendParam("ps_grain".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("PS.GRAIN: {} MS", clipped));
    }
    Ok(())
}

pub fn handle_ps_mix<F>(
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
        output("PS.MIX: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift mix")?
    };
    let clipped = value.clamp(0, 16383);
    metro_tx
        .send(MetroCommand::SendParam("ps_mix".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("PS.MIX: {}", clipped));
    }
    Ok(())
}

pub fn handle_ps_targ<F>(
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
        output("PS.TARG: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift target")?
    };
    if value != 0 && value != 1 {
        output("PS.TARG: RANGE 0-1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ps_targ".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        let targ_name = match value {
            0 => "MAIN",
            1 => "REPEAT_ONLY",
            _ => "UNKNOWN",
        };
        output(format!("PS.TARG: {}", targ_name));
    }
    Ok(())
}
