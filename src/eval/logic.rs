use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};
use rand::Rng;

pub fn eval_logic_expression(
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
        "RND" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((max, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if max <= 0 {
                    return Some((0, 1 + consumed));
                }
                let result = rand::thread_rng().gen_range(0..=max);
                return Some((result, 1 + consumed));
            }
            None
        }
        "RRND" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((mut min, min_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((mut max, max_consumed)) = eval_expr_fn(parts, start_idx + 1 + min_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    if min > max {
                        std::mem::swap(&mut min, &mut max);
                    }
                    let result = rand::thread_rng().gen_range(min..=max);
                    return Some((result, 1 + min_consumed + max_consumed));
                }
            }
            None
        }
        "TOSS" => {
            let result = if rand::thread_rng().gen_bool(0.5) { 1 } else { 0 };
            Some((result, 1))
        }
        "EITH" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if start_idx + 1 + a_consumed >= parts.len() {
                    return None;
                }
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    let result = if rand::thread_rng().gen_bool(0.5) { a } else { b };
                    return Some((result, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "TOG" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if start_idx + 1 + a_consumed >= parts.len() {
                    return None;
                }
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    let next_idx = start_idx + 1 + a_consumed + b_consumed;
                    let key = format!("{}_{}", script_index, parts[start_idx..next_idx].join("_"));
                    let counter = patterns.toggle_state.entry(key).or_insert(0);
                    let result = if *counter % 2 == 0 { a } else { b };
                    *counter = counter.wrapping_add(1);
                    return Some((result, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "MAP" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, val_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((in_min, in_min_consumed)) = eval_expr_fn(parts, start_idx + 1 + val_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    if let Some((in_max, in_max_consumed)) = eval_expr_fn(parts, start_idx + 1 + val_consumed + in_min_consumed, variables, patterns, counters, scripts, script_index, scale) {
                        if let Some((out_min, out_min_consumed)) = eval_expr_fn(parts, start_idx + 1 + val_consumed + in_min_consumed + in_max_consumed, variables, patterns, counters, scripts, script_index, scale) {
                            if let Some((out_max, _out_max_consumed)) = eval_expr_fn(parts, start_idx + 1 + val_consumed + in_min_consumed + in_max_consumed + out_min_consumed, variables, patterns, counters, scripts, script_index, scale) {
                                let result = if in_min == in_max {
                                    out_min
                                } else {
                                    let mapped = out_min as i32 + ((val as i32 - in_min as i32) * (out_max as i32 - out_min as i32)) / (in_max as i32 - in_min as i32);
                                    let clamped = if out_min <= out_max {
                                        mapped.clamp(out_min as i32, out_max as i32)
                                    } else {
                                        mapped.clamp(out_max as i32, out_min as i32)
                                    };
                                    clamped as i16
                                };
                                let total_consumed = 1 + val_consumed + in_min_consumed + in_max_consumed + out_min_consumed + _out_max_consumed;
                                return Some((result, total_consumed));
                            }
                        }
                    }
                }
            }
            None
        }
        "EZ" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                return Some((if val == 0 { 1 } else { 0 }, 1 + consumed));
            }
            None
        }
        "NZ" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                return Some((if val != 0 { 1 } else { 0 }, 1 + consumed));
            }
            None
        }
        "EQ" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a == b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "NE" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a != b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "GT" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a > b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "LT" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a < b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "GTE" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a >= b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "LTE" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expr_fn(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a <= b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        _ => None,
    }
}

pub fn eval_counter_expression(
    expr: &str,
    counters: &mut Counters,
) -> Option<(i16, usize)> {
    match expr {
        "N1" => {
            let current = counters.values[0];
            let min = counters.min[0];
            let max = counters.max[0];
            counters.values[0] = if max == 0 {
                current.wrapping_add(1)
            } else {
                let next = current + 1;
                if next > max { min } else { next }
            };
            Some((current, 1))
        }
        "N2" => {
            let current = counters.values[1];
            let min = counters.min[1];
            let max = counters.max[1];
            counters.values[1] = if max == 0 {
                current.wrapping_add(1)
            } else {
                let next = current + 1;
                if next > max { min } else { next }
            };
            Some((current, 1))
        }
        "N3" => {
            let current = counters.values[2];
            let min = counters.min[2];
            let max = counters.max[2];
            counters.values[2] = if max == 0 {
                current.wrapping_add(1)
            } else {
                let next = current + 1;
                if next > max { min } else { next }
            };
            Some((current, 1))
        }
        "N4" => {
            let current = counters.values[3];
            let min = counters.min[3];
            let max = counters.max[3];
            counters.values[3] = if max == 0 {
                current.wrapping_add(1)
            } else {
                let next = current + 1;
                if next > max { min } else { next }
            };
            Some((current, 1))
        }
        _ => None,
    }
}
