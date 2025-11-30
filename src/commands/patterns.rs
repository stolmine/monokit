use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;

pub fn handle_pattern_n<F>(
    parts: &[&str],
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("P.N = {}", patterns.working));
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE PATTERN NUMBER".to_string());
                return;
            }
        };
        if value > 5 {
            output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
            return;
        }
        patterns.working = value;
        output(format!("SET WORKING PATTERN TO {}", value));
    }
}

pub fn handle_pattern_l<F>(
    parts: &[&str],
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    if parts.len() == 1 {
        output(format!("P.L = {}", pattern.length));
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE PATTERN LENGTH".to_string());
                return;
            }
        };
        if value < 1 || value > 64 {
            output("ERROR: PATTERN LENGTH MUST BE 1-64".to_string());
            return;
        }
        pattern.length = value;
        output(format!("SET PATTERN {} LENGTH TO {}", patterns.working, value));
    }
}

pub fn handle_pattern_i<F>(
    parts: &[&str],
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    if parts.len() == 1 {
        output(format!("P.I = {}", pattern.index));
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE PATTERN INDEX".to_string());
                return;
            }
        };
        if value > 63 {
            output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
            return;
        }
        pattern.index = value;
        output(format!("SET PATTERN {} INDEX TO {}", patterns.working, value));
    }
}

pub fn handle_pattern_here<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    let value = pattern.data[pattern.index];
    output(format!("P.HERE = {}", value));
}

pub fn handle_pattern_next<F>(
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    pattern.index = (pattern.index + 1) % pattern.length;
    let value = pattern.data[pattern.index];
    output(format!("P.NEXT = {} (INDEX NOW {})", value, pattern.index));
}

pub fn handle_pattern_prev<F>(
    patterns: &mut PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    if pattern.index == 0 {
        pattern.index = pattern.length - 1;
    } else {
        pattern.index -= 1;
    }
    let value = pattern.data[pattern.index];
    output(format!("P.PREV = {} (INDEX NOW {})", value, pattern.index));
}

pub fn handle_pattern<F>(
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
    if parts.len() == 1 {
        output("ERROR: P REQUIRES AN INDEX".to_string());
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern index")?
    };
    if idx > 63 {
        output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &patterns.patterns[patterns.working];
        output(format!("P {} = {}", idx, pattern.data[idx]));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern value")?
        };
        let pattern = &mut patterns.patterns[patterns.working];
        pattern.data[idx] = value;
        output(format!("SET P {} TO {}", idx, value));
    }
    Ok(())
}

pub fn handle_pn_l<F>(
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
        output("ERROR: PN.L REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    if parts.len() == 2 {
        let pattern = &patterns.patterns[pat];
        output(format!("PN.L {} = {}", pat, pattern.length));
    } else {
        let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
            expr_val as usize
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern length")?
        };
        if value < 1 || value > 64 {
            output("ERROR: PATTERN LENGTH MUST BE 1-64".to_string());
            return Ok(());
        }
        let pattern = &mut patterns.patterns[pat];
        pattern.length = value;
        output(format!("SET PATTERN {} LENGTH TO {}", pat, value));
    }
    Ok(())
}

pub fn handle_pn_i<F>(
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
        output("ERROR: PN.I REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    if parts.len() == 2 {
        let pattern = &patterns.patterns[pat];
        output(format!("PN.I {} = {}", pat, pattern.index));
    } else {
        let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
            expr_val as usize
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern index")?
        };
        if value > 63 {
            output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
            return Ok(());
        }
        let pattern = &mut patterns.patterns[pat];
        pattern.index = value;
        output(format!("SET PATTERN {} INDEX TO {}", pat, value));
    }
    Ok(())
}

pub fn handle_pn_here<F>(
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
        output("ERROR: PN.HERE REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let value = pattern.data[pattern.index];
    output(format!("PN.HERE {} = {}", pat, value));
    Ok(())
}

pub fn handle_pn_next<F>(
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
        output("ERROR: PN.NEXT REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    pattern.index = (pattern.index + 1) % pattern.length;
    let value = pattern.data[pattern.index];
    output(format!("PN.NEXT {} = {} (INDEX NOW {})", pat, value, pattern.index));
    Ok(())
}

pub fn handle_pn_prev<F>(
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
        output("ERROR: PN.PREV REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    if pattern.index == 0 {
        pattern.index = pattern.length - 1;
    } else {
        pattern.index -= 1;
    }
    let value = pattern.data[pattern.index];
    output(format!("PN.PREV {} = {} (INDEX NOW {})", pat, value, pattern.index));
    Ok(())
}

pub fn handle_pn<F>(
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
        output("ERROR: PN REQUIRES PATTERN (0-5) AND INDEX (0-63)".to_string());
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
            .context("Failed to parse pattern index")?
    };
    if idx > 63 {
        output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
        return Ok(());
    }
    if parts.len() == 3 {
        let pattern = &patterns.patterns[pat];
        output(format!("PN {} {} = {}", pat, idx, pattern.data[idx]));
    } else {
        let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index) {
            expr_val
        } else {
            parts[3]
                .parse()
                .context("Failed to parse pattern value")?
        };
        let pattern = &mut patterns.patterns[pat];
        pattern.data[idx] = val;
        output(format!("SET PN {} {} TO {}", pat, idx, val));
    }
    Ok(())
}

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
