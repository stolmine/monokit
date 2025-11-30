use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_da<F>(
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
        output("ERROR: DA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
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

pub fn handle_denv_atk<F>(
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
        output("ERROR: DENV.ATK REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse DC env attack time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: DC ENV ATTACK MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("denv_atk".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DC ENV ATTACK TO {} MS", value));
    }
    Ok(())
}

pub fn handle_denv_crv<F>(
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
        output("ERROR: DENV.CRV REQUIRES A VALUE (-8.0 TO 8.0)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse DC env curve value")?
    };
    if !(-8.0..=8.0).contains(&value) {
        output("ERROR: DC ENV CURVE MUST BE BETWEEN -8.0 AND 8.0".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("denv_crv".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DC ENV CURVE TO {}", value));
    }
    Ok(())
}

pub fn handle_denv_mode<F>(
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
        output("ERROR: DENV.MODE REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse DC env mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: DC ENV MODE MUST BE 0 (AD), 1 (ASR), OR 2 (ADSR)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("denv_mode".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DC ENV MODE TO {}", value));
    }
    Ok(())
}
