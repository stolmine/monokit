use crate::eval::eval_expression;
use crate::types::{MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::io::Write;
use std::sync::mpsc::Sender;

pub fn handle_pf<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: PF REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: PW REQUIRES A WAVEFORM VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: MF REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: MW REQUIRES A WAVEFORM VALUE (0-3)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_dc<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: DC REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: DM REQUIRES A MODE VALUE (0-6)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_tk<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: TK REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse tracking amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: TRACKING AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("tk".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET TRACKING AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_mb<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MB REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod bus amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: MOD BUS AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mb".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD BUS AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_mp<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MP REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod -> primary value")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mp".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD -> PRIMARY FREQ TO {}", value));
    }
    Ok(())
}

pub fn handle_md<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MD REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod -> discontinuity value")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("md".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD -> DISCONTINUITY TO {}", value));
    }
    Ok(())
}

pub fn handle_mt<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MT REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod -> tracking value")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mt".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD -> TRACKING TO {}", value));
    }
    Ok(())
}

pub fn handle_ma<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MA REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod -> amplitude value")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ma".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD -> AMPLITUDE TO {}", value));
    }
    Ok(())
}

pub fn handle_fm<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FM REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse FM index")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FM INDEX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FM INDEX TO {}", value));
    }
    Ok(())
}

pub fn handle_ad<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: AD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse amp decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: AMP DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET AMP DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_pd<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: PD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: PITCH DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET PITCH DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_fd<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse FM decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: FM DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FM DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_pa<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: PA REQUIRES A MULTIPLIER VALUE (0-16)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pitch env amount")?
    };
    if !(0.0..=16.0).contains(&value) {
        output("ERROR: PITCH ENV AMOUNT MUST BE BETWEEN 0 AND 16".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET PITCH ENV AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_dd<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: DD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_mx<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MX REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mix amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: MIX AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mx".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MIX AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_mm<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MM REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod bus -> mix value")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET MOD BUS -> MIX TO {}", value));
    }
    Ok(())
}

pub fn handle_me<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: ME REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse envelope -> mix value")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("me".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET ENVELOPE -> MIX TO {}", value));
    }
    Ok(())
}

pub fn handle_fa<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse FM envelope amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FM ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
    output(format!("SET FM ENVELOPE AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_da<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: DA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_fb<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FB REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: FBA REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: FBD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_rf<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RF REQUIRES A FREQUENCY VALUE (20-5000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse resonator frequency value")?
    };
    if !(20.0..=5000.0).contains(&value) {
        output("ERROR: RESONATOR FREQUENCY MUST BE BETWEEN 20 AND 5000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rf".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RESONATOR FREQUENCY TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_rd<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RD REQUIRES A TIME VALUE (10-5000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse resonator decay time")?
    };
    if !(10..=5000).contains(&value) {
        output("ERROR: RESONATOR DECAY MUST BE BETWEEN 10 AND 5000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rd".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RESONATOR DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_rm<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RM REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse resonator mix amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: RESONATOR MIX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rm".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RESONATOR MIX TO {}", value));
    }
    Ok(())
}

pub fn handle_rk<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RK REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse resonator key tracking amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: RESONATOR KEY TRACKING MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rk".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET RESONATOR KEY TRACKING TO {}", value));
    }
    Ok(())
}

pub fn handle_d_mode<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: D.MODE REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: D.TAIL REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: DT REQUIRES A TIME VALUE (1-2000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: DF REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: DLP REQUIRES A FREQUENCY VALUE (100-20000 HZ)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: DW REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
        output("ERROR: DS REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_r_mode<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: R.MODE REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: MODE MUST BE 0 (BYPASS), 1 (INSERT), OR 2 (SEND)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rmode".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let mode_name = match value {
            0 => "BYPASS",
            1 => "INSERT",
            2 => "SEND",
            _ => "UNKNOWN",
        };
        output(format!("SET REVERB MODE TO {} ({})", value, mode_name));
    }
    Ok(())
}

pub fn handle_r_tail<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: R.TAIL REQUIRES A VALUE (0-2)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb tail mode")?
    };
    if !(0..=2).contains(&value) {
        output("ERROR: TAIL MUST BE 0 (CUT), 1 (RING), OR 2 (FREEZE)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rtail".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        let tail_name = match value {
            0 => "CUT",
            1 => "RING",
            2 => "FREEZE",
            _ => "UNKNOWN",
        };
        output(format!("SET REVERB TAIL TO {} ({})", value, tail_name));
    }
    Ok(())
}

pub fn handle_rv<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RV REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb decay")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: REVERB DECAY MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rv".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB DECAY TO {}", value));
    }
    Ok(())
}

pub fn handle_rp<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RP REQUIRES A VALUE (0-100)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb pre-delay")?
    };
    if !(0..=100).contains(&value) {
        output("ERROR: REVERB PRE-DELAY MUST BE BETWEEN 0 AND 100 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rp".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB PRE-DELAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_rh<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RH REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb damping")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: REVERB DAMPING MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rh".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB DAMPING TO {}", value));
    }
    Ok(())
}

pub fn handle_rw<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: RW REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse reverb wet mix")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: REVERB WET MIX MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("rw".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET REVERB WET MIX TO {}", value));
    }
    Ok(())
}

pub fn handle_fc<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FC REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter cutoff frequency")?
    };
    if !(20.0..=20000.0).contains(&value) {
        output("ERROR: FILTER CUTOFF MUST BE BETWEEN 20 AND 20000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fc".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER CUTOFF TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_fq<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FQ REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter resonance")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FILTER RESONANCE MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fq".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER RESONANCE TO {}", value));
    }
    Ok(())
}

pub fn handle_ft<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FT REQUIRES A VALUE (0-3)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter type")?
    };
    if !(0..=3).contains(&value) {
        output("ERROR: FILTER TYPE MUST BE 0 (LP), 1 (HP), 2 (BP), OR 3 (NOTCH)".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ft".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER TYPE TO {}", value));
    }
    Ok(())
}

pub fn handle_fe<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FE REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter envelope amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FILTER ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fe".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER ENVELOPE AMOUNT TO {}", value));
    }
    Ok(())
}

pub fn handle_fed<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FED REQUIRES A TIME VALUE (1-10000 MS)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter envelope decay time")?
    };
    if !(1..=10000).contains(&value) {
        output("ERROR: FILTER ENVELOPE DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fed".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER ENVELOPE DECAY TO {} MS", value));
    }
    Ok(())
}

pub fn handle_fk<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: FK REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse filter key tracking amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: FILTER KEY TRACKING MUST BE BETWEEN 0 AND 16383".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("fk".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET FILTER KEY TRACKING TO {}", value));
    }
    Ok(())
}

pub fn handle_mf_f<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: MF.F REQUIRES A VALUE (0 OR 1)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse modbus to filter routing")?
    };
    if !(0..=1).contains(&value) {
        output("ERROR: VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("mf_f".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET MODBUS -> FILTER TO {}", value));
    }
    Ok(())
}

pub fn handle_lb<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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

pub fn handle_el<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: EL REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ low shelf dB")?
    };
    if !(-24.0..=24.0).contains(&value) {
        output("ERROR: EQ LOW SHELF MUST BE BETWEEN -24 AND 24 DB".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("el".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ LOW SHELF TO {} DB", value));
    }
    Ok(())
}

pub fn handle_em<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: EM REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid peak dB")?
    };
    if !(-24.0..=24.0).contains(&value) {
        output("ERROR: EQ MID PEAK MUST BE BETWEEN -24 AND 24 DB".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("em".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ MID PEAK TO {} DB", value));
    }
    Ok(())
}

pub fn handle_ef<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: EF REQUIRES A FREQUENCY VALUE (200-8000)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid frequency")?
    };
    if !(200.0..=8000.0).contains(&value) {
        output("ERROR: EQ MID FREQUENCY MUST BE BETWEEN 200 AND 8000 HZ".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("ef".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ MID FREQUENCY TO {} HZ", value));
    }
    Ok(())
}

pub fn handle_eq_param<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: EQ REQUIRES A VALUE (0.1-10)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ mid Q")?
    };
    if !(0.1..=10.0).contains(&value) {
        output("ERROR: EQ MID Q MUST BE BETWEEN 0.1 AND 10".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("eq".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ MID Q TO {}", value));
    }
    Ok(())
}

pub fn handle_eh<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
        output("ERROR: EH REQUIRES A VALUE (-24 TO 24)".to_string());
        return Ok(());
    }
    let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse EQ high shelf dB")?
    };
    if !(-24.0..=24.0).contains(&value) {
        output("ERROR: EQ HIGH SHELF MUST BE BETWEEN -24 AND 24 DB".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("eh".to_string(), OscType::Float(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET EQ HIGH SHELF TO {} DB", value));
    }
    Ok(())
}

pub fn handle_pan<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
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
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
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
