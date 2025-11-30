use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::io::Write;
use std::sync::mpsc::Sender;


pub fn handle_dc<F>(
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
        output("ERROR: DC REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse discontinuity amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: DISCONTINUITY AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/monokit_debug.txt") {
        writeln!(f, "DC sending OSC: value={}", value).ok();
    }
    metro_tx
        .send(MetroCommand::SendParam("dc".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET DISCONTINUITY AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_dm<F>(
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
        output("ERROR: DM REQUIRES A MODE VALUE (0-6)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse discontinuity mode")?
    };
    if !(0..=6).contains(&value) {
        output("ERROR: MODE MUST BE 0-6 (FOLD/TANH/SOFT/HARD/ASYM/RECT/CRUSH)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET DISCONTINUITY MODE TO {}", value));
    }
    Ok(())
}

pub fn handle_dd<F>(
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
        output("ERROR: DD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse discontinuity decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: DISCONTINUITY DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("dd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET DISCONTINUITY DECAY TO {} MS", value));
    }
    Ok(())
}

