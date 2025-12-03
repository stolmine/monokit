use crate::types::MetroCommand;
use anyhow::Result;
use std::sync::mpsc::Sender;

pub fn handle_audio_out<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    audio_devices: &[String],
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        metro_tx.send(MetroCommand::QueryAudioOutDevices)?;
        output("QUERYING OUTPUT DEVICES...".to_string());
    } else {
        // Check if argument is a number (device selection by index)
        let arg = parts[1..].join(" ");
        let device_name = if let Ok(index) = arg.parse::<usize>() {
            // Numbered selection (1-based)
            if index == 0 || index > audio_devices.len() {
                output(format!("ERROR: INVALID DEVICE #{}", index));
                output("RUN AUDIO.OUT TO LIST DEVICES".to_string());
                return Ok(());
            }
            audio_devices[index - 1].clone()
        } else {
            // Direct name (for scripting/config)
            arg
        };

        metro_tx.send(MetroCommand::SetAudioOutDevice(device_name.clone()))?;
        output(format!("SETTING: {}", device_name));
        output("RESTARTING AUDIO ENGINE...".to_string());
    }
    Ok(())
}
