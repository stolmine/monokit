#[macro_export]
macro_rules! define_bool_toggle {
    ($fn_name:ident, $cmd_name:expr, $save_fn:path) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut bool,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= $crate::types::TIER_QUERIES {
                    output(format!("{}: {}", $cmd_name, if *state { "ON" } else { "OFF" }));
                }
            } else {
                match parts[1] {
                    "0" => {
                        *state = false;
                        let _ = $save_fn(*state);
                        if debug_level >= $crate::types::TIER_CONFIRMS {
                            output(format!("{}: OFF", $cmd_name));
                        }
                    }
                    "1" => {
                        *state = true;
                        let _ = $save_fn(*state);
                        if debug_level >= $crate::types::TIER_CONFIRMS {
                            output(format!("{}: ON", $cmd_name));
                        }
                    }
                    _ => output(format!("ERROR: {} TAKES 0 (OFF) OR 1 (ON)", $cmd_name)),
                }
            }
        }
    };
    ($fn_name:ident, $cmd_name:expr, $query_fmt:expr, $off_fmt:expr, $on_fmt:expr, $save_fn:path) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut bool,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= $crate::types::TIER_QUERIES {
                    output(if *state { $on_fmt.to_string() } else { $off_fmt.to_string() });
                }
            } else {
                match parts[1] {
                    "0" => {
                        *state = false;
                        let _ = $save_fn(*state);
                        if debug_level >= $crate::types::TIER_CONFIRMS {
                            output($off_fmt.to_string());
                        }
                    }
                    "1" => {
                        *state = true;
                        let _ = $save_fn(*state);
                        if debug_level >= $crate::types::TIER_CONFIRMS {
                            output($on_fmt.to_string());
                        }
                    }
                    _ => output(format!("ERROR: {} TAKES 0 (OFF) OR 1 (ON)", $cmd_name)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_enum_select {
    ($fn_name:ident, $cmd_name:expr, $save_fn:path, $err_msg:expr, $(($value:expr, $label:expr)),+ $(,)?) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut u8,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= $crate::types::TIER_QUERIES {
                    output(format!("{}: {}", $cmd_name, *state));
                }
            } else {
                let value = parts[1];
                match value {
                    $(
                        stringify!($value) => {
                            *state = $value;
                            let _ = $save_fn(*state);
                            if debug_level >= $crate::types::TIER_CONFIRMS {
                                output(format!("{}: {} ({})", $cmd_name, $value, $label));
                            }
                        }
                    )+
                    _ => output($err_msg.to_string()),
                }
            }
        }
    };
    ($fn_name:ident, $cmd_name:expr, $save_fn:path, $err_msg:expr, $($value:expr),+ $(,)?) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut u8,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= $crate::types::TIER_QUERIES {
                    output(format!("{}: {}", $cmd_name, *state));
                }
            } else if let Ok(val) = parts[1].parse::<u8>() {
                match val {
                    $(
                        $value => {
                            *state = $value;
                            let _ = $save_fn(*state);
                            if debug_level >= $crate::types::TIER_CONFIRMS {
                                output(format!("{}: {}", $cmd_name, $value));
                            }
                        }
                    )+
                    _ => output($err_msg.to_string()),
                }
            } else {
                output($err_msg.to_string());
            }
        }
    };
}
