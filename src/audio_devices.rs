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

#[cfg(target_os = "macos")]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    use coreaudio::sys::*;
    use std::mem;
    use std::ptr;

    unsafe {
        let mut property_address = AudioObjectPropertyAddress {
            mSelector: kAudioHardwarePropertyDevices,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };

        let mut property_size: u32 = 0;
        let status = AudioObjectGetPropertyDataSize(
            kAudioObjectSystemObject,
            &property_address,
            0,
            ptr::null(),
            &mut property_size,
        );

        if status != 0 {
            return Err(format!("Failed to get device list size: {}", status));
        }

        let device_count = property_size as usize / mem::size_of::<AudioDeviceID>();
        let mut device_ids: Vec<AudioDeviceID> = vec![0; device_count];

        let status = AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &property_address,
            0,
            ptr::null(),
            &mut property_size,
            device_ids.as_mut_ptr() as *mut _,
        );

        if status != 0 {
            return Err(format!("Failed to get device list: {}", status));
        }

        let mut devices = Vec::new();

        for (index, &device_id) in device_ids.iter().enumerate() {
            property_address.mSelector = kAudioDevicePropertyStreamConfiguration;
            property_address.mScope = kAudioDevicePropertyScopeOutput;

            let mut property_size: u32 = 0;
            let status = AudioObjectGetPropertyDataSize(
                device_id,
                &property_address,
                0,
                ptr::null(),
                &mut property_size,
            );

            if status != 0 {
                continue;
            }

            let buffer_list_size = property_size as usize;
            let mut buffer: Vec<u8> = vec![0; buffer_list_size];
            let buffer_list = buffer.as_mut_ptr() as *mut AudioBufferList;

            let status = AudioObjectGetPropertyData(
                device_id,
                &property_address,
                0,
                ptr::null(),
                &mut property_size,
                buffer_list as *mut _,
            );

            if status != 0 {
                continue;
            }

            let num_buffers = (*buffer_list).mNumberBuffers;
            if num_buffers == 0 {
                continue;
            }

            property_address.mSelector = kAudioObjectPropertyName;
            property_address.mScope = kAudioObjectPropertyScopeGlobal;

            let mut name_ref: CFStringRef = ptr::null();
            property_size = mem::size_of::<CFStringRef>() as u32;

            let status = AudioObjectGetPropertyData(
                device_id,
                &property_address,
                0,
                ptr::null(),
                &mut property_size,
                &mut name_ref as *mut _ as *mut _,
            );

            if status != 0 {
                continue;
            }

            let name = if !name_ref.is_null() {
                let length = CFStringGetLength(name_ref);
                let mut buffer: Vec<u8> = vec![0; (length as usize + 1) * 2];
                let result = CFStringGetCString(
                    name_ref,
                    buffer.as_mut_ptr() as *mut _,
                    buffer.len() as _,
                    kCFStringEncodingUTF8,
                );

                let name = if result != 0 {
                    String::from_utf8_lossy(&buffer)
                        .trim_end_matches('\0')
                        .to_string()
                } else {
                    format!("Device {}", index)
                };

                CFRelease(name_ref as *const _);
                name
            } else {
                format!("Device {}", index)
            };

            devices.push(AudioDevice { name, index });
        }

        Ok(devices)
    }
}

#[cfg(target_os = "linux")]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Err("Linux uses JACK/PipeWire routing. Use system audio tools to configure.".to_string())
}

#[cfg(target_os = "windows")]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Err("Windows audio device enumeration not yet implemented. Using default device.".to_string())
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    Err("Audio device enumeration not supported on this platform".to_string())
}

pub fn find_device_by_name(name: &str) -> Result<Option<AudioDevice>, String> {
    let devices = list_audio_devices()?;

    for device in devices {
        if device.name == name {
            return Ok(Some(device));
        }
    }

    Ok(None)
}
