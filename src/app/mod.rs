use crate::theme::Theme;
use crate::types::{
    Counters, MetroCommand, MetroState, Page, PatternStorage, ScaleState, ScriptStorage, Variables,
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
}

impl App {
    pub fn new(metro_tx: Sender<MetroCommand>, metro_state: Arc<Mutex<MetroState>>, theme: Theme) -> Self {
        Self {
            current_page: Page::Live,
            previous_page: Page::Live,
            input: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: None,
            output: Vec::new(),
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
            &mut self.variables,
            &mut self.patterns,
            &mut self.counters,
            &mut self.scripts,
            script_index,
            &mut self.scale,
            &mut self.theme,
            &mut self.debug_level,
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
}
