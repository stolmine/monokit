use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_vol<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: VOL REQUIRES A VALUE (0.0-1.0)".to_string());
        return Ok(());
    }
    let value: f32 = parts[1]
        .parse()
        .context("Failed to parse volume value as float")?;
    if !(0.0..=1.0).contains(&value) {
        output("ERROR: VOLUME MUST BE BETWEEN 0.0 AND 1.0".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendVolume(value))
        .context("Failed to send volume to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET VOLUME TO {}", value));
    }
    Ok(())
}

pub fn handle_pan<F>(
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
        output("ERROR: PAN REQUIRES A VALUE (-16383 TO 16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pan position")?
    };
    if !(-16383..=16383).contains(&value) {
        output("ERROR: PAN MUST BE -16383 TO 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pn".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET PAN POSITION TO {}", value));
    }
    Ok(())
}
