use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_bool_param, define_float_param, define_int_param, define_int_param_ms, define_mode_param_with_names};

define_mode_param_with_names!(handle_d_mode, "dmode", 0, 2, "D.MODE", "MODE MUST BE 0 (BYPASS), 1 (INSERT), OR 2 (SEND)", "DELAY MODE", "Failed to parse delay mode", &["BYPASS", "INSERT", "SEND"]);
define_mode_param_with_names!(handle_d_tail, "dtail", 0, 2, "D.TAIL", "TAIL MUST BE 0 (CUT), 1 (RING), OR 2 (FREEZE)", "DELAY TAIL", "Failed to parse delay tail mode", &["CUT", "RING", "FREEZE"]);
define_int_param_ms!(handle_dt, "dt", 1, 2000, "DT", "DELAY TIME", "Failed to parse delay time");
define_int_param!(handle_df, "df", 0, 16383, "DF", "DELAY FEEDBACK", "Failed to parse delay feedback");
define_float_param!(handle_dlp, "dlp", 100.0, 20000.0, "DLP", "DELAY DAMPING", "Failed to parse delay damping frequency", "HZ");
define_int_param!(handle_dw, "dw", 0, 16383, "DW", "DELAY WET MIX", "Failed to parse delay wet mix");
define_bool_param!(handle_ds, "ds", "DS", "DELAY SYNC", "Failed to parse delay sync");
