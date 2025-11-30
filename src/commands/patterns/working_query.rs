use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};

pub fn handle_pattern_min<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let min_val = pattern.data[..pattern.length].iter().copied().min().unwrap_or(0);
    output(format!("P.MIN = {}", min_val));
}

pub fn handle_pattern_max<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let max_val = pattern.data[..pattern.length].iter().copied().max().unwrap_or(0);
    output(format!("P.MAX = {}", max_val));
}

pub fn handle_pattern_sum<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    output(format!("P.SUM = {}", sum));
}

pub fn handle_pattern_avg<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    let avg = sum / pattern.length as i32;
    output(format!("P.AVG = {}", avg));
}

pub fn handle_pattern_fnd<F>(
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
    if parts.len() < 2 {
        output("ERROR: P.FND REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse find value")?
    };
    let pattern = &patterns.patterns[patterns.working];
    let index = pattern.data[..pattern.length]
        .iter()
        .position(|&x| x == val)
        .map(|i| i as i16)
        .unwrap_or(-1);
    output(format!("P.FND = {}", index));
    Ok(())
}
