use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_float_param, define_int_param, define_int_param_ms, define_mode_param};

define_float_param!(handle_pf, "pf", 20.0, 20000.0, "PF", "PRIMARY FREQUENCY", "Failed to parse frequency value", "HZ");
define_mode_param!(handle_pw, "pw", 0, 2, "PW", "RANGE 0-2", "PRIMARY WAVEFORM", "Failed to parse waveform value");
define_float_param!(handle_mf, "mf", 20.0, 20000.0, "MF", "MOD FREQUENCY", "Failed to parse frequency value", "HZ");
define_mode_param!(handle_mw, "mw", 0, 3, "MW", "RANGE 0-3", "MOD WAVEFORM", "Failed to parse waveform value");
define_int_param!(handle_fb, "fb", 0, 16383, "FB", "FEEDBACK AMOUNT", "Failed to parse feedback amount");
define_int_param!(handle_fba, "fba", 0, 16383, "FBA", "FEEDBACK ENVELOPE AMOUNT", "Failed to parse feedback envelope amount");
define_int_param_ms!(handle_fbd, "fbd", 1, 10000, "FBD", "FEEDBACK DECAY", "Failed to parse feedback decay time");
