use crate::config;
use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_header<F>(
    parts: &[&str],
    header_level: &mut u8,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("HEADER LEVEL: {}", header_level));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *header_level = 0;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 0 (NAV ONLY)".to_string());
            }
            "1" => {
                *header_level = 1;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 1 (NAV + METERS)".to_string());
            }
            "2" => {
                *header_level = 2;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 2 (NAV + H|P + METERS)".to_string());
            }
            "3" => {
                *header_level = 3;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 3 (FULL NAV + H|P + METERS)".to_string());
            }
            "4" => {
                *header_level = 4;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 4 (FULL NAV + H|P + METERS + CPU)".to_string());
            }
            _ => {
                output("ERROR: HEADER TAKES 0-4".to_string());
            }
        }
    }
}
pub fn handle_limit<F>(
    parts: &[&str],
    limiter_enabled: &mut bool,
    metro_tx: &Sender<MetroCommand>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("LIMITER: {}", if *limiter_enabled { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *limiter_enabled = false;
                metro_tx
                    .send(MetroCommand::SendParam("limit".to_string(), OscType::Int(0)))
                    .context("Failed to send limiter param")?;
                let _ = config::save_limiter_enabled(*limiter_enabled);
                output("LIMITER: OFF".to_string());
            }
            "1" => {
                *limiter_enabled = true;
                metro_tx
                    .send(MetroCommand::SendParam("limit".to_string(), OscType::Int(1)))
                    .context("Failed to send limiter param")?;
                let _ = config::save_limiter_enabled(*limiter_enabled);
                output("LIMITER: ON".to_string());
            }
            _ => {
                output("ERROR: LIMIT TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
    Ok(())
}
pub fn handle_scope_time<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    metro_tx: &Sender<MetroCommand>,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.TIME: {}MS", scope_settings.timespan_ms));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val as u32
        } else {
            parts[1].parse().context("Failed to parse timespan value")?
        };

        if value < 5 || value > 500 {
            output("SCOPE.TIME MUST BE 5-500 MS".to_string());
            return Ok(());
        }

        scope_settings.timespan_ms = value;

        metro_tx
            .send(MetroCommand::SendScopeRate(value as f32))
            .context("Failed to send scope rate")?;

        let _ = config::save_scope_settings(scope_settings);

        output(format!("SCOPE.TIME: {}MS", value));
    }
    Ok(())
}

pub fn handle_scope_clr<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.CLR: {}", scope_settings.color_mode.name()));
    } else {
        let input = parts[1];

        if let Some(mode) = crate::types::ScopeColorMode::from_str(input) {
            scope_settings.color_mode = mode;
            let _ = config::save_scope_settings(scope_settings);
            output(format!("SCOPE.CLR: {}", mode.name()));
        } else if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            if val >= 0 && val <= 8 {
                scope_settings.color_mode = crate::types::ScopeColorMode::from_u8(val as u8);
                let _ = config::save_scope_settings(scope_settings);
                output(format!("SCOPE.CLR: {} ({})", val, scope_settings.color_mode.name()));
            } else {
                output("ERROR: SCOPE.CLR NUMBER MUST BE 0-8".to_string());
            }
        } else if let Ok(val) = input.parse::<i16>() {
            if val >= 0 && val <= 8 {
                scope_settings.color_mode = crate::types::ScopeColorMode::from_u8(val as u8);
                let _ = config::save_scope_settings(scope_settings);
                output(format!("SCOPE.CLR: {} ({})", val, scope_settings.color_mode.name()));
            } else {
                output("ERROR: SCOPE.CLR NUMBER MUST BE 0-8".to_string());
            }
        } else {
            output("ERROR: INVALID COLOR NAME OR NUMBER".to_string());
            output("VALID: FOREGROUND SECONDARY HIGHLIGHT_BG".to_string());
            output("       HIGHLIGHT_FG BORDER ERROR ACCENT".to_string());
            output("       SUCCESS LABEL OR 0-8".to_string());
        }
    }
}

pub fn handle_scope_mode<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode_name = match scope_settings.display_mode {
            1 => "BLOCK",
            2 => "LINE",
            3 => "DOT",
            4 => "QUADRANT",
            _ => "BRAILLE",
        };
        output(format!("SCOPE.MODE: {} ({})", scope_settings.display_mode, mode_name));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                scope_settings.display_mode = 0;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 0 (BRAILLE)".to_string());
            }
            1 => {
                scope_settings.display_mode = 1;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 1 (BLOCK)".to_string());
            }
            2 => {
                scope_settings.display_mode = 2;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 2 (LINE)".to_string());
            }
            3 => {
                scope_settings.display_mode = 3;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 3 (DOT)".to_string());
            }
            4 => {
                scope_settings.display_mode = 4;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 4 (QUADRANT)".to_string());
            }
            _ => {
                output("ERROR: SCOPE.MODE TAKES 0-4".to_string());
            }
        }
    }
}

pub fn handle_scope_uni<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.UNI: {}", if scope_settings.unipolar { 1 } else { 0 }));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                scope_settings.unipolar = false;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.UNI: 0 (BIPOLAR)".to_string());
            }
            1 => {
                scope_settings.unipolar = true;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.UNI: 1 (UNIPOLAR)".to_string());
            }
            _ => {
                output("ERROR: SCOPE.UNI TAKES 0-1".to_string());
            }
        }
    }
}

pub fn handle_scope_gain<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    metro_tx: &Sender<MetroCommand>,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.GAIN: {}", scope_settings.gain));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().context("Failed to parse gain value")?
        };

        if value < 0 || value > 16383 {
            output("SCOPE.GAIN MUST BE 0-16383".to_string());
            return Ok(());
        }

        scope_settings.gain = value as u16;

        let gain_f32 = value as f32 / 8192.0;
        metro_tx
            .send(MetroCommand::SendParam("scopeGain".to_string(), OscType::Float(gain_f32)))
            .context("Failed to send scope gain")?;

        let _ = config::save_scope_settings(scope_settings);

        output(format!("SCOPE.GAIN: {}", value));
    }
    Ok(())
}

pub fn handle_scope_rst<F>(
    scope_settings: &mut crate::types::ScopeSettings,
    metro_tx: &Sender<MetroCommand>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    scope_settings.timespan_ms = 100;
    scope_settings.color_mode = crate::types::ScopeColorMode::Success;
    scope_settings.display_mode = 0;
    scope_settings.unipolar = false;
    scope_settings.gain = 8192;

    metro_tx
        .send(MetroCommand::SendScopeRate(100.0))
        .context("Failed to send scope rate")?;

    metro_tx
        .send(MetroCommand::SendParam("scopeGain".to_string(), OscType::Float(1.0)))
        .context("Failed to send scope gain")?;

    let _ = config::save_scope_settings(scope_settings);

    output("SCOPE RESET TO DEFAULTS".to_string());
    Ok(())
}

pub fn handle_note<F>(
    parts: &[&str],
    notes: &mut crate::types::NotesStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: NOTE REQUIRES QUOTED TEXT".to_string());
        return;
    }

    let joined = parts[1..].join(" ");

    let text = if (joined.starts_with('"') && joined.ends_with('"')) ||
                  (joined.starts_with('\'') && joined.ends_with('\'')) {
        if joined.len() < 2 {
            output("NOTE TEXT MUST BE QUOTED".to_string());
            return;
        }
        joined[1..joined.len()-1].to_string()
    } else {
        output("NOTE TEXT MUST BE QUOTED".to_string());
        return;
    };

    let mut found_empty = false;
    for i in 0..8 {
        if notes.lines[i].is_empty() {
            notes.lines[i] = text.clone();
            found_empty = true;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("NOTE ADDED TO LINE {}", i + 1));
            }
            break;
        }
    }

    if !found_empty {
        output("NOTES PAGE FULL (8 LINES MAX)".to_string());
    }
}

pub fn handle_note_clr<F>(
    notes: &mut crate::types::NotesStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    for i in 0..8 {
        notes.lines[i].clear();
    }
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("NOTES CLEARED".to_string());
    }
}

define_bool_toggle!(handle_meter_hdr, "METER.HDR", "HEADER METERS: {}", "HEADER METERS: OFF", "HEADER METERS: ON", config::save_show_meters_header);

define_bool_toggle!(handle_meter_grid, "METER.GRID", "GRID METERS: {}", "GRID METERS: OFF", "GRID METERS: ON", config::save_show_meters_grid);

define_bool_toggle!(handle_meter_ascii, "METER.ASCII", "ASCII METERS: {}", "ASCII METERS: OFF", "ASCII METERS: ON", config::save_ascii_meters);

define_bool_toggle!(handle_spectrum, "SPECTRUM", "SPECTRUM: {}", "SPECTRUM: OFF", "SPECTRUM: ON", config::save_show_spectrum);

define_bool_toggle!(handle_activity, "ACTIVITY", "ACTIVITY: {}", "ACTIVITY: OFF", "ACTIVITY: ON", config::save_show_activity);

define_bool_toggle!(handle_grid, "GRID", "GRID: {}", "GRID: OFF", "GRID: ON", config::save_show_grid);

define_bool_toggle!(handle_grid_def, "GRID.DEF", "GRID.DEF: {}", "GRID.DEF: 0 (REPL)", "GRID.DEF: 1 (GRID)", config::save_show_grid_view);

define_bool_toggle!(handle_hl_seq, "HL.SEQ", "SEQ HIGHLIGHT: {}", "SEQ HIGHLIGHT: OFF", "SEQ HIGHLIGHT: ON", config::save_show_seq_highlight);

define_enum_select!(
    handle_grid_mode,
    "GRID.MODE",
    config::save_grid_mode,
    "ERROR: GRID.MODE TAKES 0-5",
    (0, "LABELS"),
    (1, "ICONS"),
    (2, "FX VIZ"),
    (3, "MIXER"),
    (4, "FX VIZ 2"),
    (5, "SAMPLER"),
);
