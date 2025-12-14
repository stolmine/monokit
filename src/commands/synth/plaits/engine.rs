use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;
use super::super::common::define_mode_param;

define_mode_param!(
    handle_pl_eng,
    "engine",
    0,
    15,
    "PL.ENG",
    "RANGE 0-15",
    "PLAITS ENGINE",
    "Failed to parse engine value"
);
