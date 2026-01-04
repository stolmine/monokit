use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::define_int_param;

// Volume controls (0-16383)
define_int_param!(
    handle_vol_osc,
    "vol_osc",
    0,
    16383,
    "VOL.OSC",
    "COMPLEX OSC VOLUME",
    "Failed to parse complex osc volume"
);

define_int_param!(
    handle_vol_pla,
    "vol_pla",
    0,
    16383,
    "VOL.PLA",
    "PLAITS VOLUME",
    "Failed to parse Plaits volume"
);

define_int_param!(
    handle_vol_nos,
    "vol_nos",
    0,
    16383,
    "VOL.NOS",
    "NOISE VOLUME",
    "Failed to parse noise volume"
);

define_int_param!(
    handle_vol_smp,
    "vol_smp",
    0,
    16383,
    "VOL.SMP",
    "SAMPLER VOLUME",
    "Failed to parse sampler volume"
);

// Pan controls (-8192 to 8191, center=0)
define_int_param!(
    handle_pan_osc,
    "pan_osc",
    -8192,
    8191,
    "PAN.OSC",
    "COMPLEX OSC PAN",
    "Failed to parse complex osc pan"
);

define_int_param!(
    handle_pan_pla,
    "pan_pla",
    -8192,
    8191,
    "PAN.PLA",
    "PLAITS PAN",
    "Failed to parse Plaits pan"
);

define_int_param!(
    handle_pan_nos,
    "pan_nos",
    -8192,
    8191,
    "PAN.NOS",
    "NOISE PAN",
    "Failed to parse noise pan"
);

define_int_param!(
    handle_pan_smp,
    "pan_smp",
    -8192,
    8191,
    "PAN.SMP",
    "SAMPLER PAN",
    "Failed to parse sampler pan"
);

// Mute controls (0/1)
define_int_param!(
    handle_mute_osc,
    "mute_osc",
    0,
    1,
    "MUTE.OSC",
    "MUTE COMPLEX OSC",
    "Failed to parse complex osc mute"
);

define_int_param!(
    handle_mute_pla,
    "mute_pla",
    0,
    1,
    "MUTE.PLA",
    "MUTE PLAITS",
    "Failed to parse Plaits mute"
);

define_int_param!(
    handle_mute_nos,
    "mute_nos",
    0,
    1,
    "MUTE.NOS",
    "MUTE NOISE",
    "Failed to parse noise mute"
);

define_int_param!(
    handle_mute_smp,
    "mute_smp",
    0,
    1,
    "MUTE.SMP",
    "MUTE SAMPLER",
    "Failed to parse sampler mute"
);
