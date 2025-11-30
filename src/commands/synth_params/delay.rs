use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_d_mode<F>(
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
        output("ERROR: D.MODE REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: MODE MUST BE 0 (BYPASS), 1 (INSERT), OR 2 (SEND)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dmode".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let mode_name = match value {
            0 => "BYPASS",
            1 => "INSERT",
            2 => "SEND",
            _ => "UNKNOWN",
        };
        output(format!("SET DELAY MODE TO {} ({})", value, mode_name));
    }
    Ok(())
}

pub fn handle_d_tail<F>(
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
        output("ERROR: D.TAIL REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay tail mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: TAIL MUST BE 0 (CUT), 1 (RING), OR 2 (FREEZE)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dtail".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let tail_name = match value {
            0 => "CUT",
            1 => "RING",
            2 => "FREEZE",
            _ => "UNKNOWN",
        };
        output(format!("SET DELAY TAIL TO {} ({})", value, tail_name));
    }
    Ok(())
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: DT REQUIRES A TIME VALUE (1-2000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay time")?
    };
    if !(1..=2000).contains(&value) {
        output("ERROR: DELAY TIME MUST BE BETWEEN 1 AND 2000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dt".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DELAY TIME TO {} MS", value));
    }
    Ok(())
}

pub fn handle_df<F>(
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
        output("ERROR: DF REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay feedback")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: DELAY FEEDBACK MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("df".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DELAY FEEDBACK TO {}", value));
    }
    Ok(())
}

pub fn handle_dlp<F>(
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
        output("ERROR: DLP REQUIRES A FREQUENCY VALUE (100-20000 HZ)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay damping frequency")?
    };
    if !(100.0..=20000.0).contains(&value) {
        output("ERROR: DELAY DAMPING MUST BE BETWEEN 100 AND 20000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dlp".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DELAY DAMPING TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_dw<F>(
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
        output("ERROR: DW REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay wet mix")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: DELAY WET MIX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dw".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DELAY WET MIX TO {}", value));
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: DS REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse delay sync")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: DELAY SYNC MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ds".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET DELAY SYNC TO {}", value));
    }
    Ok(())
}

