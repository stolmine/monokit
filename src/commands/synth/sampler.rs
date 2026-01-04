use crate::commands::context::ExecutionContext;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SamplerMode, SampleSlot, Variables, TIER_CONFIRMS, SAMPLER_BUFFER_BASE, OSC_ADDR};
use anyhow::{Context, Result};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

use super::common::define_int_param;

macro_rules! define_sampler_envelope_param {
    ($fn_name:ident, $osc_param:expr, $state_field:ident, $error_cmd:expr, $display_name:expr, $parse_ctx:expr) => {
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
            sampler_state: &mut crate::types::SamplerState,
            out_cfm: bool,
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
            if value < 0 || value > 16383 {
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
                output(format!("{}: RANGE 0-16383", $error_cmd));
                return Ok(());
            }

            sampler_state.playback.$state_field = value as i16;

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

fn resolve_sample_path<F>(path_str: &str, debug_level: u8, mut output: F) -> Option<PathBuf>
where
    F: FnMut(String),
{
    let home_dir = dirs::home_dir();

    let expanded = if path_str.starts_with('~') {
        home_dir.as_ref().map(|home| {
            if path_str == "~" {
                home.clone()
            } else {
                home.join(&path_str[2..])
            }
        })?
    } else {
        PathBuf::from(path_str)
    };

    if expanded.is_absolute() {
        return if expanded.exists() {
            Some(expanded)
        } else {
            None
        };
    }

    let library_path = home_dir?.join(".config/monokit/samples");

    let library_relative = library_path.join(path_str);
    if library_relative.exists() {
        return Some(library_relative);
    }

    let search_name = Path::new(path_str)
        .file_name()
        .and_then(|n| n.to_str())?;

    if let Some(found) = search_library_recursive(&library_path, search_name) {
        return Some(found);
    }

    if expanded.exists() {
        Some(expanded)
    } else {
        None
    }
}

fn search_library_recursive(dir: &Path, target: &str) -> Option<PathBuf> {
    if !dir.exists() || !dir.is_dir() {
        return None;
    }

    let entries = std::fs::read_dir(dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.eq_ignore_ascii_case(target) {
                return Some(path);
            }
        }

        if path.is_dir() && !path.is_symlink() {
            if let Some(found) = search_library_recursive(&path, target) {
                return Some(found);
            }
        }
    }

    None
}

fn is_audio_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(ext, "wav" | "WAV" | "aif" | "AIF" | "aiff" | "AIFF" | "flac" | "FLAC")
    } else {
        false
    }
}

fn find_kits_and_samples(base_dir: &Path, root_dir: &Path) -> (Vec<String>, Vec<String>) {
    const MAX_DEPTH: usize = 10;
    find_kits_and_samples_impl(base_dir, root_dir, 0, MAX_DEPTH)
}

fn find_kits_and_samples_impl(base_dir: &Path, root_dir: &Path, depth: usize, max_depth: usize) -> (Vec<String>, Vec<String>) {
    let mut kits = Vec::new();
    let mut samples = Vec::new();

    if depth >= max_depth || !base_dir.exists() || !base_dir.is_dir() {
        return (kits, samples);
    }

    let Ok(entries) = std::fs::read_dir(base_dir) else {
        return (kits, samples);
    };

    let mut audio_files = Vec::new();
    let mut subdirs = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() && is_audio_file(&path) {
            audio_files.push(path);
        } else if path.is_dir() && path.read_link().is_err() {
            subdirs.push(path);
        }
    }

    if !audio_files.is_empty() {
        if let Ok(relative) = base_dir.strip_prefix(root_dir) {
            if let Some(rel_str) = relative.to_str() {
                if depth == 1 {
                    for audio_file in &audio_files {
                        if let Some(file_name) = audio_file.file_stem().and_then(|s| s.to_str()) {
                            let mut path = String::with_capacity(rel_str.len() + file_name.len() + 1);
                            path.push_str(rel_str);
                            path.push('/');
                            path.push_str(file_name);
                            samples.push(path);
                        }
                    }
                } else if depth > 1 {
                    kits.push(rel_str.to_string());
                }
            }
        }
    }

    for subdir in subdirs {
        let (mut sub_kits, mut sub_samples) = find_kits_and_samples_impl(&subdir, root_dir, depth + 1, max_depth);
        kits.append(&mut sub_kits);
        samples.append(&mut sub_samples);
    }

    (kits, samples)
}

const MAX_DISPLAY_WIDTH: usize = 46;
const INDENT_WIDTH: usize = 2;

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len.saturating_sub(3)])
    }
}

fn send_buffer_alloc_read<F>(buffer_id: u32, file_path: &str, debug_level: u8, mut output: F) -> Result<()>
where
    F: FnMut(String),
{
    let socket = UdpSocket::bind("127.0.0.1:0")
        .context("Failed to bind OSC socket for buffer allocation")?;

    let msg = OscMessage {
        addr: "/b_allocRead".to_string(),
        args: vec![
            OscType::Int(buffer_id as i32),
            OscType::String(file_path.to_string()),
        ],
    };

    let packet = OscPacket::Message(msg);
    let buf = encoder::encode(&packet)
        .context("Failed to encode OSC message")?;

    socket.send_to(&buf, OSC_ADDR)
        .context("Failed to send buffer allocation message")?;

    Ok(())
}

/// KIT <path> - Load samples (file → slice mode, directory → kit mode)
/// KIT - List available kits (REPL only)
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
        // No argument: list available kits (REPL only)
        if ctx.script_index < 10 {
            ctx.output(OutputCategory::Error, "KIT: REPL ONLY".to_string(), &mut output);
            return Ok(());
        }

        // Get samples directory
        let samples_dir = match dirs::home_dir() {
            Some(home) => home.join(".config/monokit/samples"),
            None => {
                ctx.output(OutputCategory::Error, "KIT: HOME DIR NOT FOUND".to_string(), &mut output);
                return Ok(());
            }
        };

        if !samples_dir.exists() {
            ctx.output(OutputCategory::Error, "KIT: NO SAMPLES DIR".to_string(), &mut output);
            return Ok(());
        }

        // Recursively find kits and samples
        let (mut kits, mut samples) = find_kits_and_samples(&samples_dir, &samples_dir);

        if kits.is_empty() && samples.is_empty() {
            ctx.output(OutputCategory::Query, "KIT: NO KITS OR SAMPLES FOUND".to_string(), &mut output);
            return Ok(());
        }

        // Sort alphabetically
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

    // Resolve path using library search order
    let resolved_path = match resolve_sample_path(&path_str, *ctx.debug_level, &mut output) {
        Some(p) => p,
        None => {
            output(format!("KIT: PATH NOT FOUND: {}", path_str));
            return Ok(());
        }
    };

    let resolved_path_str = resolved_path.to_string_lossy();
    let debug_level = *ctx.debug_level;

    let (mode, num_slots, slots) = if resolved_path.is_file() {
        let buffer_id = SAMPLER_BUFFER_BASE;

        if let Err(e) = send_buffer_alloc_read(buffer_id, &resolved_path_str, debug_level, &mut output) {
            output(format!("KIT: FAILED TO LOAD FILE: {}", e));
            return Ok(());
        }

        let slot = SampleSlot {
            buffer_id,
            start_frame: 0,
            end_frame: 0,
            file_path: Some(resolved_path_str.to_string()),
        };

        (SamplerMode::Slice, 1, vec![slot])
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

                file_slots.push(SampleSlot {
                    buffer_id,
                    start_frame: 0,
                    end_frame: 0,
                    file_path: Some(file_path_str.to_string()),
                });
            }
        }

        let count = file_slots.len();
        (SamplerMode::Kit, count, file_slots)
    } else {
        // Path exists but is neither file nor directory
        output(format!("KIT: INVALID PATH TYPE: {}", path_str));
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

    // Store loaded buffer slots
    sampler.slots = slots;

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

    // Get buffer ID from the slot
    let buffer_id = if slot < sampler_state.slots.len() {
        sampler_state.slots[slot].buffer_id as i32
    } else {
        // Slot not loaded, use 0 (no buffer)
        0
    };

    metro_tx
        .send(MetroCommand::SendParam(
            "s_bufnum".to_string(),
            OscType::Int(buffer_id),
        ))
        .context("Failed to send sampler buffer ID")?;

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
    "SAMPLER RATE",
    "Failed to parse sample rate"
);

define_int_param!(
    handle_s_pitch,
    "s_pitch",
    -24,
    24,
    "S.PITCH",
    "SAMPLER PITCH",
    "Failed to parse sample pitch"
);

define_int_param!(
    handle_s_fine,
    "s_fine",
    -100,
    100,
    "S.FINE",
    "SAMPLER FINE PITCH",
    "Failed to parse sample fine pitch"
);

// Playback Parameters
define_int_param!(
    handle_s_dir,
    "s_direction",
    0,
    1,
    "S.DIR",
    "SAMPLER DIRECTION",
    "Failed to parse sample direction"
);

define_int_param!(
    handle_s_loop,
    "s_loop",
    0,
    1,
    "S.LOOP",
    "SAMPLER LOOP",
    "Failed to parse sample loop"
);

define_int_param!(
    handle_s_start,
    "s_startFrame",
    0,
    16383,
    "S.START",
    "SAMPLER START",
    "Failed to parse sample start offset"
);

define_int_param!(
    handle_s_len,
    "s_endFrame",
    0,
    16383,
    "S.LEN",
    "SAMPLER LENGTH",
    "Failed to parse sample loop length"
);

define_sampler_envelope_param!(
    handle_s_atk,
    "s_atk",
    attack,
    "S.ATK",
    "SAMPLER ATTACK",
    "Failed to parse sample attack"
);

define_sampler_envelope_param!(
    handle_s_dec,
    "s_dec",
    decay,
    "S.DEC",
    "SAMPLER DECAY",
    "Failed to parse sample decay"
);

define_sampler_envelope_param!(
    handle_s_rel,
    "s_rel",
    release,
    "S.REL",
    "SAMPLER RELEASE",
    "Failed to parse sample release"
);

define_int_param!(
    handle_s_sust,
    "s_sust",
    0,
    1,
    "S.SUST",
    "SAMPLER SUSTAIN MODE",
    "Failed to parse sample sustain mode"
);

// Output Parameters
define_int_param!(
    handle_s_vol,
    "s_volume",
    0,
    16383,
    "S.VOL",
    "SAMPLER VOLUME",
    "Failed to parse sample volume"
);

define_int_param!(
    handle_s_pan,
    "s_pan",
    -8192,
    8191,
    "S.PAN",
    "SAMPLER PAN",
    "Failed to parse sample pan"
);

define_int_param!(
    handle_s_fx,
    "s_fx",
    0,
    2,
    "S.FX",
    "SAMPLER FX ROUTING",
    "Failed to parse sample fx routing"
);

// Modulation Parameters
define_int_param!(
    handle_s_ratemod,
    "s_ratemod",
    0,
    16383,
    "S.RATEMOD",
    "SAMPLER RATE MOD",
    "Failed to parse sample rate modulation"
);

define_int_param!(
    handle_s_pitchmod,
    "s_pitchmod",
    0,
    16383,
    "S.PITCHMOD",
    "SAMPLER PITCH MOD",
    "Failed to parse sample pitch modulation"
);

// FX Parameters - Filter (DFM1) - sf_ prefix for sampler
define_int_param!(
    handle_sf_cut,
    "sf_cut",
    0,
    16383,
    "SF.CUT",
    "SAMPLER FX CUTOFF",
    "Failed to parse filter cutoff"
);

define_int_param!(
    handle_sf_res,
    "sf_res",
    0,
    16383,
    "SF.RES",
    "SAMPLER FX RESONANCE",
    "Failed to parse filter resonance"
);

define_int_param!(
    handle_sf_type,
    "sf_type",
    0,
    1,
    "SF.TYPE",
    "SAMPLER FX TYPE",
    "Failed to parse filter type"
);

// FX Parameters - Decimator
define_int_param!(
    handle_sf_bits,
    "sf_bits",
    1,
    24,
    "SF.BITS",
    "SAMPLER BITS",
    "Failed to parse bit depth"
);

define_int_param!(
    handle_sf_rate,
    "sf_rate",
    0,
    16383,
    "SF.RATE",
    "SAMPLER SRR",
    "Failed to parse rate reduction"
);

define_int_param!(
    handle_sf_deci,
    "sf_deci",
    0,
    16383,
    "SF.DECI",
    "SAMPLER DECI MIX",
    "Failed to parse decimator mix"
);

// FX Parameters - Disintegrator
define_int_param!(
    handle_sf_prob,
    "sf_prob",
    0,
    16383,
    "SF.PROB",
    "SAMPLER GLIT PROB",
    "Failed to parse glitch probability"
);

define_int_param!(
    handle_sf_mult,
    "sf_mult",
    0,
    16383,
    "SF.MULT",
    "SAMPLER GLIT MULT",
    "Failed to parse glitch multiplier"
);

define_int_param!(
    handle_sf_glit,
    "sf_glit",
    0,
    16383,
    "SF.GLIT",
    "SAMPLER GLIT MIX",
    "Failed to parse disintegrator mix"
);

/// KIT.LEN - Get the number of slots in the currently loaded kit
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
