use crate::types::{MetroCommand, SyncMode, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES, TIER_CONFIRMS, TIER_VERBOSE};
use anyhow::{Context, Result};
use std::sync::mpsc::Sender;

pub fn handle_m<F>(
    parts: &[&str],
    metro_interval: &mut u64,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_qry: bool,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("METRO INTERVAL: {}MS", metro_interval));
        }
    } else {
        let value: u64 = parts[1]
            .parse()
            .context("Failed to parse interval as milliseconds")?;
        if value == 0 {
            output("ERROR: INTERVAL MUST BE GREATER THAN 0".to_string());
            return Ok(());
        }
        metro_tx
            .send(MetroCommand::SetInterval(value))
            .context("Failed to send interval to metro thread")?;
        *metro_interval = value;
        if debug_level >= TIER_ESSENTIAL || out_ess {
            output(format!("SET METRO INTERVAL TO {}MS", value));
        }
    }
    Ok(())
}

pub fn handle_m_bpm<F>(
    parts: &[&str],
    metro_interval: &mut u64,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: M.BPM REQUIRES A BPM VALUE".to_string());
        return Ok(());
    }
    let bpm: f32 = parts[1]
        .parse()
        .context("Failed to parse BPM value as number")?;
    if bpm <= 0.0 {
        output("ERROR: BPM MUST BE GREATER THAN 0".to_string());
        return Ok(());
    }
    let interval_ms = (15000.0 / bpm) as u64; // 16th note interval (60000 / 4)
    metro_tx
        .send(MetroCommand::SetInterval(interval_ms))
        .context("Failed to send interval to metro thread")?;
    *metro_interval = interval_ms;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output(format!("SET METRO TO {} BPM ({}MS)", bpm, interval_ms));
    }
    Ok(())
}

pub fn handle_m_act<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: M.ACT REQUIRES 0 OR 1".to_string());
        return Ok(());
    }
    let value: i32 = parts[1]
        .parse()
        .context("Failed to parse M.ACT value")?;
    if !(0..=1).contains(&value) {
        output("ERROR: M.ACT VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    metro_tx
        .send(MetroCommand::SetActive(value != 0))
        .context("Failed to send active state to metro thread")?;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output(format!(
            "METRO {}",
            if value != 0 {
                "ACTIVATED"
            } else {
                "DEACTIVATED"
            }
        ));
    }
    Ok(())
}

pub fn handle_m_script<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: M.SCRIPT REQUIRES A SCRIPT NUMBER (1-8 OR M)".to_string());
        return Ok(());
    }
    let value: usize = if parts[1].to_uppercase() == "M" {
        8
    } else {
        let parsed_value: usize = parts[1]
            .parse()
            .context("Failed to parse script number")?;
        if !(1..=8).contains(&parsed_value) {
            output("ERROR: M.SCRIPT VALUE MUST BE 1-8 OR M".to_string());
            return Ok(());
        }
        parsed_value
    };
    metro_tx
        .send(MetroCommand::SetScriptIndex(value - 1))
        .context("Failed to send script index to metro thread")?;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output(format!("METRO WILL CALL SCRIPT {} ON EACH TICK", value));
    }
    Ok(())
}

pub fn handle_m_sync<F>(
    parts: &[&str],
    sync_mode: &mut SyncMode,
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_qry: bool,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        let mode_str = match *sync_mode {
            SyncMode::Internal => "0 (INTERNAL)",
            SyncMode::MidiClock => "1 (MIDI CLOCK)",
        };
        if debug_level >= TIER_QUERIES || out_qry {
            output(format!("SYNC MODE: {}", mode_str));
        }
        return Ok(());
    }
    let value: i32 = parts[1]
        .parse()
        .context("Failed to parse sync mode value")?;
    let new_mode = match value {
        0 => SyncMode::Internal,
        1 => SyncMode::MidiClock,
        _ => {
            output("ERROR: M.SYNC MUST BE 0 OR 1".to_string());
            return Ok(());
        }
    };
    metro_tx
        .send(MetroCommand::SetSyncMode(new_mode))
        .context("Failed to send sync mode to metro thread")?;
    *sync_mode = new_mode;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        let mode_str = match new_mode {
            SyncMode::Internal => "INTERNAL",
            SyncMode::MidiClock => "MIDI CLOCK",
        };
        output(format!("SET SYNC MODE TO {}", mode_str));
    }
    Ok(())
}
