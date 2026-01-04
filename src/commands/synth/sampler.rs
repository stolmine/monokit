use crate::commands::context::ExecutionContext;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SamplerMode, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::common::define_int_param;

/// KIT <path> - Load samples (file → slice mode, directory → kit mode)
pub fn handle_kit<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("KIT: REQUIRES PATH".to_string());
        return Ok(());
    }

    let path_str = parts[1..].join(" ");
    let path = Path::new(&path_str);

    // Determine mode based on path type
    let (mode, num_slots) = if path.is_file() {
        // File → slice mode
        // For now, we'll use a default slice count (e.g., 16)
        // Later phases will implement S.SLICE command
        (SamplerMode::Slice, 16)
    } else if path.is_dir() {
        // Directory → kit mode
        // Count files in directory (up to 128)
        let mut count = 0;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        count += 1;
                        if count >= 128 {
                            break;
                        }
                    }
                }
            }
        }
        (SamplerMode::Kit, count)
    } else {
        output(format!("KIT: PATH NOT FOUND: {}", path_str));
        return Ok(());
    };

    // Update sampler state
    let sampler = &mut *ctx.sampler_state;
    sampler.mode = mode;
    sampler.kit_path = Some(path_str.clone());
    sampler.num_slots = num_slots;
    sampler.current_slot = 0;

    // Set slice_count for slice mode, None for kit mode
    sampler.slice_count = if mode == SamplerMode::Slice {
        Some(num_slots)
    } else {
        None
    };

    // Clear existing slots (actual buffer loading will be implemented later)
    sampler.slots.clear();

    if *ctx.debug_level >= TIER_CONFIRMS || *ctx.out_cfm {
        let mode_str = match mode {
            SamplerMode::Slice => "SLICE",
            SamplerMode::Kit => "KIT",
        };
        output(format!("KIT: LOADED {} ({} SLOTS, {})", path_str, num_slots, mode_str));
    }

    Ok(())
}

/// STR <n> - Trigger sample slot (with optional expression evaluation)
/// STR - Re-trigger current slot
pub fn handle_str<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    scale: &ScaleState,
    sampler_state: &mut crate::types::SamplerState,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let slot: usize = if parts.len() < 2 {
        // No arg → re-trigger current slot
        sampler_state.current_slot
    } else {
        // Evaluate expression for slot index
        let value: i16 = if let Some((expr_val, _)) = eval_expression(
            parts, 1, variables, patterns, counters, scripts, script_index, scale
        ) {
            expr_val
        } else {
            match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    output("STR: INVALID SLOT INDEX".to_string());
                    return Ok(());
                }
            }
        };

        // Validate range
        if value < 0 || value as usize >= sampler_state.num_slots {
            output(format!("STR: SLOT OUT OF RANGE (0-{})", sampler_state.num_slots.saturating_sub(1)));
            return Ok(());
        }

        value as usize
    };

    // Update current slot
    sampler_state.current_slot = slot;

    // Send OSC trigger via MetroCommand::SendParam
    // For now, we'll send a simple trigger message
    // Later phases will implement proper buffer playback
    metro_tx
        .send(MetroCommand::SendParam(
            "sampler_slot".to_string(),
            OscType::Int(slot as i32),
        ))
        .context("Failed to send sampler slot param")?;

    // Trigger the sampler (send t_gate)
    metro_tx
        .send(MetroCommand::SendParam(
            "t_gate_sampler".to_string(),
            OscType::Int(1),
        ))
        .context("Failed to send sampler trigger")?;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("STR: SLOT {}", slot));
    }

    Ok(())
}

// Pitch Parameters (s_ prefix to avoid conflicts with other synths)
define_int_param!(
    handle_s_rate,
    "s_rate",
    0,
    16383,
    "S.RATE",
    "SAMPLE RATE",
    "Failed to parse sample rate"
);

define_int_param!(
    handle_s_pitch,
    "s_pitch",
    -24,
    24,
    "S.PITCH",
    "SAMPLE PITCH",
    "Failed to parse sample pitch"
);

define_int_param!(
    handle_s_fine,
    "s_fine",
    -100,
    100,
    "S.FINE",
    "SAMPLE FINE PITCH",
    "Failed to parse sample fine pitch"
);

// Playback Parameters
define_int_param!(
    handle_s_dir,
    "s_direction",
    0,
    1,
    "S.DIR",
    "SAMPLE DIRECTION",
    "Failed to parse sample direction"
);

define_int_param!(
    handle_s_loop,
    "s_loop",
    0,
    1,
    "S.LOOP",
    "SAMPLE LOOP",
    "Failed to parse sample loop"
);

define_int_param!(
    handle_s_start,
    "s_startFrame",
    0,
    16383,
    "S.START",
    "SAMPLE START",
    "Failed to parse sample start offset"
);

define_int_param!(
    handle_s_len,
    "s_endFrame",
    0,
    16383,
    "S.LEN",
    "SAMPLE LENGTH",
    "Failed to parse sample loop length"
);

// Envelope Parameters
define_int_param!(
    handle_s_atk,
    "s_atk",
    0,
    16383,
    "S.ATK",
    "SAMPLE ATTACK",
    "Failed to parse sample attack"
);

define_int_param!(
    handle_s_dec,
    "s_dec",
    0,
    16383,
    "S.DEC",
    "SAMPLE DECAY",
    "Failed to parse sample decay"
);

define_int_param!(
    handle_s_rel,
    "s_rel",
    0,
    16383,
    "S.REL",
    "SAMPLE RELEASE",
    "Failed to parse sample release"
);

define_int_param!(
    handle_s_sust,
    "s_sust",
    0,
    1,
    "S.SUST",
    "SAMPLE SUSTAIN MODE",
    "Failed to parse sample sustain mode"
);

// Output Parameters
define_int_param!(
    handle_s_vol,
    "s_volume",
    0,
    16383,
    "S.VOL",
    "SAMPLE VOLUME",
    "Failed to parse sample volume"
);

define_int_param!(
    handle_s_pan,
    "s_pan",
    -8192,
    8191,
    "S.PAN",
    "SAMPLE PAN",
    "Failed to parse sample pan"
);

define_int_param!(
    handle_s_fx,
    "s_fx",
    0,
    2,
    "S.FX",
    "SAMPLE FX ROUTING",
    "Failed to parse sample fx routing"
);

// Modulation Parameters
define_int_param!(
    handle_s_ratemod,
    "s_ratemod",
    0,
    16383,
    "S.RATEMOD",
    "SAMPLE RATE MOD",
    "Failed to parse sample rate modulation"
);

define_int_param!(
    handle_s_pitchmod,
    "s_pitchmod",
    0,
    16383,
    "S.PITCHMOD",
    "SAMPLE PITCH MOD",
    "Failed to parse sample pitch modulation"
);

// FX Parameters - Filter (DFM1) - sf_ prefix for sampler
define_int_param!(
    handle_sf_cut,
    "sf_cut",
    0,
    16383,
    "SF.CUT",
    "FILTER CUTOFF",
    "Failed to parse filter cutoff"
);

define_int_param!(
    handle_sf_res,
    "sf_res",
    0,
    16383,
    "SF.RES",
    "FILTER RESONANCE",
    "Failed to parse filter resonance"
);

define_int_param!(
    handle_sf_type,
    "sf_type",
    0,
    1,
    "SF.TYPE",
    "FILTER TYPE",
    "Failed to parse filter type"
);

// FX Parameters - Decimator
define_int_param!(
    handle_sf_bits,
    "sf_bits",
    1,
    24,
    "SF.BITS",
    "BIT DEPTH",
    "Failed to parse bit depth"
);

define_int_param!(
    handle_sf_rate,
    "sf_rate",
    0,
    16383,
    "SF.RATE",
    "RATE REDUCTION",
    "Failed to parse rate reduction"
);

define_int_param!(
    handle_sf_deci,
    "sf_deci",
    0,
    16383,
    "SF.DECI",
    "DECIMATOR MIX",
    "Failed to parse decimator mix"
);

// FX Parameters - Disintegrator
define_int_param!(
    handle_sf_prob,
    "sf_prob",
    0,
    16383,
    "SF.PROB",
    "GLITCH PROBABILITY",
    "Failed to parse glitch probability"
);

define_int_param!(
    handle_sf_mult,
    "sf_mult",
    0,
    16383,
    "SF.MULT",
    "GLITCH MULTIPLIER",
    "Failed to parse glitch multiplier"
);

define_int_param!(
    handle_sf_glit,
    "sf_glit",
    0,
    16383,
    "SF.GLIT",
    "DISINTEGRATOR MIX",
    "Failed to parse disintegrator mix"
);
