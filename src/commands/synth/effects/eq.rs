use crate::eval::eval_expression;
use crate::types::{Counters, EqState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

macro_rules! define_eq_param {
    ($fn_name:ident, $cmd:expr, $min:expr, $max:expr, $osc_name:expr, $field:ident, $output_msg:expr, $unit:expr, $parse_error:expr) => {
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
            eq_state: &mut EqState,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $cmd));
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
                    .context($parse_error)?
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
                output(format!("{}: RANGE {}-{} {}", $cmd, $min as i32, $max as i32, $unit));
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam($osc_name.to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            eq_state.$field = value;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {} {}", $output_msg, value, $unit));
            }
            Ok(())
        }
    };
}

define_eq_param!(handle_el, "EL", -24.0, 24.0, "el", low_db, "EQ LOW SHELF", "DB", "Failed to parse EQ low shelf dB");
define_eq_param!(handle_elf, "ELF", 20.0, 2000.0, "elf", low_freq, "EQ LOW SHELF FREQ", "HZ", "Failed to parse EQ low shelf frequency");
define_eq_param!(handle_em, "EM", -24.0, 24.0, "em", mid_db, "EQ MID PEAK", "DB", "Failed to parse EQ mid peak dB");
define_eq_param!(handle_ef, "EF", 200.0, 8000.0, "ef", mid_freq, "EQ MID FREQUENCY", "HZ", "Failed to parse EQ mid frequency");
define_eq_param!(handle_eq_param, "EQ", 0.1, 10.0, "eq", mid_q, "EQ MID Q", "", "Failed to parse EQ mid Q");
define_eq_param!(handle_eh, "EH", -24.0, 24.0, "eh", high_db, "EQ HIGH SHELF", "DB", "Failed to parse EQ high shelf dB");
define_eq_param!(handle_ehf, "EHF", 1000.0, 20000.0, "ehf", high_freq, "EQ HIGH SHELF FREQ", "HZ", "Failed to parse EQ high shelf frequency");
