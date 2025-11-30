use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};

pub fn handle_pn_l<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PN.L REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &patterns.patterns[pat];
        output(format!("PN.L {} = {}", pat, pattern.length));
    } else {
        let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
            expr_val as usize
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern length")?
        };
        if value < 1 || value > 64 {
            output("ERROR: PATTERN LENGTH MUST BE 1-64".to_string());
            return Ok(());
        }
        let pattern = &mut patterns.patterns[pat];
        pattern.length = value;
        output(format!("SET PATTERN {} LENGTH TO {}", pat, value));
    }
    Ok(())
}

pub fn handle_pn_i<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PN.I REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &patterns.patterns[pat];
        output(format!("PN.I {} = {}", pat, pattern.index));
    } else {
        let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
            expr_val as usize
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern index")?
        };
        if value > 63 {
            output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
            return Ok(());
        }
        let pattern = &mut patterns.patterns[pat];
        pattern.index = value;
        output(format!("SET PATTERN {} INDEX TO {}", pat, value));
    }
    Ok(())
}

pub use super::working::{
    handle_pn_here,
    handle_pn_next,
    handle_pn_prev,
};

pub fn handle_pn<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PN REQUIRES PATTERN (0-5) AND INDEX (0-63)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[2]
            .parse()
            .context("Failed to parse pattern index")?
    };
    if idx > 63 {
        output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
        return Ok(());
    }
    if parts.len() == 3 {
        let pattern = &patterns.patterns[pat];
        output(format!("PN {} {} = {}", pat, idx, pattern.data[idx]));
    } else {
        let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index, scale) {
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
    Ok(())
}
