#[macro_export]
macro_rules! define_pattern_op_1val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name));
                }
                return Ok(());
            }
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            define_pattern_op_1val!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, val, patterns, output, debug_level, out_err, out_cfm);
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM AND VAL", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            define_pattern_op_1val!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), val, patterns, output, debug_level, out_err, out_cfm);
            Ok(())
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $val:expr, $patterns:expr, $output:expr, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
            $output($crate::commands::patterns::common::$impl_fn($pat_ref, $val, $patterns));
        }
    };

    (@output result, $impl_fn:ident, $pat_ref:expr, $val:expr, $patterns:expr, $output:expr, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $val, $patterns) {
            Ok(msg) => {
                if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
                    $output(msg);
                }
            }
            Err(e) => {
                if $debug_level >= $crate::types::TIER_ERRORS || $out_err {
                    $output(e.to_string());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_noarg {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $result_type:tt) => {
        pub fn $working_fn<F>(
            patterns: &mut $crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_op_noarg!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, patterns, output, $cmd_name, debug_level, out_err, out_cfm);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_op_noarg!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), patterns, output, $cmd_name, debug_level, out_err, out_cfm);
            Ok(())
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        let pat_idx = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
            $output(format!(concat!(stringify!($impl_fn), " PATTERN {}"), pat_idx).replace("_impl", "").to_uppercase());
        }
    };

    (@output result_idx, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_cfm:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns) {
            Ok(pat_idx) => {
                if $debug_level >= $crate::types::TIER_CONFIRMS || $out_cfm {
                    $output(format!(concat!(stringify!($impl_fn), " PATTERN {}"), pat_idx).replace("_impl", "").to_uppercase());
                }
            }
            Err(e) => {
                if $debug_level >= $crate::types::TIER_ERRORS || $out_err {
                    $output(e.to_string());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_pattern_op_2val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $err_msg:literal, $val_type:ty) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES {}", $cmd_name, $err_msg));
                }
                return Ok(());
            }
            let val1 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[1].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            let val2 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[2].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, val1, val2, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 4 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM, {}", $cmd_name, $err_msg));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val1 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[2].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            let val2 = if let Some((expr_val, _)) = $crate::eval::eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index, scale) {
                expr_val as $val_type
            } else {
                parts[3].parse().context(concat!("Failed to parse ", $err_msg))?
            };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), val1, val2, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
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
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES INDEX AND VALUE", $cmd_name));
                }
                return Ok(());
            }
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, idx, val, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 4 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM, IDX, VAL", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 3, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), idx, val, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
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
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES AN INDEX", $cmd_name));
                }
                return Ok(());
            }
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, idx, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
                }
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM AND IDX", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let idx = $crate::commands::patterns::common::parse_usize_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), idx, patterns) {
                Ok(msg) => {
                    if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                        output(msg);
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e);
                    }
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
            patterns: &$crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_query!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, patterns, output, $cmd_name, debug_level, out_err, out_qry);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_query!(@output $result_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), patterns, output, $cmd_name, debug_level, out_err, out_qry);
            Ok(())
        }
    };

    (@output result, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_qry:expr) => {
        match $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns) {
            Ok(val) => {
                if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
                    $output(format!("P.{} = {}", $cmd_name, val));
                }
            }
            Err(e) => {
                if $debug_level >= $crate::types::TIER_ERRORS || $out_err {
                    $output(e.to_string());
                }
            }
        }
    };

    (@output direct, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_err:expr, $out_qry:expr) => {
        let val = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
            $output(format!("P.{} = {}", $cmd_name, val));
        }
    };
}

#[macro_export]
macro_rules! define_pattern_query_1val {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: P.{} REQUIRES A VALUE", $cmd_name));
                }
                return Ok(());
            }
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
            let index = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, val, patterns);
            if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                output(format!("P.{} = {}", $cmd_name, index));
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 3 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM AND VAL", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
            let index = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), val, patterns);
            if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                output(format!("PN.{} = {}", $cmd_name, index));
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! define_pattern_nav {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal, $nav_type:tt) => {
        pub fn $working_fn<F>(
            patterns: &mut $crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            define_pattern_nav!(@output $nav_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Working, patterns, output, $cmd_name, debug_level, out_qry);
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            define_pattern_nav!(@output $nav_type, $impl_fn, $crate::commands::patterns::common::PatternRef::Explicit(pat), patterns, output, $cmd_name, debug_level, out_qry);
            Ok(())
        }
    };

    (@output here, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_qry:expr) => {
        let (value, _pat_idx) = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
            $output(format!("P.{} = {}", $cmd_name, value));
        }
    };

    (@output nav, $impl_fn:ident, $pat_ref:expr, $patterns:expr, $output:expr, $cmd_name:literal, $debug_level:expr, $out_qry:expr) => {
        let (value, _pat_idx, new_index) = $crate::commands::patterns::common::$impl_fn($pat_ref, $patterns);
        if $debug_level >= $crate::types::TIER_QUERIES || $out_qry {
            $output(format!("P.{} = {} (INDEX NOW {})", $cmd_name, value, new_index));
        }
    };
}

#[macro_export]
macro_rules! define_pattern_pop {
    ($working_fn:ident, $explicit_fn:ident, $impl_fn:ident, $cmd_name:literal) => {
        pub fn $working_fn<F>(
            patterns: &mut $crate::types::PatternStorage,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, patterns) {
                Ok((val, _pat_idx)) => {
                    if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                        output(format!("P.{} = {}", $cmd_name, val));
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
                }
            }
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            match $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), patterns) {
                Ok((val, _pat_idx)) => {
                    if debug_level >= $crate::types::TIER_QUERIES || out_qry {
                        output(format!("PN.{} = {}", $cmd_name, val));
                    }
                }
                Err(e) => {
                    if debug_level >= $crate::types::TIER_ERRORS || out_err {
                        output(e.to_string());
                    }
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
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            let (min, max) = if parts.len() >= 3 {
                let min_val = $crate::commands::patterns::common::parse_i16_expr(parts, 1, variables, patterns, counters, scripts, script_index, scale)?;
                let max_val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
                (min_val, max_val)
            } else {
                (0, 127)
            };
            let pat_idx = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Working, min, max, patterns);
            if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("RANDOMIZED PAT {} (RANGE {}-{})", pat_idx, min, max));
            }
            Ok(())
        }

        pub fn $explicit_fn<F>(
            parts: &[&str],
            variables: &$crate::types::Variables,
            patterns: &mut $crate::types::PatternStorage,
            counters: &mut $crate::types::Counters,
            scripts: &$crate::types::ScriptStorage,
            script_index: usize,
            scale: &$crate::types::ScaleState,
            debug_level: u8,
            out_err: bool,
            out_qry: bool,
            out_cfm: bool,
            mut output: F,
        ) -> $crate::anyhow::Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                if debug_level >= $crate::types::TIER_ERRORS || out_err {
                    output(format!("ERROR: PN.{} NEEDS PAT NUM", $cmd_name));
                }
                return Ok(());
            }
            let pat = $crate::commands::patterns::common::parse_pattern_num(parts, 1, variables, patterns, counters, scripts, script_index, scale, &mut output, debug_level, out_err)?;
            let pat = match pat { Some(p) => p, None => return Ok(()) };
            let (min, max) = if parts.len() >= 4 {
                let min_val = $crate::commands::patterns::common::parse_i16_expr(parts, 2, variables, patterns, counters, scripts, script_index, scale)?;
                let max_val = $crate::commands::patterns::common::parse_i16_expr(parts, 3, variables, patterns, counters, scripts, script_index, scale)?;
                (min_val, max_val)
            } else {
                (0, 127)
            };
            let pat_idx = $crate::commands::patterns::common::$impl_fn($crate::commands::patterns::common::PatternRef::Explicit(pat), min, max, patterns);
            if debug_level >= $crate::types::TIER_CONFIRMS || out_cfm {
                output(format!("RANDOMIZED PAT {} (RANGE {}-{})", pat_idx, min, max));
            }
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
