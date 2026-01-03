use crate::eval::eval_expression;
use crate::types::{Counters, EqState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
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
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("EL: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ low shelf dB")?
    };
    if value < -24.0 || value > 24.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("EL: RANGE -24-24 DB"));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("el".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.low_db = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET EQ LOW SHELF TO {} DB", value));
    }
    Ok(())
}

pub fn handle_elf<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("ELF: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ low shelf frequency")?
    };
    if value < 20.0 || value > 2000.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("ELF: RANGE 20-2000 HZ"));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("elf".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.low_freq = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET EQ LOW SHELF FREQ TO {} HZ", value));
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
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("EM: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid peak dB")?
    };
    if value < -24.0 || value > 24.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("EM: RANGE -24-24 DB"));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("em".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.mid_db = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
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
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("EF: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid frequency")?
    };
    if value < 200.0 || value > 8000.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("EF: RANGE 200-8000 HZ"));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ef".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.mid_freq = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
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
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("EQ: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid Q")?
    };
    if value < 0.1 || value > 10.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("EQ: RANGE 0.1-10 "));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("eq".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.mid_q = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET EQ MID Q TO {} ", value));
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
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("EH: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ high shelf dB")?
    };
    if value < -24.0 || value > 24.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("EH: RANGE -24-24 DB"));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("eh".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.high_db = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET EQ HIGH SHELF TO {} DB", value));
    }
    Ok(())
}

pub fn handle_ehf<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    out_cfm: bool,
    eq_state: &mut EqState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output(format!("EHF: REQUIRES VALUE"));
        return Ok(());
    }
    let state_snapshot = (
        patterns.toggle_state.clone(),
        patterns.toggle_last_value.clone()
    );
    let value: f32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if consumed > 0 && parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                patterns.direct_validation.insert(key, true);
            }
        }
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ high shelf frequency")?
    };
    if value < 1000.0 || value > 20000.0 {
        patterns.toggle_state = state_snapshot.0;
        patterns.toggle_last_value = state_snapshot.1;
        if parts.len() > 1 {
            let op = parts[1].to_uppercase();
            if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                let end_idx = parts.len().min(4);
                let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                patterns.direct_validation.insert(key, false);
            }
        }
        output(format!("EHF: RANGE 1000-20000 HZ"));
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ehf".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    eq_state.high_freq = value;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET EQ HIGH SHELF FREQ TO {} HZ", value));
    }
    Ok(())
}
