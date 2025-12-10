use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::common::{define_bool_param, define_int_param, define_int_param_ms};

define_int_param!(handle_tk, "tk", 0, 16383, "TK", "TRACKING AMOUNT", "Failed to parse tracking amount");
define_int_param!(handle_mb, "mb", 0, 16383, "MB", "MOD BUS AMOUNT", "Failed to parse mod bus amount");
define_int_param!(handle_mba, "mba", 0, 16383, "MBA", "MOD BUS ENV AMOUNT", "Failed to parse mod bus env amount");
define_int_param_ms!(handle_mbd, "mbd", 1, 10000, "MBD", "MOD BUS ENV DECAY", "Failed to parse mod bus env decay");
define_bool_param!(handle_mp, "mp", "MP", "MOD -> PRIMARY FREQ", "Failed to parse mod -> primary value");
define_bool_param!(handle_md, "md", "MD", "MOD -> DISCONTINUITY", "Failed to parse mod -> discontinuity value");
define_bool_param!(handle_mt, "mt", "MT", "MOD -> TRACKING", "Failed to parse mod -> tracking value");
define_bool_param!(handle_ma, "ma", "MA", "MOD -> AMPLITUDE", "Failed to parse mod -> amplitude value");
define_int_param!(handle_fm, "fm", 0, 16383, "FM", "FM INDEX", "Failed to parse FM index");
define_int_param!(handle_mx, "mx", 0, 16383, "MX", "MIX AMOUNT", "Failed to parse mix amount");
define_bool_param!(handle_mm, "mm", "MM", "MOD BUS -> MIX", "Failed to parse mod bus -> mix value");
define_bool_param!(handle_me, "me", "ME", "ENVELOPE -> MIX", "Failed to parse envelope -> mix value");
