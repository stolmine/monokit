use crate::commands::process_command;
use crate::midi::{MidiConnection, MidiTimingStats};
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, NotesStorage, PatternStorage, ScaleState, ScriptStorage, SyncMode, Variables};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

pub fn create_test_variables() -> Variables {
    Variables {
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

pub fn create_test_patterns() -> PatternStorage {
    PatternStorage::default()
}

pub fn create_test_scripts() -> ScriptStorage {
    ScriptStorage::default()
}

pub fn create_test_counters() -> Counters {
    Counters::default()
}

pub fn create_test_scale() -> ScaleState {
    ScaleState::default()
}

pub fn create_test_metro_tx() -> (Sender<MetroCommand>, Receiver<MetroCommand>) {
    mpsc::channel::<MetroCommand>()
}

#[macro_export]
macro_rules! test_setup {
    () => {{
        let variables = $crate::tests::common::create_test_variables();
        let mut patterns = $crate::tests::common::create_test_patterns();
        let scripts = $crate::tests::common::create_test_scripts();
        let mut counters = $crate::tests::common::create_test_counters();
        let scale = $crate::tests::common::create_test_scale();
        (variables, patterns, scripts, counters, scale)
    }};
    (mut) => {{
        let mut variables = $crate::tests::common::create_test_variables();
        let mut patterns = $crate::tests::common::create_test_patterns();
        let scripts = $crate::tests::common::create_test_scripts();
        let mut counters = $crate::tests::common::create_test_counters();
        let scale = $crate::tests::common::create_test_scale();
        (variables, patterns, scripts, counters, scale)
    }};
}

pub struct TestContext {
    pub metro_tx: Sender<MetroCommand>,
    pub metro_rx: Receiver<MetroCommand>,
    pub metro_interval: u64,
    pub br_len: usize,
    pub sync_mode: SyncMode,
    pub midi_connection: Option<MidiConnection>,
    pub midi_timing_stats: Arc<MidiTimingStats>,
    pub variables: Variables,
    pub patterns: PatternStorage,
    pub counters: Counters,
    pub scripts: ScriptStorage,
    pub script_index: usize,
    pub scale: ScaleState,
    pub theme: Theme,
    pub debug_level: u8,
    pub activity_hold_ms: f32,
    pub show_cpu: bool,
    pub show_bpm: bool,
    pub header_level: u8,
    pub limiter_enabled: bool,
    pub notes: NotesStorage,
    pub load_rst: bool,
    pub vca_mode: bool,
    pub show_conditional_highlight: bool,
    pub scope_settings: crate::types::ScopeSettings,
    pub show_meters_header: bool,
    pub show_meters_grid: bool,
    pub show_spectrum: bool,
    pub show_activity: bool,
    pub show_grid: bool,
    pub show_grid_view: bool,
    pub show_seq_highlight: bool,
    pub grid_mode: u8,
    pub current_scene_name: Option<String>,
    pub title_mode: u8,
    pub title_timer_enabled: bool,
    pub title_timer_interval_secs: u16,
    pub title_timer_last_toggle: Option<std::time::Instant>,
    pub out_err: bool,
    pub out_ess: bool,
    pub out_qry: bool,
    pub out_cfm: bool,
    pub audio_devices: Vec<String>,
    pub header_scramble: Option<crate::scramble::ScrambleAnimation>,
    pub scramble_enabled: bool,
    pub scramble_mode: u8,
    pub scramble_speed: u8,
    pub scramble_curve: u8,
    pub outputs: Vec<String>,
}

impl Default for TestContext {
    fn default() -> Self {
        let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
        Self {
            metro_tx,
            metro_rx,
            metro_interval: 500,
            br_len: 2,
            sync_mode: SyncMode::Internal,
            midi_connection: None,
            midi_timing_stats: MidiTimingStats::new(),
            variables: Variables::default(),
            patterns: PatternStorage::default(),
            counters: Counters::default(),
            scripts: ScriptStorage::default(),
            script_index: 0,
            scale: ScaleState::default(),
            theme: Theme::dark(),
            debug_level: 0,
            activity_hold_ms: 200.0,
            show_cpu: false,
            show_bpm: true,
            header_level: 4,
            limiter_enabled: true,
            notes: NotesStorage::default(),
            load_rst: false,
            vca_mode: false,
            show_conditional_highlight: true,
            scope_settings: crate::types::ScopeSettings {
                timespan_ms: 30,
                color_mode: 0,
                display_mode: 0,
                unipolar: false,
            },
            show_meters_header: true,
            show_meters_grid: true,
            show_spectrum: true,
            show_activity: true,
            show_grid: true,
            show_grid_view: false,
            show_seq_highlight: true,
            grid_mode: 1,
            current_scene_name: None,
            title_mode: 0,
            title_timer_enabled: false,
            title_timer_interval_secs: 5,
            title_timer_last_toggle: None,
            out_err: false,
            out_ess: false,
            out_qry: false,
            out_cfm: false,
            audio_devices: Vec::new(),
            header_scramble: None,
            scramble_enabled: true,
            scramble_mode: 2,
            scramble_speed: 5,
            scramble_curve: 0,
            outputs: Vec::new(),
        }
    }
}

impl TestContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, input: &str) -> anyhow::Result<Vec<usize>> {
        process_command(
            &self.metro_tx,
            &mut self.metro_interval,
            &mut self.br_len,
            &mut self.sync_mode,
            &mut self.midi_connection,
            &self.midi_timing_stats,
            &mut self.variables,
            &mut self.patterns,
            &mut self.counters,
            &mut self.scripts,
            self.script_index,
            &mut self.scale,
            &mut self.theme,
            &mut self.debug_level,
            &mut self.activity_hold_ms,
            &mut self.show_cpu,
            &mut self.show_bpm,
            &mut self.header_level,
            &mut self.limiter_enabled,
            &mut self.notes,
            &mut self.load_rst,
            &mut self.vca_mode,
            &mut self.show_conditional_highlight,
            &mut self.scope_settings,
            &mut self.show_meters_header,
            &mut self.show_meters_grid,
            &mut self.show_spectrum,
            &mut self.show_activity,
            &mut self.show_grid,
            &mut self.show_grid_view,
            &mut self.show_seq_highlight,
            &mut self.grid_mode,
            &mut self.current_scene_name,
            &mut self.title_mode,
            &mut self.title_timer_enabled,
            &mut self.title_timer_interval_secs,
            &mut self.title_timer_last_toggle,
            &mut self.out_err,
            &mut self.out_ess,
            &mut self.out_qry,
            &mut self.out_cfm,
            &self.audio_devices,
            &mut self.header_scramble,
            &mut self.scramble_enabled,
            &mut self.scramble_mode,
            &mut self.scramble_speed,
            &mut self.scramble_curve,
            input,
            |msg| {
                self.outputs.push(msg);
            },
        )
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    pub fn with_debug_level(mut self, level: u8) -> Self {
        self.debug_level = level;
        self
    }
}
