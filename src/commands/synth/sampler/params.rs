use crate::eval::eval_expression;
use crate::sampler::onset::{OnsetDetector, to_mono};
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SamplerMode, SampleSlot, Variables, TIER_CONFIRMS, SAMPLER_BUFFER_BASE};
use anyhow::{Context, Result};
use rosc::OscType;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use super::macros::{
    define_sampler_envelope_param, define_sampler_fx_param, define_sampler_playback_param,
};

define_sampler_playback_param!(
    handle_s_rate,
    "s_rate",
    0,
    16383,
    rate,
    i16,
    "S.RATE",
    "SAMPLER RATE",
    "Failed to parse sample rate"
);

define_sampler_playback_param!(
    handle_s_pitch,
    "s_pitch",
    -24,
    24,
    pitch,
    i16,
    "S.PITCH",
    "SAMPLER PITCH",
    "Failed to parse sample pitch"
);

define_sampler_playback_param!(
    handle_s_fine,
    "s_fine",
    -100,
    100,
    fine,
    i16,
    "S.FINE",
    "SAMPLER FINE PITCH",
    "Failed to parse sample fine pitch"
);

define_sampler_playback_param!(
    handle_s_dir,
    "s_direction",
    0,
    1,
    direction,
    bool,
    "S.DIR",
    "SAMPLER DIRECTION",
    "Failed to parse sample direction"
);

define_sampler_playback_param!(
    handle_s_loop,
    "s_loop",
    0,
    1,
    loop_mode,
    bool,
    "S.LOOP",
    "SAMPLER LOOP",
    "Failed to parse sample loop"
);

define_sampler_playback_param!(
    handle_s_start,
    "s_startFrame",
    0,
    16383,
    start_offset,
    i16,
    "S.START",
    "SAMPLER START",
    "Failed to parse sample start offset"
);

define_sampler_playback_param!(
    handle_s_len,
    "s_endFrame",
    0,
    16383,
    length,
    i16,
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

define_sampler_playback_param!(
    handle_s_sust,
    "s_sust",
    0,
    1,
    sustain_mode,
    bool,
    "S.SUST",
    "SAMPLER SUSTAIN MODE",
    "Failed to parse sample sustain mode"
);

define_sampler_playback_param!(
    handle_s_fx,
    "s_fx",
    0,
    1,
    fx_routing,
    u8,
    "S.FX",
    "SAMPLER FX ROUTING",
    "Failed to parse sample fx routing"
);

define_sampler_playback_param!(
    handle_s_ratemod,
    "s_ratemod",
    0,
    16383,
    rate_mod,
    i16,
    "S.RATEMOD",
    "SAMPLER RATE MOD",
    "Failed to parse sample rate modulation"
);

define_sampler_playback_param!(
    handle_s_pitchmod,
    "s_pitchmod",
    0,
    16383,
    pitch_mod,
    i16,
    "S.PITCHMOD",
    "SAMPLER PITCH MOD",
    "Failed to parse sample pitch modulation"
);

define_sampler_fx_param!(
    handle_sf_cut,
    "sf_cut",
    0,
    16383,
    filter_cut,
    i16,
    "SF.CUT",
    "SAMPLER FX CUTOFF",
    "Failed to parse filter cutoff"
);

define_sampler_fx_param!(
    handle_sf_res,
    "sf_res",
    0,
    16383,
    filter_res,
    i16,
    "SF.RES",
    "SAMPLER FX RESONANCE",
    "Failed to parse filter resonance"
);

define_sampler_fx_param!(
    handle_sf_type,
    "sf_type",
    0,
    13,
    filter_type,
    u8,
    "SF.TYPE",
    "SAMPLER FX TYPE",
    "Failed to parse filter type"
);

define_sampler_fx_param!(
    handle_sf_bits,
    "sf_bits",
    1,
    24,
    bits,
    u8,
    "SF.BITS",
    "SAMPLER BITS",
    "Failed to parse bit depth"
);

define_sampler_fx_param!(
    handle_sf_rate,
    "sf_rate",
    0,
    16383,
    rate_reduce,
    i16,
    "SF.RATE",
    "SAMPLER SRR",
    "Failed to parse rate reduction"
);

define_sampler_fx_param!(
    handle_sf_deci,
    "sf_deci",
    0,
    16383,
    deci_mix,
    i16,
    "SF.DECI",
    "SAMPLER DECI MIX",
    "Failed to parse decimator mix"
);

define_sampler_fx_param!(
    handle_sf_prob,
    "sf_prob",
    0,
    16383,
    prob,
    i16,
    "SF.PROB",
    "SAMPLER GLIT PROB",
    "Failed to parse glitch probability"
);

define_sampler_fx_param!(
    handle_sf_mult,
    "sf_mult",
    0,
    16383,
    mult,
    i16,
    "SF.MULT",
    "SAMPLER GLIT MULT",
    "Failed to parse glitch multiplier"
);

define_sampler_fx_param!(
    handle_sf_glit,
    "sf_glit",
    0,
    16383,
    glit_mix,
    i16,
    "SF.GLIT",
    "SAMPLER GLIT MIX",
    "Failed to parse disintegrator mix"
);

define_sampler_fx_param!(
    handle_sf_cutmod,
    "sf_cutmod",
    0,
    16383,
    filter_cut_mod,
    i16,
    "SF.CUTMOD",
    "SAMPLER CUTOFF MOD",
    "Failed to parse cutoff modulation"
);

define_sampler_fx_param!(
    handle_sf_resmod,
    "sf_resmod",
    0,
    16383,
    filter_res_mod,
    i16,
    "SF.RESMOD",
    "SAMPLER RES MOD",
    "Failed to parse resonance modulation"
);

pub fn handle_s_slice<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
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
        output("S.SLICE: REQUIRES VALUE".to_string());
        return Ok(());
    }

    if sampler_state.mode != SamplerMode::Slice {
        output("S.SLICE: SLICE MODE ONLY".to_string());
        return Ok(());
    }

    let total_frames = match sampler_state.total_frames {
        Some(frames) => frames,
        None => {
            output("S.SLICE: NO FILE LOADED".to_string());
            return Ok(());
        }
    };

    let n: i32 = if let Some((expr_val, _)) = eval_expression(
        parts, 1, variables, patterns, counters, scripts, script_index, scale
    ) {
        expr_val as i32
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("S.SLICE: INVALID VALUE".to_string());
                return Ok(());
            }
        }
    };

    if n < 2 || n > 128 {
        output("S.SLICE: RANGE 2-128".to_string());
        return Ok(());
    }

    let n = n as usize;

    if total_frames < n {
        output("S.SLICE: FILE TOO SHORT".to_string());
        return Ok(());
    }

    let slice_len = total_frames / n;

    sampler_state.slots.clear();
    for i in 0..n {
        let start_frame = i * slice_len;
        let end_frame = if i == n - 1 {
            total_frames
        } else {
            (i + 1) * slice_len
        };

        sampler_state.slots.push(SampleSlot {
            buffer_id: SAMPLER_BUFFER_BASE,
            start_frame,
            end_frame,
            file_path: None,
        });
    }

    sampler_state.num_slots = n;
    sampler_state.slice_count = Some(n);
    sampler_state.current_slot = 0;

    crate::eval::KIT_SLOTS.store(n as u16, std::sync::atomic::Ordering::Relaxed);

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("S.SLICE: {} SLICES", n));
    }

    Ok(())
}

pub fn handle_s_onset<F>(
    parts: &[&str],
    sampler_state: &mut crate::types::SamplerState,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if sampler_state.mode != SamplerMode::Slice {
        output("S.ONSET: SLICE MODE ONLY".to_string());
        return Ok(());
    }

    let kit_path = match &sampler_state.kit_path {
        Some(path) => path,
        None => {
            output("S.ONSET: NO FILE LOADED".to_string());
            return Ok(());
        }
    };

    let sensitivity = if parts.len() >= 2 {
        match parts[1].parse::<u32>() {
            Ok(val) if val <= 10 => val,
            _ => {
                output("S.ONSET: SENS RANGE 0-10".to_string());
                return Ok(());
            }
        }
    } else {
        sampler_state.onset_sensitivity
    };

    let path = PathBuf::from(kit_path);
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if !matches!(ext, "wav" | "WAV" | "aif" | "AIF" | "aiff" | "AIFF") {
        output("S.ONSET: WAV/AIFF ONLY".to_string());
        return Ok(());
    }

    let file = File::open(&path)
        .context("Failed to open audio file")?;
    let mut reader = hound::WavReader::new(file)
        .context("Failed to read WAV file")?;

    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let channels = spec.channels;

    let samples: Vec<f32> = reader.samples::<i32>()
        .filter_map(|s| s.ok())
        .map(|s| s as f32 / 2147483648.0)
        .collect();

    let mono = to_mono(&samples, channels);

    let detector = OnsetDetector::new(sample_rate)
        .with_sensitivity(sensitivity)
        .with_min_spacing(sampler_state.onset_min_spacing_ms);

    let onsets = detector.detect(&mono);

    if onsets.len() < 2 {
        output("S.ONSET: <2 FOUND, USING S.SLICE 16".to_string());

        let total_frames = match sampler_state.total_frames {
            Some(frames) => frames,
            None => {
                output("S.ONSET: NO FILE LOADED".to_string());
                return Ok(());
            }
        };

        let n = 16usize;
        if total_frames < n {
            output("S.SLICE: FILE TOO SHORT".to_string());
            return Ok(());
        }

        let slice_len = total_frames / n;
        sampler_state.slots.clear();

        for i in 0..n {
            let start_frame = i * slice_len;
            let end_frame = if i == n - 1 {
                total_frames
            } else {
                (i + 1) * slice_len
            };

            sampler_state.slots.push(SampleSlot {
                buffer_id: SAMPLER_BUFFER_BASE,
                start_frame,
                end_frame,
                file_path: None,
            });
        }

        sampler_state.num_slots = n;
        sampler_state.slice_count = Some(n);
        sampler_state.current_slot = 0;
        crate::eval::KIT_SLOTS.store(n as u16, std::sync::atomic::Ordering::Relaxed);

        return Ok(());
    }

    let num_slices = onsets.len();
    sampler_state.slots.clear();

    for i in 0..num_slices {
        let start_frame = onsets[i];
        let end_frame = if i == num_slices - 1 {
            mono.len()
        } else {
            onsets[i + 1]
        };

        sampler_state.slots.push(SampleSlot {
            buffer_id: SAMPLER_BUFFER_BASE,
            start_frame,
            end_frame,
            file_path: None,
        });
    }

    sampler_state.num_slots = num_slices;
    sampler_state.slice_count = Some(num_slices);
    sampler_state.current_slot = 0;
    sampler_state.onset_sensitivity = sensitivity;

    crate::eval::KIT_SLOTS.store(num_slices as u16, std::sync::atomic::Ordering::Relaxed);

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("S.ONSET: {} SLICES", num_slices));
    }

    Ok(())
}

pub fn handle_s_onset_min<F>(
    parts: &[&str],
    sampler_state: &mut crate::types::SamplerState,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("S.ONSET.MIN: REQUIRES VALUE".to_string());
        return Ok(());
    }

    let value: f32 = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => {
            output("S.ONSET.MIN: INVALID VALUE".to_string());
            return Ok(());
        }
    };

    if value < 10.0 || value > 500.0 {
        output("S.ONSET.MIN: RANGE 10-500".to_string());
        return Ok(());
    }

    sampler_state.onset_min_spacing_ms = value;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("S.ONSET.MIN: SET TO {}", value));
    }

    Ok(())
}
