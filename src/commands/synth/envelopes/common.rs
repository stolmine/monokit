use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

macro_rules! define_int_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_prefix:expr, $output_desc:expr, $unit:expr) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            metro_tx: &Sender<MetroCommand>,
            debug_level: u8,
            out_err: bool,
            scale: &ScaleState,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= crate::types::TIER_ERRORS || out_err {
                    output(format!(
                        "ERROR: {} REQUIRES A {} VALUE ({}-{} {})",
                        $error_prefix,
                        if $unit.is_empty() { "VALUE" } else { "TIME" },
                        $min,
                        $max,
                        $unit
                    ));
                }
                return Ok(());
            }
            let value: i32 = if let Some((expr_val, _)) = eval_expression(
                &parts,
                1,
                variables,
                patterns,
                counters,
                scripts,
                script_index,
                scale,
            ) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context(format!("Failed to parse {}", $output_desc.to_lowercase()))?
            };
            if !($min..=$max).contains(&value) {
                if debug_level >= crate::types::TIER_ERRORS || out_err {
                    output(format!(
                        "ERROR: {} MUST BE BETWEEN {} AND {} {}",
                        $output_desc, $min, $max, $unit
                    ));
                }
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam(
                    $osc_param.to_string(),
                    OscType::Int(value),
                ))
                .context("Failed to send param to metro thread")?;
            if debug_level >= 2 {
                output(format!(
                    "SET {} TO {} {}",
                    $output_desc,
                    value,
                    $unit
                ));
            }
            Ok(())
        }
    };
}

macro_rules! define_float_param {
    ($fn_name:ident, $osc_param:expr, $min:expr, $max:expr, $error_prefix:expr, $output_desc:expr, $unit:expr) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            metro_tx: &Sender<MetroCommand>,
            debug_level: u8,
            out_err: bool,
            scale: &ScaleState,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= crate::types::TIER_ERRORS || out_err {
                    output(format!(
                        "ERROR: {} REQUIRES A VALUE ({} TO {})",
                        $error_prefix, $min, $max
                    ));
                }
                return Ok(());
            }
            let value: f32 = if let Some((expr_val, _)) = eval_expression(
                &parts,
                1,
                variables,
                patterns,
                counters,
                scripts,
                script_index,
                scale,
            ) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context(format!("Failed to parse {}", $output_desc.to_lowercase()))?
            };
            if !($min..=$max).contains(&value) {
                if debug_level >= crate::types::TIER_ERRORS || out_err {
                    output(format!(
                        "ERROR: {} MUST BE BETWEEN {} AND {}",
                        $output_desc, $min, $max
                    ));
                }
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam(
                    $osc_param.to_string(),
                    OscType::Float(value),
                ))
                .context("Failed to send param to metro thread")?;
            if debug_level >= 2 {
                if $unit.is_empty() {
                    output(format!("SET {} TO {}", $output_desc, value));
                } else {
                    output(format!("SET {} TO {} {}", $output_desc, value, $unit));
                }
            }
            Ok(())
        }
    };
}

pub(crate) use define_float_param;
pub(crate) use define_int_param;
