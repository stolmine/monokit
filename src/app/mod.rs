use crate::midi::{MidiConnection, MidiTimingStats};
use crate::theme::Theme;
use crate::types::{
    Counters, CpuData, LineSegmentActivity, MeterData, MetroCommand, MetroState, NotesStorage, Page, ParamActivity, PatternStorage, ScaleState, ScopeData, ScriptStorage, SearchMatch, SpectrumData, SyncMode, Variables,
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
    pub scope_timespan_ms: u32,
    pub scope_color_mode: u8,
    pub scope_display_mode: u8,  // 0=braille, 1=block, 2=line, 3=dot
    pub scope_unipolar: bool,  // Rectify waveform and use full height for 0-1 range
    pub cpu_data: CpuData,
    pub show_cpu: bool,
    pub show_bpm: bool,
    pub header_level: u8,
    pub limiter_enabled: bool,
    pub show_meters_header: bool,
    pub show_meters_grid: bool,
    pub show_spectrum: bool,
    pub show_activity: bool,
    pub show_seq_highlight: bool,
    pub grid_mode: u8,
    pub notes: NotesStorage,
    pub load_rst: bool,
    pub search_mode: bool,
    pub search_query: String,
    pub search_cursor: usize,
    pub search_matches: Vec<SearchMatch>,
    pub search_current_match: usize,
    pub conditional_segments: [[LineSegmentActivity; 8]; 10],
    pub show_conditional_highlight: bool,
    pub current_scene_name: Option<String>,
    pub title_mode: u8,
}

impl App {
    pub fn new(metro_tx: Sender<MetroCommand>, metro_state: Arc<Mutex<MetroState>>, theme: Theme, config: &crate::config::Config) -> Self {
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
            recording: false,
            recording_start: None,
            debug_level: 2,
            br_len: 2,
            slew_time_ms: 0.0,
            scale: ScaleState::default(),
            sync_mode: SyncMode::Internal,
            midi_connection: None,
            midi_timing_stats: MidiTimingStats::new(),
            script_activity: [None; 10],
            trigger_activity: None,
            activity_hold_ms: crate::theme::DEFAULT_ACTIVITY_HOLD_MS,
            param_activity: ParamActivity::default(),
            show_grid_view: false,
            meter_data: MeterData::default(),
            spectrum_data: SpectrumData::default(),
            scope_data: ScopeData::default(),
            scope_timespan_ms: 30, // Default ~30ms window (matches SC phasor rate 0.1)
            scope_color_mode: 0,
            scope_display_mode: 0,
            scope_unipolar: false,
            cpu_data: CpuData::default(),
            show_cpu: false,
            show_bpm: config.display.show_bpm,
            header_level: config.display.header_level,
            limiter_enabled: true,
            show_meters_header: true,
            show_meters_grid: true,
            show_spectrum: true,
            show_activity: true,
            show_seq_highlight: true,
            grid_mode: 1,
            notes: NotesStorage::default(),
            load_rst: config.display.load_rst,
            search_mode: false,
            search_query: String::new(),
            search_cursor: 0,
            search_matches: Vec::new(),
            search_current_match: 0,
            conditional_segments: Default::default(),
            show_conditional_highlight: true,
            current_scene_name: None,
            title_mode: config.display.title_mode,
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
            &mut self.show_conditional_highlight,
            &mut self.scope_timespan_ms,
            &mut self.scope_color_mode,
            &mut self.scope_display_mode,
            &mut self.scope_unipolar,
            &mut self.show_meters_header,
            &mut self.show_meters_grid,
            &mut self.show_spectrum,
            &mut self.show_activity,
            &mut self.show_grid_view,
            &mut self.show_seq_highlight,
            &mut self.grid_mode,
            &mut self.current_scene_name,
            &mut self.title_mode,
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
                output_messages.push(format!("Error: {}", e));
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
