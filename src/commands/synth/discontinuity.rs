use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;
use super::common::{define_int_param, define_int_param_ms, define_mode_param};

// DC - Discontinuity amount (0-16383)
define_int_param!(handle_dc, "dc", 0, 16383, "DC", "DISCONTINUITY AMOUNT", "Failed to parse discontinuity amount");

define_mode_param!(handle_dm, "dm", 0, 6, "DM", "RANGE 0-6", "DISCONTINUITY MODE", "Failed to parse discontinuity mode");
define_int_param_ms!(handle_dd, "dd", 1, 10000, "DD", "DISCONTINUITY DECAY", "Failed to parse discontinuity decay time");
