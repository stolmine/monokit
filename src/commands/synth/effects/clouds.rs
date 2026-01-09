use crate::commands::context::ExecutionContext;
use crate::eval::eval_expression;
use crate::output::OutputDecider;
use crate::types::{Counters, FxMixState, MetroCommand, OutputCategory, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

use super::super::common::{define_bool_param, define_fx_mix_param, define_int_param, define_mode_param};

define_fx_mix_param!(handle_cl_wet, "cl_wet", clouds_wet, "SET CLOUDS WET MIX TO {}");

// CL.TRIG - Trigger grain playback
pub fn handle_cl_trig(
    ctx: &mut ExecutionContext,
    output: &mut Vec<String>,
) -> Result<()> {
    ctx.metro_tx.send(MetroCommand::SendParam(
        "t_cl_trig".to_string(),
        OscType::Int(1),
    ))?;

    ctx.output(OutputCategory::Confirm, "GRAINS TRIGGERED".to_string(), |msg| {
        output.push(msg);
    });

    Ok(())
}

// CL.PITCH / CLP / CLPT - Grain pitch (0-16383, 8192=center)
define_int_param!(handle_cl_pitch, "cl_pitch", 0, 16383, "CL.PITCH", "CLOUDS PITCH", "Failed to parse clouds pitch");

// CL.POS / CLO / CLPS - Buffer position (0-16383)
define_int_param!(handle_cl_pos, "cl_pos", 0, 16383, "CL.POS", "CLOUDS POSITION", "Failed to parse clouds position");

// CL.SIZE / CLS / CLSZ - Grain size (0-16383)
define_int_param!(handle_cl_size, "cl_size", 0, 16383, "CL.SIZE", "CLOUDS GRAIN SIZE", "Failed to parse clouds grain size");

// CL.DENS / CLD / CLDS - Grain density (0-16383)
define_int_param!(handle_cl_dens, "cl_dens", 0, 16383, "CL.DENS", "CLOUDS DENSITY", "Failed to parse clouds density");

// CL.TEX / CLT / CLTX - Grain texture (0-16383)
define_int_param!(handle_cl_tex, "cl_tex", 0, 16383, "CL.TEX", "CLOUDS TEXTURE", "Failed to parse clouds texture");

// CL.GAIN / CLG - Input gain (0-16383, 8192=unity)
define_int_param!(handle_cl_gain, "cl_gain", 0, 16383, "CL.GAIN", "CLOUDS INPUT GAIN", "Failed to parse clouds input gain");

// CL.SPREAD / CLSP - Stereo spread (0-16383)
define_int_param!(handle_cl_spread, "cl_spread", 0, 16383, "CL.SPREAD", "CLOUDS STEREO SPREAD", "Failed to parse clouds stereo spread");

// CL.RVB / CLR / CLRV - Internal reverb (0-16383)
define_int_param!(handle_cl_rvb, "cl_rvb", 0, 16383, "CL.RVB", "CLOUDS REVERB", "Failed to parse clouds reverb");

// CL.FB / CLF - Feedback (0-16383, >10000 risky!)
pub fn handle_cl_fb<F>(
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
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("CL.FB: REQUIRES VALUE".to_string());
        return Ok(());
    }
    let value: i32 = if let Some((expr_val, _consumed)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as i32
    } else {
        parts[1].parse().context("Failed to parse clouds feedback")?
    };
    if value < 0 || value > 16383 {
        output("CL.FB: RANGE 0-16383".to_string());
        return Ok(());
    }
    metro_tx.send(MetroCommand::SendParam("cl_fb".to_string(), OscType::Int(value)))?;

    // Warning for high feedback values
    if value > 10000 {
        output("WARNING: CL.FB >10000 MAY SELF-OSCILLATE".to_string());
    }

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET CLOUDS FEEDBACK TO {}", value));
    }
    Ok(())
}

// CL.FREEZE / CLFZ - Freeze buffer (0/1)
define_bool_param!(handle_cl_freeze, "cl_freeze", "CL.FREEZE", "CLOUDS FREEZE", "Failed to parse clouds freeze");

// CL.MODE / CLM - Processing mode (0-3)
define_mode_param!(handle_cl_mode, "cl_mode", 0, 3, "CL.MODE", "RANGE 0-3", "CLOUDS MODE", "Failed to parse clouds mode");

// CL.LOFI / CLLO - Lo-fi sample rate reduction (0-16383)
define_int_param!(handle_cl_lofi, "cl_lofi", 0, 16383, "CL.LOFI", "CLOUDS LO-FI", "Failed to parse clouds lo-fi");
