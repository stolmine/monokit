use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_float_param, define_int_param, define_int_param_ms, define_mode_param};

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

define_int_param_ms!(
    handle_na,
    "na",
    1,
    10000,
    "NA",
    "NOISE ENV ATTACK",
    "Failed to parse noise envelope attack"
);

define_int_param_ms!(
    handle_nd,
    "nd",
    1,
    10000,
    "ND",
    "NOISE ENV DECAY",
    "Failed to parse noise envelope decay"
);

define_float_param!(
    handle_nc,
    "nc",
    -8.0,
    8.0,
    "NC",
    "NOISE ENV CURVE",
    "Failed to parse noise envelope curve",
    ""
);

define_int_param!(
    handle_ne,
    "ne",
    0,
    16383,
    "NE",
    "NOISE ENV AMOUNT",
    "Failed to parse noise envelope amount"
);

define_int_param!(
    handle_np,
    "np",
    0,
    16383,
    "NP",
    "NOISE -> PRIMARY",
    "Failed to parse noise to primary"
);

define_int_param!(
    handle_nm,
    "nm",
    0,
    16383,
    "NM",
    "NOISE -> MODULATOR",
    "Failed to parse noise to modulator"
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

define_mode_param!(
    handle_ng,
    "ng",
    0,
    1,
    "NG",
    "NOISE GATE MUST BE 0 (DRONE) OR 1 (GATED)",
    "NOISE GATE",
    "Failed to parse noise gate"
);
