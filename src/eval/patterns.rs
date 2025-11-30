use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};

pub fn eval_pattern_expression(
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
        "PN.NEXT" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pat = pat_val as usize;
                if pat <= 3 {
                    let pattern = &mut patterns.patterns[pat];
                    pattern.index = (pattern.index + 1) % pattern.length;
                    return Some((pattern.data[pattern.index], 1 + consumed));
                }
            }
            None
        }
        "PN.PREV" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pat = pat_val as usize;
                if pat <= 3 {
                    let pattern = &mut patterns.patterns[pat];
                    if pattern.index == 0 {
                        pattern.index = pattern.length - 1;
                    } else {
                        pattern.index -= 1;
                    }
                    return Some((pattern.data[pattern.index], 1 + consumed));
                }
            }
            None
        }
        "PN.HERE" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pat = pat_val as usize;
                if pat <= 3 {
                    let pattern = &patterns.patterns[pat];
                    return Some((pattern.data[pattern.index], 1 + consumed));
                }
            }
            None
        }
        "PN" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pat = pat_val as usize;
                if pat <= 3 {
                    let pattern = &patterns.patterns[pat];
                    return Some((pattern.data[pattern.index], 1 + consumed));
                }
            }
            None
        }
        "PN.L" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pat = pat_val as usize;
                if pat <= 3 {
                    let pattern = &patterns.patterns[pat];
                    return Some((pattern.length as i16, 1 + consumed));
                }
            }
            None
        }
        "PN.I" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pat = pat_val as usize;
                if pat <= 3 {
                    let pattern = &patterns.patterns[pat];
                    return Some((pattern.index as i16, 1 + consumed));
                }
            }
            None
        }
        "P.NEXT" => {
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            let old_index = pattern.index;
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/monokit_debug.txt") {
                writeln!(f, "P.NEXT: working={} old_idx={} new_idx={} len={} value={}",
                    working, old_index, pattern.index, pattern.length, value).ok();
            }
            Some((value, 1))
        }
        "P.PREV" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            Some((value, 1))
        }
        "P.HERE" => {
            let pattern = &patterns.patterns[patterns.working];
            Some((pattern.data[pattern.index], 1))
        }
        "P.L" => {
            let pattern = &patterns.patterns[patterns.working];
            Some((pattern.length as i16, 1))
        }
        "P.I" => {
            let pattern = &patterns.patterns[patterns.working];
            Some((pattern.index as i16, 1))
        }
        _ => None,
    }
}
