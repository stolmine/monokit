use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_el<F>(
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
        output("ERROR: EL REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ low shelf dB")?
    };
    if !(-24.0..=24.0).contains(&value) {
        output("ERROR: EQ LOW SHELF MUST BE BETWEEN -24 AND 24 DB".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("el".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ LOW SHELF TO {} DB", value));
    }
    Ok(())
}

pub fn handle_em<F>(
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
        output("ERROR: EM REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid peak dB")?
    };
    if !(-24.0..=24.0).contains(&value) {
        output("ERROR: EQ MID PEAK MUST BE BETWEEN -24 AND 24 DB".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("em".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ MID PEAK TO {} DB", value));
    }
    Ok(())
}

pub fn handle_ef<F>(
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
        output("ERROR: EF REQUIRES A FREQUENCY VALUE (200-8000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid frequency")?
    };
    if !(200.0..=8000.0).contains(&value) {
        output("ERROR: EQ MID FREQUENCY MUST BE BETWEEN 200 AND 8000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ef".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ MID FREQUENCY TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_eq_param<F>(
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
        output("ERROR: EQ REQUIRES A VALUE (0.1-10)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid Q")?
    };
    if !(0.1..=10.0).contains(&value) {
        output("ERROR: EQ MID Q MUST BE BETWEEN 0.1 AND 10".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("eq".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ MID Q TO {}", value));
    }
    Ok(())
}

pub fn handle_eh<F>(
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
        output("ERROR: EH REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ high shelf dB")?
    };
    if !(-24.0..=24.0).contains(&value) {
        output("ERROR: EQ HIGH SHELF MUST BE BETWEEN -24 AND 24 DB".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("eh".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ HIGH SHELF TO {} DB", value));
    }
    Ok(())
}

