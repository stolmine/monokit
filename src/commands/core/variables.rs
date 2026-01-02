use crate::commands::common::parse_i16_expr;
use crate::commands::context::ExecutionContext;
use crate::output::OutputDecider;
use crate::types::OutputCategory;
use anyhow::Result;

macro_rules! define_variable_handler {
    ($fn_name:ident, $var_name:literal, $var_field:ident) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            ctx: &mut ExecutionContext,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                ctx.output(OutputCategory::Query, format!("{} = {}", $var_name, ctx.variables.$var_field), &mut output);
            } else {
                let Some(value) = parse_i16_expr(
                    parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, $var_name, &mut output
                ) else {
                    return;
                };
                ctx.variables.$var_field = value;
                ctx.output(OutputCategory::Confirm, format!("SET {} TO {}", $var_name, value), &mut output);
            }
        }
    };

    (script, $fn_name:ident, $var_name:literal, $var_field:ident) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            ctx: &mut ExecutionContext,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if ctx.script_index >= 10 {
                ctx.output(OutputCategory::Error, format!("ERROR: {} REQUIRES SCRIPT CONTEXT", $var_name), &mut output);
                return Ok(());
            }
            if parts.len() == 1 {
                ctx.output(OutputCategory::Query, format!("{} = {}", $var_name, ctx.scripts.scripts[ctx.script_index].$var_field), &mut output);
            } else {
                let value: i16 = if let Some(v) = parse_i16_expr(
                    parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, $var_name, &mut output
                ) {
                    v
                } else {
                    return Ok(());
                };
                ctx.scripts.scripts[ctx.script_index].$var_field = value;
                ctx.output(OutputCategory::Confirm, format!("SET {} TO {}", $var_name, value), &mut output);
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
    ctx: &mut ExecutionContext,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        ctx.output(OutputCategory::Query, format!("I = {}", ctx.variables.i), &mut output);
    } else {
        let value: i16 = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                ctx.output(OutputCategory::Error, "ERROR: FAILED TO PARSE VALUE FOR I".to_string(), &mut output);
                return;
            }
        };
        ctx.variables.i = value;
        ctx.output(OutputCategory::Confirm, format!("SET I TO {}", value), &mut output);
    }
}
