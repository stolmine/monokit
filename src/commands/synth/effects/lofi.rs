use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::define_int_param;

define_int_param!(handle_lb, "lb", 1, 16, "LB", "LO-FI BIT DEPTH", "Failed to parse bit depth");
define_int_param!(handle_ls, "ls", 100, 48000, "LS", "LO-FI SAMPLE RATE", "Failed to parse sample rate");
define_int_param!(handle_lm, "lm", 0, 16383, "LM", "LO-FI MIX", "Failed to parse lo-fi mix");
