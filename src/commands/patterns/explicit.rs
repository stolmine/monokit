use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_QUERIES, TIER_CONFIRMS, TIER_ERRORS};
use anyhow::{Context, Result};

pub fn handle_pn_l<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    if parts.len() < 2 {
        ctx.output(OutputCategory::Error, "ERROR: PN.L REQUIRES PATTERN NUMBER (0-5)".to_string(), &mut output);
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        ctx.output(OutputCategory::Error, "ERROR: PATTERN NUMBER MUST BE 0-5".to_string(), &mut output);
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &ctx.patterns.patterns[pat];
        ctx.output(OutputCategory::Query, format!("PN.L {} = {}", pat, pattern.length), &mut output);
    } else {
        let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
            if expr_val < 1 || expr_val > 64 {
                ctx.output(OutputCategory::Error, "ERROR: PATTERN LENGTH MUST BE 1-64".to_string(), &mut output);
                return Ok(());
            }
            expr_val as usize
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern length")?
        };
        if value < 1 || value > 64 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN LENGTH MUST BE 1-64".to_string(), &mut output);
            return Ok(());
        }
        let pattern = &mut ctx.patterns.patterns[pat];
        pattern.length = value;
        ctx.output(OutputCategory::Confirm, format!("SET PATTERN {} LENGTH TO {}", pat, value), &mut output);
    }
    Ok(())
}

pub fn handle_pn_i<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    if parts.len() < 2 {
        ctx.output(OutputCategory::Error, "ERROR: PN.I REQUIRES PATTERN NUMBER (0-5)".to_string(), &mut output);
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        ctx.output(OutputCategory::Error, "ERROR: PATTERN NUMBER MUST BE 0-5".to_string(), &mut output);
        return Ok(());
    }
    if parts.len() == 2 {
        let pattern = &ctx.patterns.patterns[pat];
        ctx.output(OutputCategory::Query, format!("PN.I {} = {}", pat, pattern.index), &mut output);
    } else {
        let value: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
            if expr_val < 0 || expr_val > 63 {
                ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
                return Ok(());
            }
            expr_val as usize
        } else {
            parts[2]
                .parse()
                .context("Failed to parse pattern index")?
        };
        if value > 63 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
            return Ok(());
        }
        let pattern = &mut ctx.patterns.patterns[pat];
        pattern.index = value;
        ctx.output(OutputCategory::Confirm, format!("SET PATTERN {} INDEX TO {}", pat, value), &mut output);
    }
    Ok(())
}

pub use super::working::{
    handle_pn_here,
    handle_pn_next,
    handle_pn_prev,
};

pub fn handle_pn<F>(
    parts: &[&str],
    ctx: &mut crate::commands::context::ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::OutputCategory;

    if parts.len() < 3 {
        ctx.output(OutputCategory::Error, "ERROR: PN NEEDS PAT (0-5) AND IDX (0-63)".to_string(), &mut output);
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        ctx.output(OutputCategory::Error, "ERROR: PATTERN NUMBER MUST BE 0-5".to_string(), &mut output);
        return Ok(());
    }
    let idx: usize = if let Some((expr_val, _)) = eval_expression(&parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
        if expr_val < 0 || expr_val > 63 {
            ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
            return Ok(());
        }
        expr_val as usize
    } else {
        parts[2]
            .parse()
            .context("Failed to parse pattern index")?
    };
    if idx > 63 {
        ctx.output(OutputCategory::Error, "ERROR: PATTERN INDEX MUST BE 0-63".to_string(), &mut output);
        return Ok(());
    }
    if parts.len() == 3 {
        let pattern = &ctx.patterns.patterns[pat];
        ctx.output(OutputCategory::Query, format!("PN {} {} = {}", pat, idx, pattern.data[idx]), &mut output);
    } else {
        let val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
            expr_val
        } else {
            parts[3]
                .parse()
                .context("Failed to parse pattern value")?
        };
        let pattern = &mut ctx.patterns.patterns[pat];
        pattern.data[idx] = val;
        ctx.output(OutputCategory::Confirm, format!("SET PN {} {} TO {}", pat, idx, val), &mut output);
    }
    Ok(())
}
