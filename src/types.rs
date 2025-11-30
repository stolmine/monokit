use rosc::OscType;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub const OSC_ADDR: &str = "127.0.0.1:57120";

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

#[derive(Debug, Clone)]
pub enum MetroCommand {
    SetInterval(u64),
    SetActive(bool),
    SetScriptIndex(usize),
    SendParam(String, OscType),
    SendTrigger,
    SendVolume(f32),
    StartRecording(String),    // String is the directory path
    StopRecording,
    SetRecordingPath(String),  // Custom path prefix
    SetSlewTime(f32),          // Slew time in seconds
    SetParamSlew(String, f32), // Per-parameter slew: (param_name, time_in_seconds)
    SetGate(f32),              // Global gate duration in seconds
    SetEnvGate(String, f32),   // Per-envelope gate (env_name, duration)
    Shutdown,                  // Signal metro thread to exit
}

#[derive(Debug, Clone)]
pub enum MetroEvent {
    ExecuteScript(usize),
}

#[derive(Debug, Clone)]
pub struct MetroState {
    pub interval_ms: u64,
    pub active: bool,
    pub script_index: usize,
}

impl Default for MetroState {
    fn default() -> Self {
        Self {
            interval_ms: 500,
            active: false,
            script_index: 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Live,
    Script1,
    Script2,
    Script3,
    Script4,
    Script5,
    Script6,
    Script7,
    Script8,
    Metro,
    Init,
    Pattern,
    Help,
}

pub const NAVIGABLE_PAGES: [Page; 12] = [
    Page::Live,
    Page::Script1,
    Page::Script2,
    Page::Script3,
    Page::Script4,
    Page::Script5,
    Page::Script6,
    Page::Script7,
    Page::Script8,
    Page::Metro,
    Page::Init,
    Page::Pattern,
];

impl Page {
    pub fn name(&self) -> &str {
        match self {
            Page::Live => "LIVE",
            Page::Script1 => "1",
            Page::Script2 => "2",
            Page::Script3 => "3",
            Page::Script4 => "4",
            Page::Script5 => "5",
            Page::Script6 => "6",
            Page::Script7 => "7",
            Page::Script8 => "8",
            Page::Metro => "M",
            Page::Init => "I",
            Page::Pattern => "P",
            Page::Help => "HELP",
        }
    }

    pub fn next(&self) -> Self {
        if *self == Page::Help {
            return Page::Help;
        }
        let idx = NAVIGABLE_PAGES.iter().position(|p| p == self).unwrap_or(0);
        NAVIGABLE_PAGES[(idx + 1) % NAVIGABLE_PAGES.len()]
    }

    pub fn prev(&self) -> Self {
        if *self == Page::Help {
            return Page::Help;
        }
        let idx = NAVIGABLE_PAGES.iter().position(|p| p == self).unwrap_or(0);
        NAVIGABLE_PAGES[(idx + NAVIGABLE_PAGES.len() - 1) % NAVIGABLE_PAGES.len()]
    }
}
