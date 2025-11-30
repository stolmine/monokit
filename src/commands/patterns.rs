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

pub fn handle_pattern_add<F>(
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
        output("ERROR: P.ADD REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse add value")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_add(val);
    }
    output(format!("ADDED {} TO PATTERN {}", val, patterns.working));
    Ok(())
}

pub fn handle_pattern_sub<F>(
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
        output("ERROR: P.SUB REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse sub value")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_sub(val);
    }
    output(format!("SUBTRACTED {} FROM PATTERN {}", val, patterns.working));
    Ok(())
}

pub fn handle_pattern_mul<F>(
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
        output("ERROR: P.MUL REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mul value")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_mul(val);
    }
    output(format!("MULTIPLIED PATTERN {} BY {}", patterns.working, val));
    Ok(())
}

pub fn handle_pattern_div<F>(
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
        output("ERROR: P.DIV REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse div value")?
    };
    if val == 0 {
        output("ERROR: DIVISION BY ZERO".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[patterns.working];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] / val;
    }
    output(format!("DIVIDED PATTERN {} BY {}", patterns.working, val));
    Ok(())
}

pub fn handle_pattern_mod<F>(
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
        output("ERROR: P.MOD REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse mod value")?
    };
    if val == 0 {
        output("ERROR: MODULO BY ZERO".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[patterns.working];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] % val;
    }
    output(format!("MODULO PATTERN {} BY {}", patterns.working, val));
    Ok(())
}

pub fn handle_pattern_scale<F>(
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
        output("ERROR: P.SCALE REQUIRES MIN AND MAX VALUES".to_string());
        return Ok(());
    }
    let new_min: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse new min value")?
    };
    let new_max: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse new max value")?
    };
    let pattern = &mut patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
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
    output(format!("SCALED PATTERN {} TO RANGE {} TO {}", patterns.working, new_min, new_max));
    Ok(())
}

pub fn handle_pattern_min<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let min_val = pattern.data[..pattern.length].iter().copied().min().unwrap_or(0);
    output(format!("P.MIN = {}", min_val));
}

pub fn handle_pattern_max<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let max_val = pattern.data[..pattern.length].iter().copied().max().unwrap_or(0);
    output(format!("P.MAX = {}", max_val));
}

pub fn handle_pattern_sum<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    output(format!("P.SUM = {}", sum));
}

pub fn handle_pattern_avg<F>(
    patterns: &PatternStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &patterns.patterns[patterns.working];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return;
    }
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    let avg = sum / pattern.length as i32;
    output(format!("P.AVG = {}", avg));
}

pub fn handle_pattern_fnd<F>(
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
        output("ERROR: P.FND REQUIRES A VALUE".to_string());
        return Ok(());
    }
    let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[1]
            .parse()
            .context("Failed to parse find value")?
    };
    let pattern = &patterns.patterns[patterns.working];
    let index = pattern.data[..pattern.length]
        .iter()
        .position(|&x| x == val)
        .map(|i| i as i16)
        .unwrap_or(-1);
    output(format!("P.FND = {}", index));
    Ok(())
}

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

pub fn handle_pn_add<F>(
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
        output("ERROR: PN.ADD REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
            .context("Failed to parse add value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_add(val);
    }
    output(format!("ADDED {} TO PATTERN {}", val, pat));
    Ok(())
}

pub fn handle_pn_sub<F>(
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
        output("ERROR: PN.SUB REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
            .context("Failed to parse sub value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_sub(val);
    }
    output(format!("SUBTRACTED {} FROM PATTERN {}", val, pat));
    Ok(())
}

pub fn handle_pn_mul<F>(
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
        output("ERROR: PN.MUL REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
            .context("Failed to parse mul value")?
    };
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_mul(val);
    }
    output(format!("MULTIPLIED PATTERN {} BY {}", pat, val));
    Ok(())
}

pub fn handle_pn_div<F>(
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
        output("ERROR: PN.DIV REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
            .context("Failed to parse div value")?
    };
    if val == 0 {
        output("ERROR: DIVISION BY ZERO".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] / val;
    }
    output(format!("DIVIDED PATTERN {} BY {}", pat, val));
    Ok(())
}

pub fn handle_pn_mod<F>(
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
        output("ERROR: PN.MOD REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
            .context("Failed to parse mod value")?
    };
    if val == 0 {
        output("ERROR: MODULO BY ZERO".to_string());
        return Ok(());
    }
    let pattern = &mut patterns.patterns[pat];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] % val;
    }
    output(format!("MODULO PATTERN {} BY {}", pat, val));
    Ok(())
}

pub fn handle_pn_scale<F>(
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
        output("ERROR: PN.SCALE REQUIRES PATTERN NUMBER, MIN, AND MAX VALUES".to_string());
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
    let new_min: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[2]
            .parse()
            .context("Failed to parse new min value")?
    };
    let new_max: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        parts[3]
            .parse()
            .context("Failed to parse new max value")?
    };
    let pattern = &mut patterns.patterns[pat];
    if pattern.length == 0 {
        output("ERROR: PATTERN LENGTH IS ZERO".to_string());
        return Ok(());
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
    output(format!("SCALED PATTERN {} TO RANGE {} TO {}", pat, new_min, new_max));
    Ok(())
}

pub fn handle_pn_min<F>(
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
        output("ERROR: PN.MIN REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let min_val = pattern.data[..pattern.length].iter().copied().min().unwrap_or(0);
    output(format!("PN.MIN {} = {}", pat, min_val));
    Ok(())
}

pub fn handle_pn_max<F>(
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
        output("ERROR: PN.MAX REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let max_val = pattern.data[..pattern.length].iter().copied().max().unwrap_or(0);
    output(format!("PN.MAX {} = {}", pat, max_val));
    Ok(())
}

pub fn handle_pn_sum<F>(
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
        output("ERROR: PN.SUM REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    output(format!("PN.SUM {} = {}", pat, sum));
    Ok(())
}

pub fn handle_pn_avg<F>(
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
        output("ERROR: PN.AVG REQUIRES PATTERN NUMBER (0-5)".to_string());
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
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    let avg = sum / pattern.length as i32;
    output(format!("PN.AVG {} = {}", pat, avg));
    Ok(())
}

pub fn handle_pn_fnd<F>(
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
        output("ERROR: PN.FND REQUIRES PATTERN NUMBER AND VALUE".to_string());
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
            .context("Failed to parse find value")?
    };
    let pattern = &patterns.patterns[pat];
    let index = pattern.data[..pattern.length]
        .iter()
        .position(|&x| x == val)
        .map(|i| i as i16)
        .unwrap_or(-1);
    output(format!("PN.FND {} = {}", pat, index));
    Ok(())
}
