use crate::midi::{MidiConnection, MidiTimingStats};
use crate::types::MetroCommand;
use anyhow::Result;
use std::sync::{mpsc::Sender, Arc};

pub fn handle_midi_in<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    midi_connection: &mut Option<MidiConnection>,
    timing_stats: &Arc<MidiTimingStats>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        // List available MIDI inputs
        match crate::midi::list_midi_inputs() {
            Ok(devices) => {
                if devices.is_empty() {
                    output("NO MIDI INPUTS FOUND".to_string());
                } else {
                    output("AVAILABLE MIDI INPUTS:".to_string());
                    for device in devices {
                        output(format!("  {}", device.to_uppercase()));
                    }
                    if midi_connection.is_some() {
                        output("CONNECTED: YES".to_string());
                    }
                }
            }
            Err(e) => {
                output(format!("ERROR LISTING MIDI INPUTS: {}", e));
            }
        }
    } else {
        // Connect to specified device
        let device_name = parts[1..].join(" ");
        match crate::midi::connect_midi_input(&device_name, metro_tx.clone(), Some(timing_stats.clone())) {
            Ok(conn) => {
                *midi_connection = Some(conn);
                output(format!("CONNECTED TO MIDI INPUT: {}", device_name.to_uppercase()));
                output("USE M.SYNC 1 TO ENABLE MIDI CLOCK SYNC".to_string());
            }
            Err(e) => {
                output(format!("ERROR CONNECTING TO MIDI: {}", e));
            }
        }
    }
    Ok(())
}

pub fn handle_midi_diag<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    timing_stats: &Arc<MidiTimingStats>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output("MIDI TIMING DIAGNOSTICS".to_string());
        output("MIDI.DIAG 1    - ENABLE DIAGNOSTICS".to_string());
        output("MIDI.DIAG 0    - DISABLE DIAGNOSTICS".to_string());
        output("MIDI.DIAG REPORT - PRINT TIMING REPORT".to_string());
    } else {
        match parts[1] {
            "1" => {
                timing_stats.enable();
                metro_tx.send(MetroCommand::EnableMidiTimingDiag)?;
                output("MIDI TIMING DIAGNOSTICS ENABLED".to_string());
                output("COLLECTING TIMING DATA...".to_string());
            }
            "0" => {
                timing_stats.disable();
                metro_tx.send(MetroCommand::DisableMidiTimingDiag)?;
                output("MIDI TIMING DIAGNOSTICS DISABLED".to_string());
            }
            "REPORT" | "R" => {
                // Write MIDI callback timing to file
                let result = timing_stats.write_report();
                output(result);
                // Also trigger metro thread report (will append to same concepts)
                metro_tx.send(MetroCommand::PrintMidiTimingReport)?;
            }
            _ => {
                output("INVALID ARGUMENT. USE 1, 0, OR REPORT".to_string());
            }
        }
    }
    Ok(())
}
