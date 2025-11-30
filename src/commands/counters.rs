use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};

// Standalone counter read commands - returns value and increments
pub fn handle_n1<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    let current = counters.values[0];
    let min = counters.min[0];
    let max = counters.max[0];
    counters.values[0] = if max == 0 {
        current.wrapping_add(1)
    } else {
        let next = current + 1;
        if next > max { min } else { next }
    };
    output(format!("{}", current));
}

pub fn handle_n2<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    let current = counters.values[1];
    let min = counters.min[1];
    let max = counters.max[1];
    counters.values[1] = if max == 0 {
        current.wrapping_add(1)
    } else {
        let next = current + 1;
        if next > max { min } else { next }
    };
    output(format!("{}", current));
}

pub fn handle_n3<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    let current = counters.values[2];
    let min = counters.min[2];
    let max = counters.max[2];
    counters.values[2] = if max == 0 {
        current.wrapping_add(1)
    } else {
        let next = current + 1;
        if next > max { min } else { next }
    };
    output(format!("{}", current));
}

pub fn handle_n4<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    let current = counters.values[3];
    let min = counters.min[3];
    let max = counters.max[3];
    counters.values[3] = if max == 0 {
        current.wrapping_add(1)
    } else {
        let next = current + 1;
        if next > max { min } else { next }
    };
    output(format!("{}", current));
}

pub fn handle_n1_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[0] = counters.min[0];
    output(format!("N1 RESET TO {}", counters.min[0]));
}

pub fn handle_n2_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[1] = counters.min[1];
    output(format!("N2 RESET TO {}", counters.min[1]));
}

pub fn handle_n3_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[2] = counters.min[2];
    output(format!("N3 RESET TO {}", counters.min[2]));
}

pub fn handle_n4_rst<F>(counters: &mut Counters, mut output: F)
where
    F: FnMut(String),
{
    counters.values[3] = counters.min[3];
    output(format!("N4 RESET TO {}", counters.min[3]));
}

pub fn handle_n1_max<F>(
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
        output("N1.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
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

    if value > 0 && value < counters.min[0] {
        output(format!("ERROR: MAX ({}) MUST BE >= MIN ({})", value, counters.min[0]));
        return;
    }

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
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N2.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
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

    if value > 0 && value < counters.min[1] {
        output(format!("ERROR: MAX ({}) MUST BE >= MIN ({})", value, counters.min[1]));
        return;
    }

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
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N3.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
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

    if value > 0 && value < counters.min[2] {
        output(format!("ERROR: MAX ({}) MUST BE >= MIN ({})", value, counters.min[2]));
        return;
    }

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
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("N4.MAX REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
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

    if value > 0 && value < counters.min[3] {
        output(format!("ERROR: MAX ({}) MUST BE >= MIN ({})", value, counters.min[3]));
        return;
    }

    counters.max[3] = value;
    if value == 0 {
        output("N4.MAX DISABLED (NO WRAP)".to_string());
    } else {
        output(format!("N4.MAX SET TO {}", value));
    }
}

pub fn handle_n1_min<F>(
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
        output("N1.MIN REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N1.MIN".to_string());
                return;
            }
        }
    };

    counters.min[0] = value;
    output(format!("N1.MIN SET TO {}", value));
}

pub fn handle_n2_min<F>(
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
        output("N2.MIN REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N2.MIN".to_string());
                return;
            }
        }
    };

    counters.min[1] = value;
    output(format!("N2.MIN SET TO {}", value));
}

pub fn handle_n3_min<F>(
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
        output("N3.MIN REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N3.MIN".to_string());
                return;
            }
        }
    };

    counters.min[2] = value;
    output(format!("N3.MIN SET TO {}", value));
}

pub fn handle_n4_min<F>(
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
        output("N4.MIN REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR N4.MIN".to_string());
                return;
            }
        }
    };

    counters.min[3] = value;
    output(format!("N4.MIN SET TO {}", value));
}
