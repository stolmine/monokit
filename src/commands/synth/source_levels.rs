use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::define_int_param;

define_int_param!(
    handle_pv,
    "pv",
    0,
    16383,
    "PV",
    "PRIMARY VOLUME",
    "Failed to parse primary volume"
);

define_int_param!(
    handle_mv,
    "mv",
    0,
    16383,
    "MV",
    "MODULATOR VOLUME",
    "Failed to parse modulator volume"
);
