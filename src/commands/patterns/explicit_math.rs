use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};

pub fn handle_pn_add<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PN.ADD REQUIRES PATTERN NUMBER AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse add value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_add(val);
    }
    output(format!("ADDED {} TO PATTERN {}", val, pat));
    Ok(())
}

pub fn handle_pn_sub<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PN.SUB REQUIRES PATTERN NUMBER AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse sub value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_sub(val);
    }
    output(format!("SUBTRACTED {} FROM PATTERN {}", val, pat));
    Ok(())
}

pub fn handle_pn_mul<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PN.MUL REQUIRES PATTERN NUMBER AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse mul value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_mul(val);
    }
    output(format!("MULTIPLIED PATTERN {} BY {}", pat, val));
    Ok(())
}

pub fn handle_pn_div<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PN.DIV REQUIRES PATTERN NUMBER AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse div value")?
    };
    if val == 0 {
        output("ERROR: DIVISION BY ZERO".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] / val;
    }
    output(format!("DIVIDED PATTERN {} BY {}", pat, val));
    Ok(())
}

pub fn handle_pn_mod<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PN.MOD REQUIRES PATTERN NUMBER AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse mod value")?
    };
    if val == 0 {
        output("ERROR: MODULO BY ZERO".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] % val;
    }
    output(format!("MODULO PATTERN {} BY {}", pat, val));
    Ok(())
}

pub fn handle_pn_scale<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 4 {
        output("ERROR: PN.SCALE REQUIRES PATTERN NUMBER, MIN, AND MAX VALUES".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
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
    let new_min: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse new min value")?
    };
    let new_max: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[3]
            .parse()
            .context("Failed to parse new max value")?
    };
    let pattern = &mut patterns.patterns[pat];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let old_min = pattern.data[..pattern.length].iter().copied().min().unwrap_or(0);
    let old_max = pattern.data[..pattern.length].iter().copied().max().unwrap_or(0);
    if old_min == old_max {
        for i in 0..pattern.length {
            pattern.data[i] = new_min;
        }
    } else {
        for i in 0..pattern.length {
            let old_val = pattern.data[i] as i32;
            let scaled = ((old_val - old_min as i32) * (new_max as i32 - new_min as i32)) / (old_max as i32 - old_min as i32) + new_min as i32;
            pattern.data[i] = scaled.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
    }
    output(format!("SCALED PATTERN {} TO RANGE {} TO {}", pat, new_min, new_max));
    Ok(())
}
