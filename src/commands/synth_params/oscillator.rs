use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_pf<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PF REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse frequency value")?
    };
    if !(20.0..=20000.0).contains(&value) {
        output("ERROR: FREQUENCY MUST BE BETWEEN 20 AND 20000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pf".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET PRIMARY FREQUENCY TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_pw<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PW REQUIRES A WAVEFORM VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse waveform value")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: WAVEFORM MUST BE 0 (SIN), 1 (TRI), OR 2 (SAW)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pw".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET PRIMARY WAVEFORM TO {}", value));
    }
    Ok(())
}

pub fn handle_mf<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: MF REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse frequency value")?
    };
    if !(20.0..=20000.0).contains(&value) {
        output("ERROR: FREQUENCY MUST BE BETWEEN 20 AND 20000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mf".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET MOD FREQUENCY TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_mw<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: MW REQUIRES A WAVEFORM VALUE (0-3)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse waveform value")?
    };
    if !(0..=3).contains(&value) {
        output("ERROR: WAVEFORM MUST BE 0 (SIN), 1 (TRI), 2 (SAW), OR 3 (FEEDBACK)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mw".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD WAVEFORM TO {}", value));
    }
    Ok(())
}

pub fn handle_fb<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: FB REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse feedback amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FEEDBACK AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fb".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FEEDBACK AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_fba<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: FBA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse feedback envelope amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FEEDBACK ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fba".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FEEDBACK ENVELOPE AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_fbd<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: FBD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse feedback decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: FEEDBACK DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fbd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FEEDBACK DECAY TO {} MS", value));
    }
    Ok(())
}

