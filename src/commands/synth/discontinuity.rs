use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::io::Write;
use std::sync::mpsc::Sender;
use super::common::{define_int_param_ms, define_mode_param};

pub fn handle_dc<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: DC REQUIRES A VALUE (0-16383)".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse discontinuity amount")?
    };
    if !(0..=16383).contains(&value) {
        output("ERROR: DC MUST BE 0-16383".to_string());
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

define_mode_param!(handle_dm, "dm", 0, 6, "DM", "MODE MUST BE 0-6 (FOLD/TANH/SOFT/HARD/ASYM/RECT/CRUSH)", "DISCONTINUITY MODE", "Failed to parse discontinuity mode");
define_int_param_ms!(handle_dd, "dd", 1, 10000, "DD", "DISCONTINUITY DECAY", "Failed to parse discontinuity decay time");
