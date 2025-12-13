use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

macro_rules! define_int_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
        pub fn $fn_name<F>(
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
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A VALUE ({}-{})", $error_cmd, $min, $max));
                return Ok(());
            }
            let state_snapshot = (
                patterns.toggle_state.clone(),
                patterns.toggle_last_value.clone()
            );
            let value: i32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                if consumed > 0 && parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                        patterns.direct_validation.insert(key, true);
                    }
                }
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context($parse_ctx)?
            };
            if value < $min || value > $max {
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
                output(format!("ERROR: {} MUST BE BETWEEN {} AND {}", $display_name, $min, $max));
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

macro_rules! define_int_param_ms {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
        pub fn $fn_name<F>(
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
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A TIME VALUE ({}-{} MS)", $error_cmd, $min, $max));
                return Ok(());
            }
            let state_snapshot = (
                patterns.toggle_state.clone(),
                patterns.toggle_last_value.clone()
            );
            let value: i32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                if consumed > 0 && parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                        patterns.direct_validation.insert(key, true);
                    }
                }
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context($parse_ctx)?
            };
            if value < $min || value > $max {
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
                output(format!("ERROR: {} MUST BE BETWEEN {} AND {} MS", $display_name, $min, $max));
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {} MS", $display_name, value));
            }
            Ok(())
        }
    };
}

macro_rules! define_float_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_cmd:expr, $display_name:expr, $parse_ctx:expr, $unit:expr) => {
        pub fn $fn_name<F>(
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
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A {} VALUE ({}-{})", $error_cmd,
                    if $unit.is_empty() { "VALUE" } else { $unit },
                    $min, $max));
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
                    .context($parse_ctx)?
            };
            if value < $min || value > $max {
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
                output(format!("ERROR: {} MUST BE BETWEEN {} AND {} {}", $display_name, $min, $max, $unit));
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {} {}", $display_name, value, $unit));
            }
            Ok(())
        }
    };
}

macro_rules! define_bool_param {
    ($fn_name:ident, $osc_param:expr, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
        pub fn $fn_name<F>(
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
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A VALUE (0 OR 1)", $error_cmd));
                return Ok(());
            }
            let state_snapshot = (
                patterns.toggle_state.clone(),
                patterns.toggle_last_value.clone()
            );
            let value: i32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                if consumed > 0 && parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                        patterns.direct_validation.insert(key, true);
                    }
                }
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context($parse_ctx)?
            };
            if !(0..=1).contains(&value) {
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
                output("ERROR: VALUE MUST BE 0 OR 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

macro_rules! define_mode_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_cmd:expr, $error_msg:expr, $display_name:expr, $parse_ctx:expr) => {
        pub fn $fn_name<F>(
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
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A VALUE ({}-{})", $error_cmd, $min, $max));
                return Ok(());
            }
            let state_snapshot = (
                patterns.toggle_state.clone(),
                patterns.toggle_last_value.clone()
            );
            let value: i32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                if consumed > 0 && parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                        patterns.direct_validation.insert(key, true);
                    }
                }
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context($parse_ctx)?
            };
            if value < $min || value > $max {
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
                output(format!("ERROR: {}", $error_msg));
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

macro_rules! define_mode_param_with_names {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_cmd:expr, $error_msg:expr, $display_name:expr, $parse_ctx:expr, $mode_names:expr) => {
        pub fn $fn_name<F>(
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
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("ERROR: {} REQUIRES A VALUE ({}-{})", $error_cmd, $min, $max));
                return Ok(());
            }
            let state_snapshot = (
                patterns.toggle_state.clone(),
                patterns.toggle_last_value.clone()
            );
            let value: i32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                if consumed > 0 && parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                        patterns.direct_validation.insert(key, true);
                    }
                }
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context($parse_ctx)?
            };
            if value < $min || value > $max {
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
                output(format!("ERROR: {}", $error_msg));
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                let mode_name = $mode_names.get(value as usize).unwrap_or(&"UNKNOWN");
                output(format!("SET {} TO {} ({})", $display_name, value, mode_name));
            }
            Ok(())
        }
    };
}

pub(crate) use define_bool_param;
pub(crate) use define_float_param;
pub(crate) use define_int_param;
pub(crate) use define_int_param_ms;
pub(crate) use define_mode_param;
pub(crate) use define_mode_param_with_names;
