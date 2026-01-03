use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_float_param, define_int_param, define_int_param_ms};

define_int_param!(handle_ct, "ct", 0, 16383, "CT", "COMPRESSOR THRESHOLD", "Failed to parse compressor threshold");
define_float_param!(handle_cr, "cr", 1.0, 20.0, "CR", "COMPRESSOR RATIO", "Failed to parse compressor ratio", "");
define_int_param_ms!(handle_ca, "ca", 1, 500, "CA", "COMPRESSOR ATTACK", "Failed to parse compressor attack");
define_int_param_ms!(handle_cl, "cl", 10, 2000, "CL", "COMPRESSOR RELEASE", "Failed to parse compressor release");
define_int_param!(handle_cm, "cm", 0, 16383, "CM", "COMPRESSOR MAKEUP GAIN", "Failed to parse compressor makeup gain");
define_int_param!(handle_cr_mix, "cr_mix", 0, 16383, "CR.MIX", "COMPRESSOR DRY/WET MIX", "Failed to parse compressor dry/wet mix");

pub fn handle_cr_auto<F>(
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
    if parts.len() == 1 {
        output("CR.AUTO: QUERY NOT SUPPORTED".to_string());
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
            .context("Failed to parse compressor auto-makeup")?
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
        output("CR.AUTO: RANGE 0-1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendParam("cr_auto".to_string(), OscType::Int(value)))
        .context("Failed to send param to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        let mode_str = if value == 0 { "MANUAL" } else { "AUTO" };
        output(format!("SET COMPRESSOR AUTO-MAKEUP TO {} ({})", value, mode_str));
    }
    Ok(())
}
