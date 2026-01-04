use crate::commands::context::ExecutionContext;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SamplerMode, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::path::Path;
use std::sync::mpsc::Sender;

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
