use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};

pub fn handle_pn_min<F>(
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
        output("ERROR: PN.MIN REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let pattern = &patterns.patterns[pat];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let min_val = pattern.data[..pattern.length].iter().copied().min().unwrap_or(0);
    output(format!("PN.MIN {} = {}", pat, min_val));
    Ok(())
}

pub fn handle_pn_max<F>(
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
        output("ERROR: PN.MAX REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let pattern = &patterns.patterns[pat];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let max_val = pattern.data[..pattern.length].iter().copied().max().unwrap_or(0);
    output(format!("PN.MAX {} = {}", pat, max_val));
    Ok(())
}

pub fn handle_pn_sum<F>(
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
        output("ERROR: PN.SUM REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let pattern = &patterns.patterns[pat];
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    output(format!("PN.SUM {} = {}", pat, sum));
    Ok(())
}

pub fn handle_pn_avg<F>(
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
        output("ERROR: PN.AVG REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let pattern = &patterns.patterns[pat];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    let avg = sum / pattern.length as i32;
    output(format!("PN.AVG {} = {}", pat, avg));
    Ok(())
}

pub fn handle_pn_fnd<F>(
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
        output("ERROR: PN.FND REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse find value")?
    };
    let pattern = &patterns.patterns[pat];
    let index = pattern.data[..pattern.length]
        .iter()
        .position(|&x| x == val)
        .map(|i| i as i16)
        .unwrap_or(-1);
    output(format!("PN.FND {} = {}", pat, index));
    Ok(())
}
