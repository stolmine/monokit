use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, MixerData, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

macro_rules! define_mixer_param {
    ($fn_name:ident, $field:ident, $osc_param:expr, $min:expr, $max:expr, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            variables: &Variables,
            patterns: &mut PatternStorage,
            counters: &mut Counters,
            scripts: &ScriptStorage,
            script_index: usize,
            metro_tx: &Sender<MetroCommand>,
            debug_level: u8,
            scale: &ScaleState,
            out_cfm: bool,
            mixer_data: &mut MixerData,
            mut output: F,
        ) -> Result<()>
        where
            F: FnMut(String),
        {
            if parts.len() < 2 {
                output(format!("{}: REQUIRES VALUE", $error_cmd));
                return Ok(());
            }
            let state_snapshot = (
                patterns.toggle_state.clone(),
                patterns.toggle_last_value.clone()
            );
            let value: i32 = if let Some((expr_val, consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
                if consumed > 0 && parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let key = format!("{}_{}", script_index, parts[1..1+consumed].join("_"));
                        patterns.direct_validation.insert(key, true);
                    }
                }
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context($parse_ctx)?
            };
            if value < $min || value > $max {
                patterns.toggle_state = state_snapshot.0;
                patterns.toggle_last_value = state_snapshot.1;
                if parts.len() > 1 {
                    let op = parts[1].to_uppercase();
                    if op == "TOG" || op == "EITH" || op.starts_with("SEQ") {
                        let end_idx = parts.len().min(4);
                        let key = format!("{}_{}", script_index, parts[1..end_idx].join("_"));
                        patterns.direct_validation.insert(key, false);
                    }
                }
                output(format!("{}: RANGE {}-{}", $error_cmd, $min, $max));
                return Ok(());
            }
            mixer_data.$field = value;
            metro_tx
                .send(MetroCommand::SendParam($osc_param.to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("SET {} TO {}", $display_name, value));
            }
            Ok(())
        }
    };
}

define_mixer_param!(handle_vol_osc, vol_osc, "vol_osc", 0, 16383, "VOL.OSC", "COMPLEX OSC VOLUME", "Failed to parse complex osc volume");
define_mixer_param!(handle_vol_pla, vol_pla, "vol_pla", 0, 16383, "VOL.PLA", "PLAITS VOLUME", "Failed to parse Plaits volume");
define_mixer_param!(handle_vol_nos, vol_nos, "vol_nos", 0, 16383, "VOL.NOS", "NOISE VOLUME", "Failed to parse noise volume");
define_mixer_param!(handle_vol_smp, vol_smp, "vol_smp", 0, 16383, "VOL.SMP", "SAMPLER VOLUME", "Failed to parse sampler volume");

define_mixer_param!(handle_pan_osc, pan_osc, "pan_osc", -8192, 8191, "PAN.OSC", "COMPLEX OSC PAN", "Failed to parse complex osc pan");
define_mixer_param!(handle_pan_pla, pan_pla, "pan_pla", -8192, 8191, "PAN.PLA", "PLAITS PAN", "Failed to parse Plaits pan");
define_mixer_param!(handle_pan_nos, pan_nos, "pan_nos", -8192, 8191, "PAN.NOS", "NOISE PAN", "Failed to parse noise pan");
define_mixer_param!(handle_pan_smp, pan_smp, "pan_smp", -8192, 8191, "PAN.SMP", "SAMPLER PAN", "Failed to parse sampler pan");

define_mixer_param!(handle_mute_osc, mute_osc, "mute_osc", 0, 1, "MUTE.OSC", "MUTE COMPLEX OSC", "Failed to parse complex osc mute");
define_mixer_param!(handle_mute_pla, mute_pla, "mute_pla", 0, 1, "MUTE.PLA", "MUTE PLAITS", "Failed to parse Plaits mute");
define_mixer_param!(handle_mute_nos, mute_nos, "mute_nos", 0, 1, "MUTE.NOS", "MUTE NOISE", "Failed to parse noise mute");
define_mixer_param!(handle_mute_smp, mute_smp, "mute_smp", 0, 1, "MUTE.SMP", "MUTE SAMPLER", "Failed to parse sampler mute");
