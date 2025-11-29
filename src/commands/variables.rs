use crate::eval::eval_expression;
use crate::types::{PatternStorage, ScriptStorage, Variables};
use anyhow::{Context, Result};

pub fn handle_variable_a<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("A = {}", variables.a));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR A".to_string());
                    return;
                }
            }
        };
        variables.a = value;
        output(format!("SET A TO {}", value));
    }
}

pub fn handle_variable_b<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("B = {}", variables.b));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR B".to_string());
                    return;
                }
            }
        };
        variables.b = value;
        output(format!("SET B TO {}", value));
    }
}

pub fn handle_variable_c<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("C = {}", variables.c));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR C".to_string());
                    return;
                }
            }
        };
        variables.c = value;
        output(format!("SET C TO {}", value));
    }
}

pub fn handle_variable_d<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("D = {}", variables.d));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR D".to_string());
                    return;
                }
            }
        };
        variables.d = value;
        output(format!("SET D TO {}", value));
    }
}

pub fn handle_variable_i<F>(
    parts: &[&str],
    variables: &mut Variables,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("I = {}", variables.i));
    } else {
        let value: i16 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR I".to_string());
                return;
            }
        };
        variables.i = value;
        output(format!("SET I TO {}", value));
    }
}

pub fn handle_variable_x<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("X = {}", variables.x));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR X".to_string());
                    return;
                }
            }
        };
        variables.x = value;
        output(format!("SET X TO {}", value));
    }
}

pub fn handle_variable_y<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("Y = {}", variables.y));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR Y".to_string());
                    return;
                }
            }
        };
        variables.y = value;
        output(format!("SET Y TO {}", value));
    }
}

pub fn handle_variable_z<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("Z = {}", variables.z));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR Z".to_string());
                    return;
                }
            }
        };
        variables.z = value;
        output(format!("SET Z TO {}", value));
    }
}

pub fn handle_variable_t<F>(
    parts: &[&str],
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &ScriptStorage,
    script_index: usize,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("T = {}", variables.t));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("ERROR: FAILED TO PARSE VALUE FOR T".to_string());
                    return;
                }
            }
        };
        variables.t = value;
        output(format!("SET T TO {}", value));
    }
}

pub fn handle_variable_j<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    scripts: &mut ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if script_index >= 10 {
        output("ERROR: J REQUIRES SCRIPT CONTEXT".to_string());
        return Ok(());
    }
    if parts.len() == 1 {
        output(format!("J = {}", scripts.scripts[script_index].j));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            parts[1]
                .parse()
                .context("Failed to parse value for J")?
        };
        scripts.scripts[script_index].j = value;
        output(format!("SET J TO {}", value));
    }
    Ok(())
}

pub fn handle_variable_k<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    scripts: &mut ScriptStorage,
    script_index: usize,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if script_index >= 10 {
        output("ERROR: K REQUIRES SCRIPT CONTEXT".to_string());
        return Ok(());
    }
    if parts.len() == 1 {
        output(format!("K = {}", scripts.scripts[script_index].k));
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
            expr_val
        } else {
            parts[1]
                .parse()
                .context("Failed to parse value for K")?
        };
        scripts.scripts[script_index].k = value;
        output(format!("SET K TO {}", value));
    }
    Ok(())
}
