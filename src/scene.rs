use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::types::{NotesStorage, PatternStorage, SamplerState, ScriptMutes, ScriptStorage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub version: u32,
    pub scripts: Vec<SceneScript>,
    pub patterns: Vec<ScenePattern>,
    pub pattern_working: usize,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub script_mutes: Vec<bool>,
    #[serde(default)]
    pub sampler: Option<SamplerState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneScript {
    pub lines: Vec<String>,
    pub j: i16,
    pub k: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenePattern {
    pub data: Vec<i16>,
    pub length: usize,
    pub index: usize,
}

#[derive(Debug)]
pub enum SceneError {
    NotFound(String),
    IoError(std::io::Error),
    ParseError(String),
    InvalidName(String),
}

pub fn get_scenes_dir() -> PathBuf {
    // Use platform-native config directory
    crate::config::monokit_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("scenes")
}

pub fn ensure_scenes_dir() -> Result<(), SceneError> {
    let dir = get_scenes_dir();
    fs::create_dir_all(&dir).map_err(SceneError::IoError)
}

pub fn scene_path(name: &str) -> PathBuf {
    get_scenes_dir().join(format!("{}.json", sanitize_name(name)))
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

pub fn save_scene(name: &str, scene: &Scene) -> Result<(), SceneError> {
    ensure_scenes_dir()?;
    let path = scene_path(name);
    let json = serde_json::to_string_pretty(scene)
        .map_err(|e| SceneError::ParseError(e.to_string()))?;
    fs::write(&path, json).map_err(SceneError::IoError)
}

pub fn load_scene(name: &str) -> Result<Scene, SceneError> {
    let path = scene_path(name);
    if !path.exists() {
        return Err(SceneError::NotFound(name.to_string()));
    }
    let json = fs::read_to_string(&path).map_err(SceneError::IoError)?;
    serde_json::from_str(&json).map_err(|e| SceneError::ParseError(e.to_string()))
}

pub fn list_scenes() -> Result<Vec<(String, u64)>, SceneError> {
    let dir = get_scenes_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut scenes = Vec::new();
    for entry in fs::read_dir(&dir).map_err(SceneError::IoError)? {
        let entry = entry.map_err(SceneError::IoError)?;
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                scenes.push((name.to_string(), size));
            }
        }
    }
    scenes.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(scenes)
}

pub fn delete_scene(name: &str) -> Result<(), SceneError> {
    let path = scene_path(name);
    if !path.exists() {
        return Err(SceneError::NotFound(name.to_string()));
    }
    fs::remove_file(&path).map_err(SceneError::IoError)
}

impl Scene {
    pub fn from_app_state(scripts: &ScriptStorage, patterns: &PatternStorage, notes: &NotesStorage, script_mutes: &ScriptMutes, sampler: &SamplerState) -> Self {
        let scene_scripts: Vec<SceneScript> = scripts
            .scripts
            .iter()
            .map(|s| SceneScript {
                lines: s.lines.to_vec(),
                j: s.j,
                k: s.k,
            })
            .collect();

        let scene_patterns: Vec<ScenePattern> = patterns
            .patterns
            .iter()
            .map(|p| ScenePattern {
                data: p.data.to_vec(),
                length: p.length,
                index: p.index,
            })
            .collect();

        Scene {
            version: 1,
            scripts: scene_scripts,
            patterns: scene_patterns,
            pattern_working: patterns.working,
            notes: notes.lines.to_vec(),
            script_mutes: script_mutes.muted.to_vec(),
            sampler: Some(sampler.clone()),
        }
    }

    pub fn apply_to_app_state(&self, scripts: &mut ScriptStorage, patterns: &mut PatternStorage, notes: &mut NotesStorage, script_mutes: &mut ScriptMutes, sampler: &mut SamplerState) {
        for (i, scene_script) in self.scripts.iter().enumerate() {
            if i < scripts.scripts.len() {
                for (j, line) in scene_script.lines.iter().enumerate() {
                    if j < 8 {
                        scripts.scripts[i].lines[j] = line.clone();
                    }
                }
                scripts.scripts[i].j = scene_script.j;
                scripts.scripts[i].k = scene_script.k;
            }
        }

        for (i, scene_pattern) in self.patterns.iter().enumerate() {
            if i < patterns.patterns.len() {
                for (j, val) in scene_pattern.data.iter().enumerate() {
                    if j < 64 {
                        patterns.patterns[i].data[j] = *val;
                    }
                }
                patterns.patterns[i].length = scene_pattern.length.min(64);
                patterns.patterns[i].index = scene_pattern.index.min(63);
            }
        }

        patterns.working = self.pattern_working.min(5);

        // Clear all notes first, then load from scene
        for i in 0..8 {
            notes.lines[i] = self.notes.get(i).cloned().unwrap_or_default();
        }

        // Load script mutes
        for i in 0..10 {
            script_mutes.muted[i] = self.script_mutes.get(i).copied().unwrap_or(false);
        }

        // Load sampler state
        if let Some(scene_sampler) = &self.sampler {
            *sampler = scene_sampler.clone();
        }
    }
}
