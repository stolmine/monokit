use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;

pub fn handle_pn_push<F>(
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
        output("ERROR: PN.PUSH REQUIRES PATTERN NUMBER AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse push value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length - 1 {
        pattern.data[i] = pattern.data[i + 1];
    }
    pattern.data[pattern.length - 1] = val;
    output(format!("PUSHED {} TO PATTERN {}", val, pat));
    Ok(())
}

pub fn handle_pn_pop<F>(
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
        output("ERROR: PN.POP REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let pattern = &patterns.patterns[pat];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let val = pattern.data[pattern.length - 1];
    output(format!("PN.POP {} = {}", pat, val));
    Ok(())
}

pub fn handle_pn_ins<F>(
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
    if parts.len() < 4 {
        output("ERROR: PN.INS REQUIRES PATTERN NUMBER, INDEX, AND VALUE".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[2]
            .parse()
            .context("Failed to parse insert index")?
    };
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[3]
            .parse()
            .context("Failed to parse insert value")?
    };
    let pattern = &mut patterns.patterns[pat];
    if idx >= pattern.length {
        output(format!("ERROR: INDEX {} OUT OF RANGE (LENGTH {})", idx, pattern.length));
        return Ok(());
    }
    for i in (idx..pattern.length - 1).rev() {
        pattern.data[i + 1] = pattern.data[i];
    }
    pattern.data[idx] = val;
    output(format!("INSERTED {} AT INDEX {} IN PATTERN {}", val, idx, pat));
    Ok(())
}

pub fn handle_pn_rm<F>(
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
        output("ERROR: PN.RM REQUIRES PATTERN NUMBER AND INDEX".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[2]
            .parse()
            .context("Failed to parse remove index")?
    };
    let pattern = &mut patterns.patterns[pat];
    if idx >= pattern.length {
        output(format!("ERROR: INDEX {} OUT OF RANGE (LENGTH {})", idx, pattern.length));
        return Ok(());
    }
    let removed = pattern.data[idx];
    for i in idx..pattern.length - 1 {
        pattern.data[i] = pattern.data[i + 1];
    }
    pattern.data[pattern.length - 1] = 0;
    output(format!("REMOVED {} FROM INDEX {} IN PATTERN {}", removed, idx, pat));
    Ok(())
}

pub fn handle_pn_rev<F>(
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
        output("ERROR: PN.REV REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    let len = pattern.length;
    for i in 0..len / 2 {
        pattern.data.swap(i, len - 1 - i);
    }
    output(format!("REVERSED PATTERN {}", pat));
    Ok(())
}

pub fn handle_pn_rot<F>(
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
        output("ERROR: PN.ROT REQUIRES PATTERN NUMBER AND ROTATION AMOUNT".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let n: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse rotation amount")?
    };
    let pattern = &mut patterns.patterns[pat];
    let len = pattern.length as i16;
    if len == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
    }
    let n = ((n % len) + len) % len;
    if n == 0 {
        output(format!("PATTERN {} UNCHANGED (ROTATION 0)", pat));
        return Ok(());
    }
    let mut temp = [0i16; 64];
    for i in 0..pattern.length {
        temp[i] = pattern.data[i];
    }
    for i in 0..pattern.length {
        pattern.data[i] = temp[(i + pattern.length - n as usize) % pattern.length];
    }
    output(format!("ROTATED PATTERN {} BY {}", pat, n));
    Ok(())
}

pub fn handle_pn_shuf<F>(
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
        output("ERROR: PN.SHUF REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    let len = pattern.length;
    let mut rng = rand::thread_rng();
    pattern.data[..len].shuffle(&mut rng);
    output(format!("SHUFFLED PATTERN {}", pat));
    Ok(())
}

pub fn handle_pn_sort<F>(
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
        output("ERROR: PN.SORT REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    let len = pattern.length;
    pattern.data[..len].sort();
    output(format!("SORTED PATTERN {}", pat));
    Ok(())
}

pub fn handle_pn_rnd<F>(
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
        output("ERROR: PN.RND REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let (min, max) = if parts.len() >= 4 {
        let min_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse min value")?
        };
        let max_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index) {
            expr_val
        } else {
            parts[3]
                .parse()
                .context("Failed to parse max value")?
        };
        (min_val, max_val)
    } else {
        (0, 127)
    };
    let pattern = &mut patterns.patterns[pat];
    let mut rng = rand::thread_rng();
    for i in 0..pattern.length {
        pattern.data[i] = rng.gen_range(min..=max);
    }
    output(format!("RANDOMIZED PATTERN {} (RANGE {} TO {})", pat, min, max));
    Ok(())
}
