use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PS.MODE REQUIRES A VALUE (0-1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift mode")?
    };
    let clipped = value.clamp(0, 1);
    metro_tx
        .send(MetroCommand::SendParam("ps_mode".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let mode_name = match clipped {
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PS.SEMI REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    if debug_level >= 2 {
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PS.GRAIN REQUIRES A VALUE (5-100)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    if debug_level >= 2 {
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PS.MIX REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    if debug_level >= 2 {
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PS.TARG REQUIRES A VALUE (0-1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch shift target")?
    };
    let clipped = value.clamp(0, 1);
    metro_tx
        .send(MetroCommand::SendParam("ps_targ".to_string(), OscType::Int(clipped)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let targ_name = match clipped {
            0 => "MAIN",
            1 => "REPEAT_ONLY",
            _ => "UNKNOWN",
        };
        output(format!("PS.TARG: {}", targ_name));
    }
    Ok(())
}
