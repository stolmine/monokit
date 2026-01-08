use crate::commands::context::ExecutionContext;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

macro_rules! define_sampler_envelope_param {
    ($fn_name:ident, $osc_param:expr, $state_field:ident, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
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
            if value < 0 || value > 16383 {
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
                output(format!("{}: RANGE 0-16383", $error_cmd));
                return Ok(());
            }

            sampler_state.playback.$state_field = value as i16;

            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

macro_rules! define_sampler_playback_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $state_field:ident, i16, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
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
                output(format!("{}: RANGE {}-{}", $error_cmd, $min, $max));
                return Ok(());
            }

            sampler_state.playback.$state_field = value as i16;

            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $state_field:ident, bool, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
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
                output(format!("{}: RANGE {}-{}", $error_cmd, $min, $max));
                return Ok(());
            }

            sampler_state.playback.$state_field = value != 0;

            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $state_field:ident, u8, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
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
                output(format!("{}: RANGE {}-{}", $error_cmd, $min, $max));
                return Ok(());
            }

            sampler_state.playback.$state_field = value as u8;

            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

macro_rules! define_sampler_fx_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $state_field:ident, i16, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
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
                output(format!("{}: RANGE {}-{}", $error_cmd, $min, $max));
                return Ok(());
            }

            sampler_state.fx.$state_field = value as i16;

            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $state_field:ident, u8, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
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
                output(format!("{}: RANGE {}-{}", $error_cmd, $min, $max));
                return Ok(());
            }

            sampler_state.fx.$state_field = value as u8;

            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

pub(crate) use define_sampler_envelope_param;
pub(crate) use define_sampler_fx_param;
pub(crate) use define_sampler_playback_param;
