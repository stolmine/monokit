use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;


pub fn handle_lb<F>(
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
        output("ERROR: LB REQUIRES A VALUE (1-16)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse bit depth")?
    };
    if !(1..=16).contains(&value) {
        output("ERROR: BIT DEPTH MUST BE BETWEEN 1 AND 16".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("lb".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET LO-FI BIT DEPTH TO {}", value));
    }
    Ok(())
}

pub fn handle_ls<F>(
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
        output("ERROR: LS REQUIRES A VALUE (100-48000)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse sample rate")?
    };
    if !(100..=48000).contains(&value) {
        output("ERROR: SAMPLE RATE MUST BE BETWEEN 100 AND 48000".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ls".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET LO-FI SAMPLE RATE TO {}", value));
    }
    Ok(())
}

pub fn handle_lm<F>(
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
        output("ERROR: LM REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse lo-fi mix")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: LO-FI MIX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("lm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET LO-FI MIX TO {}", value));
    }
    Ok(())
}

pub fn handle_rgf<F>(
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
        output("ERROR: RGF REQUIRES A FREQUENCY VALUE (20-2000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse ring mod frequency")?
    };
    if !(20.0..=2000.0).contains(&value) {
        output("ERROR: RING MOD FREQUENCY MUST BE BETWEEN 20 AND 2000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rgf".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RING MOD FREQUENCY TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_rgw<F>(
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
        output("ERROR: RGW REQUIRES A VALUE (0-3)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse ring mod waveform")?
    };
    if !(0..=3).contains(&value) {
        output("ERROR: RING MOD WAVEFORM MUST BE BETWEEN 0 AND 3".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rgw".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RING MOD WAVEFORM TO {}", value));
    }
    Ok(())
}

pub fn handle_rgm<F>(
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
        output("ERROR: RGM REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse ring mod mix")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: RING MOD MIX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rgm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RING MOD MIX TO {}", value));
    }
    Ok(())
}

pub fn handle_ct<F>(
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
        output("ERROR: CT REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse compressor threshold")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: COMPRESSOR THRESHOLD MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ct".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET COMPRESSOR THRESHOLD TO {}", value));
    }
    Ok(())
}

pub fn handle_cr<F>(
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
        output("ERROR: CR REQUIRES A VALUE (1-20)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse compressor ratio")?
    };
    if !(1.0..=20.0).contains(&value) {
        output("ERROR: COMPRESSOR RATIO MUST BE BETWEEN 1 AND 20".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("cr".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET COMPRESSOR RATIO TO {}", value));
    }
    Ok(())
}

pub fn handle_ca<F>(
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
        output("ERROR: CA REQUIRES A VALUE (1-500)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse compressor attack")?
    };
    if !(1..=500).contains(&value) {
        output("ERROR: COMPRESSOR ATTACK MUST BE BETWEEN 1 AND 500 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ca".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET COMPRESSOR ATTACK TO {} MS", value));
    }
    Ok(())
}

pub fn handle_cl<F>(
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
        output("ERROR: CL REQUIRES A VALUE (10-2000)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse compressor release")?
    };
    if !(10..=2000).contains(&value) {
        output("ERROR: COMPRESSOR RELEASE MUST BE BETWEEN 10 AND 2000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("cl".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET COMPRESSOR RELEASE TO {} MS", value));
    }
    Ok(())
}

pub fn handle_cm<F>(
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
        output("ERROR: CM REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse compressor makeup gain")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: COMPRESSOR MAKEUP GAIN MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("cm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET COMPRESSOR MAKEUP GAIN TO {}", value));
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PAN REQUIRES A VALUE (-16383 TO 16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pan position")?
    };
    if !(-16383..=16383).contains(&value) {
        output("ERROR: PAN POSITION MUST BE BETWEEN -16383 AND 16383".to_string());
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

