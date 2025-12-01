use crate::midi::MidiConnection;
use crate::types::MetroCommand;
use anyhow::Result;
use std::sync::mpsc::Sender;

pub fn handle_midi_in<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    midi_connection: &mut Option<MidiConnection>,
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
        match crate::midi::connect_midi_input(&device_name, metro_tx.clone()) {
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
