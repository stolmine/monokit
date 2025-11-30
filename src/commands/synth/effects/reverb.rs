use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_int_param, define_int_param_ms, define_mode_param_with_names};

define_mode_param_with_names!(handle_r_mode, "rmode", 0, 2, "R.MODE", "MODE MUST BE 0 (BYPASS), 1 (INSERT), OR 2 (SEND)", "REVERB MODE", "Failed to parse reverb mode", &["BYPASS", "INSERT", "SEND"]);
define_mode_param_with_names!(handle_r_tail, "rtail", 0, 2, "R.TAIL", "TAIL MUST BE 0 (CUT), 1 (RING), OR 2 (FREEZE)", "REVERB TAIL", "Failed to parse reverb tail mode", &["CUT", "RING", "FREEZE"]);
define_int_param!(handle_rv, "rv", 0, 16383, "RV", "REVERB DECAY", "Failed to parse reverb decay");
define_int_param_ms!(handle_rp, "rp", 0, 100, "RP", "REVERB PRE-DELAY", "Failed to parse reverb pre-delay");
define_int_param!(handle_rh, "rh", 0, 16383, "RH", "REVERB DAMPING", "Failed to parse reverb damping");
define_int_param!(handle_rw, "rw", 0, 16383, "RW", "REVERB WET MIX", "Failed to parse reverb wet mix");
