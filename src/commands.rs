use crate::config;
use crate::eval::eval_expression;
use crate::theme::Theme;
use crate::types::{MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rand::Rng;
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &mut ScriptStorage,
    script_index: usize,
    theme: &mut Theme,
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
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for A")?
                };
                variables.a = value;
                output(format!("SET A TO {}", value));
            }
        }
        "B" => {
            if parts.len() == 1 {
                output(format!("B = {}", variables.b));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for B")?
                };
                variables.b = value;
                output(format!("SET B TO {}", value));
            }
        }
        "C" => {
            if parts.len() == 1 {
                output(format!("C = {}", variables.c));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for C")?
                };
                variables.c = value;
                output(format!("SET C TO {}", value));
            }
        }
        "D" => {
            if parts.len() == 1 {
                output(format!("D = {}", variables.d));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for D")?
                };
                variables.d = value;
                output(format!("SET D TO {}", value));
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
                output(format!("SET I TO {}", value));
            }
        }
        "X" => {
            if parts.len() == 1 {
                output(format!("X = {}", variables.x));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for X")?
                };
                variables.x = value;
                output(format!("SET X TO {}", value));
            }
        }
        "Y" => {
            if parts.len() == 1 {
                output(format!("Y = {}", variables.y));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for Y")?
                };
                variables.y = value;
                output(format!("SET Y TO {}", value));
            }
        }
        "Z" => {
            if parts.len() == 1 {
                output(format!("Z = {}", variables.z));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for Z")?
                };
                variables.z = value;
                output(format!("SET Z TO {}", value));
            }
        }
        "T" => {
            if parts.len() == 1 {
                output(format!("T = {}", variables.t));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for T")?
                };
                variables.t = value;
                output(format!("SET T TO {}", value));
            }
        }
        "J" => {
            if script_index >= 10 {
                output("ERROR: J REQUIRES SCRIPT CONTEXT".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 1 {
                output(format!("J = {}", scripts.scripts[script_index].j));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for J")?
                };
                scripts.scripts[script_index].j = value;
                output(format!("SET J TO {}", value));
            }
        }
        "K" => {
            if script_index >= 10 {
                output("ERROR: K REQUIRES SCRIPT CONTEXT".to_string());
                return Ok(vec![]);
            }
            if parts.len() == 1 {
                output(format!("K = {}", scripts.scripts[script_index].k));
            } else {
                let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                    expr_val
                } else {
                    parts[1]
                        .parse()
                        .context("Failed to parse value for K")?
                };
                scripts.scripts[script_index].k = value;
                output(format!("SET K TO {}", value));
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
                    output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
                    return Ok(vec![]);
                }
                patterns.working = value;
                output(format!("SET WORKING PATTERN TO {}", value));
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
                    output("ERROR: PATTERN LENGTH MUST BE 1-64".to_string());
                    return Ok(vec![]);
                }
                pattern.length = value;
                output(format!("SET PATTERN {} LENGTH TO {}", patterns.working, value));
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
                    output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
                    return Ok(vec![]);
                }
                pattern.index = value;
                output(format!("SET PATTERN {} INDEX TO {}", patterns.working, value));
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
            output(format!("P.NEXT = {} (INDEX NOW {})", value, pattern.index));
        }
        "P.PREV" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            output(format!("P.PREV = {} (INDEX NOW {})", value, pattern.index));
        }
        "P" => {
            if parts.len() == 1 {
                output("ERROR: P REQUIRES AN INDEX".to_string());
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
                output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
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
                output(format!("SET P {} TO {}", idx, value));
            }
        }
        "PN.L" => {
            if parts.len() < 2 {
                output("ERROR: PN.L REQUIRES PATTERN NUMBER (0-3)".to_string());
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
                output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
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
                    output("ERROR: PATTERN LENGTH MUST BE 1-64".to_string());
                    return Ok(vec![]);
                }
                let pattern = &mut patterns.patterns[pat];
                pattern.length = value;
                output(format!("SET PATTERN {} LENGTH TO {}", pat, value));
            }
        }
        "PN.I" => {
            if parts.len() < 2 {
                output("ERROR: PN.I REQUIRES PATTERN NUMBER (0-3)".to_string());
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
                output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
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
                    output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
                    return Ok(vec![]);
                }
                let pattern = &mut patterns.patterns[pat];
                pattern.index = value;
                output(format!("SET PATTERN {} INDEX TO {}", pat, value));
            }
        }
        "PN.HERE" => {
            if parts.len() < 2 {
                output("ERROR: PN.HERE REQUIRES PATTERN NUMBER (0-3)".to_string());
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
                output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &patterns.patterns[pat];
            let value = pattern.data[pattern.index];
            output(format!("PN.HERE {} = {}", pat, value));
        }
        "PN.NEXT" => {
            if parts.len() < 2 {
                output("ERROR: PN.NEXT REQUIRES PATTERN NUMBER (0-3)".to_string());
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
                output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            output(format!("PN.NEXT {} = {} (INDEX NOW {})", pat, value, pattern.index));
        }
        "PN.PREV" => {
            if parts.len() < 2 {
                output("ERROR: PN.PREV REQUIRES PATTERN NUMBER (0-3)".to_string());
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
                output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            output(format!("PN.PREV {} = {} (INDEX NOW {})", pat, value, pattern.index));
        }
        "PN" => {
            if parts.len() < 3 {
                output("ERROR: PN REQUIRES PATTERN (0-3) AND INDEX (0-63)".to_string());
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
                output("ERROR: PATTERN NUMBER MUST BE 0-3".to_string());
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
                output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
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
                output(format!("SET PN {} {} TO {}", pat, idx, val));
            }
        }
        "TR" => {
            metro_tx
                .send(MetroCommand::SendTrigger)
                .context("Failed to send trigger to metro thread")?;
            output("SENT TRIGGER".to_string());
        }
        "VOL" => {
            if parts.len() < 2 {
                output("ERROR: VOL REQUIRES A VALUE (0.0-1.0)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse volume value as float")?;
            if !(0.0..=1.0).contains(&value) {
                output("WARNING: VOLUME SHOULD BE BETWEEN 0.0 AND 1.0".to_string());
            }
            metro_tx
                .send(MetroCommand::SendVolume(value))
                .context("Failed to send volume to metro thread")?;
            output(format!("SET VOLUME TO {}", value));
        }
        "M" => {
            if parts.len() == 1 {
                output(format!("METRO INTERVAL: {}MS", metro_interval));
            } else {
                let value: u64 = parts[1]
                    .parse()
                    .context("Failed to parse interval as milliseconds")?;
                if value == 0 {
                    output("ERROR: INTERVAL MUST BE GREATER THAN 0".to_string());
                    return Ok(vec![]);
                }
                metro_tx
                    .send(MetroCommand::SetInterval(value))
                    .context("Failed to send interval to metro thread")?;
                *metro_interval = value;
                output(format!("SET METRO INTERVAL TO {}MS", value));
            }
        }
        "M.BPM" => {
            if parts.len() < 2 {
                output("ERROR: M.BPM REQUIRES A BPM VALUE".to_string());
                return Ok(vec![]);
            }
            let bpm: f32 = parts[1]
                .parse()
                .context("Failed to parse BPM value as number")?;
            if bpm <= 0.0 {
                output("ERROR: BPM MUST BE GREATER THAN 0".to_string());
                return Ok(vec![]);
            }
            let interval_ms = (60000.0 / bpm) as u64;
            metro_tx
                .send(MetroCommand::SetInterval(interval_ms))
                .context("Failed to send interval to metro thread")?;
            *metro_interval = interval_ms;
            output(format!("SET METRO TO {} BPM ({}MS)", bpm, interval_ms));
        }
        "M.ACT" => {
            if parts.len() < 2 {
                output("ERROR: M.ACT REQUIRES 0 OR 1".to_string());
                return Ok(vec![]);
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse M.ACT value")?;
            if !(0..=1).contains(&value) {
                output("ERROR: M.ACT VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SetActive(value != 0))
                .context("Failed to send active state to metro thread")?;
            output(format!(
                "METRO {}",
                if value != 0 {
                    "ACTIVATED"
                } else {
                    "DEACTIVATED"
                }
            ));
        }
        "M.SCRIPT" => {
            if parts.len() < 2 {
                output("ERROR: M.SCRIPT REQUIRES A SCRIPT NUMBER (1-8)".to_string());
                return Ok(vec![]);
            }
            let value: usize = parts[1]
                .parse()
                .context("Failed to parse script number")?;
            if !(1..=8).contains(&value) {
                output("ERROR: M.SCRIPT VALUE MUST BE 1-8".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SetScriptIndex(value - 1))
                .context("Failed to send script index to metro thread")?;
            output(format!("METRO WILL CALL SCRIPT {} ON EACH TICK", value));
        }
        "PF" => {
            if parts.len() < 2 {
                output("ERROR: PF REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
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
                output("ERROR: FREQUENCY MUST BE BETWEEN 20 AND 20000 HZ".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET PRIMARY FREQUENCY TO {} HZ", value));
        }
        "PW" => {
            if parts.len() < 2 {
                output("ERROR: PW REQUIRES A WAVEFORM VALUE (0-2)".to_string());
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
                output("ERROR: WAVEFORM MUST BE 0 (SIN), 1 (TRI), OR 2 (SAW)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET PRIMARY WAVEFORM TO {}", value));
        }
        "MF" => {
            if parts.len() < 2 {
                output("ERROR: MF REQUIRES A FREQUENCY VALUE (20-20000)".to_string());
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
                output("ERROR: FREQUENCY MUST BE BETWEEN 20 AND 20000 HZ".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD FREQUENCY TO {} HZ", value));
        }
        "MW" => {
            if parts.len() < 2 {
                output("ERROR: MW REQUIRES A WAVEFORM VALUE (0-2)".to_string());
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
                output("ERROR: WAVEFORM MUST BE 0 (SIN), 1 (TRI), OR 2 (SAW)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD WAVEFORM TO {}", value));
        }
        "DC" => {
            if parts.len() < 2 {
                output("ERROR: DC REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: DISCONTINUITY AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/monokit_debug.txt") {
                writeln!(f, "DC sending OSC: value={}", value).ok();
            }
            metro_tx
                .send(MetroCommand::SendParam("dc".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET DISCONTINUITY AMOUNT TO {}", value));
        }
        "DM" => {
            if parts.len() < 2 {
                output("ERROR: DM REQUIRES A MODE VALUE (0-2)".to_string());
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
                output("ERROR: MODE MUST BE 0 (FOLD), 1 (TANH), OR 2 (SOFTCLIP)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("dm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET DISCONTINUITY MODE TO {}", value));
        }
        "TK" => {
            if parts.len() < 2 {
                output("ERROR: TK REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: TRACKING AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("tk".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET TRACKING AMOUNT TO {}", value));
        }
        "MB" => {
            if parts.len() < 2 {
                output("ERROR: MB REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: MOD BUS AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mb".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD BUS AMOUNT TO {}", value));
        }
        "MP" => {
            if parts.len() < 2 {
                output("ERROR: MP REQUIRES A VALUE (0 OR 1)".to_string());
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mp".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD -> PRIMARY FREQ TO {}", value));
        }
        "MD" => {
            if parts.len() < 2 {
                output("ERROR: MD REQUIRES A VALUE (0 OR 1)".to_string());
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("md".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD -> DISCONTINUITY TO {}", value));
        }
        "MT" => {
            if parts.len() < 2 {
                output("ERROR: MT REQUIRES A VALUE (0 OR 1)".to_string());
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mt".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD -> TRACKING TO {}", value));
        }
        "MA" => {
            if parts.len() < 2 {
                output("ERROR: MA REQUIRES A VALUE (0 OR 1)".to_string());
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("ma".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD -> AMPLITUDE TO {}", value));
        }
        "FM" => {
            if parts.len() < 2 {
                output("ERROR: FM REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: FM INDEX MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET FM INDEX TO {}", value));
        }
        "AD" => {
            if parts.len() < 2 {
                output("ERROR: AD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
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
                output("ERROR: AMP DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET AMP DECAY TO {} MS", value));
        }
        "PD" => {
            if parts.len() < 2 {
                output("ERROR: PD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
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
                output("ERROR: PITCH DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET PITCH DECAY TO {} MS", value));
        }
        "FD" => {
            if parts.len() < 2 {
                output("ERROR: FD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
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
                output("ERROR: FM DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET FM DECAY TO {} MS", value));
        }
        "PA" => {
            if parts.len() < 2 {
                output("ERROR: PA REQUIRES A MULTIPLIER VALUE (0-16)".to_string());
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
                output("ERROR: PITCH ENV AMOUNT MUST BE BETWEEN 0 AND 16".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET PITCH ENV AMOUNT TO {}", value));
        }
        "DD" => {
            if parts.len() < 2 {
                output("ERROR: DD REQUIRES A TIME VALUE (1-10000 MS)".to_string());
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
                output("ERROR: DISCONTINUITY DECAY MUST BE BETWEEN 1 AND 10000 MS".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("dd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET DISCONTINUITY DECAY TO {} MS", value));
        }
        "MX" => {
            if parts.len() < 2 {
                output("ERROR: MX REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: MIX AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mx".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MIX AMOUNT TO {}", value));
        }
        "MM" => {
            if parts.len() < 2 {
                output("ERROR: MM REQUIRES A VALUE (0 OR 1)".to_string());
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET MOD BUS -> MIX TO {}", value));
        }
        "ME" => {
            if parts.len() < 2 {
                output("ERROR: ME REQUIRES A VALUE (0 OR 1)".to_string());
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("me".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET ENVELOPE -> MIX TO {}", value));
        }
        "FA" => {
            if parts.len() < 2 {
                output("ERROR: FA REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: FM ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET FM ENVELOPE AMOUNT TO {}", value));
        }
        "DA" => {
            if parts.len() < 2 {
                output("ERROR: DA REQUIRES A VALUE (0-16383)".to_string());
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
                output("ERROR: DC ENVELOPE AMOUNT MUST BE BETWEEN 0 AND 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("da".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("SET DC ENVELOPE AMOUNT TO {}", value));
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
            output("RESET TO DEFAULTS".to_string());
        }
        "RND" => {
            if parts.len() < 2 {
                output("ERROR: RND REQUIRES A MAX VALUE".to_string());
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
                output("ERROR: RRND REQUIRES MIN AND MAX VALUES".to_string());
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
        "TOSS" => {
            let result = if rand::thread_rng().gen_bool(0.5) { 1 } else { 0 };
            output(format!("{}", result));
        }
        "EITH" => {
            if parts.len() < 2 {
                output("ERROR: EITH REQUIRES TWO VALUES".to_string());
                return Ok(vec![]);
            }
            if let Some((a, a_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((b, _)) = eval_expression(&parts, 1 + a_consumed, variables, patterns, scripts, script_index) {
                    let result = if rand::thread_rng().gen_bool(0.5) { a } else { b };
                    output(format!("{}", result));
                } else {
                    output("ERROR: FAILED TO EVALUATE SECOND VALUE".to_string());
                }
            } else {
                output("ERROR: FAILED TO EVALUATE FIRST VALUE".to_string());
            }
        }
        "ADD" => {
            if parts.len() < 2 {
                output("ERROR: ADD REQUIRES TWO OPERANDS".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    let result = x.saturating_add(y);
                    output(format!("{}", result));
                } else {
                    output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
                }
            } else {
                output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
            }
        }
        "SUB" => {
            if parts.len() < 2 {
                output("ERROR: SUB REQUIRES TWO OPERANDS".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    let result = x.saturating_sub(y);
                    output(format!("{}", result));
                } else {
                    output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
                }
            } else {
                output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
            }
        }
        "MUL" => {
            if parts.len() < 2 {
                output("ERROR: MUL REQUIRES TWO OPERANDS".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    let result = x.saturating_mul(y);
                    output(format!("{}", result));
                } else {
                    output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
                }
            } else {
                output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
            }
        }
        "DIV" => {
            if parts.len() < 2 {
                output("ERROR: DIV REQUIRES TWO OPERANDS".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    if y == 0 {
                        output("ERROR: DIVISION BY ZERO".to_string());
                    } else {
                        let result = x / y;
                        output(format!("{}", result));
                    }
                } else {
                    output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
                }
            } else {
                output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
            }
        }
        "MOD" => {
            if parts.len() < 2 {
                output("ERROR: MOD REQUIRES TWO OPERANDS".to_string());
                return Ok(vec![]);
            }
            if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
                if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
                    if y == 0 {
                        output("ERROR: MODULO BY ZERO".to_string());
                    } else {
                        let result = x % y;
                        output(format!("{}", result));
                    }
                } else {
                    output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
                }
            } else {
                output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
            }
        }
        "SCRIPT" => {
            if parts.len() < 2 {
                output("ERROR: SCRIPT REQUIRES NUMBER 1-8".to_string());
                return Ok(vec![]);
            }
            let num: usize = parts[1]
                .parse()
                .context("Failed to parse script number")?;
            if num < 1 || num > 8 {
                output("ERROR: SCRIPT NUMBER MUST BE 1-8".to_string());
                return Ok(vec![]);
            }
            return Ok(vec![num - 1]);
        }
        "SAVE" => {
            if parts.len() < 2 {
                output("ERROR: SAVE REQUIRES A SCENE NAME".to_string());
                return Ok(vec![]);
            }
            let name = parts[1..].join(" ");
            let scene = crate::scene::Scene::from_app_state(scripts, patterns);
            match crate::scene::save_scene(&name, &scene) {
                Ok(()) => output(format!("SAVED SCENE: {}", name)),
                Err(e) => output(format!("ERROR: {:?}", e)),
            }
        }
        "LOAD" => {
            if parts.len() < 2 {
                output("ERROR: LOAD REQUIRES A SCENE NAME".to_string());
                return Ok(vec![]);
            }
            let name = parts[1..].join(" ");
            match crate::scene::load_scene(&name) {
                Ok(scene) => {
                    scene.apply_to_app_state(scripts, patterns);
                    *variables = crate::types::Variables::default();
                    output(format!("LOADED SCENE: {}", name));
                }
                Err(crate::scene::SceneError::NotFound(_)) => {
                    output(format!("ERROR: SCENE '{}' NOT FOUND", name));
                }
                Err(e) => output(format!("ERROR: {:?}", e)),
            }
        }
        "SCENES" => {
            match crate::scene::list_scenes() {
                Ok(scenes) => {
                    if scenes.is_empty() {
                        output("NO SCENES SAVED".to_string());
                    } else {
                        output("SCENES:".to_string());
                        for (name, size) in scenes {
                            let size_kb = size as f64 / 1024.0;
                            output(format!("  {} ({:.1} KB)", name, size_kb));
                        }
                    }
                }
                Err(e) => output(format!("ERROR: {:?}", e)),
            }
        }
        "DELETE" => {
            if parts.len() < 2 {
                output("ERROR: DELETE REQUIRES A SCENE NAME".to_string());
                return Ok(vec![]);
            }
            let name = parts[1..].join(" ");
            match crate::scene::delete_scene(&name) {
                Ok(()) => output(format!("DELETED SCENE: {}", name)),
                Err(crate::scene::SceneError::NotFound(_)) => {
                    output(format!("ERROR: SCENE '{}' NOT FOUND", name));
                }
                Err(e) => output(format!("ERROR: {:?}", e)),
            }
        }
        "THEME" => {
            if parts.len() == 1 {
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
            } else {
                let mode = parts[1].to_lowercase();
                match mode.as_str() {
                    "dark" => {
                        *theme = Theme::dark();
                        if let Err(e) = config::save_theme_mode("dark") {
                            output(format!("WARNING: FAILED TO SAVE THEME: {:?}", e));
                        }
                        output("SWITCHED TO DARK THEME".to_string());
                    }
                    "light" => {
                        *theme = Theme::light();
                        if let Err(e) = config::save_theme_mode("light") {
                            output(format!("WARNING: FAILED TO SAVE THEME: {:?}", e));
                        }
                        output("SWITCHED TO LIGHT THEME".to_string());
                    }
                    "system" => {
                        *theme = Theme::system();
                        if let Err(e) = config::save_theme_mode("system") {
                            output(format!("WARNING: FAILED TO SAVE THEME: {:?}", e));
                        }
                        output("SWITCHED TO SYSTEM THEME".to_string());
                    }
                    "custom" => {
                        match config::load_config() {
                            Ok(cfg) => match config::load_theme(&cfg) {
                                Ok(custom_theme) if cfg.display.theme == "custom" => {
                                    *theme = custom_theme;
                                    if let Err(e) = config::save_theme_mode("custom") {
                                        output(format!("WARNING: FAILED TO SAVE THEME: {:?}", e));
                                    }
                                    output("SWITCHED TO CUSTOM THEME".to_string());
                                }
                                _ => {
                                    output("ERROR: NO CUSTOM THEME DEFINED IN CONFIG".to_string());
                                }
                            },
                            Err(e) => {
                                output(format!("ERROR: FAILED TO LOAD CONFIG: {:?}", e));
                            }
                        }
                    }
                    _ => {
                        output("ERROR: UNKNOWN THEME MODE (USE DARK, LIGHT, SYSTEM, OR CUSTOM)".to_string());
                    }
                }
            }
        }
        "HELP" => {
            output("=== MONOKIT COMMANDS ===".to_string());
            output("".to_string());
            output("TRIGGER: TR".to_string());
            output("VOLUME:  VOL <0.0-1.0>".to_string());
            output("RESET:   RST".to_string());
            output("".to_string());
            output("-- OSCILLATORS --".to_string());
            output("PF <HZ>     PRIMARY FREQ (20-20000)".to_string());
            output("PW <0-2>    PRIMARY WAVE (SIN/TRI/SAW)".to_string());
            output("MF <HZ>     MOD FREQ".to_string());
            output("MW <0-2>    MOD WAVE".to_string());
            output("".to_string());
            output("-- FM & DISCONTINUITY --".to_string());
            output("FM <0-16383>  FM INDEX".to_string());
            output("FA <0-16383>  FM ENV AMOUNT".to_string());
            output("FD <MS>       FM ENV DECAY".to_string());
            output("DC <0-16383>  DISCONTINUITY".to_string());
            output("DA <0-16383>  DC ENV AMOUNT".to_string());
            output("DD <MS>       DC ENV DECAY".to_string());
            output("DM <0-2>      DC MODE (FOLD/TANH/SOFT)".to_string());
            output("".to_string());
            output("-- ENVELOPES --".to_string());
            output("AD <MS>       AMP DECAY".to_string());
            output("PD <MS>       PITCH DECAY".to_string());
            output("PA <0-16>     PITCH ENV AMOUNT".to_string());
            output("".to_string());
            output("-- MOD BUS --".to_string());
            output("MB <0-16383>  MOD BUS AMOUNT".to_string());
            output("MP/MD/MT/MA <0|1>  ROUTING TOGGLES".to_string());
            output("TK <0-16383>  TRACKING".to_string());
            output("".to_string());
            output("-- MIX --".to_string());
            output("MX <0-16383>  MIX AMOUNT".to_string());
            output("MM/ME <0|1>   MIX ROUTING".to_string());
            output("".to_string());
            output("-- METRO --".to_string());
            output("M             SHOW INTERVAL".to_string());
            output("M <MS>        SET INTERVAL".to_string());
            output("M.BPM <BPM>   SET BPM".to_string());
            output("M.ACT <0|1>     START/STOP".to_string());
            output("M.SCRIPT <1-8> SET SCRIPT TO CALL ON EACH TICK".to_string());
            output("".to_string());
            output("-- SCRIPTS --".to_string());
            output("SCRIPT <1-8>  EXECUTE STORED SCRIPT".to_string());
            output("".to_string());
            output("-- SCENES --".to_string());
            output("SAVE <NAME>   SAVE CURRENT STATE".to_string());
            output("LOAD <NAME>   LOAD SAVED STATE".to_string());
            output("SCENES        LIST ALL SCENES".to_string());
            output("DELETE <NAME> DELETE A SCENE".to_string());
            output("".to_string());
            output("-- THEME --".to_string());
            output("THEME            SHOW CURRENT".to_string());
            output("THEME DARK       DARK MODE".to_string());
            output("THEME LIGHT      LIGHT MODE".to_string());
            output("THEME SYSTEM     USE OS PREFERENCE".to_string());
            output("THEME CUSTOM     USE CUSTOM THEME".to_string());
        }
        _ => {
            output(format!("UNKNOWN COMMAND: {}", cmd));
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
        // Variables: 0+ args (get or set with expression)
        // Expression evaluation handles the value parsing, so we allow any arg count
        "A" | "B" | "C" | "D" | "I" | "X" | "Y" | "Z" | "T" | "J" | "K" => {
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
        // P <idx> [val] - 1+ args (idx can be expression)
        "P" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("P requires at least 1 argument"));
            }
            Ok(())
        }
        // Explicit pattern ops with 1+ args (pattern number can be expression)
        "PN.HERE" | "PN.NEXT" | "PN.PREV" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument (pattern number)", command));
            }
            Ok(())
        }
        // Explicit pattern ops with 1+ args (accepts expressions)
        "PN.L" | "PN.I" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        // PN <pat> <idx> [val] - 2+ args (accepts expressions)
        "PN" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN requires at least 2 arguments"));
            }
            Ok(())
        }
        // Math ops - 2+ args (accept expressions)
        "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "+" | "-" | "*" | "/" | "%" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} requires at least 2 arguments", command));
            }
            Ok(())
        }
        // Random ops - accept expressions
        "RND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("RND requires at least 1 argument"));
            }
            Ok(())
        }
        "RRND" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("RRND requires at least 2 arguments"));
            }
            Ok(())
        }
        "TOSS" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("TOSS takes no arguments"));
            }
            Ok(())
        }
        "EITH" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("EITH requires at least 2 arguments"));
            }
            Ok(())
        }
        // N - 1+ arg (accepts expression)
        "N" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("N requires at least 1 argument"));
            }
            Ok(())
        }
        // Unary comparison - 1+ arg (accepts expression)
        "EZ" | "NZ" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        // Binary comparison - 2+ args (accept expressions)
        "EQ" | "NE" | "GT" | "LT" | "GTE" | "LTE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} requires at least 2 arguments", command));
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
        "THEME" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("THEME takes 0-1 arguments"));
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
        // VOL - 1 arg (doesn't use eval_expression, uses direct parse)
        "VOL" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("VOL takes exactly 1 argument"));
            }
            Ok(())
        }
        // Synth params - 1+ args (accept expressions)
        "PF" | "MF" | "PW" | "MW" | "DC" | "TK" | "MB" | "FM" | "MX" | "FA" | "DA" | "DM" | "MP" | "MD" | "MT" | "MA" | "MM" | "ME" | "AD" | "PD" | "FD" | "DD" | "PA" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        _ => {
            Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
