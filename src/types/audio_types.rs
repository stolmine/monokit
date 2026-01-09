use super::constants::{SPECTRUM_BANDS, SCOPE_SAMPLES};

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

#[derive(Debug, Clone, Default)]
pub struct VoiceMeterData {
    pub osc_l: f32,
    pub osc_r: f32,
    pub pla_l: f32,
    pub pla_r: f32,
    pub nos_l: f32,
    pub nos_r: f32,
    pub smp_l: f32,
    pub smp_r: f32,
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

#[derive(Debug, Clone)]
pub struct EqState {
    pub low_db: f32,
    pub low_freq: f32,
    pub mid_db: f32,
    pub mid_freq: f32,
    pub mid_q: f32,
    pub high_db: f32,
    pub high_freq: f32,
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

#[derive(Debug, Clone, Default)]
pub struct CompressorData {
    pub input_level: f32,
    pub output_level: f32,
    pub gain_reduction_db: f32,
}

#[derive(Debug, Clone)]
pub struct MixerData {
    pub vol_osc: i32,
    pub vol_pla: i32,
    pub vol_nos: i32,
    pub vol_smp: i32,
    pub pan_osc: i32,
    pub pan_pla: i32,
    pub pan_nos: i32,
    pub pan_smp: i32,
    pub mute_osc: i32,
    pub mute_pla: i32,
    pub mute_nos: i32,
    pub mute_smp: i32,
}

impl Default for MixerData {
    fn default() -> Self {
        Self {
            vol_osc: 16383,
            vol_pla: 16383,
            vol_nos: 16383,
            vol_smp: 16383,
            pan_osc: 0,
            pan_pla: 0,
            pan_nos: 0,
            pan_smp: 0,
            mute_osc: 0,
            mute_pla: 0,
            mute_nos: 0,
            mute_smp: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FxMixState {
    pub lofi_mix: i32,
    pub ring_mix: i32,
    pub comp_mix: i32,
    pub delay_wet: i32,
    pub reverb_wet: i32,
    pub beat_rep_mix: i32,
    pub pitch_shift_mix: i32,
    pub clouds_wet: i32,
}

impl Default for FxMixState {
    fn default() -> Self {
        Self {
            lofi_mix: 0,
            ring_mix: 0,
            comp_mix: 16383,  // Compressor default 100% wet (matches SynthDef)
            delay_wet: 0,
            reverb_wet: 0,
            beat_rep_mix: 0,
            pitch_shift_mix: 0,
            clouds_wet: 0,
        }
    }
}
