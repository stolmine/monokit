use crate::commands::context::ExecutionContext;
use crate::config;
use crate::eval::eval_expression;
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, OutputCategory, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_CONFIRMS, TIER_ESSENTIAL, TIER_QUERIES};
use anyhow::{Context, Result};
use rosc::OscType;

pub fn handle_script(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
) -> Result<Vec<usize>> {
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("SCRIPT REQUIRES NUMBER 1-8"));
    }
    let num: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 1 || expr_val > 8 {
            return Err(anyhow::anyhow!("SCRIPT NUMBER MUST BE 1-8"));
        }
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse script number")?
    };
    if num < 1 || num > 8 {
        return Err(anyhow::anyhow!("SCRIPT NUMBER MUST BE 1-8"));
    }
    Ok(vec![num - 1])
}

pub fn handle_theme<F>(
    parts: &[&str],
    theme: &mut Theme,
    color_mode: crate::types::ColorMode,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        match config::load_config() {
            Ok(cfg) => {
                let available = config::list_themes(&cfg);
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
                output("AVAILABLE THEMES:".to_string());
                for theme_name in available {
                    output(format!("  {}", theme_name.to_uppercase()));
                }
            }
            Err(_) => {
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
                output("AVAILABLE THEMES:".to_string());
                output("  DARK".to_string());
                output("  LIGHT".to_string());
                output("  SYSTEM".to_string());
            }
        }
    } else {
        let name = parts[1];
        match config::load_config() {
            Ok(cfg) => {
                match config::load_theme_by_name(name, &cfg) {
                    Ok(new_theme) => {
                        let theme_name = new_theme.name.clone();
                        *theme = if color_mode == crate::types::ColorMode::Color256 {
                            new_theme.to_256_color()
                        } else {
                            new_theme
                        };
                        if let Err(e) = config::save_theme_mode(&theme_name) {
                            output(format!("WARNING: FAILED TO SAVE THEME: {:?}", e));
                        }
                        output(format!("SWITCHED TO {} THEME", theme_name.to_uppercase()));
                    }
                    Err(e) => {
                        output(e.to_string().to_uppercase());
                    }
                }
            }
            Err(e) => {
                output(format!("FAILED TO LOAD CONFIG: {:?}", e));
            }
        }
    }
}

pub fn handle_version<F>(
    mut output: F,
) where
    F: FnMut(String),
{
    output(format!("MONOKIT v{}", env!("CARGO_PKG_VERSION")));
}

pub fn handle_help<F>(
    mut output: F,
) where
    F: FnMut(String),
{
    output("=== MONOKIT COMMANDS ===".to_string());
    output("".to_string());
    output("TRIGGER: TR".to_string());
    output("VOLUME:  VOL <0.0-1.0>".to_string());
    output("RESET:   RST".to_string());
    output("".to_string());
    output("-- REPL UTILITIES --".to_string());
    output("CLEAR / CLR     CLEAR OUTPUT HISTORY".to_string());
    output("DEBUG <0-5>  SET VERBOSITY LEVEL".to_string());
    output("HEADER       SHOW CURRENT HEADER LEVEL".to_string());
    output("HEADER <0-4>    SET HEADER VERBOSITY".to_string());
    output("                0=NAV ONLY, 1=+METERS, 2=+C/P".to_string());
    output("                3=FULL NAV, 4=+CPU (DEFAULT)".to_string());
    output("PRINT \"TEXT\"    OUTPUT LITERAL STRING".to_string());
    output("PRINT <EXPR>    EVALUATE AND PRINT EXPRESSION".to_string());
    output("".to_string());
    output("-- OSCILLATORS --".to_string());
    output("PF <HZ>     PRIMARY FREQ (20-20000)".to_string());
    output("PW <0-2>    PRIMARY WAVE (SIN/TRI/SAW)".to_string());
    output("MF <HZ>     MOD FREQ".to_string());
    output("MW <0-2>    MOD WAVE".to_string());
    output("".to_string());
    output("-- FM & DISCONTINUITY --".to_string());
    output("FM <0-16383>  FM INDEX".to_string());
    output("FA <0-16383>  FM ENV AMOUNT".to_string());
    output("FD <MS>       FM ENV DECAY".to_string());
    output("DC <0-16383>  DISCONTINUITY".to_string());
    output("DA <0-16383>  DC ENV AMOUNT".to_string());
    output("DD <MS>       DC ENV DECAY".to_string());
    output("DM <0-6>  DC MODE (FOLD/TANH/SOFT/HARD...)".to_string());
    output("".to_string());
    output("-- ENVELOPES --".to_string());
    output("AD <MS>       AMP DECAY".to_string());
    output("PD <MS>       PITCH DECAY".to_string());
    output("PA <0-16>     PITCH ENV AMOUNT".to_string());
    output("".to_string());
    output("-- MOD BUS --".to_string());
    output("MB <0-16383>  MOD BUS AMOUNT".to_string());
    output("MP/MD/MT/MA <0|1>  ROUTING TOGGLES".to_string());
    output("TK <0-16383>  TRACKING".to_string());
    output("".to_string());
    output("-- MIX --".to_string());
    output("MX <0-16383>  MIX AMOUNT".to_string());
    output("MM/ME <0|1>   MIX ROUTING".to_string());
    output("".to_string());
    output("-- METRO --".to_string());
    output("M             SHOW INTERVAL".to_string());
    output("M <MS>        SET INTERVAL".to_string());
    output("M.BPM <BPM>   SET BPM".to_string());
    output("M.ACT <0|1>     START/STOP".to_string());
    output("M.SCRIPT <1-8> SET SCRIPT TO CALL ON EACH TICK".to_string());
    output("".to_string());
    output("-- SCRIPTS --".to_string());
    output("SCRIPT <1-8>  EXECUTE STORED SCRIPT".to_string());
    output("".to_string());
    output("-- SCENES --".to_string());
    output("SAVE <NAME>   SAVE CURRENT STATE".to_string());
    output("LOAD <NAME>   LOAD SAVED STATE".to_string());
    output("AUTOLOAD <0|1> AUTO-LOAD LAST SCENE AT START".to_string());
    output("SCENES        LIST ALL SCENES".to_string());
    output("DELETE <NAME> DELETE A SCENE".to_string());
    output("".to_string());
    output("-- THEME --".to_string());
    output("THEME            SHOW CURRENT".to_string());
    output("THEME DARK       DARK MODE".to_string());
    output("THEME LIGHT      LIGHT MODE".to_string());
    output("THEME SYSTEM     USE OS PREFERENCE".to_string());
    output("THEME CUSTOM     USE CUSTOM THEME".to_string());
    output("".to_string());
    output("-- RECORDING --".to_string());
    output("REC              START RECORDING".to_string());
    output("REC.STOP         STOP RECORDING".to_string());
    output("REC.PATH <PATH>  SET CUSTOM PATH PREFIX".to_string());
}
pub fn handle_print<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PRINT REQUIRES AT LEAST 1 ARGUMENT".to_string());
        return;
    }

    if parts[1].starts_with('"') || parts[1].starts_with('\'') {
        let joined = parts[1..].join(" ");
        let quote_char = if parts[1].starts_with('"') { '"' } else { '\'' };

        if joined.starts_with(quote_char) && joined.ends_with(quote_char) && joined.len() > 1 {
            let literal = &joined[1..joined.len() - 1];
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(literal.to_string());
            }
        } else {
            output("UNTERMINATED STRING LITERAL".to_string());
        }
    } else {
        if let Some((result, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(format!("{}", result));
            }
        } else {
            output("FAILED TO EVALUATE EXPRESSION".to_string());
        }
    }
}

/// Custom DEBUG handler that sets debug_level AND synchronizes out_* flags
pub fn handle_debug<F>(
    parts: &[&str],
    debug_level: &mut u8,
    out_err: &mut bool,
    out_ess: &mut bool,
    out_qry: &mut bool,
    out_cfm: &mut bool,
    current_level: u8,
    mut output: F,
) where
    F: FnMut(String),
{
    const LABELS: &[(u8, &str)] = &[
        (0, "SILENT"),
        (1, "ERRORS"),
        (2, "ESSENTIAL"),
        (3, "QUERIES"),
        (4, "CONFIRMS"),
        (5, "VERBOSE"),
    ];

    if parts.len() == 1 {
        // Query mode
        if current_level >= TIER_QUERIES {
            output(format!("DEBUG: {}", *debug_level));
        }
    } else {
        match parts[1].parse::<u8>() {
            Ok(val) if val <= 5 => {
                *debug_level = val;
                let _ = config::save_debug_level(val);

                // Synchronize out_* flags based on tier
                // TIER_ERRORS=1, TIER_ESSENTIAL=2, TIER_QUERIES=3, TIER_CONFIRMS=4
                *out_err = val >= 1;
                *out_ess = val >= 2;
                *out_qry = val >= 3;
                *out_cfm = val >= 4;

                // Save out_* flags
                let _ = config::save_out_err(*out_err);
                let _ = config::save_out_ess(*out_ess);
                let _ = config::save_out_qry(*out_qry);
                let _ = config::save_out_cfm(*out_cfm);

                if current_level >= TIER_CONFIRMS {
                    let label = LABELS.iter().find(|(v, _)| *v == val).map(|(_, l)| *l).unwrap_or("UNKNOWN");
                    output(format!("DEBUG: {} ({})", val, label));
                }
            }
            _ => output("ERROR: DEBUG TAKES 0-5".to_string()),
        }
    }
}
pub fn handle_title<F>(
    parts: &[&str],
    title_mode: &mut u8,
    current_scene_name: &Option<String>,
    scramble_enabled: bool,
    scramble_mode: u8,
    scramble_speed: u8,
    scramble_curve: u8,
    header_scramble: &mut Option<crate::scramble::ScrambleAnimation>,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode_name = if *title_mode == 0 { "MONOKIT" } else { "SCENE" };
        output(format!("TITLE: {} ({})", title_mode, mode_name));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *title_mode = 0;
                let _ = config::save_title_mode(*title_mode);
                *header_scramble = if scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                    Some(crate::scramble::ScrambleAnimation::new_with_options("MONOKIT", mode, scramble_speed, curve))
                } else {
                    None
                };
                output("TITLE: 0 (MONOKIT)".to_string());
            }
            "1" => {
                *title_mode = 1;
                let _ = config::save_title_mode(*title_mode);
                let text = current_scene_name.as_ref().map(|s| s.as_str()).unwrap_or("[UNSAVED]");
                *header_scramble = if scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                    Some(crate::scramble::ScrambleAnimation::new_with_options(text, mode, scramble_speed, curve))
                } else {
                    None
                };
                output("TITLE: 1 (SCENE NAME)".to_string());
            }
            _ => {
                output("ERROR: TITLE TAKES 0 (MONOKIT) OR 1 (SCENE)".to_string());
            }
        }
    }
}

define_bool_toggle!(handle_scrmbl, "SCRMBL", config::save_scramble_enabled);

define_bool_toggle!(handle_scrmbl_grid, "SCRMBL.GRID", config::save_scramble_grid_enabled);

define_enum_select!(
    handle_scrmbl_mode,
    "SCRMBL.MODE",
    config::save_scramble_mode,
    "ERROR: SCRMBL.MODE TAKES 0-3",
    (0, "REGULAR"),
    (1, "SMASH"),
    (2, "ROLLING"),
    (3, "OVERSHOOT"),
);

define_enum_select!(
    handle_scrmbl_spd,
    "SCRMBL.SPD",
    config::save_scramble_speed,
    "ERROR: SCRMBL.SPD TAKES 1-10",
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
);

define_enum_select!(
    handle_scrmbl_crv,
    "SCRMBL.CRV",
    config::save_scramble_curve,
    "ERROR: SCRMBL.CRV TAKES 0 (LINEAR) OR 1 (SETTLE)",
    (0, "LINEAR"),
    (1, "SETTLE"),
);

pub fn handle_title_timer<F>(
    parts: &[&str],
    title_timer_enabled: &mut bool,
    title_timer_interval_secs: &mut u16,
    title_timer_last_toggle: &mut Option<std::time::Instant>,
    title_mode: &mut u8,
    current_scene_name: &Option<String>,
    scramble_enabled: bool,
    scramble_mode: u8,
    scramble_speed: u8,
    scramble_curve: u8,
    header_scramble: &mut Option<crate::scramble::ScrambleAnimation>,
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
        if *title_timer_enabled {
            output(format!("TITLE.TIMER: ON ({}S)", *title_timer_interval_secs));
        } else {
            output("TITLE.TIMER: OFF".to_string());
        }
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[1].parse().unwrap_or(-1)
        };
        match value {
            0 => {
                *title_timer_enabled = false;
                *title_timer_last_toggle = None;
                let _ = config::save_title_timer_enabled(false);
                output("TITLE.TIMER: OFF".to_string());
            }
            1 => {
                if parts.len() < 3 {
                    output("TITLE.TIMER 1 REQUIRES SECONDS (1-1800)".to_string());
                    return;
                }
                let secs: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
                    expr_val
                } else {
                    parts[2].parse().unwrap_or(-1)
                };
                if secs < 1 || secs > 1800 {
                    output("TITLE.TIMER SECONDS MUST BE 1-1800".to_string());
                    return;
                }
                let secs_u16 = secs as u16;
                *title_timer_enabled = true;
                *title_timer_interval_secs = secs_u16;
                *title_timer_last_toggle = Some(std::time::Instant::now());
                let _ = config::save_title_timer_enabled(true);
                let _ = config::save_title_timer_interval_secs(secs_u16);

                let text = if *title_mode == 0 {
                    "MONOKIT"
                } else {
                    current_scene_name.as_ref().map(|s| s.as_str()).unwrap_or("[UNSAVED]")
                };
                *header_scramble = if scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                    Some(crate::scramble::ScrambleAnimation::new_with_options(text, mode, scramble_speed, curve))
                } else {
                    None
                };

                output(format!("TITLE.TIMER: ON ({}S)", secs_u16));
            }
            _ => {
                output("ERROR: TITLE.TIMER TAKES 0 (OFF) OR 1 <SECONDS>".to_string());
            }
        }
    }
}

pub fn handle_compat<F>(
    terminal_caps: &crate::terminal::TerminalCapabilities,
    color_mode: crate::types::ColorMode,
    mut output: F,
) where
    F: FnMut(String),
{
    let term = terminal_caps.term_program.as_deref().unwrap_or("unknown");
    let color = match color_mode {
        crate::types::ColorMode::TrueColor => "TRUECOLOR (24-BIT)",
        crate::types::ColorMode::Color256 => "256-COLOR",
    };
    output(format!("TERMINAL: {}", term.to_uppercase()));
    output(format!("COLOR MODE: {}", color));
    if !terminal_caps.true_color {
        output("TIP: USE ITERM2 FOR BEST RESULTS".to_string());
    }
}

pub fn handle_compat_mode<F>(
    parts: &[&str],
    ascii_meters: &mut bool,
    scope_settings: &mut crate::types::ScopeSettings,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode = if *ascii_meters { 1 } else { 0 };
        output(format!("COMPAT.MODE: {}", mode));
        return;
    }

    match parts[1] {
        "0" => {
            *ascii_meters = false;
            let _ = config::save_ascii_meters(false);
            output("COMPAT.MODE: 0 (FULL)".to_string());
        }
        "1" => {
            *ascii_meters = true;
            scope_settings.display_mode = 1;
            let _ = config::save_ascii_meters(true);
            let _ = config::save_scope_settings(scope_settings);
            output("COMPAT.MODE: 1 (BASIC)".to_string());
        }
        _ => output("ERROR: COMPAT.MODE TAKES 0 OR 1".to_string()),
    }
}

fn parse_script_id(id_str: &str) -> Option<usize> {
    let upper = id_str.to_uppercase();
    match upper.as_str() {
        "M" => Some(8),
        "I" => Some(9),
        _ => {
            if let Ok(num) = id_str.parse::<usize>() {
                if num >= 1 && num <= 8 {
                    Some(num - 1)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

fn script_id_label(index: usize) -> String {
    match index {
        0..=7 => format!("{}", index + 1),
        8 => "M".to_string(),
        9 => "I".to_string(),
        _ => format!("{}", index),
    }
}

pub fn handle_mute<F>(
    parts: &[&str],
    script_mutes: &mut crate::types::ScriptMutes,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::eval::eval_expression;
    use crate::types::{TIER_CONFIRMS, TIER_QUERIES};

    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output("SCRIPT MUTES:".to_string());
            for i in 0..10 {
                let label = script_id_label(i);
                let status = if script_mutes.muted[i] { "MUTED" } else { "ACTIVE" };
                output(format!("  {}: {}", label, status));
            }
        }
        return;
    }

    let Some(index) = parse_script_id(parts[1]) else {
        output("ERROR: INVALID SCRIPT ID (USE 1-8, M, I)".to_string());
        return;
    };

    if parts.len() == 2 {
        script_mutes.muted[index] = !script_mutes.muted[index];
        let label = script_id_label(index);
        let status = if script_mutes.muted[index] { "MUTED" } else { "ACTIVE" };
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SCRIPT {}: {}", label, status));
        }
        return;
    }

    if let Some((val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
        script_mutes.muted[index] = val != 0;
        let label = script_id_label(index);
        let status = if script_mutes.muted[index] { "MUTED" } else { "ACTIVE" };
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SCRIPT {}: {}", label, status));
        }
    } else {
        output("ERROR: INVALID MUTE VALUE EXPRESSION".to_string());
    }
}

pub fn handle_page<F>(
    parts: &[&str],
    current_page: &mut crate::types::Page,
    show_grid_view: &mut bool,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::types::{Page, TIER_CONFIRMS};

    if parts.len() < 2 {
        output("ERROR: PAGE REQUIRES PAGE NAME OR NUMBER".to_string());
        return;
    }

    let page_arg = parts[1].to_uppercase();
    let target_page = match page_arg.as_str() {
        "LIVE" | "L" => Page::Live,
        "HELP" | "H" => Page::Help,
        "GRID" | "G" => {
            *show_grid_view = true;
            Page::Live
        }
        "1" => Page::Script1,
        "2" => Page::Script2,
        "3" => Page::Script3,
        "4" => Page::Script4,
        "5" => Page::Script5,
        "6" => Page::Script6,
        "7" => Page::Script7,
        "8" => Page::Script8,
        "M" => Page::Metro,
        "I" => Page::Init,
        "P" => Page::Pattern,
        "V" => Page::Variables,
        "N" => Page::Notes,
        "S" => Page::Scope,
        _ => {
            output(format!("ERROR: INVALID PAGE \"{}\"", page_arg));
            return;
        }
    };

    *current_page = target_page;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("PAGE: {}", current_page.name()));
    }
}
