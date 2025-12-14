#[macro_export]
macro_rules! define_pattern_op_1val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name), &mut output);
                return Ok(());
            }
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            define_pattern_op_1val!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, val, ctx, output);
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM AND VAL", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            define_pattern_op_1val!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), val, ctx, output);
            Ok(())
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $val:expr, $ctx:expr, $output:expr) => {
        {
            let msg = $crate::commands::patterns::common::$impl_fn($pat_ref, $val, $ctx.patterns);
            $ctx.output($crate::types::OutputCategory::Confirm, msg, $output);
        }
    };

    (@output result, $impl_fn:ident, $pat_ref:expr, $val:expr, $ctx:expr, $output:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $val, $ctx.patterns) {
            Ok(msg) => {
                $ctx.output($crate::types::OutputCategory::Confirm, msg, $output);
            }
            Err(e) => {
                $ctx.output($crate::types::OutputCategory::Error, e.to_string(), $output);
            }
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_noarg {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_op_noarg!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, ctx, output, $cmd_name);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_op_noarg!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), ctx, output, $cmd_name);
            Ok(())
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $ctx:expr, $output:expr, $cmd_name:literal) => {
        let pat_idx = $crate::commands::patterns::common::$impl_fn($pat_ref, $ctx.patterns);
        $ctx.output($crate::types::OutputCategory::Confirm, format!(concat!(stringify!($impl_fn), " PATTERN {}"), pat_idx).replace("_impl", "").to_uppercase(), $output);
    };

    (@output result_idx, $impl_fn:ident, $pat_ref:expr, $ctx:expr, $output:expr, $cmd_name:literal) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $ctx.patterns) {
            Ok(pat_idx) => {
                $ctx.output($crate::types::OutputCategory::Confirm, format!(concat!(stringify!($impl_fn), " PATTERN {}"), pat_idx).replace("_impl", "").to_uppercase(), $output);
            }
            Err(e) => {
                $ctx.output($crate::types::OutputCategory::Error, e.to_string(), $output);
            }
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_2val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $err_msg:literal, $val_type:ty) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: P.{} REQUIRES {}", $cmd_name, $err_msg), &mut output);
                return Ok(());
            }
            let val1 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
                expr_val as $val_type
            } else {
                parts[1].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            let val2 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
                expr_val as $val_type
            } else {
                parts[2].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, val1, val2, ctx.patterns) {
                Ok(msg) => {
                    ctx.output($crate::types::OutputCategory::Confirm, msg, &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e.to_string(), &mut output);
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 4 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM, {}", $cmd_name, $err_msg), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val1 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
                expr_val as $val_type
            } else {
                parts[2].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            let val2 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 3, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale) {
                expr_val as $val_type
            } else {
                parts[3].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), val1, val2, ctx.patterns) {
                Ok(msg) => {
                    ctx.output($crate::types::OutputCategory::Confirm, msg, &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e.to_string(), &mut output);
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
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: P.{} REQUIRES INDEX AND VALUE", $cmd_name), &mut output);
                return Ok(());
            }
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, idx, val, ctx.patterns) {
                Ok(msg) => {
                    ctx.output($crate::types::OutputCategory::Confirm, msg, &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e, &mut output);
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 4 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM, IDX, VAL", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 3, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), idx, val, ctx.patterns) {
                Ok(msg) => {
                    ctx.output($crate::types::OutputCategory::Confirm, msg, &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e, &mut output);
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
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: P.{} REQUIRES AN INDEX", $cmd_name), &mut output);
                return Ok(());
            }
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, idx, ctx.patterns) {
                Ok(msg) => {
                    ctx.output($crate::types::OutputCategory::Confirm, msg, &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e, &mut output);
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM AND IDX", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), idx, ctx.patterns) {
                Ok(msg) => {
                    ctx.output($crate::types::OutputCategory::Confirm, msg, &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e, &mut output);
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
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_query!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, ctx, output, $cmd_name);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_query!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), ctx, output, $cmd_name);
            Ok(())
        }
    };

    (@output result, $impl_fn:ident, $pat_ref:expr, $ctx:expr, $output:expr, $cmd_name:literal) => {
        {
            match $crate::commands::patterns::common::$impl_fn($pat_ref, $ctx.patterns) {
                Ok(val) => {
                    let msg = format!("P.{} = {}", $cmd_name, val);
                    $ctx.output($crate::types::OutputCategory::Query, msg, $output);
                }
                Err(e) => {
                    $ctx.output($crate::types::OutputCategory::Error, e.to_string(), $output);
                }
            }
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $ctx:expr, $output:expr, $cmd_name:literal) => {
        {
            let val = $crate::commands::patterns::common::$impl_fn($pat_ref, $ctx.patterns);
            let msg = format!("P.{} = {}", $cmd_name, val);
            $ctx.output($crate::types::OutputCategory::Query, msg, $output);
        }
    };
}

#[macro_export]
macro_rules! define_pattern_query_1val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name), &mut output);
                return Ok(());
            }
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            let index = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, val, ctx.patterns);
            ctx.output($crate::types::OutputCategory::Query, format!("P.{} = {}", $cmd_name, index), &mut output);
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM AND VAL", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
            let index = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), val, ctx.patterns);
            ctx.output($crate::types::OutputCategory::Query, format!("PN.{} = {}", $cmd_name, index), &mut output);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_nav {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $nav_type:tt) => {
        pub fn $working_fn<F>(
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_nav!(@output $nav_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, ctx, output, $cmd_name);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_nav!(@output $nav_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), ctx, output, $cmd_name);
            Ok(())
        }
    };

    (@output here, $impl_fn:ident, $pat_ref:expr, $ctx:expr, $output:expr, $cmd_name:literal) => {
        let (value, _pat_idx) = $crate::commands::patterns::common::$impl_fn($pat_ref, $ctx.patterns);
        $ctx.output($crate::types::OutputCategory::Query, format!("P.{} = {}", $cmd_name, value), $output);
    };

    (@output nav, $impl_fn:ident, $pat_ref:expr, $ctx:expr, $output:expr, $cmd_name:literal) => {
        let (value, _pat_idx, new_index) = $crate::commands::patterns::common::$impl_fn($pat_ref, $ctx.patterns);
        $ctx.output($crate::types::OutputCategory::Query, format!("P.{} = {} (INDEX NOW {})", $cmd_name, value, new_index), $output);
    };
}

#[macro_export]
macro_rules! define_pattern_pop {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, ctx.patterns) {
                Ok((val, _pat_idx)) => {
                    ctx.output($crate::types::OutputCategory::Query, format!("P.{} = {}", $cmd_name, val), &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e.to_string(), &mut output);
                }
            }
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), ctx.patterns) {
                Ok((val, _pat_idx)) => {
                    ctx.output($crate::types::OutputCategory::Query, format!("PN.{} = {}", $cmd_name, val), &mut output);
                }
                Err(e) => {
                    ctx.output($crate::types::OutputCategory::Error, e.to_string(), &mut output);
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
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            let (min, max) = if parts.len() >= 3 {
                let min_val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
                let max_val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
                (min_val, max_val)
            } else {
                (0, 127)
            };
            let pat_idx = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, min, max, ctx.patterns);
            ctx.output($crate::types::OutputCategory::Confirm, format!("RANDOMIZED PAT {} (RANGE {}-{})", pat_idx, min, max), &mut output);
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            ctx: &mut $crate::commands::context::ExecutionContext,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                ctx.output($crate::types::OutputCategory::Error, format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name), &mut output);
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale, &mut output, *ctx.debug_level, *ctx.out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let (min, max) = if parts.len() >= 4 {
                let min_val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
                let max_val = $crate::commands::patterns::common::parse_i16_expr(parts, 3, ctx.variables, ctx.patterns, ctx.counters, ctx.scripts, ctx.script_index, ctx.scale)?;
                (min_val, max_val)
            } else {
                (0, 127)
            };
            let pat_idx = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), min, max, ctx.patterns);
            ctx.output($crate::types::OutputCategory::Confirm, format!("RANDOMIZED PAT {} (RANGE {}-{})", pat_idx, min, max), &mut output);
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
