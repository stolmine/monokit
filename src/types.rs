use rosc::OscType;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;
use std::time::Instant;

#[cfg(not(feature = "scsynth-direct"))]
pub const OSC_ADDR: &str = "127.0.0.1:57120";

#[cfg(feature = "scsynth-direct")]
pub const OSC_ADDR: &str = "127.0.0.1:57110";

pub const MONOKIT_NODE_ID: i32 = 1000;
pub const SPECTRUM_BANDS: usize = 15;
pub const SCOPE_SAMPLES: usize = 128;

pub const TIER_SILENT: u8 = 0;
pub const TIER_ERRORS: u8 = 1;
pub const TIER_ESSENTIAL: u8 = 2;
pub const TIER_QUERIES: u8 = 3;
pub const TIER_CONFIRMS: u8 = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OutputCategory {
    Error,
    Essential,
    Query,
    Confirm,
    Verbose,
}

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

#[derive(Debug, Clone, Copy)]
pub struct ScopeSettings {
    pub timespan_ms: u32,
    pub color_mode: ScopeColorMode,
    pub display_mode: u8,
    pub unipolar: bool,
    pub gain: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeColorMode {
    Foreground,
    Secondary,
    HighlightBg,
    HighlightFg,
    Border,
    Error,
    Accent,
    Success,
    Label,
}

impl ScopeColorMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "FOREGROUND" | "FG" => Some(Self::Foreground),
            "SECONDARY" | "SEC" => Some(Self::Secondary),
            "HIGHLIGHT_BG" | "HL_BG" | "HIGHLIGHTBG" => Some(Self::HighlightBg),
            "HIGHLIGHT_FG" | "HL_FG" | "HIGHLIGHTFG" => Some(Self::HighlightFg),
            "BORDER" => Some(Self::Border),
            "ERROR" | "ERR" => Some(Self::Error),
            "ACCENT" | "ACC" => Some(Self::Accent),
            "SUCCESS" | "SUC" => Some(Self::Success),
            "LABEL" | "LBL" => Some(Self::Label),
            _ => None,
        }
    }

    pub fn from_u8(n: u8) -> Self {
        match n {
            1 => Self::Error,
            2 => Self::Foreground,
            3 => Self::Accent,
            4 => Self::Secondary,
            5 => Self::HighlightBg,
            6 => Self::HighlightFg,
            7 => Self::Border,
            8 => Self::Label,
            _ => Self::Success,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Error => 1,
            Self::Foreground => 2,
            Self::Accent => 3,
            Self::Secondary => 4,
            Self::HighlightBg => 5,
            Self::HighlightFg => 6,
            Self::Border => 7,
            Self::Label => 8,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Foreground => "FOREGROUND",
            Self::Secondary => "SECONDARY",
            Self::HighlightBg => "HIGHLIGHT_BG",
            Self::HighlightFg => "HIGHLIGHT_FG",
            Self::Border => "BORDER",
            Self::Error => "ERROR",
            Self::Accent => "ACCENT",
            Self::Success => "SUCCESS",
            Self::Label => "LABEL",
        }
    }

    pub fn get_color(&self, theme: &crate::theme::Theme) -> ratatui::style::Color {
        match self {
            Self::Foreground => theme.foreground,
            Self::Secondary => theme.secondary,
            Self::HighlightBg => theme.highlight_bg,
            Self::HighlightFg => theme.highlight_fg,
            Self::Border => theme.border,
            Self::Error => theme.error,
            Self::Accent => theme.accent,
            Self::Success => theme.success,
            Self::Label => theme.label,
        }
    }
}

impl Default for ScopeColorMode {
    fn default() -> Self {
        Self::Success
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
    SendPlaitsTrigger,
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
    SendScopeRate(f32),
    Error(String),         // Forward error to REPL via event channel
    QueryAudioOutDevices,
    SetAudioOutDevice(String),
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

#[derive(Debug, Clone)]
pub struct ScopeData {
    pub samples: [f32; SCOPE_SAMPLES],
}

impl Default for ScopeData {
    fn default() -> Self {
        Self {
            samples: [0.0; SCOPE_SAMPLES],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CpuData {
    pub avg_cpu: f32,
    pub peak_cpu: f32,
}

/// EQ parameter state for visualization
#[derive(Debug, Clone)]
pub struct EqState {
    pub low_db: f32,      // EL: -24 to 24
    pub low_freq: f32,    // ELF: 20 to 2000
    pub mid_db: f32,      // EM: -24 to 24
    pub mid_freq: f32,    // EF: 200 to 8000
    pub mid_q: f32,       // EQ: 0.1 to 10
    pub high_db: f32,     // EH: -24 to 24
    pub high_freq: f32,   // EHF: 1000 to 20000
}

impl Default for EqState {
    fn default() -> Self {
        Self {
            low_db: 0.0,
            low_freq: 200.0,
            mid_db: 0.0,
            mid_freq: 1000.0,
            mid_q: 1.0,
            high_db: 0.0,
            high_freq: 2500.0,
        }
    }
}

/// Compressor metering data for visualization
#[derive(Debug, Clone, Default)]
pub struct CompressorData {
    pub input_level: f32,       // 0.0 to 1.0
    pub output_level: f32,      // 0.0 to 1.0
    pub gain_reduction_db: f32, // 0 to -40 (negative = reduction)
}

#[derive(Debug, Clone)]
pub enum MetroEvent {
    ExecuteScript(usize),
    ExecuteDelayed(String, usize),
    MeterUpdate(MeterData),
    SpectrumUpdate(SpectrumData),
    ScopeUpdate(ScopeData),
    CpuUpdate(CpuData),
    CompressorUpdate(CompressorData),
    ScReady,
    AudioDeviceList { current: String, devices: Vec<String> },
    RestartScWithDevice(String),
    Error(String),
    StartRecordingDirect(String),  // dir path for scsynth-direct mode
    StopRecordingDirect,
    SetRecordingPathDirect(String),
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
    Scope,
    Help,
}

pub const NAVIGABLE_PAGES: [Page; 15] = [
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
    Page::Scope,
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
            Page::Scope => "S",
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
            "ct" => Some(28), "br_mix" | "br.mix" => Some(29),
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

pub const GRID_LABELS: [&str; 48] = [
    "PF", "PW", "MF", "MW", "FM", "FB", "DC", "FC",
    "FQ", "FT", "FE", "RF", "RD", "RM", "AD", "PD",
    "PA", "FD", "FA", "DD", "DA", "DT", "DF", "DW",
    "RV", "RH", "RW", "LM", "CT", "BM", "PS", "RG",
    "LB", "LS", "EQ", "TK", "MB", "MP", "MD", "MT",
    "MA", "MX", "VL", "PN", "DS", "MM", "ME", "FK",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    Help,
    Script,
}

#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub scope: SearchScope,
    pub page: Page,
    pub page_index: usize,
    pub line_index: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub matched_text: String,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct ConditionalSegment {
    pub start: usize,
    pub end: usize,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Default)]
pub struct LineSegmentActivity {
    pub segments: Vec<ConditionalSegment>,
}

#[derive(Debug, Clone, Copy)]
pub struct OutputSettings {
    pub out_err: bool,
    pub out_ess: bool,
    pub out_qry: bool,
    pub out_cfm: bool,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            out_err: false,
            out_ess: true,
            out_qry: true,
            out_cfm: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScrambleSettings {
    pub scramble_enabled: bool,
    pub scramble_mode: u8,
    pub scramble_speed: u8,
    pub scramble_curve: u8,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    Quit,
    SaveOverwrite(String),
}

impl Default for ScrambleSettings {
    fn default() -> Self {
        Self {
            scramble_enabled: false,
            scramble_mode: 0,
            scramble_speed: 5,
            scramble_curve: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UIToggles {
    pub show_meters_header: bool,
    pub show_meters_grid: bool,
    pub show_spectrum: bool,
    pub show_activity: bool,
    pub show_grid: bool,
    pub show_grid_view: bool,
    pub show_seq_highlight: bool,
    pub show_conditional_highlight: bool,
}

impl Default for UIToggles {
    fn default() -> Self {
        Self {
            show_meters_header: true,
            show_meters_grid: true,
            show_spectrum: true,
            show_activity: true,
            show_grid: true,
            show_grid_view: false,
            show_seq_highlight: true,
            show_conditional_highlight: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorMode {
    TrueColor,
    Color256,
}


// Multi-synth architecture: bus assignments
pub const PRIMARY_BUS: i32 = 16;
pub const MOD_BUS: i32 = 17;
pub const NOISE_BUS: i32 = 18;
pub const PLAITS_MAIN_BUS: i32 = 19;
pub const PLAITS_AUX_BUS: i32 = 20;

// Multi-synth architecture: node IDs for the 5 voice synths
pub const NOISE_NODE_ID: i32 = 1000;
pub const MOD_NODE_ID: i32 = 1001;
pub const PRIMARY_NODE_ID: i32 = 1002;
pub const MAIN_NODE_ID: i32 = 1003;
pub const PLAITS_NODE_ID: i32 = 1004;

// Utility synth node IDs
pub const SPECTRUM_NODE_ID: i32 = 1010;
pub const SCOPE_NODE_ID: i32 = 1011;

pub struct VoiceSynths {
    pub noise_node: i32,
    pub mod_node: i32,
    pub primary_node: i32,
    pub main_node: i32,
    pub plaits_node: i32,
}

impl VoiceSynths {
    pub fn new() -> Self {
        VoiceSynths {
            noise_node: NOISE_NODE_ID,
            mod_node: MOD_NODE_ID,
            primary_node: PRIMARY_NODE_ID,
            main_node: MAIN_NODE_ID,
            plaits_node: PLAITS_NODE_ID,
        }
    }
}

// Parameter routing: maps parameter names to their target synth node
pub fn route_param_to_node(param: &str) -> i32 {
    match param {
        // Noise synth parameters
        "nw" | "nv" => NOISE_NODE_ID,

        // Modulator synth parameters
        "mf" | "mw" | "mv" | "fb" | "fba" | "fbd" | "mb" | "mba" | "mbd" | "md" => MOD_NODE_ID,

        // Primary synth parameters
        "pf" | "pw" | "pv" | "fm" | "fa" | "fd" | "pa" | "pd" | "tk" => PRIMARY_NODE_ID,
        "dc" | "dm" | "dd" | "da" => MAIN_NODE_ID,  // Discontinuity is in MAIN synth

        // Plaits synth parameters
        "pitch" | "detune" | "engine" | "harmonics" | "timbre" | "morph" | "decay" | "lpg" | "plv" | "pav" | "pl_gate" | "t_gate_plaits" => PLAITS_NODE_ID,

        // Main synth parameters (effects, filters, etc.) - everything else
        _ => MAIN_NODE_ID,
    }
}

pub const SAMPLER_BUS: i32 = 21;
pub const SAMPLER_NODE_ID: i32 = 1005;
pub const SAMPLER_MAX_SLOTS: usize = 128;
pub const SAMPLER_BUFFER_BASE: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SamplerMode {
    Slice,
    Kit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleSlot {
    pub buffer_id: u32,
    pub start_frame: usize,
    pub end_frame: usize,
}

impl Default for SampleSlot {
    fn default() -> Self {
        Self {
            buffer_id: 0,
            start_frame: 0,
            end_frame: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerPlaybackParams {
    pub rate: i16,
    pub pitch: i16,
    pub fine: i16,
    pub direction: bool,
    pub loop_mode: bool,
    pub start_offset: i16,
    pub length: i16,
    pub attack: i16,
    pub decay: i16,
    pub release: i16,
    pub sustain_mode: bool,
    pub volume: i16,
    pub pan: i16,
    pub fx_routing: u8,
    pub rate_mod: i16,
    pub pitch_mod: i16,
}

impl Default for SamplerPlaybackParams {
    fn default() -> Self {
        Self {
            rate: 8192,
            pitch: 0,
            fine: 0,
            direction: false,
            loop_mode: false,
            start_offset: 0,
            length: 0,
            attack: 0,
            decay: 0,
            release: 0,
            sustain_mode: false,
            volume: 8192,
            pan: 0,
            fx_routing: 0,
            rate_mod: 0,
            pitch_mod: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerFxParams {
    pub filter_cut: i16,
    pub filter_res: i16,
    pub filter_type: u8,
    pub bits: u8,
    pub rate_reduce: i16,
    pub deci_mix: i16,
    pub prob: i16,
    pub mult: i16,
    pub glit_mix: i16,
}

impl Default for SamplerFxParams {
    fn default() -> Self {
        Self {
            filter_cut: 16383,
            filter_res: 0,
            filter_type: 0,
            bits: 24,
            rate_reduce: 0,
            deci_mix: 0,
            prob: 0,
            mult: 0,
            glit_mix: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerState {
    pub mode: SamplerMode,
    pub kit_path: Option<String>,
    pub slots: Vec<SampleSlot>,
    pub num_slots: usize,
    pub current_slot: usize,
    pub slice_count: Option<usize>,
    pub playback: SamplerPlaybackParams,
    pub fx: SamplerFxParams,
    pub playing: bool,
}

impl Default for SamplerState {
    fn default() -> Self {
        Self {
            mode: SamplerMode::Slice,
            kit_path: None,
            slots: Vec::new(),
            num_slots: 0,
            current_slot: 0,
            slice_count: None,
            playback: SamplerPlaybackParams::default(),
            fx: SamplerFxParams::default(),
            playing: false,
        }
    }
}
