use crate::config;
use crate::eval::eval_expression;
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_tr<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx
        .send(MetroCommand::SendTrigger)
        .context("Failed to send trigger to metro thread")?;
    if debug_level >= 2 {
        output("SENT TRIGGER".to_string());
    }
    Ok(())
}

pub fn handle_vol<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
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
        output("WARNING: VOLUME SHOULD BE BETWEEN 0.0 AND 1.0".to_string());
    }
    metro_tx
        .send(MetroCommand::SendVolume(value))
        .context("Failed to send volume to metro thread")?;
    if debug_level >= 2 {
        output(format!("SET VOLUME TO {}", value));
    }
    Ok(())
}

pub fn handle_rst<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(200.0)))?;
    metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(50.0)))?;
    metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("dc".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("dm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("dd".to_string(), OscType::Int(100)))?;
    metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("ad".to_string(), OscType::Int(100)))?;
    metro_tx.send(MetroCommand::SendParam("pd".to_string(), OscType::Int(10)))?;
    metro_tx.send(MetroCommand::SendParam("fd".to_string(), OscType::Int(10)))?;
    metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(4.0)))?;
    metro_tx.send(MetroCommand::SendParam("mx".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("mm".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("me".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(0)))?;
    metro_tx.send(MetroCommand::SendVolume(1.0))?;
    if debug_level >= 2 {
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
        return Err(anyhow::anyhow!("ERROR: SCRIPT REQUIRES NUMBER 1-8"));
    }
    let num: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse script number")?
    };
    if num < 1 || num > 8 {
        return Err(anyhow::anyhow!("ERROR: SCRIPT NUMBER MUST BE 1-8"));
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
                        output(format!("ERROR: {}", e.to_string().to_uppercase()));
                    }
                }
            }
            Err(e) => {
                output(format!("ERROR: FAILED TO LOAD CONFIG: {:?}", e));
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
    output("DEBUG <0-2>     SET VERBOSITY (0=SILENT, 1=IMPORTANT, 2=VERBOSE)".to_string());
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
    output("DM <0-6>      DC MODE (FOLD/TANH/SOFT/HARD/ASYM/RECT/CRUSH)".to_string());
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
    if debug_level >= 2 {
        output("RECORDING STARTED".to_string());
    }
    Ok(())
}

pub fn handle_rec_stop<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx
        .send(MetroCommand::StopRecording)
        .context("Failed to send stop recording command")?;
    if debug_level >= 2 {
        output("RECORDING STOPPED".to_string());
    }
    Ok(())
}

pub fn handle_rec_path<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: REC.PATH REQUIRES A PATH PREFIX".to_string());
        return Ok(());
    }

    let path = parts[1].to_string();
    metro_tx
        .send(MetroCommand::SetRecordingPath(path.clone()))
        .context("Failed to send recording path")?;
    if debug_level >= 2 {
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
            if debug_level >= 1 {
                output(literal.to_string());
            }
        } else {
            output("ERROR: UNTERMINATED STRING LITERAL".to_string());
        }
    } else {
        if let Some((result, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            if debug_level >= 1 {
                output(format!("{}", result));
            }
        } else {
            output("ERROR: FAILED TO EVALUATE EXPRESSION".to_string());
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
                output("DEBUG LEVEL: 0 (SILENT)".to_string());
            }
            "1" => {
                *debug_level = 1;
                output("DEBUG LEVEL: 1 (IMPORTANT)".to_string());
            }
            "2" => {
                *debug_level = 2;
                output("DEBUG LEVEL: 2 (VERBOSE)".to_string());
            }
            _ => {
                output("ERROR: DEBUG TAKES 0 (SILENT), 1 (IMPORTANT), OR 2 (VERBOSE)".to_string());
            }
        }
    }
}
