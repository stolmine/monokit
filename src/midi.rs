use crate::types::MetroCommand;
use midir::{MidiInput, MidiInputConnection};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

const MIDI_CLOCK: u8 = 0xF8;
const MIDI_START: u8 = 0xFA;
const MIDI_STOP: u8 = 0xFC;
const PULSES_PER_16TH: u8 = 6;

/// Type alias for the MIDI input connection - must be kept alive for connection to persist
pub type MidiConnection = MidiInputConnection<()>;

pub fn list_midi_inputs() -> Result<Vec<String>, Box<dyn Error>> {
    let midi_in = MidiInput::new("monokit-list")?;
    let ports = midi_in.ports();

    let mut names = Vec::new();
    for port in &ports {
        if let Ok(name) = midi_in.port_name(port) {
            names.push(name);
        }
    }

    Ok(names)
}

pub fn connect_midi_input(
    device_name: &str,
    metro_tx: Sender<MetroCommand>,
) -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    let midi_in = MidiInput::new("monokit")?;
    let ports = midi_in.ports();

    let port = ports
        .iter()
        .find(|p| {
            if let Ok(name) = midi_in.port_name(p) {
                name.to_lowercase().contains(&device_name.to_lowercase())
            } else {
                false
            }
        })
        .ok_or_else(|| format!("MIDI device '{}' not found", device_name))?;

    let pulse_counter = Arc::new(Mutex::new(0u8));

    let connection = midi_in.connect(
        port,
        "monokit-input",
        move |_stamp, message, _| {
            if message.is_empty() {
                return;
            }

            let status = message[0];

            match status {
                MIDI_CLOCK => {
                    let mut counter = pulse_counter.lock().unwrap();
                    *counter += 1;

                    if *counter >= PULSES_PER_16TH {
                        *counter = 0;
                        let _ = metro_tx.send(MetroCommand::MidiClockTick);
                    }
                }
                MIDI_START => {
                    let mut counter = pulse_counter.lock().unwrap();
                    *counter = 0;
                    let _ = metro_tx.send(MetroCommand::MidiTransportStart);
                }
                MIDI_STOP => {
                    let _ = metro_tx.send(MetroCommand::MidiTransportStop);
                }
                _ => {}
            }
        },
        (),
    )?;

    Ok(connection)
}

pub fn spawn_midi_thread(
    device_name: String,
    metro_tx: Sender<MetroCommand>,
) -> Result<std::thread::JoinHandle<()>, Box<dyn Error>> {
    let connection = connect_midi_input(&device_name, metro_tx)?;

    let handle = std::thread::spawn(move || {
        let _conn = connection;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    Ok(handle)
}
