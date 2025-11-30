pub mod factory;

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub version: u32,
    pub name: String,
    pub category: String,
    pub lines: Vec<String>,
    pub j: i16,
    pub k: i16,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetType {
    Factory,
    User,
}

#[derive(Debug)]
pub enum PresetError {
    NotFound(String),
    IoError(std::io::Error),
    ParseError(String),
    InvalidName(String),
}

pub fn get_presets_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".monokit").join("presets")
}

pub fn ensure_presets_dir() -> Result<(), PresetError> {
    let dir = get_presets_dir();
    fs::create_dir_all(&dir).map_err(PresetError::IoError)
}

pub fn preset_path(name: &str) -> PathBuf {
    get_presets_dir().join(format!("{}.json", sanitize_name(name)))
}

pub fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

pub fn save_preset(name: &str, preset: &Preset) -> Result<(), PresetError> {
    ensure_presets_dir()?;
    let path = preset_path(name);
    let json = serde_json::to_string_pretty(preset)
        .map_err(|e| PresetError::ParseError(e.to_string()))?;
    fs::write(&path, json).map_err(PresetError::IoError)
}

pub fn load_user_preset(name: &str) -> Result<Preset, PresetError> {
    let path = preset_path(name);
    if !path.exists() {
        return Err(PresetError::NotFound(name.to_string()));
    }
    let json = fs::read_to_string(&path).map_err(PresetError::IoError)?;
    serde_json::from_str(&json).map_err(|e| PresetError::ParseError(e.to_string()))
}

pub fn get_preset(name: &str) -> Result<(Preset, PresetType), PresetError> {
    if let Some(preset) = factory::get_factory_preset(name) {
        return Ok((preset, PresetType::Factory));
    }

    match load_user_preset(name) {
        Ok(preset) => Ok((preset, PresetType::User)),
        Err(e) => Err(e),
    }
}

pub fn list_user_presets() -> Result<Vec<(String, u64)>, PresetError> {
    let dir = get_presets_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut presets = Vec::new();
    for entry in fs::read_dir(&dir).map_err(PresetError::IoError)? {
        let entry = entry.map_err(PresetError::IoError)?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                presets.push((name.to_string(), size));
            }
        }
    }
    presets.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(presets)
}

pub fn delete_preset(name: &str) -> Result<(), PresetError> {
    let path = preset_path(name);
    if !path.exists() {
        return Err(PresetError::NotFound(name.to_string()));
    }
    fs::remove_file(&path).map_err(PresetError::IoError)
}
