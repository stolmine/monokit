use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_float_param, define_int_param, define_int_param_ms};

define_float_param!(handle_rf, "rf", 20.0, 5000.0, "RF", "RESONATOR FREQUENCY", "Failed to parse resonator frequency value", "HZ");
define_int_param_ms!(handle_rd, "rd", 10, 5000, "RD", "RESONATOR DECAY", "Failed to parse resonator decay time");
define_int_param!(handle_rm, "rm", 0, 16383, "RM", "RESONATOR MIX", "Failed to parse resonator mix amount");
define_int_param!(handle_rk, "rk", 0, 16383, "RK", "RESONATOR KEY TRACKING", "Failed to parse resonator key tracking amount");
