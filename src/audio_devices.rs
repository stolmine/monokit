use cpal::traits::{DeviceTrait, HostTrait};
use std::fmt;

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub host: String,
    pub index: usize,
}

impl fmt::Display for AudioDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.index, self.name, self.host)
    }
}

/// Format host ID as a short, readable string
fn format_host_id(host_id: cpal::HostId) -> String {
    // HostId debug format is like "HostId(Wasapi)" - extract the name
    let debug_str = format!("{:?}", host_id);
    debug_str
        .trim_start_matches("HostId(")
        .trim_end_matches(')')
        .to_uppercase()
}

/// List all available audio output devices across all hosts using cpal.
/// Works on macOS, Windows, and Linux.
/// Each device includes its host/backend (WASAPI, ASIO, CoreAudio, ALSA, etc.)
#[allow(deprecated)] // name() is deprecated in cpal 0.17 but still works
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    let mut devices = Vec::new();
    let mut index = 0;

    // Enumerate all available hosts (WASAPI, ASIO, CoreAudio, ALSA, JACK, etc.)
    for host_id in cpal::available_hosts() {
        let host = match cpal::host_from_id(host_id) {
            Ok(h) => h,
            Err(_) => continue,
        };

        let host_name = format_host_id(host_id);

        let output_devices = match host.output_devices() {
            Ok(d) => d,
            Err(_) => continue,
        };

        for device in output_devices {
            if let Ok(name) = device.name() {
                devices.push(AudioDevice {
                    name,
                    host: host_name.clone(),
                    index,
                });
                index += 1;
            }
        }
    }

    if devices.is_empty() {
        return Err("No audio output devices found".to_string());
    }

    Ok(devices)
}

/// Find an audio device by name (searches across all hosts).
pub fn find_device_by_name(name: &str) -> Result<Option<AudioDevice>, String> {
    let devices = list_audio_devices()?;

    for device in devices {
        if device.name == name {
            return Ok(Some(device));
        }
    }

    Ok(None)
}
