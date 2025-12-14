use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS};
use anyhow::{Context, Result};

use super::common::{define_pattern_nav};

pub fn handle_pattern_n<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    if parts.len() == 1 {
        ctx.output(OutputCategory::Query, format!("P.N = {}", ctx.patterns.working), &mut output);
    } else {
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                ctx.output(OutputCategory::Error, "ERROR: FAILED TO PARSE PATTERN NUMBER".to_string(), &mut output);
                return;
            }
        };
        if value > 5 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN NUMBER MUST BE 0-5".to_string(), &mut output);
            return;
        }
        ctx.patterns.working = value;
        ctx.output(OutputCategory::Confirm, format!("SET WORKING PATTERN TO {}", value), &mut output);
    }
}

pub fn handle_pattern_l<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    let working = ctx.patterns.working;
    if parts.len() == 1 {
        let length = ctx.patterns.patterns[working].length;
        ctx.output(OutputCategory::Query, format!("P.L = {}", length), &mut output);
    } else {
        let pattern = &mut ctx.patterns.patterns[working];
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                ctx.output(OutputCategory::Error, "ERROR: FAILED TO PARSE PATTERN LENGTH".to_string(), &mut output);
                return;
            }
        };
        if value < 1 || value > 64 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN LENGTH MUST BE 1-64".to_string(), &mut output);
            return;
        }
        pattern.length = value;
        ctx.output(OutputCategory::Confirm, format!("SET PATTERN {} LENGTH TO {}", working, value), &mut output);
    }
}

pub fn handle_pattern_i<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    let working = ctx.patterns.working;
    if parts.len() == 1 {
        let index = ctx.patterns.patterns[working].index;
        ctx.output(OutputCategory::Query, format!("P.I = {}", index), &mut output);
    } else {
        let pattern = &mut ctx.patterns.patterns[working];
        let value: usize = match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                ctx.output(OutputCategory::Error, "ERROR: FAILED TO PARSE PATTERN INDEX".to_string(), &mut output);
                return;
            }
        };
        if value > 63 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
            return;
        }
        pattern.index = value;
        ctx.output(OutputCategory::Confirm, format!("SET PATTERN {} INDEX TO {}", working, value), &mut output);
    }
}

define_pattern_nav!(handle_pattern_here, handle_pn_here, pattern_here_impl, "HERE", here);
define_pattern_nav!(handle_pattern_next, handle_pn_next, pattern_next_impl, "NEXT", nav);
define_pattern_nav!(handle_pattern_prev, handle_pn_prev, pattern_prev_impl, "PREV", nav);

pub fn handle_pattern<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    if parts.len() == 1 {
        ctx.output(OutputCategory::Error, "ERROR: P REQUIRES AN INDEX".to_string(), &mut output);
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
        if expr_val < 0 || expr_val > 63 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
            return Ok(());
        }
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern index")?
    };
    if idx > 63 {
        ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &ctx.patterns.patterns[ctx.patterns.working];
        ctx.output(OutputCategory::Query, format!("P {} = {}", idx, pattern.data[idx]), &mut output);
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern value")?
        };
        let pattern = &mut ctx.patterns.patterns[ctx.patterns.working];
        pattern.data[idx] = value;
        ctx.output(OutputCategory::Confirm, format!("SET P {} TO {}", idx, value), &mut output);
    }
    Ok(())
}
