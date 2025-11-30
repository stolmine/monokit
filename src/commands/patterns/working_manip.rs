use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;

pub fn handle_pattern_push<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: P.PUSH REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse push value")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    for i in 0..pattern.length - 1 {
        pattern.data[i] = pattern.data[i + 1];
    }
    pattern.data[pattern.length - 1] = val;
    output(format!("PUSHED {} TO PATTERN {}", val, patterns.working));
    Ok(())
}

pub fn handle_pattern_pop<F>(
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let val = pattern.data[pattern.length - 1];
    output(format!("P.POP = {}", val));
}

pub fn handle_pattern_ins<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: P.INS REQUIRES INDEX AND VALUE".to_string());
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse insert index")?
    };
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse insert value")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    if idx >= pattern.length {
        output(format!("ERROR: INDEX {} OUT OF RANGE (LENGTH {})", idx, pattern.length));
        return Ok(());
    }
    for i in (idx..pattern.length - 1).rev() {
        pattern.data[i + 1] = pattern.data[i];
    }
    pattern.data[idx] = val;
    output(format!("INSERTED {} AT INDEX {} IN PATTERN {}", val, idx, patterns.working));
    Ok(())
}

pub fn handle_pattern_rm<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: P.RM REQUIRES AN INDEX".to_string());
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse remove index")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    if idx >= pattern.length {
        output(format!("ERROR: INDEX {} OUT OF RANGE (LENGTH {})", idx, pattern.length));
        return Ok(());
    }
    let removed = pattern.data[idx];
    for i in idx..pattern.length - 1 {
        pattern.data[i] = pattern.data[i + 1];
    }
    pattern.data[pattern.length - 1] = 0;
    output(format!("REMOVED {} FROM INDEX {} IN PATTERN {}", removed, idx, patterns.working));
    Ok(())
}

pub fn handle_pattern_rev<F>(
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    let len = pattern.length;
    for i in 0..len / 2 {
        pattern.data.swap(i, len - 1 - i);
    }
    output(format!("REVERSED PATTERN {}", patterns.working));
}

pub fn handle_pattern_rot<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: P.ROT REQUIRES A ROTATION AMOUNT".to_string());
        return Ok(());
    }
    let n: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse rotation amount")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    let len = pattern.length as i16;
    if len == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let n = ((n % len) + len) % len;
    if n == 0 {
        output(format!("PATTERN {} UNCHANGED (ROTATION 0)", patterns.working));
        return Ok(());
    }
    let mut temp = [0i16; 64];
    for i in 0..pattern.length {
        temp[i] = pattern.data[i];
    }
    for i in 0..pattern.length {
        pattern.data[i] = temp[(i + pattern.length - n as usize) % pattern.length];
    }
    output(format!("ROTATED PATTERN {} BY {}", patterns.working, n));
    Ok(())
}

pub fn handle_pattern_shuf<F>(
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    let len = pattern.length;
    let mut rng = rand::thread_rng();
    pattern.data[..len].shuffle(&mut rng);
    output(format!("SHUFFLED PATTERN {}", patterns.working));
}

pub fn handle_pattern_sort<F>(
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    let len = pattern.length;
    pattern.data[..len].sort();
    output(format!("SORTED PATTERN {}", patterns.working));
}

pub fn handle_pattern_rnd<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let (min, max) = if parts.len() >= 3 {
        let min_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
            expr_val
        } else {
            parts[1]
                .parse()
                .context("Failed to parse min value")?
        };
        let max_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse max value")?
        };
        (min_val, max_val)
    } else {
        (0, 127)
    };
    let pattern = &mut patterns.patterns[patterns.working];
    let mut rng = rand::thread_rng();
    for i in 0..pattern.length {
        pattern.data[i] = rng.gen_range(min..=max);
    }
    output(format!("RANDOMIZED PATTERN {} (RANGE {} TO {})", patterns.working, min, max));
    Ok(())
}
