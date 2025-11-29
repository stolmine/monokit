use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScriptStorage, Variables};

pub fn handle_n1_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[0] = 0;
    output("N1 RESET TO 0".to_string());
}

pub fn handle_n2_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[1] = 0;
    output("N2 RESET TO 0".to_string());
}

pub fn handle_n3_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[2] = 0;
    output("N3 RESET TO 0".to_string());
}

pub fn handle_n4_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[3] = 0;
    output("N4 RESET TO 0".to_string());
}

pub fn handle_n1_max<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N1.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N1.MAX".to_string());
                return;
            }
        }
    };

    counters.max[0] = value;
    if value == 0 {
        output("N1.MAX DISABLED (NO WRAP)".to_string());
    } else {
        output(format!("N1.MAX SET TO {}", value));
    }
}

pub fn handle_n2_max<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N2.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N2.MAX".to_string());
                return;
            }
        }
    };

    counters.max[1] = value;
    if value == 0 {
        output("N2.MAX DISABLED (NO WRAP)".to_string());
    } else {
        output(format!("N2.MAX SET TO {}", value));
    }
}

pub fn handle_n3_max<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N3.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N3.MAX".to_string());
                return;
            }
        }
    };

    counters.max[2] = value;
    if value == 0 {
        output("N3.MAX DISABLED (NO WRAP)".to_string());
    } else {
        output(format!("N3.MAX SET TO {}", value));
    }
}

pub fn handle_n4_max<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N4.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N4.MAX".to_string());
                return;
            }
        }
    };

    counters.max[3] = value;
    if value == 0 {
        output("N4.MAX DISABLED (NO WRAP)".to_string());
    } else {
        output(format!("N4.MAX SET TO {}", value));
    }
}
