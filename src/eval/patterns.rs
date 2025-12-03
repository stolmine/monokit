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
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &mut patterns.patterns[pat];
                pattern.index = (pattern.index + 1) % pattern.length;
                return Some((pattern.data[pattern.index], 1 + consumed));
            }
            None
        }
        "PN.PREV" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &mut patterns.patterns[pat];
                if pattern.index == 0 {
                    pattern.index = pattern.length - 1;
                } else {
                    pattern.index -= 1;
                }
                return Some((pattern.data[pattern.index], 1 + consumed));
            }
            None
        }
        "PN.HERE" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                return Some((pattern.data[pattern.index], 1 + consumed));
            }
            None
        }
        "PN" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                return Some((pattern.data[pattern.index], 1 + consumed));
            }
            None
        }
        "PN.L" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                return Some((pattern.length as i16, 1 + consumed));
            }
            None
        }
        "PN.I" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                return Some((pattern.index as i16, 1 + consumed));
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
        "P.N" => {
            Some((patterns.working as i16, 1))
        }
        "P.MIN" => {
            let pattern = &patterns.patterns[patterns.working];
            let slice = &pattern.data[..pattern.length];
            Some((*slice.iter().min().unwrap_or(&0), 1))
        }
        "P.MAX" => {
            let pattern = &patterns.patterns[patterns.working];
            let slice = &pattern.data[..pattern.length];
            Some((*slice.iter().max().unwrap_or(&0), 1))
        }
        "P.SUM" => {
            let pattern = &patterns.patterns[patterns.working];
            let sum: i16 = pattern.data[..pattern.length].iter().sum();
            Some((sum, 1))
        }
        "P.AVG" => {
            let pattern = &patterns.patterns[patterns.working];
            if pattern.length > 0 {
                let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
                Some(((sum / pattern.length as i32) as i16, 1))
            } else {
                Some((0, 1))
            }
        }
        "P.FND" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((search_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let pattern = &patterns.patterns[patterns.working];
                let slice = &pattern.data[..pattern.length];
                let result = slice.iter().position(|&x| x == search_val)
                    .map(|i| i as i16)
                    .unwrap_or(-1);
                return Some((result, 1 + consumed));
            }
            None
        }
        "PN.MIN" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                let slice = &pattern.data[..pattern.length];
                return Some((*slice.iter().min().unwrap_or(&0), 1 + consumed));
            }
            None
        }
        "PN.MAX" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                let slice = &pattern.data[..pattern.length];
                return Some((*slice.iter().max().unwrap_or(&0), 1 + consumed));
            }
            None
        }
        "PN.SUM" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                let sum: i16 = pattern.data[..pattern.length].iter().sum();
                return Some((sum, 1 + consumed));
            }
            None
        }
        "PN.AVG" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                if pattern.length > 0 {
                    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
                    return Some(((sum / pattern.length as i32) as i16, 1 + consumed));
                } else {
                    return Some((0, 1 + consumed));
                }
            }
            None
        }
        "PN.FND" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((search_val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let pattern = &patterns.patterns[pat];
                    let slice = &pattern.data[..pattern.length];
                    let result = slice.iter().position(|&x| x == search_val)
                        .map(|i| i as i16)
                        .unwrap_or(-1);
                    return Some((result, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        _ => None,
    }
}
