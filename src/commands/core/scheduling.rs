use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES, TIER_CONFIRMS, TIER_VERBOSE};
use anyhow::{Context, Result};
use std::sync::mpsc::Sender;

pub fn handle_del<F>(
    _parts: &[&str],
    input: &str,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    scale: &ScaleState,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let colon_pos = match input.find(':') {
        Some(pos) => pos,
        None => {
            output("ERROR: DEL REQUIRES FORMAT: DEL <ms>: <cmd>".to_string());
            return Ok(());
        }
    };

    let before_colon = &input[..colon_pos];
    let after_colon = input[colon_pos + 1..].trim();

    if after_colon.is_empty() {
        output("ERROR: DEL REQUIRES A COMMAND AFTER COLON".to_string());
        return Ok(());
    }

    let prefix_parts: Vec<&str> = before_colon.split_whitespace().collect();
    if prefix_parts.len() < 2 {
        output("ERROR: DEL REQUIRES A DELAY TIME".to_string());
        return Ok(());
    }

    let delay_ms: u64 = if let Some((expr_val, _)) = eval_expression(&prefix_parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 0 {
            output("ERROR: DELAY TIME CANNOT BE NEGATIVE".to_string());
            return Ok(());
        }
        if expr_val > 16000 {
            output("ERROR: DELAY TIME MAX 16000MS".to_string());
            return Ok(());
        }
        expr_val as u64
    } else {
        match prefix_parts[1].parse::<u64>() {
            Ok(v) => {
                if v > 16000 {
                    output("ERROR: DELAY TIME MAX 16000MS".to_string());
                    return Ok(());
                }
                v
            }
            Err(_) => {
                output("ERROR: FAILED TO PARSE DELAY TIME".to_string());
                return Ok(());
            }
        }
    };

    metro_tx
        .send(MetroCommand::ScheduleDelayed(after_colon.to_string(), delay_ms, script_index))
        .context("Failed to send delayed command to metro thread")?;

    if debug_level >= TIER_CONFIRMS {
        output(format!("DELAYED {}MS: {}", delay_ms, after_colon));
    }

    Ok(())
}

pub fn handle_del_clr<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx
        .send(MetroCommand::ClearDelayed)
        .context("Failed to send clear delayed to metro thread")?;

    if debug_level >= TIER_ESSENTIAL || out_ess {
        output("CLEARED ALL DELAYED COMMANDS".to_string());
    }

    Ok(())
}

pub fn handle_del_x<F>(
    _parts: &[&str],
    input: &str,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    scale: &ScaleState,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let colon_pos = match input.find(':') {
        Some(pos) => pos,
        None => {
            output("ERROR: DEL.X FORMAT: DEL.X N MS: CMD".to_string());
            return Ok(());
        }
    };

    let before_colon = &input[..colon_pos];
    let after_colon = input[colon_pos + 1..].trim();

    if after_colon.is_empty() {
        output("ERROR: DEL.X REQUIRES A COMMAND AFTER COLON".to_string());
        return Ok(());
    }

    let prefix_parts: Vec<&str> = before_colon.split_whitespace().collect();
    if prefix_parts.len() < 3 {
        output("ERROR: DEL.X REQUIRES COUNT AND INTERVAL".to_string());
        return Ok(());
    }

    let count: i16 = if let Some((expr_val, _)) = eval_expression(&prefix_parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 1 {
            output("ERROR: COUNT MUST BE AT LEAST 1".to_string());
            return Ok(());
        }
        expr_val
    } else {
        match prefix_parts[1].parse() {
            Ok(v) => {
                if v < 1 {
                    output("ERROR: COUNT MUST BE AT LEAST 1".to_string());
                    return Ok(());
                }
                v
            }
            Err(_) => {
                output("ERROR: FAILED TO PARSE COUNT".to_string());
                return Ok(());
            }
        }
    };

    let interval_ms: u64 = if let Some((expr_val, _)) = eval_expression(&prefix_parts, 2, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 0 {
            output("ERROR: INTERVAL CANNOT BE NEGATIVE".to_string());
            return Ok(());
        }
        if expr_val > 16000 {
            output("ERROR: INTERVAL MAX 16000MS".to_string());
            return Ok(());
        }
        expr_val as u64
    } else {
        match prefix_parts[2].parse::<u64>() {
            Ok(v) => {
                if v > 16000 {
                    output("ERROR: INTERVAL MAX 16000MS".to_string());
                    return Ok(());
                }
                v
            }
            Err(_) => {
                output("ERROR: FAILED TO PARSE INTERVAL".to_string());
                return Ok(());
            }
        }
    };

    metro_tx
        .send(MetroCommand::ScheduleRepeated(after_colon.to_string(), count, interval_ms, script_index))
        .context("Failed to send repeated command to metro thread")?;

    if debug_level >= TIER_CONFIRMS {
        output(format!("REPEAT {}x @{}MS: {}", count, interval_ms, after_colon));
    }

    Ok(())
}

pub fn handle_del_r<F>(
    _parts: &[&str],
    input: &str,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    scale: &ScaleState,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let colon_pos = match input.find(':') {
        Some(pos) => pos,
        None => {
            output("ERROR: DEL.R FORMAT: DEL.R N MS: CMD".to_string());
            return Ok(());
        }
    };

    let before_colon = &input[..colon_pos];
    let after_colon = input[colon_pos + 1..].trim();

    if after_colon.is_empty() {
        output("ERROR: DEL.R REQUIRES A COMMAND AFTER COLON".to_string());
        return Ok(());
    }

    let prefix_parts: Vec<&str> = before_colon.split_whitespace().collect();
    if prefix_parts.len() < 3 {
        output("ERROR: DEL.R REQUIRES COUNT AND INTERVAL".to_string());
        return Ok(());
    }

    let count: i16 = if let Some((expr_val, _)) = eval_expression(&prefix_parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 1 {
            output("ERROR: COUNT MUST BE AT LEAST 1".to_string());
            return Ok(());
        }
        expr_val
    } else {
        match prefix_parts[1].parse() {
            Ok(v) => {
                if v < 1 {
                    output("ERROR: COUNT MUST BE AT LEAST 1".to_string());
                    return Ok(());
                }
                v
            }
            Err(_) => {
                output("ERROR: FAILED TO PARSE COUNT".to_string());
                return Ok(());
            }
        }
    };

    let interval_ms: u64 = if let Some((expr_val, _)) = eval_expression(&prefix_parts, 2, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 0 {
            output("ERROR: INTERVAL CANNOT BE NEGATIVE".to_string());
            return Ok(());
        }
        if expr_val > 16000 {
            output("ERROR: INTERVAL MAX 16000MS".to_string());
            return Ok(());
        }
        expr_val as u64
    } else {
        match prefix_parts[2].parse::<u64>() {
            Ok(v) => {
                if v > 16000 {
                    output("ERROR: INTERVAL MAX 16000MS".to_string());
                    return Ok(());
                }
                v
            }
            Err(_) => {
                output("ERROR: FAILED TO PARSE INTERVAL".to_string());
                return Ok(());
            }
        }
    };

    metro_tx
        .send(MetroCommand::ScheduleDelayed(after_colon.to_string(), 0, script_index))
        .context("Failed to send immediate delayed command to metro thread")?;

    if count > 1 {
        metro_tx
            .send(MetroCommand::ScheduleRepeated(after_colon.to_string(), count - 1, interval_ms, script_index))
            .context("Failed to send repeated command to metro thread")?;

        if debug_level >= TIER_CONFIRMS {
            output(format!("IMMEDIATE +{}x @{}MS: {}", count - 1, interval_ms, after_colon));
        }
    } else {
        if debug_level >= TIER_CONFIRMS {
            output(format!("IMMEDIATE: {}", after_colon));
        }
    }

    Ok(())
}
