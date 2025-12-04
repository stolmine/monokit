use crate::config;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS, TIER_QUERIES};
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

pub fn handle_vca<F>(
    parts: &[&str],
    vca_mode: &mut bool,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("VCA: {}", if *vca_mode { 1 } else { 0 }));
        }
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[1]
                .parse()
                .context("Failed to parse VCA mode value")?
        };
        match value {
            0 => {
                *vca_mode = false;
                metro_tx
                    .send(MetroCommand::SendParam("vca_mode".to_string(), OscType::Int(0)))
                    .context("Failed to send VCA mode param")?;
                let _ = config::save_vca_mode(*vca_mode);
                if debug_level >= TIER_CONFIRMS || out_cfm {
                    output("VCA: DRONE".to_string());
                }
            }
            1 => {
                *vca_mode = true;
                metro_tx
                    .send(MetroCommand::SendParam("vca_mode".to_string(), OscType::Int(1)))
                    .context("Failed to send VCA mode param")?;
                let _ = config::save_vca_mode(*vca_mode);
                if debug_level >= TIER_CONFIRMS || out_cfm {
                    output("VCA: GATED".to_string());
                }
            }
            _ => {
                output("ERROR: VCA TAKES 0 (DRONE) OR 1 (GATED)".to_string());
            }
        }
    }
    Ok(())
}
