use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_int_param, define_mode_param};

define_mode_param!(
    handle_nw,
    "nw",
    0,
    2,
    "NW",
    "NOISE TYPE MUST BE 0 (WHITE), 1 (PINK), OR 2 (BROWN)",
    "NOISE TYPE",
    "Failed to parse noise type"
);

define_int_param!(
    handle_nv,
    "nv",
    0,
    16383,
    "NV",
    "NOISE VOLUME",
    "Failed to parse noise volume"
);
