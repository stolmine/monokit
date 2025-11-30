use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};

pub fn parse_i16_expr<F>(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    param_name: &str,
    mut output: F,
) -> Option<i16>
where
    F: FnMut(String),
{
    if idx >= parts.len() {
        output(format!("ERROR: {} REQUIRES A VALUE", param_name));
        return None;
    }

    if let Some((expr_val, _)) = eval_expression(
        parts, idx, variables, patterns, counters, scripts, script_index, scale
    ) {
        Some(expr_val)
    } else {
        match parts[idx].parse() {
            Ok(v) => Some(v),
            Err(_) => {
                output(format!("ERROR: FAILED TO PARSE VALUE FOR {}", param_name));
                None
            }
        }
    }
}

pub fn parse_f32_expr<F>(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    param_name: &str,
    mut output: F,
) -> Option<f32>
where
    F: FnMut(String),
{
    if idx >= parts.len() {
        output(format!("ERROR: {} REQUIRES A VALUE", param_name));
        return None;
    }

    if let Some((expr_val, _)) = eval_expression(
        parts, idx, variables, patterns, counters, scripts, script_index, scale
    ) {
        Some(expr_val as f32)
    } else {
        match parts[idx].parse() {
            Ok(v) => Some(v),
            Err(_) => {
                output(format!("ERROR: FAILED TO PARSE VALUE FOR {}", param_name));
                None
            }
        }
    }
}
