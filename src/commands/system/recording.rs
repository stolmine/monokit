use crate::types::{MetroCommand, TIER_CONFIRMS};
use anyhow::{Context, Result};
use std::sync::mpsc::Sender;

pub fn handle_rec(
    metro_tx: &Sender<MetroCommand>,
) -> Result<()> {
    // Get current working directory
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    metro_tx
        .send(MetroCommand::StartRecording(cwd))
        .context("Failed to send recording command")?;
    // Output handled by UI event handler when recording actually starts
    Ok(())
}

pub fn handle_rec_stop(
    metro_tx: &Sender<MetroCommand>,
) -> Result<()> {
    metro_tx
        .send(MetroCommand::StopRecording)
        .context("Failed to send stop recording command")?;
    // Output handled by UI event handler when recording actually stops
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
