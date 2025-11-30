use crate::eval::eval_expression;
use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables};

macro_rules! define_counter_read {
    ($fn_name:ident, $counter_idx:expr) => {
        pub fn $fn_name<F>(counters: &mut Counters, mut output: F)
        where
            F: FnMut(String),
        {
            let current = counters.values[$counter_idx];
            let min = counters.min[$counter_idx];
            let max = counters.max[$counter_idx];
            counters.values[$counter_idx] = if max == 0 {
                current.wrapping_add(1)
            } else {
                let next = current + 1;
                if next > max { min } else { next }
            };
            output(format!("{}", current));
        }
    };
}

macro_rules! define_counter_reset {
    ($fn_name:ident, $counter_name:literal, $counter_idx:expr) => {
        pub fn $fn_name<F>(counters: &mut Counters, mut output: F)
        where
            F: FnMut(String),
        {
            counters.values[$counter_idx] = counters.min[$counter_idx];
            output(format!("{} RESET TO {}", $counter_name, counters.min[$counter_idx]));
        }
    };
}

macro_rules! define_counter_max {
    ($fn_name:ident, $counter_name:literal, $counter_idx:expr) => {
        pub fn $fn_name<F>(
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
                output(format!("{}.MAX REQUIRES A VALUE", $counter_name));
                return;
            }

            let value: i16 = if let Some((expr_val, _)) = eval_expression(
                &parts, 1, variables, patterns, counters, scripts, script_index, scale
            ) {
                expr_val
            } else {
                match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        output(format!("ERROR: FAILED TO PARSE VALUE FOR {}.MAX", $counter_name));
                        return;
                    }
                }
            };

            if value > 0 && value < counters.min[$counter_idx] {
                output(format!(
                    "ERROR: MAX ({}) MUST BE >= MIN ({})",
                    value,
                    counters.min[$counter_idx]
                ));
                return;
            }

            counters.max[$counter_idx] = value;
            if value == 0 {
                output(format!("{}.MAX DISABLED (NO WRAP)", $counter_name));
            } else {
                output(format!("{}.MAX SET TO {}", $counter_name, value));
            }
        }
    };
}

macro_rules! define_counter_min {
    ($fn_name:ident, $counter_name:literal, $counter_idx:expr) => {
        pub fn $fn_name<F>(
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
                output(format!("{}.MIN REQUIRES A VALUE", $counter_name));
                return;
            }

            let value: i16 = if let Some((expr_val, _)) = eval_expression(
                &parts, 1, variables, patterns, counters, scripts, script_index, scale
            ) {
                expr_val
            } else {
                match parts[1].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        output(format!("ERROR: FAILED TO PARSE VALUE FOR {}.MIN", $counter_name));
                        return;
                    }
                }
            };

            counters.min[$counter_idx] = value;
            output(format!("{}.MIN SET TO {}", $counter_name, value));
        }
    };
}

macro_rules! define_counter {
    ($read_fn:ident, $rst_fn:ident, $max_fn:ident, $min_fn:ident, $name:literal, $idx:expr) => {
        define_counter_read!($read_fn, $idx);
        define_counter_reset!($rst_fn, $name, $idx);
        define_counter_max!($max_fn, $name, $idx);
        define_counter_min!($min_fn, $name, $idx);
    };
}

define_counter!(handle_n1, handle_n1_rst, handle_n1_max, handle_n1_min, "N1", 0);
define_counter!(handle_n2, handle_n2_rst, handle_n2_max, handle_n2_min, "N2", 1);
define_counter!(handle_n3, handle_n3_rst, handle_n3_max, handle_n3_min, "N3", 2);
define_counter!(handle_n4, handle_n4_rst, handle_n4_max, handle_n4_min, "N4", 3);
