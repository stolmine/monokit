use crate::commands::context::ExecutionContext;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SamplerMode, SampleSlot, Variables, TIER_CONFIRMS, SAMPLER_BUFFER_BASE};
use anyhow::{Context, Result};
use rosc::OscType;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::utils::{
    find_kits_and_samples, is_audio_file, read_wav_frame_count, resolve_sample_path,
    send_buffer_alloc_read, truncate_name, INDENT_WIDTH, MAX_DISPLAY_WIDTH,
};

pub fn handle_kit<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::OutputCategory;
    use crate::output::OutputDecider;

    if parts.len() < 2 {
        if ctx.script_index < 10 {
            ctx.output(OutputCategory::Error, "KIT: REPL ONLY".to_string(), &mut output);
            return Ok(());
        }

        let samples_dir = match crate::config::monokit_config_dir() {
            Ok(config_dir) => config_dir.join("samples"),
            Err(_) => {
                ctx.output(OutputCategory::Error, "KIT: CONFIG DIR NOT FOUND".to_string(), &mut output);
                return Ok(());
            }
        };

        if !samples_dir.exists() {
            ctx.output(OutputCategory::Error, "KIT: NO SAMPLES DIR".to_string(), &mut output);
            return Ok(());
        }

        let (mut kits, mut samples) = find_kits_and_samples(&samples_dir, &samples_dir);

        if kits.is_empty() && samples.is_empty() {
            ctx.output(OutputCategory::Query, "KIT: NO KITS OR SAMPLES FOUND".to_string(), &mut output);
            return Ok(());
        }

        kits.sort();
        samples.sort();

        if !kits.is_empty() {
            ctx.output(OutputCategory::Query, "KITS:".to_string(), &mut output);
            for kit in kits {
                ctx.output(OutputCategory::Query, format!("  {}", truncate_name(&kit, MAX_DISPLAY_WIDTH - INDENT_WIDTH)), &mut output);
            }
        }

        if !samples.is_empty() {
            ctx.output(OutputCategory::Query, "SAMPLES:".to_string(), &mut output);
            for sample in samples {
                ctx.output(OutputCategory::Query, format!("  {}", truncate_name(&sample, MAX_DISPLAY_WIDTH - INDENT_WIDTH)), &mut output);
            }
        }

        return Ok(());
    }

    let path_str = parts[1..].join(" ");

    let resolved_path = match resolve_sample_path(&path_str, *ctx.debug_level, &mut output) {
        Some(p) => p,
        None => {
            output(format!("KIT: PATH NOT FOUND: {}", path_str));
            return Ok(());
        }
    };

    let resolved_path_str = resolved_path.to_string_lossy();
    let debug_level = *ctx.debug_level;

    let (mode, num_slots, slots, total_frames) = if resolved_path.is_file() {
        let buffer_id = SAMPLER_BUFFER_BASE;

        if let Err(e) = send_buffer_alloc_read(buffer_id, &resolved_path_str, debug_level, &mut output) {
            output(format!("KIT: FAILED TO LOAD FILE: {}", e));
            return Ok(());
        }

        let frame_count = read_wav_frame_count(&resolved_path);

        let slot = SampleSlot {
            buffer_id,
            start_frame: 0,
            end_frame: frame_count.unwrap_or(0),
            file_path: Some(resolved_path_str.to_string()),
        };

        (SamplerMode::Slice, 1, vec![slot], frame_count)
    } else if resolved_path.is_dir() {
        let mut file_slots = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&resolved_path) {
            let mut file_entries: Vec<_> = entries
                .flatten()
                .filter(|entry| {
                    entry.metadata()
                        .map(|m| m.is_file())
                        .unwrap_or(false)
                        && is_audio_file(&entry.path())
                })
                .collect();

            file_entries.sort_by_key(|entry| entry.file_name());

            for (idx, entry) in file_entries.iter().enumerate().take(128) {
                let buffer_id = SAMPLER_BUFFER_BASE + idx as u32;
                let file_path = entry.path();
                let file_path_str = file_path.to_string_lossy();

                if let Err(e) = send_buffer_alloc_read(buffer_id, &file_path_str, debug_level, &mut output) {
                    output(format!("KIT: FAILED TO LOAD {}: {}", file_path.display(), e));
                    continue;
                }

                let frame_count = read_wav_frame_count(&file_path).unwrap_or(0);
                file_slots.push(SampleSlot {
                    buffer_id,
                    start_frame: 0,
                    end_frame: frame_count,
                    file_path: Some(file_path_str.to_string()),
                });
            }
        }

        let count = file_slots.len();
        (SamplerMode::Kit, count, file_slots, None)
    } else {
        output(format!("KIT: INVALID PATH TYPE: {}", path_str));
        return Ok(());
    };

    let sampler = &mut *ctx.sampler_state;
    sampler.mode = mode;
    sampler.kit_path = Some(resolved_path_str.to_string());
    sampler.num_slots = num_slots;
    sampler.current_slot = 0;

    sampler.slice_count = if mode == SamplerMode::Slice {
        Some(num_slots)
    } else {
        None
    };

    sampler.slots = slots;

    sampler.total_frames = if mode == SamplerMode::Slice {
        total_frames
    } else {
        None
    };

    crate::eval::KIT_SLOTS.store(num_slots as u16, std::sync::atomic::Ordering::Relaxed);

    if *ctx.debug_level >= TIER_CONFIRMS || *ctx.out_cfm {
        let mode_str = match mode {
            SamplerMode::Slice => "SLICE",
            SamplerMode::Kit => "KIT",
        };
        output(format!("KIT: LOADED {} ({} SLOTS, {})", path_str, num_slots, mode_str));
    }

    Ok(())
}

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
        sampler_state.current_slot
    } else {
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

        if value < 0 || value as usize >= sampler_state.num_slots {
            output(format!("STR: SLOT OUT OF RANGE (0-{})", sampler_state.num_slots.saturating_sub(1)));
            return Ok(());
        }

        value as usize
    };

    sampler_state.current_slot = slot;

    let (buffer_id, start_frame, end_frame) = if slot < sampler_state.slots.len() {
        let slot_data = &sampler_state.slots[slot];
        (
            slot_data.buffer_id as i32,
            slot_data.start_frame as i32,
            slot_data.end_frame as i32,
        )
    } else {
        (0, 0, 0)
    };

    metro_tx
        .send(MetroCommand::SendParam(
            "s_bufnum".to_string(),
            OscType::Int(buffer_id),
        ))
        .context("Failed to send sampler buffer ID")?;

    metro_tx
        .send(MetroCommand::SendParam(
            "s_startFrame".to_string(),
            OscType::Int(start_frame),
        ))
        .context("Failed to send sampler start frame")?;

    metro_tx
        .send(MetroCommand::SendParam(
            "s_endFrame".to_string(),
            OscType::Int(end_frame),
        ))
        .context("Failed to send sampler end frame")?;

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

pub fn handle_kit_len<F>(
    sampler_state: &crate::types::SamplerState,
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::{TIER_QUERIES};

    let num_slots = sampler_state.num_slots as i16;
    if debug_level >= TIER_QUERIES || out_qry {
        output(format!("{}", num_slots));
    }
    Ok(())
}

pub fn handle_kit_info<F>(
    sampler_state: &crate::types::SamplerState,
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::TIER_QUERIES;
    use std::path::Path;

    if debug_level >= TIER_QUERIES || out_qry {
        let kit_name = match &sampler_state.kit_path {
            Some(name) => name,
            None => {
                output("NO KIT LOADED".to_string());
                return Ok(());
            }
        };

        let mode_str = match sampler_state.mode {
            crate::types::SamplerMode::Slice => "SLICE",
            crate::types::SamplerMode::Kit => "KIT",
        };

        let display_name = Path::new(kit_name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(kit_name);

        output(format!("{} ({})", truncate_name(display_name, MAX_DISPLAY_WIDTH - INDENT_WIDTH), mode_str));
        output(format!("SLOTS: {}", sampler_state.num_slots));

        for (idx, slot) in sampler_state.slots.iter().enumerate() {
            if let Some(ref path) = slot.file_path {
                let file_name = Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(path);

                let prefix_len = idx.to_string().len() + 2;
                let max_name_len = MAX_DISPLAY_WIDTH - prefix_len;
                output(format!("{}: {}", idx, truncate_name(file_name, max_name_len)));
            }
        }
    }
    Ok(())
}
