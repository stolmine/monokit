use crate::commands::context::ExecutionContext;
use crate::midi::MidiConnection;
use crate::types::MetroCommand;
use anyhow::Result;

pub fn handle_midi_in<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        match crate::midi::list_midi_inputs() {
            Ok(devices) => {
                if devices.is_empty() {
                    output("NO MIDI INPUTS FOUND".to_string());
                } else {
                    output("AVAILABLE MIDI INPUTS:".to_string());
                    for device in devices {
                        output(format!("  {}", device.to_uppercase()));
                    }
                    if ctx.midi_connection.is_some() {
                        output("CONNECTED: YES".to_string());
                    }
                }
            }
            Err(e) => {
                output(format!("ERROR LISTING MIDI INPUTS: {}", e));
            }
        }
    } else {
        let device_name = parts[1..].join(" ");
        match crate::midi::connect_midi_input(&device_name, ctx.metro_tx.clone(), Some(ctx.midi_timing_stats.clone())) {
            Ok(conn) => {
                *ctx.midi_connection = Some(conn);
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
    ctx: &mut ExecutionContext,
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
                ctx.midi_timing_stats.enable();
                ctx.metro_tx.send(MetroCommand::EnableMidiTimingDiag)?;
                output("MIDI TIMING DIAGNOSTICS ENABLED".to_string());
                output("COLLECTING TIMING DATA...".to_string());
            }
            "0" => {
                ctx.midi_timing_stats.disable();
                ctx.metro_tx.send(MetroCommand::DisableMidiTimingDiag)?;
                output("MIDI TIMING DIAGNOSTICS DISABLED".to_string());
            }
            "REPORT" | "R" => {
                let result = ctx.midi_timing_stats.write_report();
                output(result);
                ctx.metro_tx.send(MetroCommand::PrintMidiTimingReport)?;
            }
            _ => {
                output("INVALID ARGUMENT. USE 1, 0, OR REPORT".to_string());
            }
        }
    }
    Ok(())
}
