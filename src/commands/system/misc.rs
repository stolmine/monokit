use crate::config;
use crate::eval::eval_expression;
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES, TIER_CONFIRMS, TIER_VERBOSE};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_tr<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx
        .send(MetroCommand::SendTrigger)
        .context("Failed to send trigger to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("SENT TRIGGER".to_string());
    }
    Ok(())
}

pub fn handle_vol<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: VOL REQUIRES A VALUE (0.0-1.0)".to_string());
        return Ok(());
    }
    let value: f32 = parts[1]
        .parse()
        .context("Failed to parse volume value as float")?;
    if !(0.0..=1.0).contains(&value) {
        output("VOLUME MUST BE BETWEEN 0.0 AND 1.0".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SendVolume(value))
        .context("Failed to send volume to metro thread")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET VOLUME TO {}", value));
    }
    Ok(())
}

pub fn handle_rst<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(131.0)))?;
    metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(262.0)))?;
    metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("dc".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("dm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mx".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("me".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("fb".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fba".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fbd".to_string(), OscType::Int(10)))?;

    metro_tx.send(MetroCommand::SendParam("ad".to_string(), OscType::Int(100)))?;
    metro_tx.send(MetroCommand::SendParam("pd".to_string(), OscType::Int(10)))?;
    metro_tx.send(MetroCommand::SendParam("fd".to_string(), OscType::Int(10)))?;
    metro_tx.send(MetroCommand::SendParam("dd".to_string(), OscType::Int(10)))?;
    metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(0.0)))?;
    metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("fc".to_string(), OscType::Float(10000.0)))?;
    metro_tx.send(MetroCommand::SendParam("fq".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ft".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fe".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fed".to_string(), OscType::Int(100)))?;
    metro_tx.send(MetroCommand::SendParam("fk".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mf_f".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("rf".to_string(), OscType::Float(440.0)))?;
    metro_tx.send(MetroCommand::SendParam("rd".to_string(), OscType::Int(500)))?;
    metro_tx.send(MetroCommand::SendParam("rm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("rk".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("dt".to_string(), OscType::Int(250)))?;
    metro_tx.send(MetroCommand::SendParam("df".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("dlp".to_string(), OscType::Int(5000)))?;
    metro_tx.send(MetroCommand::SendParam("dw".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ds".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("dmode".to_string(), OscType::Int(2)))?;
    metro_tx.send(MetroCommand::SendParam("dtail".to_string(), OscType::Int(1)))?;

    metro_tx.send(MetroCommand::SendParam("rv".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("rp".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("rh".to_string(), OscType::Int(8000)))?;
    metro_tx.send(MetroCommand::SendParam("rw".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("rmode".to_string(), OscType::Int(2)))?;
    metro_tx.send(MetroCommand::SendParam("rtail".to_string(), OscType::Int(1)))?;

    metro_tx.send(MetroCommand::SendParam("lb".to_string(), OscType::Int(16)))?;
    metro_tx.send(MetroCommand::SendParam("ls".to_string(), OscType::Int(48000)))?;
    metro_tx.send(MetroCommand::SendParam("lm".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("rgf".to_string(), OscType::Float(440.0)))?;
    metro_tx.send(MetroCommand::SendParam("rgw".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("rgm".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("ct".to_string(), OscType::Int(8192)))?;
    metro_tx.send(MetroCommand::SendParam("cr".to_string(), OscType::Int(1)))?;
    metro_tx.send(MetroCommand::SendParam("ca".to_string(), OscType::Int(10)))?;
    metro_tx.send(MetroCommand::SendParam("cl".to_string(), OscType::Int(100)))?;
    metro_tx.send(MetroCommand::SendParam("cm".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("el".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("em".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ef".to_string(), OscType::Float(1000.0)))?;
    metro_tx.send(MetroCommand::SendParam("eq".to_string(), OscType::Float(1.0)))?;
    metro_tx.send(MetroCommand::SendParam("eh".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("pn".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("gate".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("br_act".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("br_len".to_string(), OscType::Int(250)))?;
    metro_tx.send(MetroCommand::SendParam("br_rev".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("br_win".to_string(), OscType::Int(5)))?;
    metro_tx.send(MetroCommand::SendParam("br_mix".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendParam("ps_mode".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ps_semi".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ps_grain".to_string(), OscType::Int(20)))?;
    metro_tx.send(MetroCommand::SendParam("ps_mix".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ps_targ".to_string(), OscType::Int(0)))?;

    metro_tx.send(MetroCommand::SendVolume(1.0))?;

    metro_tx.send(MetroCommand::SendParam("slew_time".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("slew_pf".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_mf".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_fc".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_fm".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_mx".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_dc".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_fb".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_fq".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_fk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_fe".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_rf".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_rm".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_dt".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_df".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_dw".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_rv".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_rw".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_volume".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_pn".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_lb".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_ls".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_lm".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_rgf".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_rgm".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_ct".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_cm".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_el".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_em".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_eh".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("slew_ef".to_string(), OscType::Int(-1)))?;

    metro_tx.send(MetroCommand::SendParam("env_atk".to_string(), OscType::Int(1)))?;
    metro_tx.send(MetroCommand::SendParam("env_crv".to_string(), OscType::Int(-4)))?;
    metro_tx.send(MetroCommand::SendParam("aenv_atk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("penv_atk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("fmev_atk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("denv_atk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("fbev_atk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("flev_atk".to_string(), OscType::Int(-1)))?;
    metro_tx.send(MetroCommand::SendParam("aenv_crv".to_string(), OscType::Int(-100)))?;
    metro_tx.send(MetroCommand::SendParam("penv_crv".to_string(), OscType::Int(-100)))?;
    metro_tx.send(MetroCommand::SendParam("fmev_crv".to_string(), OscType::Int(-100)))?;
    metro_tx.send(MetroCommand::SendParam("denv_crv".to_string(), OscType::Int(-100)))?;
    metro_tx.send(MetroCommand::SendParam("fbev_crv".to_string(), OscType::Int(-100)))?;
    metro_tx.send(MetroCommand::SendParam("flev_crv".to_string(), OscType::Int(-100)))?;

    if debug_level >= TIER_ESSENTIAL || out_ess {
        output("RESET TO DEFAULTS".to_string());
    }
    Ok(())
}

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
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        // Show current theme and list available themes
        match config::load_config() {
            Ok(cfg) => {
                let available = config::list_themes(&cfg);
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
                output(format!("AVAILABLE: {}", available.join(", ").to_uppercase()));
            }
            Err(_) => {
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
                output("AVAILABLE: DARK, LIGHT, SYSTEM".to_string());
            }
        }
    } else {
        let name = parts[1];
        match config::load_config() {
            Ok(cfg) => {
                match config::load_theme_by_name(name, &cfg) {
                    Ok(new_theme) => {
                        let theme_name = new_theme.name.clone();
                        *theme = new_theme;
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
    output("CLEAR           CLEAR OUTPUT HISTORY".to_string());
    output("DEBUG <0-5>  SET VERBOSITY LEVEL".to_string());
    output("HEADER       SHOW CURRENT HEADER LEVEL".to_string());
    output("HEADER <0-4>    SET HEADER VERBOSITY".to_string());
    output("                0=NAV ONLY, 1=+METERS, 2=+TR".to_string());
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

pub fn handle_rec<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    // Get current working directory
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    metro_tx
        .send(MetroCommand::StartRecording(cwd))
        .context("Failed to send recording command")?;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output("RECORDING STARTED".to_string());
    }
    Ok(())
}

pub fn handle_rec_stop<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx
        .send(MetroCommand::StopRecording)
        .context("Failed to send stop recording command")?;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output("RECORDING STOPPED".to_string());
    }
    Ok(())
}

pub fn handle_rec_path<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("REC.PATH REQUIRES A PATH PREFIX".to_string());
        return Ok(());
    }

    let path = parts[1].to_string();
    metro_tx
        .send(MetroCommand::SetRecordingPath(path.clone()))
        .context("Failed to send recording path")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET RECORDING PATH PREFIX TO: {}", path.to_uppercase()));
    }
    Ok(())
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
            if debug_level >= TIER_ERRORS || out_ess {
                output(literal.to_string());
            }
        } else {
            output("UNTERMINATED STRING LITERAL".to_string());
        }
    } else {
        if let Some((result, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            if debug_level >= TIER_ERRORS || out_ess {
                output(format!("{}", result));
            }
        } else {
            output("FAILED TO EVALUATE EXPRESSION".to_string());
        }
    }
}

pub fn handle_debug<F>(
    parts: &[&str],
    debug_level: &mut u8,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("DEBUG LEVEL: {}", debug_level));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *debug_level = 0;
                let _ = config::save_debug_level(*debug_level);
                output("DEBUG: 0 (SILENT)".to_string());
            }
            "1" => {
                *debug_level = 1;
                let _ = config::save_debug_level(*debug_level);
                output("DEBUG: 1 (ERRORS)".to_string());
            }
            "2" => {
                *debug_level = 2;
                let _ = config::save_debug_level(*debug_level);
                output("DEBUG: 2 (ESSENTIAL)".to_string());
            }
            "3" => {
                *debug_level = 3;
                let _ = config::save_debug_level(*debug_level);
                output("DEBUG: 3 (QUERIES)".to_string());
            }
            "4" => {
                *debug_level = 4;
                let _ = config::save_debug_level(*debug_level);
                output("DEBUG: 4 (CONFIRMS)".to_string());
            }
            "5" => {
                *debug_level = 5;
                let _ = config::save_debug_level(*debug_level);
                output("DEBUG: 5 (VERBOSE)".to_string());
            }
            _ => {
                output("ERROR: DEBUG TAKES 0-5".to_string());
            }
        }
    }
}

pub fn handle_cpu<F>(
    parts: &[&str],
    show_cpu: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("CPU DISPLAY: {}", if *show_cpu { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_cpu = false;
                let _ = config::save_show_cpu(*show_cpu);
                output("CPU DISPLAY: OFF".to_string());
            }
            "1" => {
                *show_cpu = true;
                let _ = config::save_show_cpu(*show_cpu);
                output("CPU DISPLAY: ON".to_string());
            }
            _ => {
                output("ERROR: CPU TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_bpm<F>(
    parts: &[&str],
    show_bpm: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("BPM: {}", if *show_bpm { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_bpm = false;
                let _ = config::save_show_bpm(*show_bpm);
                output("BPM: OFF".to_string());
            }
            "1" => {
                *show_bpm = true;
                let _ = config::save_show_bpm(*show_bpm);
                output("BPM: ON".to_string());
            }
            _ => {
                output("ERROR: BPM TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

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
                output("HEADER LEVEL: 2 (NAV + TR + METERS)".to_string());
            }
            "3" => {
                *header_level = 3;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 3 (FULL NAV + TR + METERS)".to_string());
            }
            "4" => {
                *header_level = 4;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 4 (FULL NAV + TR + METERS + CPU)".to_string());
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
    debug_level: u8,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("LIMITER: {}", if *limiter_enabled { 1 } else { 0 }));
        }
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *limiter_enabled = false;
                metro_tx
                    .send(MetroCommand::SendParam("limit".to_string(), OscType::Int(0)))
                    .context("Failed to send limiter param")?;
                let _ = config::save_limiter_enabled(*limiter_enabled);
                if debug_level >= TIER_CONFIRMS || out_cfm {
                    output("LIMITER: OFF".to_string());
                }
            }
            "1" => {
                *limiter_enabled = true;
                metro_tx
                    .send(MetroCommand::SendParam("limit".to_string(), OscType::Int(1)))
                    .context("Failed to send limiter param")?;
                let _ = config::save_limiter_enabled(*limiter_enabled);
                if debug_level >= TIER_CONFIRMS || out_cfm {
                    output("LIMITER: ON".to_string());
                }
            }
            _ => {
                output("ERROR: LIMIT TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
    Ok(())
}

pub fn handle_load_rst<F>(
    parts: &[&str],
    load_rst: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("LOAD.RST: {}", if *load_rst { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *load_rst = false;
                let _ = config::save_load_rst(*load_rst);
                output("LOAD.RST: OFF (PERSIST PARAMS)".to_string());
            }
            "1" => {
                *load_rst = true;
                let _ = config::save_load_rst(*load_rst);
                output("LOAD.RST: ON (RESET BEFORE LOAD)".to_string());
            }
            _ => {
                output("ERROR: LOAD.RST TAKES 0-1".to_string());
            }
        }
    }
}

pub fn handle_scope_time<F>(
    parts: &[&str],
    scope_timespan_ms: &mut u32,
    scope_color_mode: &u8,
    scope_display_mode: &u8,
    scope_unipolar: &bool,
    metro_tx: &Sender<MetroCommand>,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("SCOPE.TIME: {}MS", scope_timespan_ms));
        }
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

        *scope_timespan_ms = value;

        metro_tx
            .send(MetroCommand::SendScopeRate(value as f32))
            .context("Failed to send scope rate")?;

        let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);

        if debug_level >= TIER_VERBOSE {
            output(format!("SCOPE.TIME: {}MS", value));
        }
    }
    Ok(())
}

pub fn handle_scope_clr<F>(
    parts: &[&str],
    scope_timespan_ms: &u32,
    scope_color_mode: &mut u8,
    scope_display_mode: &u8,
    scope_unipolar: &bool,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            let mode_name = match *scope_color_mode {
                1 => "ERROR",
                2 => "FOREGROUND",
                3 => "ACCENT",
                _ => "SUCCESS",
            };
            output(format!("SCOPE.CLR: {} ({})", scope_color_mode, mode_name));
        }
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                *scope_color_mode = 0;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE {
                    output("SCOPE.CLR: 0 (SUCCESS)".to_string());
                }
            }
            1 => {
                *scope_color_mode = 1;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE {
                    output("SCOPE.CLR: 1 (ERROR)".to_string());
                }
            }
            2 => {
                *scope_color_mode = 2;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE {
                    output("SCOPE.CLR: 2 (FOREGROUND)".to_string());
                }
            }
            3 => {
                *scope_color_mode = 3;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE {
                    output("SCOPE.CLR: 3 (ACCENT)".to_string());
                }
            }
            _ => {
                output("ERROR: SCOPE.CLR TAKES 0-3".to_string());
            }
        }
    }
}

pub fn handle_scope_mode<F>(
    parts: &[&str],
    scope_timespan_ms: &u32,
    scope_color_mode: &u8,
    scope_display_mode: &mut u8,
    scope_unipolar: &bool,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            let mode_name = match *scope_display_mode {
                1 => "BLOCK",
                2 => "LINE",
                3 => "DOT",
                4 => "QUADRANT",
                _ => "BRAILLE",
            };
            output(format!("SCOPE.MODE: {} ({})", scope_display_mode, mode_name));
        }
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                *scope_display_mode = 0;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.MODE: 0 (BRAILLE)".to_string()); }
            }
            1 => {
                *scope_display_mode = 1;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.MODE: 1 (BLOCK)".to_string()); }
            }
            2 => {
                *scope_display_mode = 2;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.MODE: 2 (LINE)".to_string()); }
            }
            3 => {
                *scope_display_mode = 3;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.MODE: 3 (DOT)".to_string()); }
            }
            4 => {
                *scope_display_mode = 4;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.MODE: 4 (QUADRANT)".to_string()); }
            }
            _ => {
                output("ERROR: SCOPE.MODE TAKES 0-4".to_string());
            }
        }
    }
}

pub fn handle_scope_uni<F>(
    parts: &[&str],
    scope_timespan_ms: &u32,
    scope_color_mode: &u8,
    scope_display_mode: &u8,
    scope_unipolar: &mut bool,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("SCOPE.UNI: {}", if *scope_unipolar { 1 } else { 0 }));
        }
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                *scope_unipolar = false;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.UNI: 0 (BIPOLAR)".to_string()); }
            }
            1 => {
                *scope_unipolar = true;
                let _ = config::save_scope_settings(*scope_timespan_ms, *scope_color_mode, *scope_display_mode, *scope_unipolar);
                if debug_level >= TIER_VERBOSE { output("SCOPE.UNI: 1 (UNIPOLAR)".to_string()); }
            }
            _ => {
                output("ERROR: SCOPE.UNI TAKES 0-1".to_string());
            }
        }
    }
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

pub fn handle_meter_hdr<F>(
    parts: &[&str],
    show_meters_header: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("HEADER METERS: {}", if *show_meters_header { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_meters_header = false;
                let _ = config::save_show_meters_header(*show_meters_header);
                output("HEADER METERS: OFF".to_string());
            }
            "1" => {
                *show_meters_header = true;
                let _ = config::save_show_meters_header(*show_meters_header);
                output("HEADER METERS: ON".to_string());
            }
            _ => {
                output("ERROR: METER.HDR TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_meter_grid<F>(
    parts: &[&str],
    show_meters_grid: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("GRID METERS: {}", if *show_meters_grid { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_meters_grid = false;
                let _ = config::save_show_meters_grid(*show_meters_grid);
                output("GRID METERS: OFF".to_string());
            }
            "1" => {
                *show_meters_grid = true;
                let _ = config::save_show_meters_grid(*show_meters_grid);
                output("GRID METERS: ON".to_string());
            }
            _ => {
                output("ERROR: METER.GRID TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_spectrum<F>(
    parts: &[&str],
    show_spectrum: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SPECTRUM: {}", if *show_spectrum { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_spectrum = false;
                let _ = config::save_show_spectrum(*show_spectrum);
                output("SPECTRUM: OFF".to_string());
            }
            "1" => {
                *show_spectrum = true;
                let _ = config::save_show_spectrum(*show_spectrum);
                output("SPECTRUM: ON".to_string());
            }
            _ => {
                output("ERROR: SPECTRUM TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_activity<F>(
    parts: &[&str],
    show_activity: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("ACTIVITY: {}", if *show_activity { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_activity = false;
                let _ = config::save_show_activity(*show_activity);
                output("ACTIVITY: OFF".to_string());
            }
            "1" => {
                *show_activity = true;
                let _ = config::save_show_activity(*show_activity);
                output("ACTIVITY: ON".to_string());
            }
            _ => {
                output("ERROR: ACTIVITY TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_grid<F>(
    parts: &[&str],
    show_grid: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("GRID: {}", if *show_grid { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_grid = false;
                let _ = config::save_show_grid(*show_grid);
                output("GRID: OFF".to_string());
            }
            "1" => {
                *show_grid = true;
                let _ = config::save_show_grid(*show_grid);
                output("GRID: ON".to_string());
            }
            _ => {
                output("ERROR: GRID TAKES 0-1".to_string());
            }
        }
    }
}

pub fn handle_grid_def<F>(
    parts: &[&str],
    show_grid_view: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("GRID.DEF: {}", if *show_grid_view { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_grid_view = false;
                let _ = config::save_show_grid_view(*show_grid_view);
                output("GRID.DEF: 0 (REPL)".to_string());
            }
            "1" => {
                *show_grid_view = true;
                let _ = config::save_show_grid_view(*show_grid_view);
                output("GRID.DEF: 1 (GRID)".to_string());
            }
            _ => {
                output("ERROR: GRID.DEF TAKES 0-1".to_string());
            }
        }
    }
}

pub fn handle_hl_seq<F>(
    parts: &[&str],
    show_seq_highlight: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SEQ HIGHLIGHT: {}", if *show_seq_highlight { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *show_seq_highlight = false;
                let _ = config::save_show_seq_highlight(*show_seq_highlight);
                output("SEQ HIGHLIGHT: OFF".to_string());
            }
            "1" => {
                *show_seq_highlight = true;
                let _ = config::save_show_seq_highlight(*show_seq_highlight);
                output("SEQ HIGHLIGHT: ON".to_string());
            }
            _ => {
                output("ERROR: HL.SEQ TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_grid_mode<F>(
    parts: &[&str],
    grid_mode: &mut u8,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode_name = if *grid_mode == 0 { "LABELS" } else { "ICONS" };
        output(format!("GRID.MODE: {} ({})", grid_mode, mode_name));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *grid_mode = 0;
                let _ = config::save_grid_mode(*grid_mode);
                output("GRID.MODE: 0 (LABELS)".to_string());
            }
            "1" => {
                *grid_mode = 1;
                let _ = config::save_grid_mode(*grid_mode);
                output("GRID.MODE: 1 (ICONS)".to_string());
            }
            _ => {
                output("ERROR: GRID.MODE TAKES 0-1".to_string());
            }
        }
    }
}

pub fn handle_title<F>(
    parts: &[&str],
    title_mode: &mut u8,
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
                output("TITLE: 0 (MONOKIT)".to_string());
            }
            "1" => {
                *title_mode = 1;
                let _ = config::save_title_mode(*title_mode);
                output("TITLE: 1 (SCENE NAME)".to_string());
            }
            _ => {
                output("ERROR: TITLE TAKES 0 (MONOKIT) OR 1 (SCENE)".to_string());
            }
        }
    }
}

pub fn handle_out_err<F>(
    parts: &[&str],
    out_err: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("OUT.ERR: {}", if *out_err { "ON" } else { "OFF" }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *out_err = false;
                let _ = config::save_out_err(*out_err);
                output("OUT.ERR: OFF".to_string());
            }
            "1" => {
                *out_err = true;
                let _ = config::save_out_err(*out_err);
                output("OUT.ERR: ON".to_string());
            }
            _ => {
                output("ERROR: OUT.ERR TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_out_ess<F>(
    parts: &[&str],
    out_ess: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("OUT.ESS: {}", if *out_ess { "ON" } else { "OFF" }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *out_ess = false;
                let _ = config::save_out_ess(*out_ess);
                output("OUT.ESS: OFF".to_string());
            }
            "1" => {
                *out_ess = true;
                let _ = config::save_out_ess(*out_ess);
                output("OUT.ESS: ON".to_string());
            }
            _ => {
                output("ERROR: OUT.ESS TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_out_qry<F>(
    parts: &[&str],
    out_qry: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("OUT.QRY: {}", if *out_qry { "ON" } else { "OFF" }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *out_qry = false;
                let _ = config::save_out_qry(*out_qry);
                output("OUT.QRY: OFF".to_string());
            }
            "1" => {
                *out_qry = true;
                let _ = config::save_out_qry(*out_qry);
                output("OUT.QRY: ON".to_string());
            }
            _ => {
                output("ERROR: OUT.QRY TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}

pub fn handle_out_cfm<F>(
    parts: &[&str],
    out_cfm: &mut bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("OUT.CFM: {}", if *out_cfm { "ON" } else { "OFF" }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *out_cfm = false;
                let _ = config::save_out_cfm(*out_cfm);
                output("OUT.CFM: OFF".to_string());
            }
            "1" => {
                *out_cfm = true;
                let _ = config::save_out_cfm(*out_cfm);
                output("OUT.CFM: ON".to_string());
            }
            _ => {
                output("ERROR: OUT.CFM TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
}
