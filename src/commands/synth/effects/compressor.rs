use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_float_param, define_int_param, define_int_param_ms};

define_int_param!(handle_ct, "ct", 0, 16383, "CT", "COMPRESSOR THRESHOLD", "Failed to parse compressor threshold");
define_float_param!(handle_cr, "cr", 1.0, 20.0, "CR", "COMPRESSOR RATIO", "Failed to parse compressor ratio", "");
define_int_param_ms!(handle_ca, "ca", 1, 500, "CA", "COMPRESSOR ATTACK", "Failed to parse compressor attack");
define_int_param_ms!(handle_cl, "cl", 10, 2000, "CL", "COMPRESSOR RELEASE", "Failed to parse compressor release");
define_int_param!(handle_cm, "cm", 0, 16383, "CM", "COMPRESSOR MAKEUP GAIN", "Failed to parse compressor makeup gain");
