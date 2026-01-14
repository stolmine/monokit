use anyhow::{Context, Result};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::fs::File;
use std::net::UdpSocket;
use std::path::{Path, PathBuf};

use crate::types::OSC_ADDR;

pub const MAX_DISPLAY_WIDTH: usize = 46;
pub const INDENT_WIDTH: usize = 2;

pub fn resolve_sample_path<F>(path_str: &str, debug_level: u8, mut output: F) -> Option<PathBuf>
where
    F: FnMut(String),
{
    let home_dir = dirs::home_dir();

    let expanded = if path_str.starts_with('~') {
        home_dir.as_ref().map(|home| {
            if path_str == "~" {
                home.clone()
            } else {
                home.join(&path_str[2..])
            }
        })?
    } else {
        PathBuf::from(path_str)
    };

    if expanded.is_absolute() {
        return if expanded.exists() {
            Some(expanded)
        } else {
            None
        };
    }

    let library_path = crate::config::monokit_config_dir().ok()?.join("samples");

    let library_relative = library_path.join(path_str);
    if library_relative.exists() {
        return Some(library_relative);
    }

    let search_name = Path::new(path_str)
        .file_name()
        .and_then(|n| n.to_str())?;

    if let Some(found) = search_library_recursive(&library_path, search_name) {
        return Some(found);
    }

    if expanded.exists() {
        Some(expanded)
    } else {
        None
    }
}

pub fn search_library_recursive(dir: &Path, target: &str) -> Option<PathBuf> {
    if !dir.exists() || !dir.is_dir() {
        return None;
    }

    let entries = std::fs::read_dir(dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.eq_ignore_ascii_case(target) {
                return Some(path);
            }
        }

        if path.is_dir() && !path.is_symlink() {
            if let Some(found) = search_library_recursive(&path, target) {
                return Some(found);
            }
        }
    }

    None
}

pub fn is_audio_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(ext, "wav" | "WAV" | "aif" | "AIF" | "aiff" | "AIFF" | "flac" | "FLAC")
    } else {
        false
    }
}

pub fn read_wav_frame_count(path: &Path) -> Option<usize> {
    let file = File::open(path).ok()?;
    let reader = hound::WavReader::new(file).ok()?;
    let spec = reader.spec();
    let channels = spec.channels as usize;
    if channels == 0 {
        return None;
    }
    let total_samples = reader.len() as usize;
    Some(total_samples / channels)
}

pub fn find_kits_and_samples(base_dir: &Path, root_dir: &Path) -> (Vec<String>, Vec<String>) {
    const MAX_DEPTH: usize = 10;
    find_kits_and_samples_impl(base_dir, root_dir, 0, MAX_DEPTH)
}

fn find_kits_and_samples_impl(base_dir: &Path, root_dir: &Path, depth: usize, max_depth: usize) -> (Vec<String>, Vec<String>) {
    let mut kits = Vec::new();
    let mut samples = Vec::new();

    if depth >= max_depth || !base_dir.exists() || !base_dir.is_dir() {
        return (kits, samples);
    }

    let Ok(entries) = std::fs::read_dir(base_dir) else {
        return (kits, samples);
    };

    let mut audio_files = Vec::new();
    let mut subdirs = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() && is_audio_file(&path) {
            audio_files.push(path);
        } else if path.is_dir() && path.read_link().is_err() {
            subdirs.push(path);
        }
    }

    if !audio_files.is_empty() {
        if let Ok(relative) = base_dir.strip_prefix(root_dir) {
            if let Some(rel_str) = relative.to_str() {
                if depth == 1 {
                    for audio_file in &audio_files {
                        if let Some(fname) = audio_file.file_name().and_then(|s| s.to_str()) {
                            let mut path = String::with_capacity(rel_str.len() + fname.len() + 1);
                            path.push_str(rel_str);
                            path.push('/');
                            path.push_str(fname);
                            samples.push(path);
                        }
                    }
                } else if depth > 1 {
                    kits.push(rel_str.to_string());
                }
            }
        }
    }

    for subdir in subdirs {
        let (mut sub_kits, mut sub_samples) = find_kits_and_samples_impl(&subdir, root_dir, depth + 1, max_depth);
        kits.append(&mut sub_kits);
        samples.append(&mut sub_samples);
    }

    (kits, samples)
}

pub fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len.saturating_sub(3)])
    }
}

pub fn send_buffer_alloc_read<F>(buffer_id: u32, file_path: &str, debug_level: u8, mut output: F) -> Result<()>
where
    F: FnMut(String),
{
    let socket = UdpSocket::bind("127.0.0.1:0")
        .context("Failed to bind OSC socket for buffer allocation")?;

    let msg = OscMessage {
        addr: "/b_allocRead".to_string(),
        args: vec![
            OscType::Int(buffer_id as i32),
            OscType::String(file_path.to_string()),
        ],
    };

    let packet = OscPacket::Message(msg);
    let buf = encoder::encode(&packet)
        .context("Failed to encode OSC message")?;

    socket.send_to(&buf, OSC_ADDR)
        .context("Failed to send buffer allocation message")?;

    Ok(())
}
