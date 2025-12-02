use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS};
use anyhow::{Context, Result};

use super::common::{define_pattern_nav};

pub fn handle_pattern_n<F>(
    parts: &[&str],
    patterns: &mut PatternStorage,
    debug_level: u8,
    out_err: bool,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("P.N = {}", patterns.working));
        }
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                if debug_level >= TIER_ERRORS || out_err {
                    output("ERROR: FAILED TO PARSE PATTERN NUMBER".to_string());
                }
                return;
            }
        };
        if value > 5 {
            if debug_level >= TIER_ERRORS || out_err {
                output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
            }
            return;
        }
        patterns.working = value;
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SET WORKING PATTERN TO {}", value));
        }
    }
}

pub fn handle_pattern_l<F>(
    parts: &[&str],
    patterns: &mut PatternStorage,
    debug_level: u8,
    out_err: bool,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("P.L = {}", pattern.length));
        }
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                if debug_level >= TIER_ERRORS || out_err {
                    output("ERROR: FAILED TO PARSE PATTERN LENGTH".to_string());
                }
                return;
            }
        };
        if value < 1 || value > 64 {
            if debug_level >= TIER_ERRORS || out_err {
                output("ERROR: PATTERN LENGTH MUST BE 1-64".to_string());
            }
            return;
        }
        pattern.length = value;
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SET PATTERN {} LENGTH TO {}", patterns.working, value));
        }
    }
}

pub fn handle_pattern_i<F>(
    parts: &[&str],
    patterns: &mut PatternStorage,
    debug_level: u8,
    out_err: bool,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    let pattern = &mut patterns.patterns[patterns.working];
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("P.I = {}", pattern.index));
        }
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                if debug_level >= TIER_ERRORS || out_err {
                    output("ERROR: FAILED TO PARSE PATTERN INDEX".to_string());
                }
                return;
            }
        };
        if value > 63 {
            if debug_level >= TIER_ERRORS || out_err {
                output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
            }
            return;
        }
        pattern.index = value;
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SET PATTERN {} INDEX TO {}", patterns.working, value));
        }
    }
}

define_pattern_nav!(handle_pattern_here, handle_pn_here, pattern_here_impl, "HERE", here);
define_pattern_nav!(handle_pattern_next, handle_pn_next, pattern_next_impl, "NEXT", nav);
define_pattern_nav!(handle_pattern_prev, handle_pn_prev, pattern_prev_impl, "PREV", nav);

pub fn handle_pattern<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_err: bool,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_ERRORS || out_err {
            output("ERROR: P REQUIRES AN INDEX".to_string());
        }
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern index")?
    };
    if idx > 63 {
        if debug_level >= TIER_ERRORS || out_err {
            output("ERROR: PATTERN INDEX MUST BE 0-63".to_string());
        }
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &patterns.patterns[patterns.working];
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("P {} = {}", idx, pattern.data[idx]));
        }
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern value")?
        };
        let pattern = &mut patterns.patterns[patterns.working];
        pattern.data[idx] = value;
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SET P {} TO {}", idx, value));
        }
    }
    Ok(())
}
