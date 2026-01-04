mod patterns;
mod math;
mod logic;
pub mod rhythm;
pub mod seq;

use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};
use rand::Rng;
use std::sync::atomic::{AtomicU16, Ordering};

pub static KIT_SLOTS: AtomicU16 = AtomicU16::new(0);

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

    if expr == "KL" || expr == "KIT.LEN" {
        return Some((KIT_SLOTS.load(Ordering::Relaxed) as i16, 1));
    }

    if let Some(result) = patterns::eval_pattern_expression(
        &expr, parts, start_idx, variables, patterns, counters, scripts, script_index, scale, &eval_expression
    ) {
        return Some(result);
    }

    if let Some(result) = math::eval_math_expression(
        &expr, parts, start_idx, variables, patterns, counters, scripts, script_index, scale, &eval_expression
    ) {
        return Some(result);
    }

    if let Some(result) = logic::eval_logic_expression(
        &expr, parts, start_idx, variables, patterns, counters, scripts, script_index, scale, &eval_expression
    ) {
        return Some(result);
    }

    if let Some(result) = logic::eval_counter_expression(&expr, counters) {
        return Some(result);
    }

    if let Some(result) = seq::eval_seq_expression(&expr, parts, start_idx, patterns, script_index) {
        return Some(result);
    }

    match expr.as_str() {
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

    let parts: Vec<&str> = cond.split_whitespace().collect();
    if let Some((value, _)) = eval_expression(&parts, 0, variables, patterns, counters, scripts, script_index, scale) {
        return value != 0;
    }
    false
}

pub fn quantize_note(note: i16, scale: &ScaleState) -> i16 {
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
