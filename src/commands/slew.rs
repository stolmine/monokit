use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use std::sync::mpsc::Sender;

pub fn handle_slew_all<F>(
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
        output("ERROR: SLEW.ALL REQUIRES A TIME VALUE (0-10000 MS)".to_string());
        return Ok(());
    }
    let value_ms: f32 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as f32
    } else {
        parts[1]
            .parse()
            .context("Failed to parse slew time value")?
    };
    if !(0.0..=10000.0).contains(&value_ms) {
        output("ERROR: SLEW TIME MUST BE BETWEEN 0 AND 10000 MS".to_string());
        return Ok(());
    }
    let time_sec = value_ms / 1000.0;
    metro_tx
        .send(MetroCommand::SetSlewTime(time_sec))
        .context("Failed to send slew time to metro thread")?;
    if debug_level >= 1 {
        output(format!("SET SLEW TIME TO {} MS", value_ms));
    }
    Ok(())
}
