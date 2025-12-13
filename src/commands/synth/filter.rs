use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_bool_param, define_float_param, define_int_param, define_int_param_ms, define_mode_param};

define_float_param!(handle_fc, "fc", 20.0, 20000.0, "FC", "FILTER CUTOFF", "Failed to parse filter cutoff frequency", "HZ");
define_int_param!(handle_fq, "fq", 0, 16383, "FQ", "FILTER RESONANCE", "Failed to parse filter resonance");
define_mode_param!(handle_ft, "ft", 0, 13, "FT", "FILTER TYPE MUST BE 0-13", "FILTER TYPE", "Failed to parse filter type");
define_int_param!(handle_fe, "fe", 0, 16383, "FE", "FILTER ENVELOPE AMOUNT", "Failed to parse filter envelope amount");
define_int_param_ms!(handle_fed, "fed", 1, 10000, "FED", "FILTER ENVELOPE DECAY", "Failed to parse filter envelope decay time");
define_int_param!(handle_fk, "fk", 0, 16383, "FK", "FILTER KEY TRACKING", "Failed to parse filter key tracking amount");
define_int_param!(handle_mff, "mf_f", 0, 16383, "MFF", "MOD->FILTER CUT AMOUNT", "Failed to parse mod to filter cutoff amount");
define_int_param!(handle_mfq, "mf_q", 0, 16383, "MFQ", "MOD->FILTER RES AMOUNT", "Failed to parse mod to filter resonance amount");
define_bool_param!(handle_mc, "mf_f", "MC", "MODBUS -> FILTER CUTOFF", "Failed to parse modbus to filter cutoff routing");
define_bool_param!(handle_mq, "mf_q", "MQ", "MODBUS -> FILTER RES", "Failed to parse modbus to filter resonance routing");
