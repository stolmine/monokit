use crate::midi::{MidiConnection, MidiTimingStats};
use crate::terminal::TerminalCapabilities;
use crate::theme::Theme;
use crate::types::{
    ColorMode, Counters, CpuData, LineSegmentActivity, MeterData, MetroCommand, MetroState, NotesStorage, OutputCategory, Page, ParamActivity, PatternStorage, ScaleState, ScopeData, ScriptStorage, SearchMatch, SpectrumData, SyncMode, Variables,
    TIER_CONFIRMS, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES, TIER_VERBOSE,
};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod input;
mod script_exec;

pub struct App {
    pub current_page: Page,
    pub previous_page: Page,
    pub input: String,
    pub cursor_position: usize,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub output: Vec<String>,
    pub output_scroll: usize,
    pub help_scroll: usize,
    pub help_page: usize,
    pub metro_state: Arc<Mutex<MetroState>>,
    pub metro_tx: Sender<MetroCommand>,
    pub scripts: ScriptStorage,
    pub selected_line: Option<usize>,
    pub variables: Variables,
    pub patterns: PatternStorage,
    pub counters: Counters,
    pub pattern_cursor: (usize, usize),
    pub pattern_input: String,
    pub ev_counters: [[u32; 8]; 10],
    pub if_else_condition: bool,
    pub clipboard: String,
    pub should_quit: bool,
    pub script_error: Option<String>,
    pub script_error_time: Option<Instant>,
    pub theme: Theme,
    pub color_mode: ColorMode,
    pub terminal_caps: TerminalCapabilities,
    pub recording: bool,
    pub recording_start: Option<Instant>,
    pub debug_level: u8,
    pub br_len: usize,
    pub slew_time_ms: f32,
    pub scale: ScaleState,
    pub sync_mode: SyncMode,
    pub midi_connection: Option<MidiConnection>,
    pub midi_timing_stats: Arc<MidiTimingStats>,
    pub script_activity: [Option<Instant>; 10],
    pub trigger_activity: Option<Instant>,
    pub activity_hold_ms: f32,
    pub param_activity: ParamActivity,
    pub show_grid_view: bool,
    pub meter_data: MeterData,
    pub spectrum_data: SpectrumData,
    pub scope_data: ScopeData,
    pub scope_settings: crate::types::ScopeSettings,
    pub cpu_data: CpuData,
    pub show_cpu: bool,
    pub show_bpm: bool,
    pub header_level: u8,
    pub limiter_enabled: bool,
    pub show_meters_header: bool,
    pub show_meters_grid: bool,
    pub show_spectrum: bool,
    pub show_activity: bool,
    pub show_grid: bool,
    pub show_seq_highlight: bool,
    pub grid_mode: u8,
    pub notes: NotesStorage,
    pub load_rst: bool,
    pub vca_mode: bool,
    pub search_mode: bool,
    pub search_query: String,
    pub search_cursor: usize,
    pub search_matches: Vec<SearchMatch>,
    pub search_current_match: usize,
    pub conditional_segments: [[LineSegmentActivity; 8]; 10],
    pub show_conditional_highlight: bool,
    pub current_scene_name: Option<String>,
    pub title_mode: u8,
    pub title_timer_enabled: bool,
    pub title_timer_interval_secs: u16,
    pub title_timer_last_toggle: Option<Instant>,
    pub out_err: bool,
    pub out_ess: bool,
    pub out_qry: bool,
    pub out_cfm: bool,
    pub audio_devices: Vec<String>,
    pub audio_device_current: String,
    pub header_scramble: Option<crate::scramble::ScrambleAnimation>,
    pub scramble_enabled: bool,
    pub scramble_mode: u8,
    pub scramble_speed: u8,
    pub scramble_curve: u8,
    pub ui_scrambles: Vec<(String, crate::scramble::ScrambleAnimation)>,
    pub grid_scrambles: Vec<crate::scramble::ScrambleAnimation>,
    pub ascii_meters: bool,
    pub awaiting_audio_restart: bool,
    pub script_break: bool,
}

impl App {
    pub fn new(metro_tx: Sender<MetroCommand>, metro_state: Arc<Mutex<MetroState>>, theme: Theme, color_mode: ColorMode, config: &crate::config::Config, terminal_caps: TerminalCapabilities) -> Self {
        Self {
            current_page: Page::Live,
            previous_page: Page::Live,
            input: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: None,
            output: Vec::new(),
            output_scroll: 0,
            help_scroll: 0,
            help_page: 0,
            metro_state,
            metro_tx,
            scripts: ScriptStorage::default(),
            selected_line: None,
            variables: Variables::default(),
            patterns: PatternStorage::default(),
            counters: Counters::default(),
            pattern_cursor: (0, 0),
            pattern_input: String::new(),
            ev_counters: [[0; 8]; 10],
            if_else_condition: true,
            clipboard: String::new(),
            should_quit: false,
            script_error: None,
            script_error_time: None,
            theme,
            color_mode,
            terminal_caps,
            recording: false,
            recording_start: None,
            debug_level: config.display.debug_level,
            br_len: 2,
            slew_time_ms: 0.0,
            scale: ScaleState::default(),
            sync_mode: SyncMode::Internal,
            midi_connection: None,
            midi_timing_stats: MidiTimingStats::new(),
            script_activity: [None; 10],
            trigger_activity: None,
            activity_hold_ms: config.display.activity_hold_ms as f32,
            param_activity: ParamActivity::default(),
            show_grid_view: config.display.show_grid_view,
            meter_data: MeterData::default(),
            spectrum_data: SpectrumData::default(),
            scope_data: ScopeData::default(),
            scope_settings: crate::types::ScopeSettings {
                timespan_ms: config.display.scope_timespan_ms,
                color_mode: crate::types::ScopeColorMode::from_u8(config.display.scope_color_mode),
                display_mode: config.display.scope_display_mode,
                unipolar: config.display.scope_unipolar,
            },
            cpu_data: CpuData::default(),
            show_cpu: config.display.show_cpu,
            show_bpm: config.display.show_bpm,
            header_level: config.display.header_level,
            limiter_enabled: config.display.limiter_enabled,
            show_meters_header: config.display.show_meters_header,
            show_meters_grid: config.display.show_meters_grid,
            show_spectrum: config.display.show_spectrum,
            show_activity: config.display.show_activity,
            show_grid: config.display.show_grid,
            show_seq_highlight: config.display.show_seq_highlight,
            grid_mode: config.display.grid_mode,
            notes: NotesStorage::default(),
            load_rst: config.display.load_rst,
            vca_mode: config.display.vca_mode,
            search_mode: false,
            search_query: String::new(),
            search_cursor: 0,
            search_matches: Vec::new(),
            search_current_match: 0,
            conditional_segments: Default::default(),
            show_conditional_highlight: config.display.show_conditional_highlight,
            current_scene_name: None,
            title_mode: config.display.title_mode,
            title_timer_enabled: config.display.title_timer_enabled,
            title_timer_interval_secs: config.display.title_timer_interval_secs,
            title_timer_last_toggle: None,
            out_err: config.display.out_err,
            out_ess: config.display.out_ess,
            out_qry: config.display.out_qry,
            out_cfm: config.display.out_cfm,
            audio_devices: Vec::new(),
            audio_device_current: String::new(),
            header_scramble: if config.display.scramble_enabled {
                let mode = crate::scramble::ScrambleMode::from_u8(config.display.scramble_mode);
                let curve = crate::scramble::ScrambleCurve::from_u8(config.display.scramble_curve);
                Some(crate::scramble::ScrambleAnimation::new_with_options("MONOKIT", mode, config.display.scramble_speed, curve))
            } else {
                None
            },
            scramble_enabled: config.display.scramble_enabled,
            scramble_mode: config.display.scramble_mode,
            scramble_speed: config.display.scramble_speed,
            scramble_curve: config.display.scramble_curve,
            ui_scrambles: {
                let mut scrambles = Vec::new();
                if config.display.scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(config.display.scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(config.display.scramble_curve);
                    scrambles.push(("CPU".to_string(), crate::scramble::ScrambleAnimation::new_with_options("CPU", mode, config.display.scramble_speed, curve)));
                    scrambles.push(("BPM".to_string(), crate::scramble::ScrambleAnimation::new_with_options("BPM", mode, config.display.scramble_speed, curve)));
                }
                scrambles
            },
            grid_scrambles: Vec::new(),
            ascii_meters: config.display.ascii_meters,
            awaiting_audio_restart: false,
            script_break: false,
        }
    }

    pub fn trigger_grid_scramble(&mut self) {
        use crate::types::{GRID_LABELS, GRID_ICONS};
        if !self.scramble_enabled {
            return;
        }
        let mode = crate::scramble::ScrambleMode::from_u8(self.scramble_mode);
        let curve = crate::scramble::ScrambleCurve::from_u8(self.scramble_curve);

        // Use icons or labels based on current grid_mode
        if self.grid_mode == 1 {
            self.grid_scrambles = GRID_ICONS
                .iter()
                .map(|icon| {
                    let s: String = icon.to_string();
                    crate::scramble::ScrambleAnimation::new_with_options(&s, mode, self.scramble_speed, curve)
                })
                .collect();
        } else {
            self.grid_scrambles = GRID_LABELS
                .iter()
                .map(|label| crate::scramble::ScrambleAnimation::new_with_options(label, mode, self.scramble_speed, curve))
                .collect();
        }
    }

    pub fn go_to_page(&mut self, page: Page) {
        if page != Page::Help {
            self.previous_page = page;
        }
        self.current_page = page;
        self.selected_line = None;
    }

    pub fn toggle_help(&mut self) {
        if self.current_page == Page::Help {
            self.current_page = self.previous_page;
        } else {
            self.previous_page = self.current_page;
            self.current_page = Page::Help;
        }
    }

    pub fn next_help_page(&mut self) {
        use crate::ui::pages::HELP_CATEGORIES;
        self.help_page = (self.help_page + 1) % HELP_CATEGORIES.len();
        self.help_scroll = 0;
    }

    pub fn prev_help_page(&mut self) {
        use crate::ui::pages::HELP_CATEGORIES;
        if self.help_page == 0 {
            self.help_page = HELP_CATEGORIES.len() - 1;
        } else {
            self.help_page -= 1;
        }
        self.help_scroll = 0;
    }

    pub fn next_page(&mut self) {
        self.current_page = self.current_page.next();
        self.selected_line = None;
    }

    pub fn prev_page(&mut self) {
        self.current_page = self.current_page.prev();
        self.selected_line = None;
    }

    pub fn is_script_page(&self) -> bool {
        matches!(
            self.current_page,
            Page::Script1
                | Page::Script2
                | Page::Script3
                | Page::Script4
                | Page::Script5
                | Page::Script6
                | Page::Script7
                | Page::Script8
                | Page::Metro
                | Page::Init
        )
    }

    pub fn current_script_index(&self) -> Option<usize> {
        match self.current_page {
            Page::Script1 => Some(0),
            Page::Script2 => Some(1),
            Page::Script3 => Some(2),
            Page::Script4 => Some(3),
            Page::Script5 => Some(4),
            Page::Script6 => Some(5),
            Page::Script7 => Some(6),
            Page::Script8 => Some(7),
            Page::Metro => Some(8),
            Page::Init => Some(9),
            _ => None,
        }
    }

    pub fn add_output(&mut self, msg: String) {
        self.output.push(msg);
        if self.output.len() > 100 {
            self.output.remove(0);
        }
        // Reset scroll to bottom when new output is added
        self.output_scroll = 0;
    }

    /// Check if output should be shown based on tier and category overrides
    pub fn should_output(&self, category: OutputCategory) -> bool {
        let tier = match category {
            OutputCategory::Error => TIER_ERRORS,
            OutputCategory::Essential => TIER_ESSENTIAL,
            OutputCategory::Query => TIER_QUERIES,
            OutputCategory::Confirm => TIER_CONFIRMS,
            OutputCategory::Verbose => TIER_VERBOSE,
        };

        // Tier check OR category override
        self.debug_level >= tier || match category {
            OutputCategory::Error => self.out_err,
            OutputCategory::Essential => self.out_ess,
            OutputCategory::Query => self.out_qry,
            OutputCategory::Confirm => self.out_cfm,
            OutputCategory::Verbose => false, // No override for verbose
        }
    }

    pub fn execute_script(&mut self, script_index: usize) {
        self.if_else_condition = true;
        self.execute_script_with_depth(script_index, 0);
    }

    pub fn execute_delayed_command(&mut self, command: &str, script_index: usize) {
        let mut metro_interval = {
            let state = self.metro_state.lock().unwrap();
            state.interval_ms
        };
        self.if_else_condition = true;

        let mut output_messages = Vec::new();
        let result = crate::commands::process_command(
            &self.metro_tx,
            &mut metro_interval,
            &mut self.br_len,
            &mut self.sync_mode,
            &mut self.midi_connection,
            &self.midi_timing_stats,
            &mut self.variables,
            &mut self.patterns,
            &mut self.counters,
            &mut self.scripts,
            script_index,
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
            &mut self.ascii_meters,
            &self.terminal_caps,
            self.color_mode,
            &mut self.script_break,
            command,
            |msg| {
                output_messages.push(msg);
            },
        );

        match result {
            Ok(scripts_to_run) => {
                for msg in output_messages {
                    self.add_output(msg);
                }
                for script_idx in scripts_to_run {
                    self.execute_script(script_idx);
                }
            }
            Err(e) => {
                output_messages.push(format!("ERROR: {}", e.to_string().to_uppercase()));
                for msg in output_messages {
                    self.add_output(msg);
                }
            }
        }
    }

    pub fn clear_expired_error(&mut self) {
        if let Some(time) = self.script_error_time {
            if time.elapsed().as_secs() >= 3 {
                self.script_error = None;
                self.script_error_time = None;
            }
        }
    }

    pub fn enter_search_mode(&mut self) {
        self.search_mode = true;
        self.search_query.clear();
        self.search_cursor = 0;
        self.search_matches.clear();
        self.search_current_match = 0;
    }

    pub fn exit_search_mode(&mut self) {
        self.search_mode = false;
        self.search_query.clear();
        self.search_matches.clear();
        self.search_current_match = 0;
    }

    pub fn perform_search(&mut self) {
        if self.current_page == Page::Help {
            use crate::ui::pages::HELP_CATEGORIES;
            self.search_matches = crate::ui::search::search_help(&self.search_query, HELP_CATEGORIES);
        } else {
            self.search_matches = crate::ui::search::search_scripts(&self.search_query, &self.scripts);
        }
        if !self.search_matches.is_empty() {
            self.search_current_match = 0;
            self.jump_to_current_match();
        } else {
            self.search_current_match = 0;
        }
    }

    pub fn next_search_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        self.search_current_match = (self.search_current_match + 1) % self.search_matches.len();
        self.jump_to_current_match();
    }

    pub fn prev_search_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        if self.search_current_match == 0 {
            self.search_current_match = self.search_matches.len() - 1;
        } else {
            self.search_current_match -= 1;
        }
        self.jump_to_current_match();
    }

    fn jump_to_current_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        let match_data = &self.search_matches[self.search_current_match];

        if match_data.page != self.current_page {
            if match_data.page != Page::Help {
                self.previous_page = match_data.page;
            }
            self.current_page = match_data.page;
        }

        match match_data.scope {
            crate::types::SearchScope::Help => {
                self.help_page = match_data.page_index;
                self.help_scroll = match_data.line_index;
            }
            crate::types::SearchScope::Script => {
                self.selected_line = Some(match_data.line_index);
            }
        }
    }

    pub fn search_insert_char(&mut self, c: char) {
        let c = c.to_ascii_uppercase();
        let byte_pos = self
            .search_query
            .char_indices()
            .nth(self.search_cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.search_query.len());
        self.search_query.insert(byte_pos, c);
        self.search_cursor += 1;
        self.perform_search();
    }

    pub fn search_delete_char(&mut self) {
        if self.search_cursor > 0 {
            let byte_pos = self
                .search_query
                .char_indices()
                .nth(self.search_cursor - 1)
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.search_query.remove(byte_pos);
            self.search_cursor -= 1;
            self.perform_search();
        }
    }

    pub fn search_move_cursor_left(&mut self) {
        if self.search_cursor > 0 {
            self.search_cursor -= 1;
        }
    }

    pub fn search_move_cursor_right(&mut self) {
        if self.search_cursor < self.search_query.chars().count() {
            self.search_cursor += 1;
        }
    }
}
