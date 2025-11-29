use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_r_mode<F>(
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
        output("ERROR: R.MODE REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: MODE MUST BE 0 (BYPASS), 1 (INSERT), OR 2 (SEND)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rmode".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let mode_name = match value {
            0 => "BYPASS",
            1 => "INSERT",
            2 => "SEND",
            _ => "UNKNOWN",
        };
        output(format!("SET REVERB MODE TO {} ({})", value, mode_name));
    }
    Ok(())
}

pub fn handle_r_tail<F>(
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
        output("ERROR: R.TAIL REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb tail mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: TAIL MUST BE 0 (CUT), 1 (RING), OR 2 (FREEZE)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rtail".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let tail_name = match value {
            0 => "CUT",
            1 => "RING",
            2 => "FREEZE",
            _ => "UNKNOWN",
        };
        output(format!("SET REVERB TAIL TO {} ({})", value, tail_name));
    }
    Ok(())
}

pub fn handle_rv<F>(
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
        output("ERROR: RV REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb decay")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: REVERB DECAY MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rv".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB DECAY TO {}", value));
    }
    Ok(())
}

pub fn handle_rp<F>(
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
        output("ERROR: RP REQUIRES A VALUE (0-100)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb pre-delay")?
    };
    if !(0..=100).contains(&value) {
        output("ERROR: REVERB PRE-DELAY MUST BE BETWEEN 0 AND 100 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rp".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB PRE-DELAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_rh<F>(
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
        output("ERROR: RH REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb damping")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: REVERB DAMPING MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rh".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB DAMPING TO {}", value));
    }
    Ok(())
}

pub fn handle_rw<F>(
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
        output("ERROR: RW REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb wet mix")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: REVERB WET MIX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rw".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB WET MIX TO {}", value));
    }
    Ok(())
}

