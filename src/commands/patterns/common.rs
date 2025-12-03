use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ERRORS};
use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;

pub enum PatternRef {
    Working,
    Explicit(usize),
}

impl PatternRef {
    pub fn index(&self, patterns: &PatternStorage) -> usize {
        match self {
            PatternRef::Working => patterns.working,
            PatternRef::Explicit(idx) => *idx,
        }
    }
}

pub fn pattern_add_impl(pat_ref: PatternRef, val: i16, patterns: &mut PatternStorage) -> String {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_add(val);
    }
    format!("ADDED {} TO PAT {}", val, pat_idx)
}

pub fn pattern_sub_impl(pat_ref: PatternRef, val: i16, patterns: &mut PatternStorage) -> String {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_sub(val);
    }
    format!("SUBTRACTED {} FROM PAT {}", val, pat_idx)
}

pub fn pattern_mul_impl(pat_ref: PatternRef, val: i16, patterns: &mut PatternStorage) -> String {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i].saturating_mul(val);
    }
    format!("MULTIPLIED PAT {} BY {}", pat_idx, val)
}

pub fn pattern_div_impl(pat_ref: PatternRef, val: i16, patterns: &mut PatternStorage) -> Result<String, &'static str> {
    if val == 0 {
        return Err("DIVISION BY ZERO");
    }
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] / val;
    }
    Ok(format!("DIVIDED PAT {} BY {}", pat_idx, val))
}

pub fn pattern_mod_impl(pat_ref: PatternRef, val: i16, patterns: &mut PatternStorage) -> Result<String, &'static str> {
    if val == 0 {
        return Err("MODULO BY ZERO");
    }
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    for i in 0..pattern.length {
        pattern.data[i] = pattern.data[i] % val;
    }
    Ok(format!("MODULO PAT {} BY {}", pat_idx, val))
}

pub fn pattern_scale_impl(pat_ref: PatternRef, new_min: i16, new_max: i16, patterns: &mut PatternStorage) -> Result<String, String> {
    if new_min == new_max {
        return Err("MIN AND MAX CANNOT BE EQUAL".to_string());
    }
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("PAT LENGTH IS ZERO".to_string());
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
    Ok(format!("SCALED PAT {} TO {}-{}", pat_idx, new_min, new_max))
}

pub fn pattern_push_impl(pat_ref: PatternRef, val: i16, patterns: &mut PatternStorage) -> Result<String, &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("CANNOT OPERATE ON EMPTY PAT");
    }
    for i in 0..pattern.length - 1 {
        pattern.data[i] = pattern.data[i + 1];
    }
    pattern.data[pattern.length - 1] = val;
    Ok(format!("PUSHED {} TO PAT {}", val, pat_idx))
}

pub fn pattern_pop_impl(pat_ref: PatternRef, patterns: &PatternStorage) -> Result<(i16, usize), &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("PAT LENGTH IS ZERO");
    }
    let val = pattern.data[pattern.length - 1];
    Ok((val, pat_idx))
}

pub fn pattern_ins_impl(pat_ref: PatternRef, idx: usize, val: i16, patterns: &mut PatternStorage) -> Result<String, String> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    if idx >= pattern.length {
        return Err(format!("IDX {} OUT OF RANGE (LEN {})", idx, pattern.length));
    }
    for i in (idx..pattern.length - 1).rev() {
        pattern.data[i + 1] = pattern.data[i];
    }
    pattern.data[idx] = val;
    Ok(format!("INSERTED {} AT IDX {} IN PAT {}", val, idx, pat_idx))
}

pub fn pattern_rm_impl(pat_ref: PatternRef, idx: usize, patterns: &mut PatternStorage) -> Result<String, String> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    if idx >= pattern.length {
        return Err(format!("IDX {} OUT OF RANGE (LEN {})", idx, pattern.length));
    }
    let removed = pattern.data[idx];
    for i in idx..pattern.length - 1 {
        pattern.data[i] = pattern.data[i + 1];
    }
    pattern.data[pattern.length - 1] = 0;
    Ok(format!("REMOVED {} FROM IDX {} IN PAT {}", removed, idx, pat_idx))
}

pub fn pattern_rev_impl(pat_ref: PatternRef, patterns: &mut PatternStorage) -> usize {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    let len = pattern.length;
    for i in 0..len / 2 {
        pattern.data.swap(i, len - 1 - i);
    }
    pat_idx
}

pub fn pattern_rot_impl(pat_ref: PatternRef, n: i16, patterns: &mut PatternStorage) -> Result<String, &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    let len = pattern.length as i16;
    if len == 0 {
        return Err("PAT LENGTH IS ZERO");
    }
    let n = ((n % len) + len) % len;
    if n == 0 {
        return Ok(format!("PAT {} UNCHANGED (ROT 0)", pat_idx));
    }
    let n_usize = n as usize;
    let mut temp = [0i16; 64];
    for i in 0..pattern.length {
        temp[i] = pattern.data[i];
    }
    for i in 0..pattern.length {
        pattern.data[i] = temp[(i + pattern.length - n_usize) % pattern.length];
    }
    Ok(format!("ROTATED PAT {} BY {}", pat_idx, n))
}

pub fn pattern_shuf_impl(pat_ref: PatternRef, patterns: &mut PatternStorage) -> Result<usize, &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("CANNOT OPERATE ON EMPTY PAT");
    }
    let len = pattern.length;
    let mut rng = rand::thread_rng();
    pattern.data[..len].shuffle(&mut rng);
    Ok(pat_idx)
}

pub fn pattern_sort_impl(pat_ref: PatternRef, patterns: &mut PatternStorage) -> usize {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    let len = pattern.length;
    pattern.data[..len].sort();
    pat_idx
}

pub fn pattern_rnd_impl(pat_ref: PatternRef, min: i16, max: i16, patterns: &mut PatternStorage) -> usize {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    let mut rng = rand::thread_rng();
    for i in 0..pattern.length {
        pattern.data[i] = rng.gen_range(min..=max);
    }
    pat_idx
}

pub fn pattern_min_impl(pat_ref: PatternRef, patterns: &PatternStorage) -> Result<i16, &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("PAT LENGTH IS ZERO");
    }
    let min_val = pattern.data[..pattern.length].iter().copied().min().unwrap_or(0);
    Ok(min_val)
}

pub fn pattern_max_impl(pat_ref: PatternRef, patterns: &PatternStorage) -> Result<i16, &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("PAT LENGTH IS ZERO");
    }
    let max_val = pattern.data[..pattern.length].iter().copied().max().unwrap_or(0);
    Ok(max_val)
}

pub fn pattern_sum_impl(pat_ref: PatternRef, patterns: &PatternStorage) -> i32 {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    pattern.data[..pattern.length].iter().map(|&x| x as i32).sum()
}

pub fn pattern_avg_impl(pat_ref: PatternRef, patterns: &PatternStorage) -> Result<i32, &'static str> {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    if pattern.length == 0 {
        return Err("PAT LENGTH IS ZERO");
    }
    let sum: i32 = pattern.data[..pattern.length].iter().map(|&x| x as i32).sum();
    let avg = sum / pattern.length as i32;
    Ok(avg)
}

pub fn pattern_fnd_impl(pat_ref: PatternRef, val: i16, patterns: &PatternStorage) -> i16 {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    pattern.data[..pattern.length]
        .iter()
        .position(|&x| x == val)
        .map(|i| i as i16)
        .unwrap_or(-1)
}

pub fn pattern_here_impl(pat_ref: PatternRef, patterns: &PatternStorage) -> (i16, usize) {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &patterns.patterns[pat_idx];
    let value = pattern.data[pattern.index];
    (value, pat_idx)
}

pub fn pattern_next_impl(pat_ref: PatternRef, patterns: &mut PatternStorage) -> (i16, usize, usize) {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    pattern.index = (pattern.index + 1) % pattern.length;
    let value = pattern.data[pattern.index];
    (value, pat_idx, pattern.index)
}

pub fn pattern_prev_impl(pat_ref: PatternRef, patterns: &mut PatternStorage) -> (i16, usize, usize) {
    let pat_idx = pat_ref.index(patterns);
    let pattern = &mut patterns.patterns[pat_idx];
    if pattern.index == 0 {
        pattern.index = pattern.length - 1;
    } else {
        pattern.index -= 1;
    }
    let value = pattern.data[pattern.index];
    (value, pat_idx, pattern.index)
}

pub fn parse_pattern_num<F>(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    output: &mut F,
    debug_level: u8,
    out_err: bool,
) -> Result<Option<usize>>
where
    F: FnMut(String),
{
    let pat: usize = if let Some((expr_val, _)) = eval_expression(parts, idx, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 0 || expr_val > 5 {
            if debug_level >= TIER_ERRORS || out_err {
                output("ERROR: PAT NUM MUST BE 0-5".to_string());
            }
            return Ok(None);
        }
        expr_val as usize
    } else {
        parts[idx].parse().context("Failed to parse pattern number")?
    };
    if pat > 5 {
        if debug_level >= TIER_ERRORS || out_err {
            output("ERROR: PAT NUM MUST BE 0-5".to_string());
        }
        return Ok(None);
    }
    Ok(Some(pat))
}

pub fn parse_i16_expr(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
) -> Result<i16> {
    if let Some((expr_val, _)) = eval_expression(parts, idx, variables, patterns, counters, scripts, script_index, scale) {
        Ok(expr_val)
    } else {
        parts[idx].parse().context("Failed to parse value")
    }
}

pub fn parse_usize_expr(
    parts: &[&str],
    idx: usize,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
) -> Result<usize> {
    if let Some((expr_val, _)) = eval_expression(parts, idx, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 0 {
            anyhow::bail!("Value must be non-negative, got {}", expr_val);
        }
        Ok(expr_val as usize)
    } else {
        parts[idx].parse().context("Failed to parse value")
    }
}

#[macro_export]
macro_rules! define_pattern_op_1val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name));
                }
                return Ok(());
            }
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            define_pattern_op_1val!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, val, patterns, output, debug_level, out_err, out_cfm);
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM AND VAL", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            define_pattern_op_1val!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), val, patterns, output, debug_level, out_err, out_cfm);
            Ok(())
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $val:expr, $patterns:expr, $output:expr, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
            $output($crate::commands::patterns::common::$impl_fn($pat_ref, $val, $patterns));
        }
    };

    (@output result, $impl_fn:ident, $pat_ref:expr, $val:expr, $patterns:expr, $output:expr, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $val, $patterns) {
            Ok(msg) => {
                if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
                    $output(msg);
                }
            }
            Err(e) => {
                if $debug_level >= $crate::types::TIER_ERRORS || $out_err {
                    $output(e.to_string());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_noarg {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            patterns: &mut $crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_op_noarg!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, patterns, output, $cmd_name, debug_level, out_err, out_cfm);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_op_noarg!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), patterns, output, $cmd_name, debug_level, out_err, out_cfm);
            Ok(())
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        let pat_idx = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
            $output(format!(concat!(stringify!($impl_fn), " PATTERN {}"), pat_idx).replace("_impl", "").to_uppercase());
        }
    };

    (@output result_idx, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns) {
            Ok(pat_idx) => {
                if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
                    $output(format!(concat!(stringify!($impl_fn), " PATTERN {}"), pat_idx).replace("_impl", "").to_uppercase());
                }
            }
            Err(e) => {
                if $debug_level >= $crate::types::TIER_ERRORS || $out_err {
                    $output(e.to_string());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_2val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $err_msg:literal, $val_type:ty) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES {}", $cmd_name, $err_msg));
                }
                return Ok(());
            }
            let val1 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[1].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            let val2 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[2].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, val1, val2, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 4 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM, {}", $cmd_name, $err_msg));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val1 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[2].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            let val2 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[3].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), val1, val2, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
                }
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_idx_val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES INDEX AND VALUE", $cmd_name));
                }
                return Ok(());
            }
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, idx, val, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 4 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM, IDX, VAL", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 3, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), idx, val, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
                }
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_idx {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES AN INDEX", $cmd_name));
                }
                return Ok(());
            }
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, idx, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM AND IDX", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), idx, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
                }
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_query {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            patterns: &$crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_query!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, patterns, output, $cmd_name, debug_level, out_err, out_qry);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_query!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), patterns, output, $cmd_name, debug_level, out_err, out_qry);
            Ok(())
        }
    };

    (@output result, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_qry:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns) {
            Ok(val) => {
                if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
                    $output(format!("P.{} = {}", $cmd_name, val));
                }
            }
            Err(e) => {
                if $debug_level >= $crate::types::TIER_ERRORS || $out_err {
                    $output(e.to_string());
                }
            }
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_qry:expr) => {
        let val = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
            $output(format!("P.{} = {}", $cmd_name, val));
        }
    };
}

#[macro_export]
macro_rules! define_pattern_query_1val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name));
                }
                return Ok(());
            }
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            let index = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, val, patterns);
            if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                output(format!("P.{} = {}", $cmd_name, index));
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM AND VAL", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            let index = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), val, patterns);
            if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                output(format!("PN.{} = {}", $cmd_name, index));
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_nav {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $nav_type:tt) => {
        pub fn $working_fn<F>(
            patterns: &mut $crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_nav!(@output $nav_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, patterns, output, $cmd_name, debug_level, out_qry);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_nav!(@output $nav_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), patterns, output, $cmd_name, debug_level, out_qry);
            Ok(())
        }
    };

    (@output here, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_qry:expr) => {
        let (value, _pat_idx) = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
            $output(format!("P.{} = {}", $cmd_name, value));
        }
    };

    (@output nav, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_qry:expr) => {
        let (value, _pat_idx, new_index) = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
            $output(format!("P.{} = {} (INDEX NOW {})", $cmd_name, value, new_index));
        }
    };
}

#[macro_export]
macro_rules! define_pattern_pop {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            patterns: &mut $crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, patterns) {
                Ok((val, _pat_idx)) => {
                    if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                        output(format!("P.{} = {}", $cmd_name, val));
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
                }
            }
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), patterns) {
                Ok((val, _pat_idx)) => {
                    if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                        output(format!("PN.{} = {}", $cmd_name, val));
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
                }
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_rnd {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            let (min, max) = if parts.len() >= 3 {
                let min_val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
                let max_val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
                (min_val, max_val)
            } else {
                (0, 127)
            };
            let pat_idx = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, min, max, patterns);
            if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("RANDOMIZED PAT {} (RANGE {}-{})", pat_idx, min, max));
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let (min, max) = if parts.len() >= 4 {
                let min_val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
                let max_val = $crate::commands::patterns::common::parse_i16_expr(parts, 3, variables, patterns, counters, scripts, script_index, scale)?;
                (min_val, max_val)
            } else {
                (0, 127)
            };
            let pat_idx = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), min, max, patterns);
            if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("RANDOMIZED PAT {} (RANGE {}-{})", pat_idx, min, max));
            }
            Ok(())
        }
    };
}

pub(crate) use {
    define_pattern_nav, define_pattern_op_1val, define_pattern_op_2val,
    define_pattern_op_idx, define_pattern_op_idx_val, define_pattern_op_noarg,
    define_pattern_pop, define_pattern_query, define_pattern_query_1val,
    define_pattern_rnd,
};
