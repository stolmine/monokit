use crate::commands::context::ExecutionContext;
use crate::types::{MetroCommand, OutputCategory, SyncMode};
use anyhow::{Context, Result};

pub fn handle_m<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        ctx.output(
            OutputCategory::Query,
            format!("METRO INTERVAL: {}MS", ctx.metro_interval),
            &mut output,
        );
    } else {
        let value: u64 = parts[1]
            .parse()
            .context("Failed to parse interval as milliseconds")?;
        if value == 0 {
            output("INTERVAL MUST BE GREATER THAN 0".to_string());
            return Ok(());
        }
        ctx.metro_tx
            .send(MetroCommand::SetInterval(value))
            .context("Failed to send interval to metro thread")?;
        *ctx.metro_interval = value;
        ctx.output(
            OutputCategory::Essential,
            format!("SET METRO INTERVAL TO {}MS", value),
            &mut output,
        );
    }
    Ok(())
}

pub fn handle_m_bpm<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("M.BPM REQUIRES A BPM VALUE".to_string());
        return Ok(());
    }
    let bpm: f32 = parts[1]
        .parse()
        .context("Failed to parse BPM value as number")?;
    if bpm <= 0.0 {
        output("BPM MUST BE GREATER THAN 0".to_string());
        return Ok(());
    }
    let interval_ms = (15000.0 / bpm) as u64;
    ctx.metro_tx
        .send(MetroCommand::SetInterval(interval_ms))
        .context("Failed to send interval to metro thread")?;
    *ctx.metro_interval = interval_ms;
    ctx.output(
        OutputCategory::Essential,
        format!("SET METRO TO {} BPM ({}MS)", bpm, interval_ms),
        &mut output,
    );
    Ok(())
}

pub fn handle_m_act<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("M.ACT REQUIRES 0 OR 1".to_string());
        return Ok(());
    }
    let value: i32 = parts[1]
        .parse()
        .context("Failed to parse M.ACT value")?;
    if !(0..=1).contains(&value) {
        output("M.ACT VALUE MUST BE 0 OR 1".to_string());
        return Ok(());
    }
    ctx.metro_tx
        .send(MetroCommand::SetActive(value != 0))
        .context("Failed to send active state to metro thread")?;
    ctx.output(
        OutputCategory::Essential,
        format!(
            "METRO {}",
            if value != 0 {
                "ACTIVATED"
            } else {
                "DEACTIVATED"
            }
        ),
        &mut output,
    );
    Ok(())
}

pub fn handle_m_script<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("M.SCRIPT REQUIRES A SCRIPT NUMBER (1-8 OR M)".to_string());
        return Ok(());
    }
    let value: usize = if parts[1].to_uppercase() == "M" {
        8
    } else {
        let parsed_value: usize = parts[1]
            .parse()
            .context("Failed to parse script number")?;
        if !(1..=8).contains(&parsed_value) {
            output("M.SCRIPT VALUE MUST BE 1-8 OR M".to_string());
            return Ok(());
        }
        parsed_value
    };
    ctx.metro_tx
        .send(MetroCommand::SetScriptIndex(value - 1))
        .context("Failed to send script index to metro thread")?;
    ctx.output(
        OutputCategory::Essential,
        format!("METRO WILL CALL SCRIPT {} ON EACH TICK", value),
        &mut output,
    );
    Ok(())
}

pub fn handle_m_sync<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        let mode_str = match *ctx.sync_mode {
            SyncMode::Internal => "0 (INTERNAL)",
            SyncMode::MidiClock => "1 (MIDI CLOCK)",
        };
        output(format!("SYNC MODE: {}", mode_str));
        return Ok(());
    }
    let value: i32 = parts[1]
        .parse()
        .context("Failed to parse sync mode value")?;
    let new_mode = match value {
        0 => SyncMode::Internal,
        1 => SyncMode::MidiClock,
        _ => {
            output("M.SYNC MUST BE 0 OR 1".to_string());
            return Ok(());
        }
    };
    ctx.metro_tx
        .send(MetroCommand::SetSyncMode(new_mode))
        .context("Failed to send sync mode to metro thread")?;
    *ctx.sync_mode = new_mode;
    let mode_str = match new_mode {
        SyncMode::Internal => "INTERNAL",
        SyncMode::MidiClock => "MIDI CLOCK",
    };
    output(format!("SET SYNC MODE TO {}", mode_str));
    Ok(())
}
