use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};
use rand::Rng;

pub fn resolve_value(s: &str, variables: &Variables, scripts: &ScriptStorage, script_index: usize) -> i16 {
    match s.trim().to_uppercase().as_str() {
        "A" => variables.a,
        "B" => variables.b,
        "C" => variables.c,
        "D" => variables.d,
        "I" => variables.i,
        "J" => {
            if script_index < 10 {
                scripts.scripts[script_index].j
            } else {
                0
            }
        }
        "K" => {
            if script_index < 10 {
                scripts.scripts[script_index].k
            } else {
                0
            }
        }
        "X" => variables.x,
        "Y" => variables.y,
        "Z" => variables.z,
        "T" => variables.t,
        _ => s.trim().parse::<i16>().unwrap_or(0),
    }
}

pub fn eval_expression(parts: &[&str], start_idx: usize, variables: &Variables, patterns: &mut PatternStorage, counters: &mut Counters, scripts: &ScriptStorage, script_index: usize, scale: &ScaleState) -> Option<(i16, usize)> {
    if start_idx >= parts.len() {
        return None;
    }

    let expr = parts[start_idx].trim().to_uppercase();

    match expr.as_str() {
        "PN.NEXT" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((pat_val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((pat_val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((pat_val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((pat_val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((pat_val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((pat_val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
        "RND" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((max, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((mut min, min_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((mut max, max_consumed)) = eval_expression(parts, start_idx + 1 + min_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
        "ADD" | "+" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
        "MAP" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, val_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((in_min, in_min_consumed)) = eval_expression(parts, start_idx + 1 + val_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    if let Some((in_max, in_max_consumed)) = eval_expression(parts, start_idx + 1 + val_consumed + in_min_consumed, variables, patterns, counters, scripts, script_index, scale) {
                        if let Some((out_min, out_min_consumed)) = eval_expression(parts, start_idx + 1 + val_consumed + in_min_consumed + in_max_consumed, variables, patterns, counters, scripts, script_index, scale) {
                            if let Some((out_max, _out_max_consumed)) = eval_expression(parts, start_idx + 1 + val_consumed + in_min_consumed + in_max_consumed + out_min_consumed, variables, patterns, counters, scripts, script_index, scale) {
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
        "Q" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((note, consumed)) = eval_expression(
                parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale
            ) {
                let quantized = quantize_note(note, scale);
                return Some((quantized, 1 + consumed));
            }
            None
        }
        "N" => {
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((step, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                let divisions = scale.divisions as f32;
                const C3_HZ: f32 = 130.8128;
                let freq = C3_HZ * 2f32.powf(step as f32 / divisions);
                let freq_clamped = freq.round().clamp(1.0, 32767.0) as i16;
                return Some((freq_clamped, 1 + consumed));
            }
            None
        }
        // Comparison operators (return 1 for true, 0 for false)
        "EZ" => {
            // Equals Zero
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                return Some((if val == 0 { 1 } else { 0 }, 1 + consumed));
            }
            None
        }
        "NZ" => {
            // Not Zero
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((val, consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                return Some((if val != 0 { 1 } else { 0 }, 1 + consumed));
            }
            None
        }
        "EQ" => {
            // Equals
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a == b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "NE" => {
            // Not Equals
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a != b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "GT" => {
            // Greater Than
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a > b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "LT" => {
            // Less Than
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a < b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "GTE" => {
            // Greater Than or Equal
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a >= b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
        "LTE" => {
            // Less Than or Equal
            if start_idx + 1 >= parts.len() {
                return None;
            }
            if let Some((a, a_consumed)) = eval_expression(parts, start_idx + 1, variables, patterns, counters, scripts, script_index, scale) {
                if let Some((b, b_consumed)) = eval_expression(parts, start_idx + 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
                    return Some((if a <= b { 1 } else { 0 }, 1 + a_consumed + b_consumed));
                }
            }
            None
        }
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
        "A" => Some((variables.a, 1)),
        "B" => Some((variables.b, 1)),
        "C" => Some((variables.c, 1)),
        "D" => Some((variables.d, 1)),
        "I" => Some((variables.i, 1)),
        "J" => {
            if script_index < 10 {
                Some((scripts.scripts[script_index].j, 1))
            } else {
                Some((0, 1))
            }
        }
        "K" => {
            if script_index < 10 {
                Some((scripts.scripts[script_index].k, 1))
            } else {
                Some((0, 1))
            }
        }
        "X" => Some((variables.x, 1)),
        "Y" => Some((variables.y, 1)),
        "Z" => Some((variables.z, 1)),
        "T" => Some((variables.t, 1)),
        _ => {
            if let Ok(val) = expr.parse::<i16>() {
                Some((val, 1))
            } else {
                None
            }
        }
    }
}

pub fn eval_condition(cond: &str, variables: &Variables, patterns: &mut PatternStorage, counters: &mut Counters, scripts: &ScriptStorage, script_index: usize, scale: &ScaleState) -> bool {
    let cond = cond.trim();

    if cond.starts_with("PROB ") {
        let pct_str = cond.strip_prefix("PROB ").unwrap_or("0").trim();
        let parts: Vec<&str> = pct_str.split_whitespace().collect();
        if let Some((pct_val, _)) = eval_expression(&parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            let pct = (pct_val as u8).min(100);
            let roll: u8 = rand::thread_rng().gen_range(0..100);
            return roll < pct;
        }
        return false;
    }

    // Strip IF prefix case-insensitively
    let cond = if cond.to_uppercase().starts_with("IF ") {
        &cond[3..]
    } else {
        cond
    };

    if let Some(pos) = cond.find(">=") {
        let left_parts: Vec<&str> = cond[..pos].split_whitespace().collect();
        let right_parts: Vec<&str> = cond[pos + 2..].split_whitespace().collect();
        if let Some((left, _)) = eval_expression(&left_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            if let Some((right, _)) = eval_expression(&right_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
                return left >= right;
            }
        }
        return false;
    }

    if let Some(pos) = cond.find("<=") {
        let left_parts: Vec<&str> = cond[..pos].split_whitespace().collect();
        let right_parts: Vec<&str> = cond[pos + 2..].split_whitespace().collect();
        if let Some((left, _)) = eval_expression(&left_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            if let Some((right, _)) = eval_expression(&right_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
                return left <= right;
            }
        }
        return false;
    }

    if let Some(pos) = cond.find("!=") {
        let left_parts: Vec<&str> = cond[..pos].split_whitespace().collect();
        let right_parts: Vec<&str> = cond[pos + 2..].split_whitespace().collect();
        if let Some((left, _)) = eval_expression(&left_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            if let Some((right, _)) = eval_expression(&right_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
                return left != right;
            }
        }
        return false;
    }

    if let Some(pos) = cond.find("==") {
        let left_parts: Vec<&str> = cond[..pos].split_whitespace().collect();
        let right_parts: Vec<&str> = cond[pos + 2..].split_whitespace().collect();
        if let Some((left, _)) = eval_expression(&left_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            if let Some((right, _)) = eval_expression(&right_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
                return left == right;
            }
        }
        return false;
    }

    if let Some(pos) = cond.find('>') {
        let left_parts: Vec<&str> = cond[..pos].split_whitespace().collect();
        let right_parts: Vec<&str> = cond[pos + 1..].split_whitespace().collect();
        if let Some((left, _)) = eval_expression(&left_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            if let Some((right, _)) = eval_expression(&right_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
                return left > right;
            }
        }
        return false;
    }

    if let Some(pos) = cond.find('<') {
        let left_parts: Vec<&str> = cond[..pos].split_whitespace().collect();
        let right_parts: Vec<&str> = cond[pos + 1..].split_whitespace().collect();
        if let Some((left, _)) = eval_expression(&left_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
            if let Some((right, _)) = eval_expression(&right_parts, 0, variables, patterns, counters, scripts, script_index, scale) {
                return left < right;
            }
        }
        return false;
    }

    // No comparison operator found - evaluate as truthy/falsy (non-zero = true)
    let parts: Vec<&str> = cond.split_whitespace().collect();
    if let Some((value, _)) = eval_expression(&parts, 0, variables, patterns, counters, scripts, script_index, scale) {
        return value != 0;
    }
    false
}

fn quantize_note(note: i16, scale: &ScaleState) -> i16 {
    let divisions = scale.divisions as i16;
    let root = scale.root as i16;

    if scale.mask.is_empty() || scale.mask.iter().all(|&x| !x) {
        return note;
    }

    let relative = (note - root).rem_euclid(divisions);
    let octave = (note - root).div_euclid(divisions);

    let active: Vec<i16> = scale.mask.iter()
        .enumerate()
        .filter(|(_, &a)| a)
        .map(|(i, _)| i as i16)
        .collect();

    if active.is_empty() { return note; }

    let best = active.iter()
        .min_by_key(|&&d| {
            let dist = (relative - d).abs().min(divisions - (relative - d).abs());
            (dist, d)
        })
        .copied()
        .unwrap_or(relative);

    root + octave * divisions + best
}
