use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_int_param, define_float_param};

define_int_param!(handle_fd, "fd", 1, 10000, "FD", "FM DECAY", "MS");
define_int_param!(handle_fa, "fa", 0, 16383, "FA", "FM ENV AMOUNT", "");
define_int_param!(handle_fmev_atk, "fmev_atk", 1, 10000, "FMEV.ATK", "FM ENV ATTACK", "MS");
define_float_param!(handle_fmev_crv, "fmev_crv", -8.0, 8.0, "FMEV.CRV", "FM ENV CURVE", "");
