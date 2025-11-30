use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};

pub fn eval_math_expression(
    expr: &str,
    parts: &[&str],
    start_idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    eval_expr_fn: &dyn Fn(&[&str], usize, &Variables, &mut PatternStorage, &mut Counters, &ScriptStorage, usize, &ScaleState) -> Option<(i16, usize)>,
) -> Option<(i16, usize)> {
    match expr {
        "ADD" | "+" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    let result = a.saturating_add(b);
                    return Some((result, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "SUB" | "-" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    let result = a.saturating_sub(b);
                    return Some((result, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "MUL" | "*" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    let result = a.saturating_mul(b);
                    return Some((result, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "DIV" | "/" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    if b == 0 {
                        return Some((0, 1 + a_consumed + b_consumed));
                    } else {
                        let result = a / b;
                        return Some((result, 1 + a_consumed + b_consumed));
                    }
                }
            }
            None
        }
        "MOD" | "%" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    if b == 0 {
                        return Some((0, 1 + a_consumed + b_consumed));
                    } else {
                        let result = a % b;
                        return Some((result, 1 + a_consumed + b_consumed));
                    }
                }
            }
            None
        }
        "Q" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((note, consumed)) = eval_expr_fn(
                parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale
            ) {
                let quantized = super::quantize_note(note, scale);
                return Some((quantized, 1 + consumed));
            }
            None
        }
        "N" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((step, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let divisions = scale.divisions as f32;
                const C3_HZ: f32 = 130.8128;
                let freq = C3_HZ * 2f32.powf(step as f32 / divisions);
                let freq_clamped = freq.round().clamp(1.0, 32767.0) as i16;
                return Some((freq_clamped, 1 + consumed));
            }
            None
        }
        _ => None,
    }
}
