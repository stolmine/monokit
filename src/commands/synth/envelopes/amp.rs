use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_int_param, define_float_param};

define_int_param!(handle_ad, "ad", 1, 10000, "AD", "AMP DECAY", "MS");
define_int_param!(handle_aenv_atk, "aenv_atk", 1, 10000, "AENV.ATK", "AMP ENV ATTACK", "MS");
define_float_param!(handle_aenv_crv, "aenv_crv", -8.0, 8.0, "AENV.CRV", "AMP ENV CURVE", "");
