use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rand::Rng;

pub fn handle_rnd<F>(
    parts: &[&str],
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: RND REQUIRES A MAX VALUE".to_string());
        return Ok(());
    }
    let max: i16 = parts[1]
        .parse()
        .context("Failed to parse max value as number")?;
    if max <= 0 {
        output("0".to_string());
    } else {
        let result = rand::thread_rng().gen_range(0..=max);
        output(format!("{}", result));
    }
    Ok(())
}

pub fn handle_rrnd<F>(
    parts: &[&str],
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: RRND REQUIRES MIN AND MAX VALUES".to_string());
        return Ok(());
    }
    let mut min: i16 = parts[1]
        .parse()
        .context("Failed to parse min value as number")?;
    let mut max: i16 = parts[2]
        .parse()
        .context("Failed to parse max value as number")?;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    let result = rand::thread_rng().gen_range(min..=max);
    output(format!("{}", result));
    Ok(())
}

pub fn handle_toss<F>(
    mut output: F,
) where
    F: FnMut(String),
{
    let result = if rand::thread_rng().gen_bool(0.5) { 1 } else { 0 };
    output(format!("{}", result));
}

pub fn handle_eith<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: EITH REQUIRES TWO VALUES".to_string());
        return;
    }
    if let Some((a, a_consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if let Some((b, _b_consumed)) = eval_expression(&parts, 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
            let key = format!("cmd_{}_{}", script_index, parts.join("_"));
            let selected_index = if rand::thread_rng().gen_bool(0.5) { 0 } else { 1 };
            patterns.toggle_state.insert(key, selected_index);
            let result = if selected_index == 0 { a } else { b };
            output(format!("{}", result));
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND VALUE".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST VALUE".to_string());
    }
}

pub fn handle_tog<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: TOG REQUIRES AT LEAST TWO VALUES".to_string());
        return;
    }
    if let Some((a, a_consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if let Some((b, _b_consumed)) = eval_expression(&parts, 1 + a_consumed, variables, patterns, counters, scripts, script_index, scale) {
            let key = format!("cmd_{}_{}", script_index, parts.join("_"));
            let counter = patterns.toggle_state.entry(key).or_insert(0);
            let result = if *counter % 2 == 0 { a } else { b };
            *counter = counter.wrapping_add(1);
            output(format!("{}", result));
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND VALUE".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST VALUE".to_string());
    }
}
