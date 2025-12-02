use rosc::OscType;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;
use std::time::Instant;

pub const OSC_ADDR: &str = "127.0.0.1:57120";
pub const SPECTRUM_BANDS: usize = 15;

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

#[derive(Debug, Clone)]
pub struct DelayedCommand {
    pub due_at_ms: u64,
    pub command: String,
    pub script_index: usize,
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
    ScheduleDelayed(String, u64, usize),      // (cmd, delay_ms, script_idx)
    ScheduleRepeated(String, i16, u64, usize), // (cmd, count, interval_ms, script_idx) for DEL.X
    ClearDelayed,                              // DEL.CLR
    SetSyncMode(SyncMode),
    MidiClockTick,
    MidiTransportStart,
    MidiTransportStop,
    EnableMidiTimingDiag,
    DisableMidiTimingDiag,
    PrintMidiTimingReport,
    SendScDiag(i32),       // Send /monokit/diag with 0 or 1
    SendScDiagReport,      // Send /monokit/diag/report
    GetTriggerCount,       // Get the current trigger count
    ResetTriggerCount,     // Reset the trigger counter to 0
}

#[derive(Debug, Clone, Default)]
pub struct MeterData {
    pub peak_l: f32,
    pub peak_r: f32,
    pub rms_l: f32,
    pub rms_r: f32,
    pub peak_hold_l: f32,
    pub peak_hold_r: f32,
    pub clip_l: bool,
    pub clip_r: bool,
}

#[derive(Debug, Clone)]
pub struct SpectrumData {
    pub bands: [f32; SPECTRUM_BANDS],
    pub peak_hold: [f32; SPECTRUM_BANDS],
    pub clip: [bool; SPECTRUM_BANDS],
}

impl Default for SpectrumData {
    fn default() -> Self {
        Self {
            bands: [0.0; SPECTRUM_BANDS],
            peak_hold: [0.0; SPECTRUM_BANDS],
            clip: [false; SPECTRUM_BANDS],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CpuData {
    pub avg_cpu: f32,
    pub peak_cpu: f32,
}

#[derive(Debug, Clone)]
pub enum MetroEvent {
    ExecuteScript(usize),
    ExecuteDelayed(String, usize),
    MeterUpdate(MeterData),
    SpectrumUpdate(SpectrumData),
    CpuUpdate(CpuData),
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
    Variables,
    Notes,
    Help,
}

pub const NAVIGABLE_PAGES: [Page; 14] = [
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
    Page::Variables,
    Page::Notes,
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
            Page::Variables => "V",
            Page::Notes => "N",
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

#[derive(Debug, Clone)]
pub struct ParamActivity {
    pub timestamps: [Option<Instant>; 48],
}

impl Default for ParamActivity {
    fn default() -> Self {
        Self {
            timestamps: [None; 48],
        }
    }
}

impl ParamActivity {
    pub fn mark(&mut self, param_name: &str) {
        if let Some(idx) = Self::param_to_index(param_name) {
            self.timestamps[idx] = Some(Instant::now());
        }
    }

    pub fn param_to_index(param: &str) -> Option<usize> {
        match param.to_lowercase().as_str() {
            "pf" => Some(0), "pw" => Some(1), "mf" => Some(2), "mw" => Some(3),
            "fm" => Some(4), "fb" => Some(5), "dc" => Some(6), "fc" => Some(7),
            "fq" => Some(8), "ft" => Some(9), "fe" => Some(10), "rf" => Some(11),
            "rd" => Some(12), "rm" => Some(13), "ad" => Some(14), "pd" => Some(15),
            "pa" => Some(16), "fd" => Some(17), "fa" => Some(18), "dd" => Some(19),
            "da" => Some(20), "dt" => Some(21), "df" => Some(22), "dw" => Some(23),
            "rv" => Some(24), "rh" => Some(25), "rw" => Some(26), "lm" => Some(27),
            "ct" => Some(28), "br_act" | "br.act" => Some(29),
            "ps_semi" | "ps.semi" => Some(30), "rgf" => Some(31),
            "lb" => Some(32), "ls" => Some(33), "eq" => Some(34), "tk" => Some(35),
            "mb" => Some(36), "mp" => Some(37), "md" => Some(38), "mt" => Some(39),
            "ma" => Some(40), "mx" => Some(41), "vol" | "volume" => Some(42),
            "pan" | "pn" => Some(43), "ds" => Some(44), "mm" => Some(45),
            "me" => Some(46), "fk" => Some(47),
            _ => None,
        }
    }
}

pub const GRID_ICONS: [char; 48] = [
    '~', '≈', '∿', '∞', '×', '⟲', '↯', '⊂',
    '◎', '⊏', '⊐', '∥', '⊥', '⊡', '▼', '↘',
    '↑', '↓', '↗', '◢', '◣', '⋮', '⟳', '◐',
    '⌓', '⌐', '◑', '⊞', '▣', '⇆', '⤴', '⊛',
    '⊟', '⊠', '≡', '⊕', '⫰', '⧫', '⧪', '⬡',
    '⬢', '⬣', '▮', '⬌', '⟿', '✱', '◉', '⊙',
];
