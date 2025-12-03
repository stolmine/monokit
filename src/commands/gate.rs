use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use std::sync::mpsc::Sender;

pub fn handle_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse gate time value")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetGate(time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET GLOBAL GATE TO {} MS", value_ms));
    }
    Ok(())
}

pub fn handle_aenv_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("AENV.GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse amp env gate time")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("AMP ENV GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetEnvGate("aenv".to_string(), time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET AMP ENV GATE TO {} MS", value_ms));
    }
    Ok(())
}

pub fn handle_penv_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("PENV.GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch env gate time")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("PITCH ENV GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetEnvGate("penv".to_string(), time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET PITCH ENV GATE TO {} MS", value_ms));
    }
    Ok(())
}

pub fn handle_fmev_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("FMEV.GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse FM env gate time")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("FM ENV GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetEnvGate("fmev".to_string(), time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FM ENV GATE TO {} MS", value_ms));
    }
    Ok(())
}

pub fn handle_denv_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("DENV.GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse DC env gate time")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("DC ENV GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetEnvGate("denv".to_string(), time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DC ENV GATE TO {} MS", value_ms));
    }
    Ok(())
}

pub fn handle_fbev_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("FBEV.GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse feedback env gate time")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("FEEDBACK ENV GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetEnvGate("fbev".to_string(), time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FEEDBACK ENV GATE TO {} MS", value_ms));
    }
    Ok(())
}

pub fn handle_flev_gate<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("FLEV.GATE NEEDS TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter env gate time")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("FILTER ENV GATE TIME 0-10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetEnvGate("flev".to_string(), time_sec))
        .context("Failed to send gate to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER ENV GATE TO {} MS", value_ms));
    }
    Ok(())
}
