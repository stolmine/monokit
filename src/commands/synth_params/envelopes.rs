use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_ad<F>(
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
        output("ERROR: AD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse amp decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: AMP DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET AMP DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_pd<F>(
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
        output("ERROR: PD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: PITCH DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET PITCH DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_fd<F>(
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
        output("ERROR: FD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse FM decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: FM DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FM DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_pa<F>(
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
        output("ERROR: PA REQUIRES A MULTIPLIER VALUE (0-16)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch env amount")?
    };
    if !(0.0..=16.0).contains(&value) {
        output("ERROR: PITCH ENV AMOUNT MUST BE BETWEEN 0 AND 16".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET PITCH ENV AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_fa<F>(
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
        output("ERROR: FA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse FM envelope amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FM ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FM ENVELOPE AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_da<F>(
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
        output("ERROR: DA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse DC envelope amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: DC ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("da".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET DC ENVELOPE AMOUNT TO {}", value));
    }
    Ok(())
}

