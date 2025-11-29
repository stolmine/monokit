use crate::eval::eval_expression;
use crate::types::{MetroCommand, OSC_ADDR, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rand::Rng;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::sync::mpsc::Sender;

pub fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &mut ScriptStorage,
    script_index: usize,
    input: &str,
    mut output: F,
) -> Result<Vec<usize>>
where
    F: FnMut(String),
{
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "A" => {
            if parts.len() == 1 {
                output(format!("A = {}", variables.a));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for A")?;
                variables.a = value;
                output(format!("Set A to {}", value));
            }
        }
        "B" => {
            if parts.len() == 1 {
                output(format!("B = {}", variables.b));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for B")?;
                variables.b = value;
                output(format!("Set B to {}", value));
            }
        }
        "C" => {
            if parts.len() == 1 {
                output(format!("C = {}", variables.c));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for C")?;
                variables.c = value;
                output(format!("Set C to {}", value));
            }
        }
        "D" => {
            if parts.len() == 1 {
                output(format!("D = {}", variables.d));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for D")?;
                variables.d = value;
                output(format!("Set D to {}", value));
            }
        }
        "I" => {
            if parts.len() == 1 {
                output(format!("I = {}", variables.i));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for I")?;
                variables.i = value;
                output(format!("Set I to {}", value));
            }
        }
        "X" => {
            if parts.len() == 1 {
                output(format!("X = {}", variables.x));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for X")?;
                variables.x = value;
                output(format!("Set X to {}", value));
            }
        }
        "Y" => {
            if parts.len() == 1 {
                output(format!("Y = {}", variables.y));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for Y")?;
                variables.y = value;
                output(format!("Set Y to {}", value));
            }
        }
        "Z" => {
            if parts.len() == 1 {
                output(format!("Z = {}", variables.z));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for Z")?;
                variables.z = value;
                output(format!("Set Z to {}", value));
            }
        }
        "T" => {
            if parts.len() == 1 {
                output(format!("T = {}", variables.t));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for T")?;
                variables.t = value;
                output(format!("Set T to {}", value));
            }
        }
        "J" => {
            if script_index >= 10 {
                output("Error: J requires script context".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 1 {
                output(format!("J = {}", scripts.scripts[script_index].j));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for J")?;
                scripts.scripts[script_index].j = value;
                output(format!("Set J to {}", value));
            }
        }
        "K" => {
            if script_index >= 10 {
                output("Error: K requires script context".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 1 {
                output(format!("K = {}", scripts.scripts[script_index].k));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for K")?;
                scripts.scripts[script_index].k = value;
                output(format!("Set K to {}", value));
            }
        }
        "P.N" => {
            if parts.len() == 1 {
                output(format!("P.N = {}", patterns.working));
            } else {
                let value: usize = parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?;
                if value > 3 {
                    output("Error: Pattern number must be 0-3".to_string());
                    return Ok(vec![]);
                }
                patterns.working = value;
                output(format!("Set working pattern to {}", value));
            }
        }
        "P.L" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if parts.len() == 1 {
                output(format!("P.L = {}", pattern.length));
            } else {
                let value: usize = parts[1]
                    .parse()
                    .context("Failed to parse pattern length")?;
                if value < 1 || value > 64 {
                    output("Error: Pattern length must be 1-64".to_string());
                    return Ok(vec![]);
                }
                pattern.length = value;
                output(format!("Set pattern {} length to {}", patterns.working, value));
            }
        }
        "P.I" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if parts.len() == 1 {
                output(format!("P.I = {}", pattern.index));
            } else {
                let value: usize = parts[1]
                    .parse()
                    .context("Failed to parse pattern index")?;
                if value > 63 {
                    output("Error: Pattern index must be 0-63".to_string());
                    return Ok(vec![]);
                }
                pattern.index = value;
                output(format!("Set pattern {} index to {}", patterns.working, value));
            }
        }
        "P.HERE" => {
            let pattern = &patterns.patterns[patterns.working];
            let value = pattern.data[pattern.index];
            output(format!("P.HERE = {}", value));
        }
        "P.NEXT" => {
            let pattern = &mut patterns.patterns[patterns.working];
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            output(format!("P.NEXT = {} (index now {})", value, pattern.index));
        }
        "P.PREV" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            output(format!("P.PREV = {} (index now {})", value, pattern.index));
        }
        "P" => {
            if parts.len() == 1 {
                output("Error: P requires an index".to_string());
                return Ok(vec![]);
            }
            let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern index")?
            };
            if idx > 63 {
                output("Error: Pattern index must be 0-63".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 2 {
                let pattern = &patterns.patterns[patterns.working];
                output(format!("P {} = {}", idx, pattern.data[idx]));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[2]
                        .parse()
                        .context("Failed to parse pattern value")?
                };
                let pattern = &mut patterns.patterns[patterns.working];
                pattern.data[idx] = value;
                output(format!("Set P {} to {}", idx, value));
            }
        }
        "PN.L" => {
            if parts.len() < 2 {
                output("Error: PN.L requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?
            };
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 2 {
                let pattern = &patterns.patterns[pat];
                output(format!("PN.L {} = {}", pat, pattern.length));
            } else {
                let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, scripts, script_index) {
                    expr_val as usize
                } else {
                    parts[2]
                        .parse()
                        .context("Failed to parse pattern length")?
                };
                if value < 1 || value > 64 {
                    output("Error: Pattern length must be 1-64".to_string());
                    return Ok(vec![]);
                }
                let pattern = &mut patterns.patterns[pat];
                pattern.length = value;
                output(format!("Set pattern {} length to {}", pat, value));
            }
        }
        "PN.I" => {
            if parts.len() < 2 {
                output("Error: PN.I requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?
            };
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 2 {
                let pattern = &patterns.patterns[pat];
                output(format!("PN.I {} = {}", pat, pattern.index));
            } else {
                let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, scripts, script_index) {
                    expr_val as usize
                } else {
                    parts[2]
                        .parse()
                        .context("Failed to parse pattern index")?
                };
                if value > 63 {
                    output("Error: Pattern index must be 0-63".to_string());
                    return Ok(vec![]);
                }
                let pattern = &mut patterns.patterns[pat];
                pattern.index = value;
                output(format!("Set pattern {} index to {}", pat, value));
            }
        }
        "PN.HERE" => {
            if parts.len() < 2 {
                output("Error: PN.HERE requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?
            };
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &patterns.patterns[pat];
            let value = pattern.data[pattern.index];
            output(format!("PN.HERE {} = {}", pat, value));
        }
        "PN.NEXT" => {
            if parts.len() < 2 {
                output("Error: PN.NEXT requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?
            };
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            output(format!("PN.NEXT {} = {} (index now {})", pat, value, pattern.index));
        }
        "PN.PREV" => {
            if parts.len() < 2 {
                output("Error: PN.PREV requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?
            };
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            output(format!("PN.PREV {} = {} (index now {})", pat, value, pattern.index));
        }
        "PN" => {
            if parts.len() < 3 {
                output("Error: PN requires pattern (0-3) and index (0-63)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?
            };
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, scripts, script_index) {
                expr_val as usize
            } else {
                parts[2]
                    .parse()
                    .context("Failed to parse pattern index")?
            };
            if idx > 63 {
                output("Error: Pattern index must be 0-63".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 3 {
                let pattern = &patterns.patterns[pat];
                output(format!("PN {} {} = {}", pat, idx, pattern.data[idx]));
            } else {
                let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[3]
                        .parse()
                        .context("Failed to parse pattern value")?
                };
                let pattern = &mut patterns.patterns[pat];
                pattern.data[idx] = val;
                output(format!("Set PN {} {} to {}", pat, idx, val));
            }
        }
        "TR" => {
            metro_tx
                .send(MetroCommand::SendTrigger)
                .context("Failed to send trigger to metro thread")?;
            output("Sent trigger".to_string());
        }
        "VOL" => {
            if parts.len() < 2 {
                output("Error: VOL requires a value (0.0-1.0)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse volume value as float")?;
            if !(0.0..=1.0).contains(&value) {
                output("Warning: Volume should be between 0.0 and 1.0".to_string());
            }
            metro_tx
                .send(MetroCommand::SendVolume(value))
                .context("Failed to send volume to metro thread")?;
            output(format!("Set volume to {}", value));
        }
        "M" => {
            if parts.len() == 1 {
                output(format!("Metro interval: {}ms", metro_interval));
            } else {
                let value: u64 = parts[1]
                    .parse()
                    .context("Failed to parse interval as milliseconds")?;
                if value == 0 {
                    output("Error: Interval must be greater than 0".to_string());
                    return Ok(vec![]);
                }
                metro_tx
                    .send(MetroCommand::SetInterval(value))
                    .context("Failed to send interval to metro thread")?;
                *metro_interval = value;
                output(format!("Set metro interval to {}ms", value));
            }
        }
        "M.BPM" => {
            if parts.len() < 2 {
                output("Error: M.BPM requires a BPM value".to_string());
                return Ok(vec![]);
            }
            let bpm: f32 = parts[1]
                .parse()
                .context("Failed to parse BPM value as number")?;
            if bpm <= 0.0 {
                output("Error: BPM must be greater than 0".to_string());
                return Ok(vec![]);
            }
            let interval_ms = (60000.0 / bpm) as u64;
            metro_tx
                .send(MetroCommand::SetInterval(interval_ms))
                .context("Failed to send interval to metro thread")?;
            *metro_interval = interval_ms;
            output(format!("Set metro to {} BPM ({}ms)", bpm, interval_ms));
        }
        "M.ACT" => {
            if parts.len() < 2 {
                output("Error: M.ACT requires 0 or 1".to_string());
                return Ok(vec![]);
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse M.ACT value")?;
            if !(0..=1).contains(&value) {
                output("Error: M.ACT value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SetActive(value != 0))
                .context("Failed to send active state to metro thread")?;
            output(format!(
                "Metro {}",
                if value != 0 {
                    "activated"
                } else {
                    "deactivated"
                }
            ));
        }
        "M.SCRIPT" => {
            if parts.len() < 2 {
                output("Error: M.SCRIPT requires a script number (1-8)".to_string());
                return Ok(vec![]);
            }
            let value: usize = parts[1]
                .parse()
                .context("Failed to parse script number")?;
            if !(1..=8).contains(&value) {
                output("Error: M.SCRIPT value must be 1-8".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SetScriptIndex(value - 1))
                .context("Failed to send script index to metro thread")?;
            output(format!("Metro will call Script {} on each tick", value));
        }
        "PF" => {
            if parts.len() < 2 {
                output("Error: PF requires a frequency value (20-20000)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse frequency value")?
            };
            if !(20.0..=20000.0).contains(&value) {
                output("Error: Frequency must be between 20 and 20000 Hz".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set primary frequency to {} Hz", value));
        }
        "PW" => {
            if parts.len() < 2 {
                output("Error: PW requires a waveform value (0-2)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse waveform value")?
            };
            if !(0..=2).contains(&value) {
                output("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set primary waveform to {}", value));
        }
        "MF" => {
            if parts.len() < 2 {
                output("Error: MF requires a frequency value (20-20000)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse frequency value")?
            };
            if !(20.0..=20000.0).contains(&value) {
                output("Error: Frequency must be between 20 and 20000 Hz".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod frequency to {} Hz", value));
        }
        "MW" => {
            if parts.len() < 2 {
                output("Error: MW requires a waveform value (0-2)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse waveform value")?
            };
            if !(0..=2).contains(&value) {
                output("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod waveform to {}", value));
        }
        "DC" => {
            if parts.len() < 2 {
                output("Error: DC requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse discontinuity amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Discontinuity amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/monokit_debug.txt") {
                writeln!(f, "DC sending OSC: value={}", value).ok();
            }
            metro_tx
                .send(MetroCommand::SendParam("dc".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity amount to {}", value));
        }
        "DM" => {
            if parts.len() < 2 {
                output("Error: DM requires a mode value (0-2)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse discontinuity mode")?
            };
            if !(0..=2).contains(&value) {
                output("Error: Mode must be 0 (fold), 1 (tanh), or 2 (softclip)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("dm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity mode to {}", value));
        }
        "TK" => {
            if parts.len() < 2 {
                output("Error: TK requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse tracking amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Tracking amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("tk".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set tracking amount to {}", value));
        }
        "MB" => {
            if parts.len() < 2 {
                output("Error: MB requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod bus amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Mod bus amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mb".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod bus amount to {}", value));
        }
        "MP" => {
            if parts.len() < 2 {
                output("Error: MP requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> primary value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mp".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> primary freq to {}", value));
        }
        "MD" => {
            if parts.len() < 2 {
                output("Error: MD requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> discontinuity value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("md".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> discontinuity to {}", value));
        }
        "MT" => {
            if parts.len() < 2 {
                output("Error: MT requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> tracking value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mt".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> tracking to {}", value));
        }
        "MA" => {
            if parts.len() < 2 {
                output("Error: MA requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> amplitude value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("ma".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> amplitude to {}", value));
        }
        "FM" => {
            if parts.len() < 2 {
                output("Error: FM requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse FM index")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: FM index must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM index to {}", value));
        }
        "AD" => {
            if parts.len() < 2 {
                output("Error: AD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse amp decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: Amp decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set amp decay to {} ms", value));
        }
        "PD" => {
            if parts.len() < 2 {
                output("Error: PD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pitch decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: Pitch decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set pitch decay to {} ms", value));
        }
        "FD" => {
            if parts.len() < 2 {
                output("Error: FD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse FM decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: FM decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM decay to {} ms", value));
        }
        "PA" => {
            if parts.len() < 2 {
                output("Error: PA requires a multiplier value (0-16)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pitch env amount")?
            };
            if !(0.0..=16.0).contains(&value) {
                output("Error: Pitch env amount must be between 0 and 16".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set pitch env amount to {}", value));
        }
        "DD" => {
            if parts.len() < 2 {
                output("Error: DD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse discontinuity decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: Discontinuity decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("dd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity decay to {} ms", value));
        }
        "MX" => {
            if parts.len() < 2 {
                output("Error: MX requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mix amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Mix amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mx".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mix amount to {}", value));
        }
        "MM" => {
            if parts.len() < 2 {
                output("Error: MM requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod bus -> mix value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod bus -> mix to {}", value));
        }
        "ME" => {
            if parts.len() < 2 {
                output("Error: ME requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse envelope -> mix value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("me".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set envelope -> mix to {}", value));
        }
        "FA" => {
            if parts.len() < 2 {
                output("Error: FA requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse FM envelope amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: FM envelope amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM envelope amount to {}", value));
        }
        "DA" => {
            if parts.len() < 2 {
                output("Error: DA requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse DC envelope amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: DC envelope amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("da".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set DC envelope amount to {}", value));
        }
        "RST" => {
            metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(200.0)))?;
            metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(50.0)))?;
            metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dc".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dd".to_string(), OscType::Int(100)))?;
            metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("ad".to_string(), OscType::Int(100)))?;
            metro_tx.send(MetroCommand::SendParam("pd".to_string(), OscType::Int(10)))?;
            metro_tx.send(MetroCommand::SendParam("fd".to_string(), OscType::Int(10)))?;
            metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(4.0)))?;
            metro_tx.send(MetroCommand::SendParam("mx".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("me".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendVolume(1.0))?;
            output("Reset to defaults".to_string());
        }
        "RND" => {
            if parts.len() < 2 {
                output("Error: RND requires a max value".to_string());
                return Ok(vec![]);
            }
            let max: i16 = parts[1]
                .parse()
                .context("Failed to parse max value as number")?;
            if max <= 0 {
                output("0".to_string());
            } else {
                let result = rand::thread_rng().gen_range(0..max);
                output(format!("{}", result));
            }
        }
        "RRND" => {
            if parts.len() < 3 {
                output("Error: RRND requires min and max values".to_string());
                return Ok(vec![]);
            }
            let mut min: i16 = parts[1]
                .parse()
                .context("Failed to parse min value as number")?;
            let mut max: i16 = parts[2]
                .parse()
                .context("Failed to parse max value as number")?;
            if min > max {
                std::mem::swap(&mut min, &mut max);
            }
            let result = rand::thread_rng().gen_range(min..=max);
            output(format!("{}", result));
        }
        "ADD" => {
            if parts.len() < 2 {
                output("Error: ADD requires two operands".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    let result = x.saturating_add(y);
                    output(format!("{}", result));
                } else {
                    output("Error: Failed to evaluate second operand".to_string());
                }
            } else {
                output("Error: Failed to evaluate first operand".to_string());
            }
        }
        "SUB" => {
            if parts.len() < 2 {
                output("Error: SUB requires two operands".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    let result = x.saturating_sub(y);
                    output(format!("{}", result));
                } else {
                    output("Error: Failed to evaluate second operand".to_string());
                }
            } else {
                output("Error: Failed to evaluate first operand".to_string());
            }
        }
        "MUL" => {
            if parts.len() < 2 {
                output("Error: MUL requires two operands".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    let result = x.saturating_mul(y);
                    output(format!("{}", result));
                } else {
                    output("Error: Failed to evaluate second operand".to_string());
                }
            } else {
                output("Error: Failed to evaluate first operand".to_string());
            }
        }
        "DIV" => {
            if parts.len() < 2 {
                output("Error: DIV requires two operands".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    if y == 0 {
                        output("Error: Division by zero".to_string());
                    } else {
                        let result = x / y;
                        output(format!("{}", result));
                    }
                } else {
                    output("Error: Failed to evaluate second operand".to_string());
                }
            } else {
                output("Error: Failed to evaluate first operand".to_string());
            }
        }
        "MOD" => {
            if parts.len() < 2 {
                output("Error: MOD requires two operands".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    if y == 0 {
                        output("Error: Modulo by zero".to_string());
                    } else {
                        let result = x % y;
                        output(format!("{}", result));
                    }
                } else {
                    output("Error: Failed to evaluate second operand".to_string());
                }
            } else {
                output("Error: Failed to evaluate first operand".to_string());
            }
        }
        "SCRIPT" => {
            if parts.len() < 2 {
                output("Error: SCRIPT requires number 1-8".to_string());
                return Ok(vec![]);
            }
            let num: usize = parts[1]
                .parse()
                .context("Failed to parse script number")?;
            if num < 1 || num > 8 {
                output("Error: SCRIPT number must be 1-8".to_string());
                return Ok(vec![]);
            }
            return Ok(vec![num - 1]);
        }
        "SAVE" => {
            if parts.len() < 2 {
                output("Error: SAVE requires a scene name".to_string());
                return Ok(vec![]);
            }
            let name = parts[1..].join(" ");
            let scene = crate::scene::Scene::from_app_state(scripts, patterns);
            match crate::scene::save_scene(&name, &scene) {
                Ok(()) => output(format!("Saved scene: {}", name)),
                Err(e) => output(format!("Error: {:?}", e)),
            }
        }
        "LOAD" => {
            if parts.len() < 2 {
                output("Error: LOAD requires a scene name".to_string());
                return Ok(vec![]);
            }
            let name = parts[1..].join(" ");
            match crate::scene::load_scene(&name) {
                Ok(scene) => {
                    scene.apply_to_app_state(scripts, patterns);
                    *variables = crate::types::Variables::default();
                    output(format!("Loaded scene: {}", name));
                }
                Err(crate::scene::SceneError::NotFound(_)) => {
                    output(format!("Error: Scene '{}' not found", name));
                }
                Err(e) => output(format!("Error: {:?}", e)),
            }
        }
        "SCENES" => {
            match crate::scene::list_scenes() {
                Ok(scenes) => {
                    if scenes.is_empty() {
                        output("No scenes saved".to_string());
                    } else {
                        output("Scenes:".to_string());
                        for (name, size) in scenes {
                            let size_kb = size as f64 / 1024.0;
                            output(format!("  {} ({:.1} KB)", name, size_kb));
                        }
                    }
                }
                Err(e) => output(format!("Error: {:?}", e)),
            }
        }
        "DELETE" => {
            if parts.len() < 2 {
                output("Error: DELETE requires a scene name".to_string());
                return Ok(vec![]);
            }
            let name = parts[1..].join(" ");
            match crate::scene::delete_scene(&name) {
                Ok(()) => output(format!("Deleted scene: {}", name)),
                Err(crate::scene::SceneError::NotFound(_)) => {
                    output(format!("Error: Scene '{}' not found", name));
                }
                Err(e) => output(format!("Error: {:?}", e)),
            }
        }
        "HELP" => {
            output("=== MONOKIT COMMANDS ===".to_string());
            output("".to_string());
            output("TRIGGER: TR".to_string());
            output("VOLUME:  VOL <0.0-1.0>".to_string());
            output("RESET:   RST".to_string());
            output("".to_string());
            output("-- Oscillators --".to_string());
            output("PF <hz>     Primary freq (20-20000)".to_string());
            output("PW <0-2>    Primary wave (sin/tri/saw)".to_string());
            output("MF <hz>     Mod freq".to_string());
            output("MW <0-2>    Mod wave".to_string());
            output("".to_string());
            output("-- FM & Discontinuity --".to_string());
            output("FM <0-16383>  FM index".to_string());
            output("FA <0-16383>  FM env amount".to_string());
            output("FD <ms>       FM env decay".to_string());
            output("DC <0-16383>  Discontinuity".to_string());
            output("DA <0-16383>  DC env amount".to_string());
            output("DD <ms>       DC env decay".to_string());
            output("DM <0-2>      DC mode (fold/tanh/soft)".to_string());
            output("".to_string());
            output("-- Envelopes --".to_string());
            output("AD <ms>       Amp decay".to_string());
            output("PD <ms>       Pitch decay".to_string());
            output("PA <0-16>     Pitch env amount".to_string());
            output("".to_string());
            output("-- Mod Bus --".to_string());
            output("MB <0-16383>  Mod bus amount".to_string());
            output("MP/MD/MT/MA <0|1>  Routing toggles".to_string());
            output("TK <0-16383>  Tracking".to_string());
            output("".to_string());
            output("-- Mix --".to_string());
            output("MX <0-16383>  Mix amount".to_string());
            output("MM/ME <0|1>   Mix routing".to_string());
            output("".to_string());
            output("-- Metro --".to_string());
            output("M             Show interval".to_string());
            output("M <ms>        Set interval".to_string());
            output("M.BPM <bpm>   Set BPM".to_string());
            output("M.ACT <0|1>     Start/stop".to_string());
            output("M.SCRIPT <1-8> Set script to call on each tick".to_string());
            output("".to_string());
            output("-- Scripts --".to_string());
            output("SCRIPT <1-8>  Execute stored script".to_string());
            output("".to_string());
            output("-- Scenes --".to_string());
            output("SAVE <name>   Save current state".to_string());
            output("LOAD <name>   Load saved state".to_string());
            output("SCENES        List all scenes".to_string());
            output("DELETE <name> Delete a scene".to_string());
        }
        _ => {
            output(format!("Unknown command: {}", cmd));
        }
    }

    Ok(vec![])
}

pub fn validate_script_command(cmd: &str) -> Result<()> {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    if trimmed.contains(':') {
        let colon_pos = trimmed.find(':').unwrap();
        let prefix = trimmed[..colon_pos].trim().to_uppercase();
        if prefix.starts_with("IF ")
            || prefix.starts_with("ELIF ")
            || prefix == "ELSE"
            || prefix.starts_with("PROB ")
            || prefix.starts_with("EV ")
            || prefix.starts_with("SKIP ")
            || prefix.starts_with("L ")
        {
            return Ok(());
        }
    }

    if trimmed.to_uppercase().starts_with("L ") {
        return Ok(());
    }

    if trimmed.contains(';') {
        return Ok(());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let command = parts[0].to_uppercase();
    let argc = parts.len() - 1; // argument count (excluding command)

    match command.as_str() {
        // Variables: 0-1 args (get or set)
        "A" | "B" | "C" | "D" | "I" | "X" | "Y" | "Z" | "T" | "J" | "K" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} takes 0-1 arguments", command));
            }
            Ok(())
        }
        // Working pattern ops with no args
        "P.HERE" | "P.NEXT" | "P.PREV" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
            }
            Ok(())
        }
        // Working pattern ops with 0-1 args
        "P.N" | "P.L" | "P.I" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} takes 0-1 arguments", command));
            }
            Ok(())
        }
        // P <idx> [val] - 1-2 args
        "P" => {
            if argc < 1 || argc > 2 {
                return Err(anyhow::anyhow!("P takes 1-2 arguments"));
            }
            Ok(())
        }
        // Explicit pattern ops with exactly 1 arg
        "PN.HERE" | "PN.NEXT" | "PN.PREV" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument (pattern number)", command));
            }
            Ok(())
        }
        // Explicit pattern ops with 1-2 args
        "PN.L" | "PN.I" => {
            if argc < 1 || argc > 2 {
                return Err(anyhow::anyhow!("{} takes 1-2 arguments", command));
            }
            Ok(())
        }
        // PN <pat> <idx> [val] - 2-3 args
        "PN" => {
            if argc < 2 || argc > 3 {
                return Err(anyhow::anyhow!("PN takes 2-3 arguments"));
            }
            Ok(())
        }
        // Math ops - exactly 2 args
        "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "+" | "-" | "*" | "/" | "%" => {
            if argc != 2 {
                return Err(anyhow::anyhow!("{} takes exactly 2 arguments", command));
            }
            Ok(())
        }
        // RND - 1 arg, RRND - 2 args
        "RND" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("RND takes exactly 1 argument"));
            }
            Ok(())
        }
        "RRND" => {
            if argc != 2 {
                return Err(anyhow::anyhow!("RRND takes exactly 2 arguments"));
            }
            Ok(())
        }
        // N - 1 arg
        "N" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("N takes exactly 1 argument"));
            }
            Ok(())
        }
        // Unary comparison - 1 arg
        "EZ" | "NZ" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        // Binary comparison - 2 args
        "EQ" | "NE" | "GT" | "LT" | "GTE" | "LTE" => {
            if argc != 2 {
                return Err(anyhow::anyhow!("{} takes exactly 2 arguments", command));
            }
            Ok(())
        }
        // SCRIPT - 1 arg
        "SCRIPT" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("SCRIPT takes exactly 1 argument"));
            }
            Ok(())
        }
        // Scene commands
        "SAVE" | "LOAD" | "DELETE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "SCENES" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("SCENES takes no arguments"));
            }
            Ok(())
        }
        // Metro commands
        "M" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("M takes 0-1 arguments"));
            }
            Ok(())
        }
        "M.BPM" | "M.ACT" | "M.SCRIPT" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        // No-arg commands
        "TR" | "RST" | "HELP" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
            }
            Ok(())
        }
        // VOL - 1 arg
        "VOL" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("VOL takes exactly 1 argument"));
            }
            Ok(())
        }
        // Synth params - 1 arg
        "PF" | "MF" | "PW" | "MW" | "DC" | "TK" | "MB" | "FM" | "MX" | "FA" | "DA" | "DM" | "MP" | "MD" | "MT" | "MA" | "MM" | "ME" | "AD" | "PD" | "FD" | "DD" | "PA" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        _ => {
            Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
