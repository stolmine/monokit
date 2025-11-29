use crate::eval::eval_expression;
use crate::types::{PatternStorage, ScriptStorage, Variables};
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
        let result = rand::thread_rng().gen_range(0..max);
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
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: EITH REQUIRES TWO VALUES".to_string());
        return;
    }
    if let Some((a, a_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        if let Some((b, _)) = eval_expression(&parts, 1 + a_consumed, variables, patterns, scripts, script_index) {
            let result = if rand::thread_rng().gen_bool(0.5) { a } else { b };
            output(format!("{}", result));
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND VALUE".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST VALUE".to_string());
    }
}
