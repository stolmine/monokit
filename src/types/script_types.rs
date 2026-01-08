use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyncMode {
    #[default]
    Internal = 0,
    MidiClock = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub lines: [String; 8],
    pub j: i16,
    pub k: i16,
}

impl Default for Script {
    fn default() -> Self {
        Self {
            lines: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            j: 0,
            k: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variables {
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub d: i16,
    pub i: i16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub t: i16,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            i: 0,
            x: 0,
            y: 0,
            z: 0,
            t: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Counters {
    pub values: [i16; 4],
    pub max: [i16; 4],
    pub min: [i16; 4],
}

impl Default for Counters {
    fn default() -> Self {
        Self {
            values: [0; 4],
            max: [0; 4],
            min: [0; 4],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScaleState {
    pub root: u8,
    pub scale_preset: Option<u8>,
    pub mask: Vec<bool>,
    pub divisions: u8,
}

impl Default for ScaleState {
    fn default() -> Self {
        Self {
            root: 0,
            scale_preset: Some(1),
            mask: vec![true,false,true,false,true,true,false,true,false,true,false,true],
            divisions: 12,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    #[serde(with = "BigArray")]
    pub data: [i16; 64],
    pub length: usize,
    pub index: usize,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            data: [0; 64],
            length: 64,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternStorage {
    pub patterns: [Pattern; 6],
    pub working: usize,
    #[serde(skip)]
    pub toggle_state: HashMap<String, usize>,
    #[serde(skip)]
    pub toggle_last_value: HashMap<String, i16>,
    #[serde(skip)]
    pub direct_validation: HashMap<String, bool>,
}

impl Default for PatternStorage {
    fn default() -> Self {
        Self {
            patterns: [
                Pattern::default(),
                Pattern::default(),
                Pattern::default(),
                Pattern::default(),
                Pattern::default(),
                Pattern::default(),
            ],
            working: 0,
            toggle_state: HashMap::new(),
            toggle_last_value: HashMap::new(),
            direct_validation: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStorage {
    pub scripts: [Script; 10],
}

impl Default for ScriptStorage {
    fn default() -> Self {
        Self {
            scripts: [
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
                Script::default(),
            ],
        }
    }
}

impl ScriptStorage {
    pub fn get_script(&self, index: usize) -> &Script {
        &self.scripts[index]
    }

    pub fn get_script_mut(&mut self, index: usize) -> &mut Script {
        &mut self.scripts[index]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMutes {
    pub muted: [bool; 10],
}

impl Default for ScriptMutes {
    fn default() -> Self {
        Self {
            muted: [false; 10],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesStorage {
    pub lines: [String; 8],
}

impl Default for NotesStorage {
    fn default() -> Self {
        Self {
            lines: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
        }
    }
}
