use crate::eval::eval_expression;
use crate::types::{Counters, FxMixState, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_float_param, define_fx_mix_param, define_int_param};

define_float_param!(handle_rgf, "rgf", 20.0, 2000.0, "RGF", "RING MOD FREQUENCY", "Failed to parse ring mod frequency", "HZ");
define_int_param!(handle_rgw, "rgw", 0, 3, "RGW", "RING MOD WAVEFORM", "Failed to parse ring mod waveform");
define_fx_mix_param!(handle_rgm, "rgm", ring_mix, "SET RING MOD MIX TO {}");

