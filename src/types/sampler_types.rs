use serde::{Deserialize, Serialize};

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
    pub file_path: Option<String>,
}

impl Default for SampleSlot {
    fn default() -> Self {
        Self {
            buffer_id: 0,
            start_frame: 0,
            end_frame: 0,
            file_path: None,
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
    pub filter_cut_mod: i16,
    pub filter_res_mod: i16,
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
            filter_cut_mod: 0,
            filter_res_mod: 0,
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
    pub total_frames: Option<usize>,
    pub onset_sensitivity: u32,
    pub onset_min_spacing_ms: f32,
}

impl Default for SamplerState {
    fn default() -> Self {
        Self {
            mode: SamplerMode::Slice,
            kit_path: None,
            slots: Vec::new(),
            num_slots: 128,
            current_slot: 0,
            slice_count: None,
            playback: SamplerPlaybackParams::default(),
            fx: SamplerFxParams::default(),
            playing: false,
            total_frames: None,
            onset_sensitivity: 5,
            onset_min_spacing_ms: 50.0,
        }
    }
}
