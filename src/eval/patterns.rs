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
            let debug_path = std::env::temp_dir().join("monokit_debug.txt");
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open(&debug_path) {
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
        "P.PUSH" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                if pattern.length == 0 {
                    return None;
                }
                for i in 0..pattern.length - 1 {
                    pattern.data[i] = pattern.data[i + 1];
                }
                pattern.data[pattern.length - 1] = val;
                return Some((val, 1 + consumed));
            }
            None
        }
        "P.POP" => {
            let working = patterns.working;
            let pattern = &patterns.patterns[working];
            if pattern.length == 0 {
                return None;
            }
            let val = pattern.data[pattern.length - 1];
            Some((val, 1))
        }
        "P.REV" => {
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            let len = pattern.length;
            for i in 0..len / 2 {
                pattern.data.swap(i, len - 1 - i);
            }
            Some((len as i16, 1))
        }
        "P.SHUF" => {
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            if pattern.length == 0 {
                return None;
            }
            let len = pattern.length;
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            pattern.data[..len].shuffle(&mut rng);
            Some((len as i16, 1))
        }
        "P.SORT" => {
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            let len = pattern.length;
            pattern.data[..len].sort();
            Some((len as i16, 1))
        }
        "P.ROT" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((n, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                let len = pattern.length as i16;
                if len == 0 {
                    return None;
                }
                let n_normalized = ((n % len) + len) % len;
                if n_normalized != 0 {
                    let n_usize = n_normalized as usize;
                    let mut temp = [0i16; 64];
                    for i in 0..pattern.length {
                        temp[i] = pattern.data[i];
                    }
                    for i in 0..pattern.length {
                        pattern.data[i] = temp[(i + pattern.length - n_usize) % pattern.length];
                    }
                }
                return Some((n, 1 + consumed));
            }
            None
        }
        "P.ADD" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                for i in 0..pattern.length {
                    pattern.data[i] = pattern.data[i].saturating_add(val);
                }
                return Some((val, 1 + consumed));
            }
            None
        }
        "P.SUB" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                for i in 0..pattern.length {
                    pattern.data[i] = pattern.data[i].saturating_sub(val);
                }
                return Some((val, 1 + consumed));
            }
            None
        }
        "P.MUL" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                for i in 0..pattern.length {
                    pattern.data[i] = pattern.data[i].saturating_mul(val);
                }
                return Some((val, 1 + consumed));
            }
            None
        }
        "P.DIV" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if val == 0 {
                    return None;
                }
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                for i in 0..pattern.length {
                    pattern.data[i] = pattern.data[i] / val;
                }
                return Some((val, 1 + consumed));
            }
            None
        }
        "P.MOD" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if val == 0 {
                    return None;
                }
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                for i in 0..pattern.length {
                    pattern.data[i] = pattern.data[i] % val;
                }
                return Some((val, 1 + consumed));
            }
            None
        }
        "P.INS" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((idx_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if idx_val < 0 {
                    return None;
                }
                let idx = idx_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let working = patterns.working;
                    let pattern = &mut patterns.patterns[working];
                    if idx >= pattern.length {
                        return None;
                    }
                    for i in (idx..pattern.length - 1).rev() {
                        pattern.data[i + 1] = pattern.data[i];
                    }
                    pattern.data[idx] = val;
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "P.RM" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((idx_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if idx_val < 0 {
                    return None;
                }
                let idx = idx_val as usize;
                let working = patterns.working;
                let pattern = &mut patterns.patterns[working];
                if idx >= pattern.length {
                    return None;
                }
                let removed = pattern.data[idx];
                for i in idx..pattern.length - 1 {
                    pattern.data[i] = pattern.data[i + 1];
                }
                pattern.data[pattern.length - 1] = 0;
                return Some((removed, 1 + consumed));
            }
            None
        }
        "P.RND" => {
            let (min, max) = if start_idx + 2 < parts.len() {
                if let Some((min_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                    if let Some((max_val, _consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                        (min_val, max_val)
                    } else {
                        (0, 127)
                    }
                } else {
                    (0, 127)
                }
            } else {
                (0, 127)
            };
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            use rand::Rng;
            let mut rng = rand::thread_rng();
            for i in 0..pattern.length {
                pattern.data[i] = rng.gen_range(min..=max);
            }
            let len = pattern.length;
            Some((len as i16, 1))
        }
        "P.SCALE" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((new_min, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((new_max, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    if new_min == new_max {
                        return None;
                    }
                    let working = patterns.working;
                    let pattern = &mut patterns.patterns[working];
                    if pattern.length == 0 {
                        return None;
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
                    let len = pattern.length;
                    return Some((len as i16, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "P.CLR" => {
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            for i in 0..pattern.length {
                pattern.data[i] = 0;
            }
            Some((0, 1))
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
        "PN.PUSH" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let pattern = &mut patterns.patterns[pat];
                    if pattern.length == 0 {
                        return None;
                    }
                    for i in 0..pattern.length - 1 {
                        pattern.data[i] = pattern.data[i + 1];
                    }
                    pattern.data[pattern.length - 1] = val;
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.POP" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &patterns.patterns[pat];
                if pattern.length == 0 {
                    return None;
                }
                let val = pattern.data[pattern.length - 1];
                return Some((val, 1 + consumed));
            }
            None
        }
        "PN.REV" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &mut patterns.patterns[pat];
                let len = pattern.length;
                for i in 0..len / 2 {
                    pattern.data.swap(i, len - 1 - i);
                }
                return Some((len as i16, 1 + consumed));
            }
            None
        }
        "PN.SHUF" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &mut patterns.patterns[pat];
                if pattern.length == 0 {
                    return None;
                }
                let len = pattern.length;
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                pattern.data[..len].shuffle(&mut rng);
                return Some((len as i16, 1 + consumed));
            }
            None
        }
        "PN.SORT" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &mut patterns.patterns[pat];
                let len = pattern.length;
                pattern.data[..len].sort();
                return Some((len as i16, 1 + consumed));
            }
            None
        }
        "PN.ROT" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((n, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let pattern = &mut patterns.patterns[pat];
                    let len = pattern.length as i16;
                    if len == 0 {
                        return None;
                    }
                    let n_normalized = ((n % len) + len) % len;
                    if n_normalized != 0 {
                        let n_usize = n_normalized as usize;
                        let mut temp = [0i16; 64];
                        for i in 0..pattern.length {
                            temp[i] = pattern.data[i];
                        }
                        for i in 0..pattern.length {
                            pattern.data[i] = temp[(i + pattern.length - n_usize) % pattern.length];
                        }
                    }
                    return Some((n, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.ADD" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let pattern = &mut patterns.patterns[pat];
                    for i in 0..pattern.length {
                        pattern.data[i] = pattern.data[i].saturating_add(val);
                    }
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.SUB" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let pattern = &mut patterns.patterns[pat];
                    for i in 0..pattern.length {
                        pattern.data[i] = pattern.data[i].saturating_sub(val);
                    }
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.MUL" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    let pattern = &mut patterns.patterns[pat];
                    for i in 0..pattern.length {
                        pattern.data[i] = pattern.data[i].saturating_mul(val);
                    }
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.DIV" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    if val == 0 {
                        return None;
                    }
                    let pattern = &mut patterns.patterns[pat];
                    for i in 0..pattern.length {
                        pattern.data[i] = pattern.data[i] / val;
                    }
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.MOD" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    if val == 0 {
                        return None;
                    }
                    let pattern = &mut patterns.patterns[pat];
                    for i in 0..pattern.length {
                        pattern.data[i] = pattern.data[i] % val;
                    }
                    return Some((val, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.INS" => {
            if start_idx + 3 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((idx_val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    if idx_val < 0 {
                        return None;
                    }
                    let idx = idx_val as usize;
                    if let Some((val, consumed3)) = eval_expr_fn(parts, start_idx + 1 + consumed1 + consumed2, variables, patterns, counters, scripts, script_index, scale) {
                        let pattern = &mut patterns.patterns[pat];
                        if idx >= pattern.length {
                            return None;
                        }
                        for i in (idx..pattern.length - 1).rev() {
                            pattern.data[i + 1] = pattern.data[i];
                        }
                        pattern.data[idx] = val;
                        return Some((val, 1 + consumed1 + consumed2 + consumed3));
                    }
                }
            }
            None
        }
        "PN.RM" => {
            if start_idx + 2 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((idx_val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    if idx_val < 0 {
                        return None;
                    }
                    let idx = idx_val as usize;
                    let pattern = &mut patterns.patterns[pat];
                    if idx >= pattern.length {
                        return None;
                    }
                    let removed = pattern.data[idx];
                    for i in idx..pattern.length - 1 {
                        pattern.data[i] = pattern.data[i + 1];
                    }
                    pattern.data[pattern.length - 1] = 0;
                    return Some((removed, 1 + consumed1 + consumed2));
                }
            }
            None
        }
        "PN.RND" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let (min, max, total_consumed) = if start_idx + 3 < parts.len() {
                    if let Some((min_val, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                        if let Some((max_val, consumed3)) = eval_expr_fn(parts, start_idx + 1 + consumed1 + consumed2, variables, patterns, counters, scripts, script_index, scale) {
                            (min_val, max_val, consumed1 + consumed2 + consumed3)
                        } else {
                            (0, 127, consumed1)
                        }
                    } else {
                        (0, 127, consumed1)
                    }
                } else {
                    (0, 127, consumed1)
                };
                let pattern = &mut patterns.patterns[pat];
                use rand::Rng;
                let mut rng = rand::thread_rng();
                for i in 0..pattern.length {
                    pattern.data[i] = rng.gen_range(min..=max);
                }
                let len = pattern.length;
                return Some((len as i16, 1 + total_consumed));
            }
            None
        }
        "PN.SCALE" => {
            if start_idx + 3 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed1)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                if let Some((new_min, consumed2)) = eval_expr_fn(parts, start_idx + 1 + consumed1, variables, patterns, counters, scripts, script_index, scale) {
                    if let Some((new_max, consumed3)) = eval_expr_fn(parts, start_idx + 1 + consumed1 + consumed2, variables, patterns, counters, scripts, script_index, scale) {
                        if new_min == new_max {
                            return None;
                        }
                        let pattern = &mut patterns.patterns[pat];
                        if pattern.length == 0 {
                            return None;
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
                        let len = pattern.length;
                        return Some((len as i16, 1 + consumed1 + consumed2 + consumed3));
                    }
                }
            }
            None
        }
        "PN.CLR" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expr_fn(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if pat_val < 0 || pat_val > 5 {
                    return None;
                }
                let pat = pat_val as usize;
                let pattern = &mut patterns.patterns[pat];
                for i in 0..pattern.length {
                    pattern.data[i] = 0;
                }
                return Some((0, 1 + consumed));
            }
            None
        }
        _ => None,
    }
}
