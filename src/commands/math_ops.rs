use crate::eval::eval_expression;
use crate::types::{PatternStorage, ScriptStorage, Variables};

pub fn handle_add<F>(
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
        output("ERROR: ADD REQUIRES TWO OPERANDS".to_string());
        return;
    }
    if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
            let result = x.saturating_add(y);
            output(format!("{}", result));
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
    }
}

pub fn handle_sub<F>(
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
        output("ERROR: SUB REQUIRES TWO OPERANDS".to_string());
        return;
    }
    if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
            let result = x.saturating_sub(y);
            output(format!("{}", result));
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
    }
}

pub fn handle_mul<F>(
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
        output("ERROR: MUL REQUIRES TWO OPERANDS".to_string());
        return;
    }
    if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
            let result = x.saturating_mul(y);
            output(format!("{}", result));
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
    }
}

pub fn handle_div<F>(
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
        output("ERROR: DIV REQUIRES TWO OPERANDS".to_string());
        return;
    }
    if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
            if y == 0 {
                output("ERROR: DIVISION BY ZERO".to_string());
            } else {
                let result = x / y;
                output(format!("{}", result));
            }
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
    }
}

pub fn handle_mod<F>(
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
        output("ERROR: MOD REQUIRES TWO OPERANDS".to_string());
        return;
    }
    if let Some((x, x_consumed)) = eval_expression(&parts, 1, variables, patterns, scripts, script_index) {
        if let Some((y, _)) = eval_expression(&parts, 1 + x_consumed, variables, patterns, scripts, script_index) {
            if y == 0 {
                output("ERROR: MODULO BY ZERO".to_string());
            } else {
                let result = x % y;
                output(format!("{}", result));
            }
        } else {
            output("ERROR: FAILED TO EVALUATE SECOND OPERAND".to_string());
        }
    } else {
        output("ERROR: FAILED TO EVALUATE FIRST OPERAND".to_string());
    }
}
