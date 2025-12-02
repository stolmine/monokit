use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_int_param, define_float_param};

define_int_param!(handle_pd, "pd", 1, 10000, "PD", "PITCH DECAY", "MS");
define_float_param!(handle_pa, "pa", 0.0, 16.0, "PA", "PITCH ENV AMOUNT", "");
define_int_param!(handle_penv_atk, "penv_atk", 1, 10000, "PENV.ATK", "PITCH ENV ATTACK", "MS");
define_float_param!(handle_penv_crv, "penv_crv", -8.0, 8.0, "PENV.CRV", "PITCH ENV CURVE", "");
