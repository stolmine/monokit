use cpal::traits::{DeviceTrait, HostTrait};
use std::fmt;

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub index: usize,
}

impl fmt::Display for AudioDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.index, self.name)
    }
}

/// List all available audio output devices using cpal.
/// Works on macOS, Windows, and Linux.
#[allow(deprecated)] // name() is deprecated in cpal 0.17 but still works
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    let host = cpal::default_host();

    let output_devices = host
        .output_devices()
        .map_err(|e| format!("Failed to enumerate audio devices: {}", e))?;

    let devices: Vec<AudioDevice> = output_devices
        .enumerate()
        .filter_map(|(index, device)| device.name().ok().map(|name| AudioDevice { name, index }))
        .collect();

    if devices.is_empty() {
        return Err("No audio output devices found".to_string());
    }

    Ok(devices)
}

/// Find an audio device by name.
pub fn find_device_by_name(name: &str) -> Result<Option<AudioDevice>, String> {
    let devices = list_audio_devices()?;

    for device in devices {
        if device.name == name {
            return Ok(Some(device));
        }
    }

    Ok(None)
}
