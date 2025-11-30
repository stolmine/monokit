use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_int_param, define_float_param};

define_int_param!(handle_flev_atk, "flev_atk", 1, 10000, "FLEV.ATK", "FILTER ENV ATTACK", "MS");
define_float_param!(handle_flev_crv, "flev_crv", -8.0, 8.0, "FLEV.CRV", "FILTER ENV CURVE", "");
