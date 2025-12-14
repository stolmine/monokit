use crate::midi::{MidiConnection, MidiTimingStats};
use crate::terminal::TerminalCapabilities;
use crate::theme::Theme;
use crate::types::{
    ColorMode, Counters, MetroCommand, NotesStorage, OutputCategory, PatternStorage, ScaleState,
    ScriptStorage, ScopeSettings, SyncMode, Variables, TIER_CONFIRMS, TIER_ERRORS,
    TIER_ESSENTIAL, TIER_QUERIES,
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
    pub scope_settings: &'a mut ScopeSettings,

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
}

impl<'a> ExecutionContext<'a> {
    /// Check if output should be shown for a given category
    /// Combines tier check with category override: tier OR override
    pub fn should_output(&self, category: OutputCategory) -> bool {
        let tier = match category {
            OutputCategory::Error => TIER_ERRORS,
            OutputCategory::Essential => TIER_ESSENTIAL,
            OutputCategory::Query => TIER_QUERIES,
            OutputCategory::Confirm => TIER_CONFIRMS,
            OutputCategory::Verbose => return false, // No tier for verbose
        };

        // Tier check OR category override
        *self.debug_level >= tier
            || match category {
                OutputCategory::Error => *self.out_err,
                OutputCategory::Essential => *self.out_ess,
                OutputCategory::Query => *self.out_qry,
                OutputCategory::Confirm => *self.out_cfm,
                OutputCategory::Verbose => false,
            }
    }

    /// Conditionally output a message based on category
    pub fn output<F>(&self, category: OutputCategory, message: String, mut output: F)
    where
        F: FnMut(String),
    {
        if self.should_output(category) {
            output(message);
        }
    }
}
