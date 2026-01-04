use crate::midi::{MidiConnection, MidiTimingStats};
use crate::output::OutputDecider;
use crate::terminal::TerminalCapabilities;
use crate::theme::Theme;
use crate::types::{
    ColorMode, ConfirmAction, Counters, EqState, MetroCommand, NotesStorage, Page, PatternStorage, SamplerState, ScaleState,
    ScriptMutes, ScriptStorage, ScopeSettings, SyncMode, Variables,
};
use std::sync::{mpsc::Sender, Arc};
use std::time::Instant;

/// Execution context for command processing
/// Groups all command parameters to reduce process_command signature from 109 to 3 parameters
pub struct ExecutionContext<'a> {
    // Core state
    pub metro_tx: &'a Sender<MetroCommand>,
    pub metro_interval: &'a mut u64,
    pub variables: &'a mut Variables,
    pub patterns: &'a mut PatternStorage,
    pub counters: &'a mut Counters,
    pub scripts: &'a mut ScriptStorage,
    pub script_index: usize,
    pub scale: &'a mut ScaleState,

    // Output control
    pub debug_level: &'a mut u8,
    pub out_err: &'a mut bool,
    pub out_ess: &'a mut bool,
    pub out_qry: &'a mut bool,
    pub out_cfm: &'a mut bool,

    // Display settings
    pub theme: &'a mut Theme,
    pub activity_hold_ms: &'a mut f32,
    pub show_cpu: &'a mut bool,
    pub show_bpm: &'a mut bool,
    pub header_level: &'a mut u8,
    pub limiter_enabled: &'a mut bool,
    pub show_meters_header: &'a mut bool,
    pub show_meters_grid: &'a mut bool,
    pub show_spectrum: &'a mut bool,
    pub show_activity: &'a mut bool,
    pub show_grid: &'a mut bool,
    pub show_grid_view: &'a mut bool,
    pub show_seq_highlight: &'a mut bool,
    pub grid_mode: &'a mut u8,
    pub eq_state: &'a mut EqState,
    pub scope_settings: &'a mut ScopeSettings,
    pub current_page: &'a mut Page,

    // System state
    pub br_len: &'a mut usize,
    pub sync_mode: &'a mut SyncMode,
    pub midi_connection: &'a mut Option<MidiConnection>,
    pub midi_timing_stats: &'a Arc<MidiTimingStats>,
    pub notes: &'a mut NotesStorage,
    pub load_rst: &'a mut bool,
    pub load_clr: &'a mut bool,
    pub vca_mode: &'a mut bool,
    pub show_conditional_highlight: &'a mut bool,
    pub current_scene_name: &'a mut Option<String>,
    pub title_mode: &'a mut u8,
    pub title_timer_enabled: &'a mut bool,
    pub title_timer_interval_secs: &'a mut u16,
    pub title_timer_last_toggle: &'a mut Option<Instant>,
    pub audio_devices: &'a [String],
    pub header_scramble: &'a mut Option<crate::scramble::ScrambleAnimation>,
    pub scramble_enabled: &'a mut bool,
    pub scramble_mode: &'a mut u8,
    pub scramble_speed: &'a mut u8,
    pub scramble_curve: &'a mut u8,
    pub ascii_meters: &'a mut bool,
    pub autoload: &'a mut bool,
    pub terminal_caps: &'a TerminalCapabilities,
    pub color_mode: ColorMode,
    pub script_break: &'a mut bool,
    pub ev_counters: &'a mut [[u32; 8]; 10],
    pub script_mutes: &'a mut ScriptMutes,
    pub confirm_quit_unsaved: &'a mut bool,
    pub confirm_overwrite_scene: &'a mut bool,
    pub scene_modified: &'a mut bool,
    pub pending_confirmation: &'a mut Option<ConfirmAction>,
    pub sampler_state: &'a mut SamplerState,
}

impl<'a> ExecutionContext<'a> {}

impl<'a> OutputDecider for ExecutionContext<'a> {
    fn debug_level(&self) -> u8 {
        *self.debug_level
    }

    fn out_err(&self) -> bool {
        *self.out_err
    }

    fn out_ess(&self) -> bool {
        *self.out_ess
    }

    fn out_qry(&self) -> bool {
        *self.out_qry
    }

    fn out_cfm(&self) -> bool {
        *self.out_cfm
    }
}
