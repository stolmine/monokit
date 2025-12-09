use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_int_param, define_float_param};

define_int_param!(handle_da, "da", 0, 16383, "DA", "DISC ENV AMOUNT", "");
define_int_param!(handle_denv_atk, "denv_atk", 1, 10000, "DENV.ATK", "DISC ENV ATTACK", "MS");
define_float_param!(handle_denv_crv, "denv_crv", -8.0, 8.0, "DENV.CRV", "DISC ENV CURVE", "");
