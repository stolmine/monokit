use crate::commands::common::parse_i16_expr;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ERRORS, TIER_QUERIES, TIER_CONFIRMS};
use anyhow::Result;

macro_rules! define_variable_handler {
    ($fn_name:ident, $var_name:literal, $var_field:ident) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &mut Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            scale: &ScaleState,
            debug_level: u8,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= TIER_QUERIES || out_qry {
                    output(format!("{} = {}", $var_name, variables.$var_field));
                }
            } else {
                let Some(value) = parse_i16_expr(
                    parts, 1, variables, patterns, counters, scripts, script_index, scale, $var_name, &mut output
                ) else {
                    return;
                };
                variables.$var_field = value;
                if debug_level >= TIER_CONFIRMS || out_cfm {
                    output(format!("SET {} TO {}", $var_name, value));
                }
            }
        }
    };

    (script, $fn_name:ident, $var_name:literal, $var_field:ident) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &mut ScriptStorage,
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
            if script_index >= 10 {
                if debug_level >= TIER_ERRORS || out_err {
                    output(format!("ERROR: {} REQUIRES SCRIPT CONTEXT", $var_name));
                }
                return Ok(());
            }
            if parts.len() == 1 {
                if debug_level >= TIER_QUERIES || out_qry {
                    output(format!("{} = {}", $var_name, scripts.scripts[script_index].$var_field));
                }
            } else {
                let value: i16 = if let Some(v) = parse_i16_expr(
                    parts, 1, variables, patterns, counters, scripts, script_index, scale, $var_name, &mut output
                ) {
                    v
                } else {
                    return Ok(());
                };
                scripts.scripts[script_index].$var_field = value;
                if debug_level >= TIER_CONFIRMS || out_cfm {
                    output(format!("SET {} TO {}", $var_name, value));
                }
            }
            Ok(())
        }
    };
}

define_variable_handler!(handle_variable_a, "A", a);
define_variable_handler!(handle_variable_b, "B", b);
define_variable_handler!(handle_variable_c, "C", c);
define_variable_handler!(handle_variable_d, "D", d);
define_variable_handler!(handle_variable_x, "X", x);
define_variable_handler!(handle_variable_y, "Y", y);
define_variable_handler!(handle_variable_z, "Z", z);
define_variable_handler!(handle_variable_t, "T", t);
define_variable_handler!(script, handle_variable_j, "J", j);
define_variable_handler!(script, handle_variable_k, "K", k);

pub fn handle_variable_i<F>(
    parts: &[&str],
    variables: &mut Variables,
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
            output(format!("I = {}", variables.i));
        }
    } else {
        let value: i16 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                if debug_level >= TIER_ERRORS || out_err {
                    output("ERROR: FAILED TO PARSE VALUE FOR I".to_string());
                }
                return;
            }
        };
        variables.i = value;
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SET I TO {}", value));
        }
    }
}
