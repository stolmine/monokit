use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::define_float_param;

define_float_param!(handle_el, "el", -24.0, 24.0, "EL", "EQ LOW SHELF", "Failed to parse EQ low shelf dB", "DB");
define_float_param!(handle_elf, "elf", 20.0, 2000.0, "ELF", "EQ LOW SHELF FREQ", "Failed to parse EQ low shelf frequency", "HZ");
define_float_param!(handle_em, "em", -24.0, 24.0, "EM", "EQ MID PEAK", "Failed to parse EQ mid peak dB", "DB");
define_float_param!(handle_ef, "ef", 200.0, 8000.0, "EF", "EQ MID FREQUENCY", "Failed to parse EQ mid frequency", "HZ");
define_float_param!(handle_eq_param, "eq", 0.1, 10.0, "EQ", "EQ MID Q", "Failed to parse EQ mid Q", "");
define_float_param!(handle_eh, "eh", -24.0, 24.0, "EH", "EQ HIGH SHELF", "Failed to parse EQ high shelf dB", "DB");
define_float_param!(handle_ehf, "ehf", 1000.0, 20000.0, "EHF", "EQ HIGH SHELF FREQ", "Failed to parse EQ high shelf frequency", "HZ");
