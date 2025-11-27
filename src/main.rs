use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use rand::Rng;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::io;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const OSC_ADDR: &str = "127.0.0.1:57120";

#[derive(Debug, Clone)]
struct Script {
    lines: [String; 8],
    j: i16,
    k: i16,
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
struct Variables {
    a: i16,
    b: i16,
    c: i16,
    d: i16,
    x: i16,
    y: i16,
    z: i16,
    t: i16,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            x: 0,
            y: 0,
            z: 0,
            t: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    data: [i16; 64],
    length: usize,
    index: usize,
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

#[derive(Debug, Clone)]
struct PatternStorage {
    patterns: [Pattern; 4],
    working: usize,
}

impl Default for PatternStorage {
    fn default() -> Self {
        Self {
            patterns: [
                Pattern::default(),
                Pattern::default(),
                Pattern::default(),
                Pattern::default(),
            ],
            working: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct ScriptStorage {
    scripts: [Script; 10],
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
    fn get_script(&self, index: usize) -> &Script {
        &self.scripts[index]
    }

    fn get_script_mut(&mut self, index: usize) -> &mut Script {
        &mut self.scripts[index]
    }
}

#[derive(Debug, Clone)]
enum MetroCommand {
    SetInterval(u64),
    SetActive(bool),
    SetScriptIndex(usize),
    SendParam(String, OscType),
    SendTrigger,
    SendVolume(f32),
}

#[derive(Debug, Clone)]
enum MetroEvent {
    ExecuteScript(usize),
}

#[derive(Debug, Clone)]
struct MetroState {
    interval_ms: u64,
    active: bool,
    script_index: usize,
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
enum Page {
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
    Help,
}

const NAVIGABLE_PAGES: [Page; 12] = [
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
];

impl Page {
    fn name(&self) -> &str {
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
            Page::Help => "HELP",
        }
    }

    fn next(&self) -> Self {
        if *self == Page::Help {
            return Page::Help;
        }
        let idx = NAVIGABLE_PAGES.iter().position(|p| p == self).unwrap_or(0);
        NAVIGABLE_PAGES[(idx + 1) % NAVIGABLE_PAGES.len()]
    }

    fn prev(&self) -> Self {
        if *self == Page::Help {
            return Page::Help;
        }
        let idx = NAVIGABLE_PAGES.iter().position(|p| p == self).unwrap_or(0);
        NAVIGABLE_PAGES[(idx + NAVIGABLE_PAGES.len() - 1) % NAVIGABLE_PAGES.len()]
    }
}

struct App {
    current_page: Page,
    previous_page: Page,
    input: String,
    cursor_position: usize,
    history: Vec<String>,
    history_index: Option<usize>,
    output: Vec<String>,
    help_scroll: usize,
    metro_state: Arc<Mutex<MetroState>>,
    metro_tx: Sender<MetroCommand>,
    scripts: ScriptStorage,
    selected_line: Option<usize>,
    variables: Variables,
    patterns: PatternStorage,
    pattern_cursor: (usize, usize),
    pattern_input: String,
}

impl App {
    fn new(metro_tx: Sender<MetroCommand>, metro_state: Arc<Mutex<MetroState>>) -> Self {
        Self {
            current_page: Page::Live,
            previous_page: Page::Live,
            input: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: None,
            output: Vec::new(),
            help_scroll: 0,
            metro_state,
            metro_tx,
            scripts: ScriptStorage::default(),
            selected_line: None,
            variables: Variables::default(),
            patterns: PatternStorage::default(),
            pattern_cursor: (0, 0),
            pattern_input: String::new(),
        }
    }

    fn go_to_page(&mut self, page: Page) {
        if page != Page::Help {
            self.previous_page = page;
        }
        self.current_page = page;
        self.selected_line = None;
    }

    fn toggle_help(&mut self) {
        if self.current_page == Page::Help {
            self.current_page = self.previous_page;
        } else {
            self.previous_page = self.current_page;
            self.current_page = Page::Help;
        }
    }

    fn next_page(&mut self) {
        self.current_page = self.current_page.next();
        self.selected_line = None;
    }

    fn prev_page(&mut self) {
        self.current_page = self.current_page.prev();
        self.selected_line = None;
    }

    fn is_script_page(&self) -> bool {
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

    fn current_script_index(&self) -> Option<usize> {
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

    fn add_output(&mut self, msg: String) {
        self.output.push(msg);
        if self.output.len() > 100 {
            self.output.remove(0);
        }
    }

    fn execute_script(&mut self, script_index: usize) {
        self.execute_script_with_depth(script_index, 0);
    }

    fn execute_script_with_depth(&mut self, script_index: usize, depth: usize) {
        if script_index >= 10 {
            self.add_output(format!("Error: Invalid script index {}", script_index));
            return;
        }

        if depth > 10 {
            self.add_output("Error: Script recursion depth exceeded (max 10)".to_string());
            return;
        }

        let script = self.scripts.get_script(script_index);
        let lines: Vec<String> = script.lines.to_vec();

        let mut metro_interval = {
            let state = self.metro_state.lock().unwrap();
            state.interval_ms
        };

        // Debug: append to file
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/tmp/monokit_debug.txt") {
            writeln!(f, "--- Executing ---").ok();
            for (i, l) in lines.iter().enumerate() {
                writeln!(f, "  Line {}: empty={} content='{}'", i + 1, l.is_empty(), l).ok();
            }
        }

        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Debug each line execution
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/tmp/monokit_debug.txt") {
                writeln!(f, "Processing line {}: '{}'", line_num + 1, line).ok();
            }

            for sub_cmd in line.split(';') {
                let sub_cmd = sub_cmd.trim();
                if sub_cmd.is_empty() {
                    continue;
                }

                // Debug each sub-command
                if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/tmp/monokit_debug.txt") {
                    writeln!(f, "  sub_cmd: '{}'", sub_cmd).ok();
                }

                let (condition, cmd_to_run) = if let Some(colon_pos) = sub_cmd.find(':') {
                    let cond = &sub_cmd[..colon_pos];
                    let cmd = sub_cmd[colon_pos + 1..].trim();
                    (Some(cond), cmd)
                } else {
                    (None, sub_cmd)
                };

                if cmd_to_run.is_empty() {
                    continue;
                }

                if let Some(cond) = condition {
                    if !eval_condition(cond, &self.variables) {
                        continue;
                    }
                }

                let mut output_messages = Vec::new();
                let result = process_command(&self.metro_tx, &mut metro_interval, &mut self.variables, &mut self.patterns, cmd_to_run, |msg| {
                    output_messages.push(msg);
                });

                // Debug result
                if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/tmp/monokit_debug.txt") {
                    writeln!(f, "  cmd='{}' result={:?} output={:?}", cmd_to_run, result.is_ok(), output_messages).ok();
                }

                match result {
                    Ok(scripts_to_run) => {
                        for msg in output_messages {
                            self.add_output(msg);
                        }
                        for script_idx in scripts_to_run {
                            self.execute_script_with_depth(script_idx, depth + 1);
                        }
                    }
                    Err(e) => {
                        output_messages.push(format!("Error: {}", e));
                        for msg in output_messages {
                            self.add_output(msg);
                        }
                    }
                }

                let mut state = self.metro_state.lock().unwrap();
                state.interval_ms = metro_interval;
                if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                    if let Some(parts) = cmd_to_run.split_whitespace().nth(1) {
                        if let Ok(val) = parts.parse::<i32>() {
                            state.active = val != 0;
                        }
                    }
                }
                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                    if let Some(parts) = cmd_to_run.split_whitespace().nth(1) {
                        if let Ok(idx) = parts.parse::<usize>() {
                            if idx >= 1 && idx <= 8 {
                                state.script_index = idx - 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn execute_command(&mut self) {
        let cmd = self.input.trim().to_string();
        if cmd.is_empty() {
            return;
        }

        self.history.push(cmd.clone());
        self.history_index = None;
        self.input.clear();
        self.cursor_position = 0;

        let mut metro_interval = {
            let state = self.metro_state.lock().unwrap();
            state.interval_ms
        };

        for sub_cmd in cmd.split(';') {
            let sub_cmd = sub_cmd.trim();
            if sub_cmd.is_empty() {
                continue;
            }

            let (condition, cmd_to_run) = if let Some(colon_pos) = sub_cmd.find(':') {
                let cond = &sub_cmd[..colon_pos];
                let cmd = sub_cmd[colon_pos + 1..].trim();
                (Some(cond), cmd)
            } else {
                (None, sub_cmd)
            };

            if cmd_to_run.is_empty() {
                continue;
            }

            if let Some(cond) = condition {
                if !eval_condition(cond, &self.variables) {
                    continue;
                }
            }

            let mut output_messages = Vec::new();
            let result = process_command(&self.metro_tx, &mut metro_interval, &mut self.variables, &mut self.patterns, cmd_to_run, |msg| {
                output_messages.push(msg);
            });

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

            let mut state = self.metro_state.lock().unwrap();
            state.interval_ms = metro_interval;
            if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                if let Some(parts) = cmd_to_run.split_whitespace().nth(1) {
                    if let Ok(val) = parts.parse::<i32>() {
                        state.active = val != 0;
                    }
                }
            }
            if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                if let Some(parts) = cmd_to_run.split_whitespace().nth(1) {
                    if let Ok(idx) = parts.parse::<usize>() {
                        if idx >= 1 && idx <= 8 {
                            state.script_index = idx - 1;
                        }
                    }
                }
            }
        }
    }

    fn insert_char(&mut self, c: char) {
        let byte_pos = self.input.char_indices()
            .nth(self.cursor_position)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(byte_pos, c);
        self.cursor_position += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let byte_pos = self.input.char_indices()
                .nth(self.cursor_position - 1)
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(byte_pos);
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.chars().count() {
            self.cursor_position += 1;
        }
    }

    fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let idx = match self.history_index {
            None => self.history.len() - 1,
            Some(i) if i > 0 => i - 1,
            Some(i) => i,
        };
        self.history_index = Some(idx);
        self.input = self.history[idx].clone();
        self.cursor_position = self.input.len();
    }

    fn history_next(&mut self) {
        match self.history_index {
            None => {}
            Some(i) if i < self.history.len() - 1 => {
                self.history_index = Some(i + 1);
                self.input = self.history[i + 1].clone();
                self.cursor_position = self.input.len();
            }
            Some(_) => {
                self.history_index = None;
                self.input.clear();
                self.cursor_position = 0;
            }
        }
    }

    fn delete_to_start(&mut self) {
        self.input = self.input.chars().skip(self.cursor_position).collect();
        self.cursor_position = 0;
    }

    fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    fn select_line_up(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            let new_selection = match self.selected_line {
                None => 7,
                Some(0) => 0,
                Some(n) => n - 1,
            };
            self.selected_line = Some(new_selection);
            let script = self.scripts.get_script(script_idx);
            self.input = script.lines[new_selection].clone();
            self.cursor_position = self.input.len();
        }
    }

    fn select_line_down(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            let new_selection = match self.selected_line {
                None => 0,
                Some(7) => 7,
                Some(n) => n + 1,
            };
            self.selected_line = Some(new_selection);
            let script = self.scripts.get_script(script_idx);
            self.input = script.lines[new_selection].clone();
            self.cursor_position = self.input.len();
        }
    }

    fn save_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            let line_idx = if let Some(selected) = self.selected_line {
                selected
            } else {
                let script = self.scripts.get_script(script_idx);
                let mut first_empty = None;
                for i in 0..8 {
                    if script.lines[i].is_empty() {
                        first_empty = Some(i);
                        break;
                    }
                }
                first_empty.unwrap_or(7)
            };

            let script = self.scripts.get_script_mut(script_idx);
            script.lines[line_idx] = self.input.clone();
            self.selected_line = Some(line_idx);
            self.input.clear();
            self.cursor_position = 0;
        }
    }
}

fn precise_sleep(duration: Duration) {
    let start = Instant::now();
    let spin_threshold = Duration::from_micros(100);

    if duration > spin_threshold {
        thread::sleep(duration - spin_threshold);
    }

    while start.elapsed() < duration {
        std::hint::spin_loop();
    }
}

fn metro_thread(rx: mpsc::Receiver<MetroCommand>, state: Arc<Mutex<MetroState>>, event_tx: mpsc::Sender<MetroEvent>) {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Metro thread: Failed to bind UDP socket: {}", e);
            return;
        }
    };

    if let Err(e) = socket.connect(OSC_ADDR) {
        eprintln!("Metro thread: Failed to connect to OSC address: {}", e);
        return;
    }

    let mut interval_ms: u64 = 500;
    let mut active = false;
    let mut next_tick = Instant::now();

    loop {
        let mut interval_changed = false;
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                MetroCommand::SetInterval(ms) => {
                    interval_ms = ms;
                    interval_changed = true;
                }
                MetroCommand::SetActive(act) => {
                    active = act;
                }
                MetroCommand::SetScriptIndex(idx) => {
                    let mut state = state.lock().unwrap();
                    state.script_index = idx;
                }
                MetroCommand::SendParam(name, value) => {
                    let msg = OscMessage {
                        addr: "/monokit/param".to_string(),
                        args: vec![OscType::String(name), value],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SendTrigger => {
                    let msg = OscMessage {
                        addr: "/monokit/trigger".to_string(),
                        args: vec![],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SendVolume(value) => {
                    let msg = OscMessage {
                        addr: "/monokit/volume".to_string(),
                        args: vec![OscType::Float(value)],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
            }
        }

        if interval_changed {
            next_tick = Instant::now();
        }

        if active {
            let script_index = {
                let st = state.lock().unwrap();
                st.script_index
            };

            let _ = event_tx.send(MetroEvent::ExecuteScript(script_index));

            next_tick += Duration::from_millis(interval_ms);

            let now = Instant::now();
            if next_tick > now {
                let sleep_duration = next_tick - now;
                precise_sleep(sleep_duration);
            } else {
                next_tick = now;
            }
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn resolve_value(s: &str, variables: &Variables) -> i16 {
    match s.trim().to_uppercase().as_str() {
        "A" => variables.a,
        "B" => variables.b,
        "C" => variables.c,
        "D" => variables.d,
        "X" => variables.x,
        "Y" => variables.y,
        "Z" => variables.z,
        "T" => variables.t,
        _ => s.trim().parse::<i16>().unwrap_or(0),
    }
}

fn eval_expression(expr: &str, variables: &Variables, patterns: &mut PatternStorage) -> Option<i16> {
    let expr = expr.trim().to_uppercase();

    if expr.starts_with("PN.NEXT ") {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(pat) = parts[1].parse::<usize>() {
                if pat <= 3 {
                    let pattern = &mut patterns.patterns[pat];
                    pattern.index = (pattern.index + 1) % pattern.length;
                    return Some(pattern.data[pattern.index]);
                }
            }
        }
        return None;
    }

    if expr.starts_with("PN.PREV ") {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(pat) = parts[1].parse::<usize>() {
                if pat <= 3 {
                    let pattern = &mut patterns.patterns[pat];
                    if pattern.index == 0 {
                        pattern.index = pattern.length - 1;
                    } else {
                        pattern.index -= 1;
                    }
                    return Some(pattern.data[pattern.index]);
                }
            }
        }
        return None;
    }

    if expr.starts_with("PN.HERE ") {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(pat) = parts[1].parse::<usize>() {
                if pat <= 3 {
                    let pattern = &patterns.patterns[pat];
                    return Some(pattern.data[pattern.index]);
                }
            }
        }
        return None;
    }

    match expr.as_str() {
        "P.NEXT" => {
            let working = patterns.working;
            let pattern = &mut patterns.patterns[working];
            let old_index = pattern.index;
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            // Debug
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/monokit_debug.txt") {
                writeln!(f, "P.NEXT: working={} old_idx={} new_idx={} len={} value={}",
                    working, old_index, pattern.index, pattern.length, value).ok();
            }
            Some(value)
        }
        "P.PREV" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            Some(value)
        }
        "P.HERE" => {
            let pattern = &patterns.patterns[patterns.working];
            Some(pattern.data[pattern.index])
        }
        "A" => Some(variables.a),
        "B" => Some(variables.b),
        "C" => Some(variables.c),
        "D" => Some(variables.d),
        "X" => Some(variables.x),
        "Y" => Some(variables.y),
        "Z" => Some(variables.z),
        "T" => Some(variables.t),
        _ => {
            if let Ok(val) = expr.parse::<i16>() {
                Some(val)
            } else {
                None
            }
        }
    }
}

fn eval_condition(cond: &str, variables: &Variables) -> bool {
    let cond = cond.trim();

    if cond.starts_with("PROB ") {
        let pct_str = cond.strip_prefix("PROB ").unwrap_or("0").trim();
        let pct: u8 = pct_str.parse().unwrap_or(0);
        let pct = pct.min(100);
        let roll: u8 = rand::thread_rng().gen_range(0..100);
        return roll < pct;
    }

    let cond = cond.strip_prefix("IF ").unwrap_or(cond);

    if let Some(pos) = cond.find(">=") {
        let left = resolve_value(&cond[..pos], variables);
        let right = resolve_value(&cond[pos + 2..], variables);
        return left >= right;
    }

    if let Some(pos) = cond.find("<=") {
        let left = resolve_value(&cond[..pos], variables);
        let right = resolve_value(&cond[pos + 2..], variables);
        return left <= right;
    }

    if let Some(pos) = cond.find("!=") {
        let left = resolve_value(&cond[..pos], variables);
        let right = resolve_value(&cond[pos + 2..], variables);
        return left != right;
    }

    if let Some(pos) = cond.find("==") {
        let left = resolve_value(&cond[..pos], variables);
        let right = resolve_value(&cond[pos + 2..], variables);
        return left == right;
    }

    if let Some(pos) = cond.find('>') {
        let left = resolve_value(&cond[..pos], variables);
        let right = resolve_value(&cond[pos + 1..], variables);
        return left > right;
    }

    if let Some(pos) = cond.find('<') {
        let left = resolve_value(&cond[..pos], variables);
        let right = resolve_value(&cond[pos + 1..], variables);
        return left < right;
    }

    true
}

fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    input: &str,
    mut output: F,
) -> Result<Vec<usize>>
where
    F: FnMut(String),
{
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "A" => {
            if parts.len() == 1 {
                output(format!("A = {}", variables.a));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for A")?;
                variables.a = value;
                output(format!("Set A to {}", value));
            }
        }
        "B" => {
            if parts.len() == 1 {
                output(format!("B = {}", variables.b));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for B")?;
                variables.b = value;
                output(format!("Set B to {}", value));
            }
        }
        "C" => {
            if parts.len() == 1 {
                output(format!("C = {}", variables.c));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for C")?;
                variables.c = value;
                output(format!("Set C to {}", value));
            }
        }
        "D" => {
            if parts.len() == 1 {
                output(format!("D = {}", variables.d));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for D")?;
                variables.d = value;
                output(format!("Set D to {}", value));
            }
        }
        "X" => {
            if parts.len() == 1 {
                output(format!("X = {}", variables.x));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for X")?;
                variables.x = value;
                output(format!("Set X to {}", value));
            }
        }
        "Y" => {
            if parts.len() == 1 {
                output(format!("Y = {}", variables.y));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for Y")?;
                variables.y = value;
                output(format!("Set Y to {}", value));
            }
        }
        "Z" => {
            if parts.len() == 1 {
                output(format!("Z = {}", variables.z));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for Z")?;
                variables.z = value;
                output(format!("Set Z to {}", value));
            }
        }
        "T" => {
            if parts.len() == 1 {
                output(format!("T = {}", variables.t));
            } else {
                let value: i16 = parts[1]
                    .parse()
                    .context("Failed to parse value for T")?;
                variables.t = value;
                output(format!("Set T to {}", value));
            }
        }
        "P.N" => {
            if parts.len() == 1 {
                output(format!("P.N = {}", patterns.working));
            } else {
                let value: usize = parts[1]
                    .parse()
                    .context("Failed to parse pattern number")?;
                if value > 3 {
                    output("Error: Pattern number must be 0-3".to_string());
                    return Ok(vec![]);
                }
                patterns.working = value;
                output(format!("Set working pattern to {}", value));
            }
        }
        "P.L" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if parts.len() == 1 {
                output(format!("P.L = {}", pattern.length));
            } else {
                let value: usize = parts[1]
                    .parse()
                    .context("Failed to parse pattern length")?;
                if value < 1 || value > 64 {
                    output("Error: Pattern length must be 1-64".to_string());
                    return Ok(vec![]);
                }
                pattern.length = value;
                output(format!("Set pattern {} length to {}", patterns.working, value));
            }
        }
        "P.I" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if parts.len() == 1 {
                output(format!("P.I = {}", pattern.index));
            } else {
                let value: usize = parts[1]
                    .parse()
                    .context("Failed to parse pattern index")?;
                if value > 63 {
                    output("Error: Pattern index must be 0-63".to_string());
                    return Ok(vec![]);
                }
                pattern.index = value;
                output(format!("Set pattern {} index to {}", patterns.working, value));
            }
        }
        "P.HERE" => {
            let pattern = &patterns.patterns[patterns.working];
            let value = pattern.data[pattern.index];
            output(format!("P.HERE = {}", value));
        }
        "P.NEXT" => {
            let pattern = &mut patterns.patterns[patterns.working];
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            output(format!("P.NEXT = {} (index now {})", value, pattern.index));
        }
        "P.PREV" => {
            let pattern = &mut patterns.patterns[patterns.working];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            output(format!("P.PREV = {} (index now {})", value, pattern.index));
        }
        "P" => {
            if parts.len() == 1 {
                output("Error: P requires an index".to_string());
                return Ok(vec![]);
            }
            let idx: usize = parts[1]
                .parse()
                .context("Failed to parse pattern index")?;
            if idx > 63 {
                output("Error: Pattern index must be 0-63".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[patterns.working];
            if parts.len() == 2 {
                output(format!("P {} = {}", idx, pattern.data[idx]));
            } else {
                let value: i16 = parts[2]
                    .parse()
                    .context("Failed to parse pattern value")?;
                pattern.data[idx] = value;
                output(format!("Set P {} to {}", idx, value));
            }
        }
        "PN.L" => {
            if parts.len() < 2 {
                output("Error: PN.L requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = parts[1]
                .parse()
                .context("Failed to parse pattern number")?;
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            if parts.len() == 2 {
                output(format!("PN.L {} = {}", pat, pattern.length));
            } else {
                let value: usize = parts[2]
                    .parse()
                    .context("Failed to parse pattern length")?;
                if value < 1 || value > 64 {
                    output("Error: Pattern length must be 1-64".to_string());
                    return Ok(vec![]);
                }
                pattern.length = value;
                output(format!("Set pattern {} length to {}", pat, value));
            }
        }
        "PN.I" => {
            if parts.len() < 2 {
                output("Error: PN.I requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = parts[1]
                .parse()
                .context("Failed to parse pattern number")?;
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            if parts.len() == 2 {
                output(format!("PN.I {} = {}", pat, pattern.index));
            } else {
                let value: usize = parts[2]
                    .parse()
                    .context("Failed to parse pattern index")?;
                if value > 63 {
                    output("Error: Pattern index must be 0-63".to_string());
                    return Ok(vec![]);
                }
                pattern.index = value;
                output(format!("Set pattern {} index to {}", pat, value));
            }
        }
        "PN.HERE" => {
            if parts.len() < 2 {
                output("Error: PN.HERE requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = parts[1]
                .parse()
                .context("Failed to parse pattern number")?;
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &patterns.patterns[pat];
            let value = pattern.data[pattern.index];
            output(format!("PN.HERE {} = {}", pat, value));
        }
        "PN.NEXT" => {
            if parts.len() < 2 {
                output("Error: PN.NEXT requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = parts[1]
                .parse()
                .context("Failed to parse pattern number")?;
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            pattern.index = (pattern.index + 1) % pattern.length;
            let value = pattern.data[pattern.index];
            output(format!("PN.NEXT {} = {} (index now {})", pat, value, pattern.index));
        }
        "PN.PREV" => {
            if parts.len() < 2 {
                output("Error: PN.PREV requires pattern number (0-3)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = parts[1]
                .parse()
                .context("Failed to parse pattern number")?;
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            if pattern.index == 0 {
                pattern.index = pattern.length - 1;
            } else {
                pattern.index -= 1;
            }
            let value = pattern.data[pattern.index];
            output(format!("PN.PREV {} = {} (index now {})", pat, value, pattern.index));
        }
        "PN" => {
            if parts.len() < 3 {
                output("Error: PN requires pattern (0-3) and index (0-63)".to_string());
                return Ok(vec![]);
            }
            let pat: usize = parts[1]
                .parse()
                .context("Failed to parse pattern number")?;
            if pat > 3 {
                output("Error: Pattern number must be 0-3".to_string());
                return Ok(vec![]);
            }
            let idx: usize = parts[2]
                .parse()
                .context("Failed to parse pattern index")?;
            if idx > 63 {
                output("Error: Pattern index must be 0-63".to_string());
                return Ok(vec![]);
            }
            let pattern = &mut patterns.patterns[pat];
            if parts.len() == 3 {
                output(format!("PN {} {} = {}", pat, idx, pattern.data[idx]));
            } else {
                let val: i16 = parts[3]
                    .parse()
                    .context("Failed to parse pattern value")?;
                pattern.data[idx] = val;
                output(format!("Set PN {} {} to {}", pat, idx, val));
            }
        }
        "TR" => {
            metro_tx
                .send(MetroCommand::SendTrigger)
                .context("Failed to send trigger to metro thread")?;
            output("Sent trigger".to_string());
        }
        "VOL" => {
            if parts.len() < 2 {
                output("Error: VOL requires a value (0.0-1.0)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse volume value as float")?;
            if !(0.0..=1.0).contains(&value) {
                output("Warning: Volume should be between 0.0 and 1.0".to_string());
            }
            metro_tx
                .send(MetroCommand::SendVolume(value))
                .context("Failed to send volume to metro thread")?;
            output(format!("Set volume to {}", value));
        }
        "M" => {
            if parts.len() == 1 {
                output(format!("Metro interval: {}ms", metro_interval));
            } else {
                let value: u64 = parts[1]
                    .parse()
                    .context("Failed to parse interval as milliseconds")?;
                if value == 0 {
                    output("Error: Interval must be greater than 0".to_string());
                    return Ok(vec![]);
                }
                metro_tx
                    .send(MetroCommand::SetInterval(value))
                    .context("Failed to send interval to metro thread")?;
                *metro_interval = value;
                output(format!("Set metro interval to {}ms", value));
            }
        }
        "M.BPM" => {
            if parts.len() < 2 {
                output("Error: M.BPM requires a BPM value".to_string());
                return Ok(vec![]);
            }
            let bpm: f32 = parts[1]
                .parse()
                .context("Failed to parse BPM value as number")?;
            if bpm <= 0.0 {
                output("Error: BPM must be greater than 0".to_string());
                return Ok(vec![]);
            }
            let interval_ms = (60000.0 / bpm) as u64;
            metro_tx
                .send(MetroCommand::SetInterval(interval_ms))
                .context("Failed to send interval to metro thread")?;
            *metro_interval = interval_ms;
            output(format!("Set metro to {} BPM ({}ms)", bpm, interval_ms));
        }
        "M.ACT" => {
            if parts.len() < 2 {
                output("Error: M.ACT requires 0 or 1".to_string());
                return Ok(vec![]);
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse M.ACT value")?;
            if !(0..=1).contains(&value) {
                output("Error: M.ACT value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SetActive(value != 0))
                .context("Failed to send active state to metro thread")?;
            output(format!(
                "Metro {}",
                if value != 0 {
                    "activated"
                } else {
                    "deactivated"
                }
            ));
        }
        "M.SCRIPT" => {
            if parts.len() < 2 {
                output("Error: M.SCRIPT requires a script number (1-8)".to_string());
                return Ok(vec![]);
            }
            let value: usize = parts[1]
                .parse()
                .context("Failed to parse script number")?;
            if !(1..=8).contains(&value) {
                output("Error: M.SCRIPT value must be 1-8".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SetScriptIndex(value - 1))
                .context("Failed to send script index to metro thread")?;
            output(format!("Metro will call Script {} on each tick", value));
        }
        "PF" => {
            if parts.len() < 2 {
                output("Error: PF requires a frequency value (20-20000)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse frequency value")?
            };
            if !(20.0..=20000.0).contains(&value) {
                output("Error: Frequency must be between 20 and 20000 Hz".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set primary frequency to {} Hz", value));
        }
        "PW" => {
            if parts.len() < 2 {
                output("Error: PW requires a waveform value (0-2)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse waveform value")?
            };
            if !(0..=2).contains(&value) {
                output("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set primary waveform to {}", value));
        }
        "MF" => {
            if parts.len() < 2 {
                output("Error: MF requires a frequency value (20-20000)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse frequency value")?
            };
            if !(20.0..=20000.0).contains(&value) {
                output("Error: Frequency must be between 20 and 20000 Hz".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod frequency to {} Hz", value));
        }
        "MW" => {
            if parts.len() < 2 {
                output("Error: MW requires a waveform value (0-2)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse waveform value")?
            };
            if !(0..=2).contains(&value) {
                output("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod waveform to {}", value));
        }
        "DC" => {
            if parts.len() < 2 {
                output("Error: DC requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse discontinuity amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Discontinuity amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            // Debug DC
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/monokit_debug.txt") {
                writeln!(f, "DC sending OSC: value={}", value).ok();
            }
            metro_tx
                .send(MetroCommand::SendParam("dc".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity amount to {}", value));
        }
        "DM" => {
            if parts.len() < 2 {
                output("Error: DM requires a mode value (0-2)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse discontinuity mode")?
            };
            if !(0..=2).contains(&value) {
                output("Error: Mode must be 0 (fold), 1 (tanh), or 2 (softclip)".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("dm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity mode to {}", value));
        }
        "TK" => {
            if parts.len() < 2 {
                output("Error: TK requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse tracking amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Tracking amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("tk".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set tracking amount to {}", value));
        }
        "MB" => {
            if parts.len() < 2 {
                output("Error: MB requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod bus amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Mod bus amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mb".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod bus amount to {}", value));
        }
        "MP" => {
            if parts.len() < 2 {
                output("Error: MP requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> primary value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mp".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> primary freq to {}", value));
        }
        "MD" => {
            if parts.len() < 2 {
                output("Error: MD requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> discontinuity value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("md".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> discontinuity to {}", value));
        }
        "MT" => {
            if parts.len() < 2 {
                output("Error: MT requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> tracking value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mt".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> tracking to {}", value));
        }
        "MA" => {
            if parts.len() < 2 {
                output("Error: MA requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod -> amplitude value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("ma".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> amplitude to {}", value));
        }
        "FM" => {
            if parts.len() < 2 {
                output("Error: FM requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse FM index")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: FM index must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM index to {}", value));
        }
        "AD" => {
            if parts.len() < 2 {
                output("Error: AD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse amp decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: Amp decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set amp decay to {} ms", value));
        }
        "PD" => {
            if parts.len() < 2 {
                output("Error: PD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pitch decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: Pitch decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set pitch decay to {} ms", value));
        }
        "FD" => {
            if parts.len() < 2 {
                output("Error: FD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse FM decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: FM decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM decay to {} ms", value));
        }
        "PA" => {
            if parts.len() < 2 {
                output("Error: PA requires a multiplier value (0-16)".to_string());
                return Ok(vec![]);
            }
            let value: f32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as f32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse pitch env amount")?
            };
            if !(0.0..=16.0).contains(&value) {
                output("Error: Pitch env amount must be between 0 and 16".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set pitch env amount to {}", value));
        }
        "DD" => {
            if parts.len() < 2 {
                output("Error: DD requires a time value (1-10000 ms)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse discontinuity decay time")?
            };
            if !(1..=10000).contains(&value) {
                output("Error: Discontinuity decay must be between 1 and 10000 ms".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("dd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity decay to {} ms", value));
        }
        "MX" => {
            if parts.len() < 2 {
                output("Error: MX requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mix amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: Mix amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mx".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mix amount to {}", value));
        }
        "MM" => {
            if parts.len() < 2 {
                output("Error: MM requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse mod bus -> mix value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("mm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod bus -> mix to {}", value));
        }
        "ME" => {
            if parts.len() < 2 {
                output("Error: ME requires a value (0 or 1)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse envelope -> mix value")?
            };
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("me".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set envelope -> mix to {}", value));
        }
        "FA" => {
            if parts.len() < 2 {
                output("Error: FA requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse FM envelope amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: FM envelope amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM envelope amount to {}", value));
        }
        "DA" => {
            if parts.len() < 2 {
                output("Error: DA requires a value (0-16383)".to_string());
                return Ok(vec![]);
            }
            let value: i32 = if let Some(expr_val) = eval_expression(parts[1], variables, patterns) {
                expr_val as i32
            } else {
                parts[1]
                    .parse()
                    .context("Failed to parse DC envelope amount")?
            };
            if !(0..=16383).contains(&value) {
                output("Error: DC envelope amount must be between 0 and 16383".to_string());
                return Ok(vec![]);
            }
            metro_tx
                .send(MetroCommand::SendParam("da".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set DC envelope amount to {}", value));
        }
        "RST" => {
            metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(200.0)))?;
            metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(50.0)))?;
            metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dc".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dd".to_string(), OscType::Int(100)))?;
            metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("ad".to_string(), OscType::Int(100)))?;
            metro_tx.send(MetroCommand::SendParam("pd".to_string(), OscType::Int(10)))?;
            metro_tx.send(MetroCommand::SendParam("fd".to_string(), OscType::Int(10)))?;
            metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(4.0)))?;
            metro_tx.send(MetroCommand::SendParam("mx".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("me".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendVolume(1.0))?;
            output("Reset to defaults".to_string());
        }
        "SCRIPT" => {
            if parts.len() < 2 {
                output("Error: SCRIPT requires number 1-8".to_string());
                return Ok(vec![]);
            }
            let num: usize = parts[1]
                .parse()
                .context("Failed to parse script number")?;
            if num < 1 || num > 8 {
                output("Error: SCRIPT number must be 1-8".to_string());
                return Ok(vec![]);
            }
            return Ok(vec![num - 1]);
        }
        "HELP" => {
            output("=== MONOKIT COMMANDS ===".to_string());
            output("".to_string());
            output("TRIGGER: TR".to_string());
            output("VOLUME:  VOL <0.0-1.0>".to_string());
            output("RESET:   RST".to_string());
            output("".to_string());
            output("-- Oscillators --".to_string());
            output("PF <hz>     Primary freq (20-20000)".to_string());
            output("PW <0-2>    Primary wave (sin/tri/saw)".to_string());
            output("MF <hz>     Mod freq".to_string());
            output("MW <0-2>    Mod wave".to_string());
            output("".to_string());
            output("-- FM & Discontinuity --".to_string());
            output("FM <0-16383>  FM index".to_string());
            output("FA <0-16383>  FM env amount".to_string());
            output("FD <ms>       FM env decay".to_string());
            output("DC <0-16383>  Discontinuity".to_string());
            output("DA <0-16383>  DC env amount".to_string());
            output("DD <ms>       DC env decay".to_string());
            output("DM <0-2>      DC mode (fold/tanh/soft)".to_string());
            output("".to_string());
            output("-- Envelopes --".to_string());
            output("AD <ms>       Amp decay".to_string());
            output("PD <ms>       Pitch decay".to_string());
            output("PA <0-16>     Pitch env amount".to_string());
            output("".to_string());
            output("-- Mod Bus --".to_string());
            output("MB <0-16383>  Mod bus amount".to_string());
            output("MP/MD/MT/MA <0|1>  Routing toggles".to_string());
            output("TK <0-16383>  Tracking".to_string());
            output("".to_string());
            output("-- Mix --".to_string());
            output("MX <0-16383>  Mix amount".to_string());
            output("MM/ME <0|1>   Mix routing".to_string());
            output("".to_string());
            output("-- Metro --".to_string());
            output("M             Show interval".to_string());
            output("M <ms>        Set interval".to_string());
            output("M.BPM <bpm>   Set BPM".to_string());
            output("M.ACT <0|1>     Start/stop".to_string());
            output("M.SCRIPT <1-8> Set script to call on each tick".to_string());
            output("".to_string());
            output("-- Scripts --".to_string());
            output("SCRIPT <1-8>  Execute stored script".to_string());
        }
        _ => {
            output(format!("Unknown command: {}", cmd));
        }
    }

    Ok(vec![])
}

fn validate_script_command(cmd: &str) -> Result<()> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let command = parts[0].to_uppercase();

    match command.as_str() {
        "TR" | "RST" => Ok(()),
        "VOL" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("VOL requires a value"));
            }
            let _value: f32 = parts[1].parse().context("Failed to parse volume value")?;
            Ok(())
        }
        "PF" | "MF" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a frequency value", command));
            }
            let value: f32 = parts[1].parse().context("Failed to parse frequency")?;
            if !(20.0..=20000.0).contains(&value) {
                return Err(anyhow::anyhow!("Frequency must be between 20 and 20000"));
            }
            Ok(())
        }
        "PW" | "MW" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a waveform value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse waveform")?;
            if !(0..=2).contains(&value) {
                return Err(anyhow::anyhow!("Waveform must be 0-2"));
            }
            Ok(())
        }
        "DC" | "TK" | "MB" | "FM" | "MX" | "FA" | "DA" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse value")?;
            if !(0..=16383).contains(&value) {
                return Err(anyhow::anyhow!("Value must be between 0 and 16383"));
            }
            Ok(())
        }
        "DM" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("DM requires a mode value"));
            }
            let value: i32 = parts[1].parse().context("Failed to parse mode")?;
            if !(0..=2).contains(&value) {
                return Err(anyhow::anyhow!("Mode must be 0-2"));
            }
            Ok(())
        }
        "MP" | "MD" | "MT" | "MA" | "MM" | "ME" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse value")?;
            if !(0..=1).contains(&value) {
                return Err(anyhow::anyhow!("Value must be 0 or 1"));
            }
            Ok(())
        }
        "AD" | "PD" | "FD" | "DD" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a time value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse time")?;
            if !(1..=10000).contains(&value) {
                return Err(anyhow::anyhow!("Time must be between 1 and 10000 ms"));
            }
            Ok(())
        }
        "PA" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("PA requires a multiplier value"));
            }
            let value: f32 = parts[1].parse().context("Failed to parse multiplier")?;
            if !(0.0..=16.0).contains(&value) {
                return Err(anyhow::anyhow!("Multiplier must be between 0 and 16"));
            }
            Ok(())
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown command in script: {}", command));
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let is_help = app.current_page == Page::Help;
    let is_pattern = app.current_page == Page::Pattern;

    let chunks = if is_help || is_pattern {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(f.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area())
    };

    let header = render_header(app);
    f.render_widget(header, chunks[0]);

    let content = match app.current_page {
        Page::Live => render_live_page(app, chunks[1].height as usize),
        Page::Script1 => render_script_page(app, 1),
        Page::Script2 => render_script_page(app, 2),
        Page::Script3 => render_script_page(app, 3),
        Page::Script4 => render_script_page(app, 4),
        Page::Script5 => render_script_page(app, 5),
        Page::Script6 => render_script_page(app, 6),
        Page::Script7 => render_script_page(app, 7),
        Page::Script8 => render_script_page(app, 8),
        Page::Metro => render_metro_page(app),
        Page::Init => render_init_page(app),
        Page::Pattern => render_pattern_page(app),
        Page::Help => render_help_page(app.help_scroll, chunks[1].height as usize),
    };
    f.render_widget(content, chunks[1]);

    let is_pattern = app.current_page == Page::Pattern;
    if !is_help && !is_pattern {
        let footer = render_footer(app);
        f.render_widget(footer, chunks[2]);
    }
}

fn render_header(app: &App) -> Paragraph<'static> {
    let mut spans = vec![Span::raw(" ")];

    if app.current_page == Page::Help {
        spans.push(Span::styled(
            "[HELP]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        for page in NAVIGABLE_PAGES.iter() {
            if *page == app.current_page {
                spans.push(Span::styled(
                    format!("[{}]", page.name()),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    page.name().to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
            }
            spans.push(Span::raw(" "));
        }
    }

    Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title(" MONOKIT "))
}

fn render_metro_page(app: &App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 60000.0 / state.interval_ms as f32;
    let status = if state.active { "ON" } else { "OFF" };
    let status_color = if state.active {
        Color::Green
    } else {
        Color::Red
    };

    let mut text = Vec::new();
    text.push(Line::from(vec![
        Span::styled("  BPM: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{:.1}", bpm)),
        Span::raw("  "),
        Span::styled("Interval: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{}ms", state.interval_ms)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  Status: ", Style::default().fg(Color::Cyan)),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  M Script Lines:", Style::default().fg(Color::Cyan)),
    ]));

    let metro_script = app.scripts.get_script(8);
    for i in 0..8 {
        let line_num = i + 1;
        let line_content = &metro_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            if line_content.is_empty() {
                text.push(Line::from(vec![
                    Span::styled(format!("  > {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                text.push(Line::from(vec![
                    Span::styled(format!("  > {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(line_content.clone(), Style::default().fg(Color::Yellow)),
                ]));
            }
        } else if line_content.is_empty() {
            text.push(Line::from(vec![
                Span::styled(format!("    {}: ", line_num), Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            text.push(Line::from(vec![
                Span::styled(format!("    {}: ", line_num), Style::default().fg(Color::Cyan)),
                Span::raw(line_content.clone()),
            ]));
        }
    }

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Metro "))
        .wrap(Wrap { trim: false })
}

fn render_live_page(app: &App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    let start_idx = if app.output.len() > visible_lines {
        app.output.len() - visible_lines
    } else {
        0
    };

    let text: Vec<Line> = app.output[start_idx..]
        .iter()
        .map(|line| Line::from(format!("  {}", line)))
        .collect();

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Live "))
}

fn render_script_page(app: &App, num: u8) -> Paragraph<'static> {
    let script_index = (num - 1) as usize;
    let script = app.scripts.get_script(script_index);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_num = i + 1;
        let line_content = &script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            if line_content.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(line_content.clone(), Style::default().fg(Color::Yellow)),
                ]));
            }
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::Cyan)),
                Span::raw(line_content.clone()),
            ]));
        }
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(format!(" Script {} ", num)))
}

fn render_init_page(app: &App) -> Paragraph<'static> {
    let init_script = app.scripts.get_script(9);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_num = i + 1;
        let line_content = &init_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            if line_content.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(line_content.clone(), Style::default().fg(Color::Yellow)),
                ]));
            }
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::Cyan)),
                Span::raw(line_content.clone()),
            ]));
        }
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Init "))
}

fn render_pattern_page(app: &App) -> Paragraph<'static> {
    let (cursor_pattern, cursor_step) = app.pattern_cursor;

    let visible_rows = 16;
    let scroll_offset = if cursor_step < visible_rows / 2 {
        0
    } else if cursor_step >= 64 - visible_rows / 2 {
        64 - visible_rows
    } else {
        cursor_step.saturating_sub(visible_rows / 2)
    };

    let mut lines = vec![];

    // Header row with pattern labels
    let mut header_spans = vec![Span::raw("     ")];
    for pattern_idx in 0..4 {
        let label = format!("P{}", pattern_idx);
        let style = if pattern_idx == app.patterns.working {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        header_spans.push(Span::styled(format!(" {:^5} ", label), style));
    }
    lines.push(Line::from(header_spans));

    // Length row
    let mut len_spans = vec![Span::styled(" len ", Style::default().fg(Color::DarkGray))];
    for pattern_idx in 0..4 {
        let pattern = &app.patterns.patterns[pattern_idx];
        len_spans.push(Span::styled(
            format!(" {:^5} ", pattern.length),
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::from(len_spans));

    // Pattern data rows
    for step in scroll_offset..(scroll_offset + visible_rows).min(64) {
        let mut row_spans = vec![
            Span::styled(format!("{:3}: ", step), Style::default().fg(Color::DarkGray)),
        ];

        for pattern_idx in 0..4 {
            let pattern = &app.patterns.patterns[pattern_idx];
            let value = pattern.data[step];
            let is_cursor = cursor_pattern == pattern_idx && cursor_step == step;
            let is_playhead = pattern.index == step;
            let is_beyond_length = step >= pattern.length;

            let display = if is_cursor && !app.pattern_input.is_empty() {
                format!(" {:>5} ", app.pattern_input)
            } else {
                format!(" {:>5} ", value)
            };

            let style = if is_cursor {
                // Cursor: white on black (inverse)
                Style::default().bg(Color::White).fg(Color::Black)
            } else if is_playhead && !is_beyond_length {
                // Playhead: cyan on dark (subtle inverse)
                Style::default().bg(Color::Cyan).fg(Color::Black)
            } else if is_beyond_length {
                // Beyond length: dimmed
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };

            row_spans.push(Span::styled(display, style));
        }

        lines.push(Line::from(row_spans));
    }

    let title = format!(" Pattern ({}/64) ", cursor_step);
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
}

const HELP_LINES: &[&str] = &[
    "",
    "  NAVIGATION",
    "  [ ]         Cycle pages",
    "  Alt+L       Live page",
    "  Alt+1-8     Script 1-8",
    "  Alt+M       Metro page",
    "  Alt+I       Init page",
    "  Alt+P       Pattern page",
    "  Alt+H       Toggle help",
    "  q           Quit",
    "",
    "  TRIGGER & VOLUME",
    "  TR          Trigger voice",
    "  VOL 0-1     Master volume",
    "  RST         Reset to defaults",
    "",
    "  OSCILLATORS",
    "  PF <hz>     Primary freq (20-20000)",
    "  PW <0-2>    Primary wave (sin/tri/saw)",
    "  MF <hz>     Mod freq (20-20000)",
    "  MW <0-2>    Mod wave (sin/tri/saw)",
    "",
    "  FM SYNTHESIS",
    "  FM <0-16383>  FM index",
    "  FA <0-16383>  FM env amount",
    "  FD <ms>       FM env decay",
    "",
    "  DISCONTINUITY",
    "  DC <0-16383>  Discontinuity amount",
    "  DA <0-16383>  DC env amount",
    "  DD <ms>       DC env decay",
    "  DM <0-2>      Mode (fold/tanh/soft)",
    "",
    "  ENVELOPES",
    "  AD <ms>       Amp decay",
    "  PD <ms>       Pitch decay",
    "  PA <0-16>     Pitch env amount",
    "",
    "  MOD BUS",
    "  MB <0-16383>  Mod bus amount",
    "  TK <0-16383>  Tracking amount",
    "  MP <0|1>      Mod -> primary freq",
    "  MD <0|1>      Mod -> discontinuity",
    "  MT <0|1>      Mod -> tracking",
    "  MA <0|1>      Mod -> amplitude",
    "",
    "  MIX",
    "  MX <0-16383>  Mix amount",
    "  MM <0|1>      Mod bus -> mix",
    "  ME <0|1>      Envelope -> mix",
    "",
    "  METRO",
    "  M             Show interval",
    "  M <ms>        Set interval",
    "  M.BPM <bpm>   Set BPM",
    "  M.ACT <0|1>     Start/stop",
    "  M.SCRIPT <1-8>  Set script to call on each tick",
    "",
    "  SCRIPTS",
    "  SCRIPT <1-8>  Execute stored script",
    "",
    "  PATTERNS (Working Pattern)",
    "  P.N           Show working pattern",
    "  P.N <0-3>     Set working pattern",
    "  P.L           Show pattern length",
    "  P.L <1-64>    Set pattern length",
    "  P.I           Show pattern index",
    "  P.I <0-63>    Set pattern index",
    "  P.HERE        Get value at index",
    "  P.NEXT        Advance index, return value",
    "  P.PREV        Reverse index, return value",
    "  P <idx>       Get value at index",
    "  P <idx> <val> Set value at index",
    "",
    "  PATTERNS (Explicit Pattern)",
    "  PN.L <pat>           Get pattern length",
    "  PN.L <pat> <len>     Set pattern length",
    "  PN.I <pat>           Get pattern index",
    "  PN.I <pat> <idx>     Set pattern index",
    "  PN.HERE <pat>        Get value at index",
    "  PN.NEXT <pat>        Advance index, return value",
    "  PN.PREV <pat>        Reverse index, return value",
    "  PN <pat> <idx>       Get value at index",
    "  PN <pat> <idx> <val> Set value at index",
    "",
];

fn render_help_page(scroll: usize, height: usize) -> Paragraph<'static> {
    let visible = if height > 2 { height - 2 } else { 1 };
    let total = HELP_LINES.len();
    let start = scroll.min(total.saturating_sub(visible));

    let lines: Vec<Line> = HELP_LINES
        .iter()
        .skip(start)
        .take(visible)
        .map(|&s| {
            if s.starts_with("  ") && s.chars().nth(2).map_or(false, |c| c.is_uppercase()) && !s.contains('<') && !s.contains("0-") {
                Line::from(Span::styled(s, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            } else {
                Line::from(s)
            }
        })
        .collect();

    let title = if total > visible {
        format!(" Help ({}/{}) ", start + 1, total.saturating_sub(visible) + 1)
    } else {
        " Help ".to_string()
    };

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
}

fn render_footer(app: &App) -> Paragraph<'static> {
    let input = &app.input;
    let pos = app.cursor_position;

    let before: String = input.chars().take(pos).collect();
    let cursor_char = input.chars().nth(pos).unwrap_or(' ');
    let after: String = input.chars().skip(pos + 1).collect();

    let input_line = Line::from(vec![
        Span::raw("> "),
        Span::raw(before),
        Span::styled(
            cursor_char.to_string(),
            Style::default().bg(Color::White).fg(Color::Black),
        ),
        Span::raw(after),
    ]);

    let footer_text = vec![
        input_line,
        Line::from(Span::styled(
            "[ ] pages  Alt+H help  q quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL))
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    metro_event_rx: mpsc::Receiver<MetroEvent>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        while let Ok(event) = metro_event_rx.try_recv() {
            match event {
                MetroEvent::ExecuteScript(index) => {
                    app.execute_script(index);
                }
            }
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let is_help = app.current_page == Page::Help;
                let has_alt = key.modifiers.contains(KeyModifiers::ALT);

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if !has_alt => {
                        return Ok(());
                    }
                    KeyCode::Char('[') => {
                        app.prev_page();
                    }
                    KeyCode::Char(']') => {
                        app.next_page();
                    }
                    // Alt+key page hotkeys
                    KeyCode::Char('h') if has_alt => {
                        app.toggle_help();
                    }
                    KeyCode::Char('l') if has_alt => {
                        app.go_to_page(Page::Live);
                    }
                    KeyCode::Char('m') if has_alt => {
                        app.go_to_page(Page::Metro);
                    }
                    KeyCode::Char('i') if has_alt => {
                        app.go_to_page(Page::Init);
                    }
                    KeyCode::Char('p') if has_alt => {
                        app.go_to_page(Page::Pattern);
                    }
                    KeyCode::Char('1') if has_alt => {
                        app.go_to_page(Page::Script1);
                    }
                    KeyCode::Char('2') if has_alt => {
                        app.go_to_page(Page::Script2);
                    }
                    KeyCode::Char('3') if has_alt => {
                        app.go_to_page(Page::Script3);
                    }
                    KeyCode::Char('4') if has_alt => {
                        app.go_to_page(Page::Script4);
                    }
                    KeyCode::Char('5') if has_alt => {
                        app.go_to_page(Page::Script5);
                    }
                    KeyCode::Char('6') if has_alt => {
                        app.go_to_page(Page::Script6);
                    }
                    KeyCode::Char('7') if has_alt => {
                        app.go_to_page(Page::Script7);
                    }
                    KeyCode::Char('8') if has_alt => {
                        app.go_to_page(Page::Script8);
                    }
                    // Help page scrolling
                    KeyCode::Up if is_help => {
                        app.help_scroll = app.help_scroll.saturating_sub(1);
                    }
                    KeyCode::Down if is_help => {
                        app.help_scroll = app.help_scroll.saturating_add(1).min(HELP_LINES.len().saturating_sub(1));
                    }
                    // Pattern page grid navigation
                    KeyCode::Up if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.1 > 0 {
                            app.pattern_cursor.1 -= 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Down if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.1 < 63 {
                            app.pattern_cursor.1 += 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Left if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.0 > 0 {
                            app.pattern_cursor.0 -= 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Right if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.0 < 3 {
                            app.pattern_cursor.0 += 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Char('-') if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_input.is_empty() {
                            app.pattern_input.push('-');
                        }
                    }
                    KeyCode::Char(c) if !is_help && app.current_page == Page::Pattern && c.is_ascii_digit() => {
                        app.pattern_input.push(c);
                    }
                    KeyCode::Backspace if !is_help && app.current_page == Page::Pattern => {
                        app.pattern_input.pop();
                    }
                    KeyCode::Esc if !is_help && app.current_page == Page::Pattern => {
                        app.pattern_input.clear();
                    }
                    KeyCode::Enter if !is_help && app.current_page == Page::Pattern => {
                        if !app.pattern_input.is_empty() {
                            if let Ok(value) = app.pattern_input.parse::<i16>() {
                                let (pattern_idx, step_idx) = app.pattern_cursor;
                                app.patterns.patterns[pattern_idx].data[step_idx] = value;
                                app.pattern_input.clear();
                            }
                        }
                    }
                    // Script page line navigation
                    KeyCode::Up if !is_help && app.is_script_page() => {
                        app.select_line_up();
                    }
                    KeyCode::Down if !is_help && app.is_script_page() => {
                        app.select_line_down();
                    }
                    KeyCode::Enter if !is_help && app.is_script_page() => {
                        app.save_line();
                    }
                    // Normal input handling (non-Help, non-Script, non-Pattern pages)
                    KeyCode::Enter if !is_help && app.current_page != Page::Pattern => {
                        app.execute_command();
                    }
                    KeyCode::Backspace if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.clear_input();
                    }
                    KeyCode::Delete if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.clear_input();
                    }
                    KeyCode::Char('u') if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_to_start();
                    }
                    KeyCode::Char(c) if !is_help && app.current_page != Page::Pattern => {
                        app.insert_char(c);
                    }
                    KeyCode::Backspace if !is_help && app.current_page != Page::Pattern => {
                        app.delete_char();
                    }
                    KeyCode::Left if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_right();
                    }
                    KeyCode::Up if !is_help && app.current_page != Page::Pattern => {
                        app.history_prev();
                    }
                    KeyCode::Down if !is_help && app.current_page != Page::Pattern => {
                        app.history_next();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let metro_state = Arc::new(Mutex::new(MetroState::default()));
    let (metro_tx, metro_rx) = mpsc::channel();
    let (metro_event_tx, metro_event_rx) = mpsc::channel::<MetroEvent>();

    let metro_state_clone = metro_state.clone();
    thread::spawn(move || {
        metro_thread(metro_rx, metro_state_clone, metro_event_tx);
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(metro_tx, metro_state);
    app.add_output("MONOKIT - Teletype-style scripting for complex oscillator".to_string());
    app.add_output("Type commands and press Enter. Use [ ] to navigate pages.".to_string());

    app.execute_script(9);

    let res = run_app(&mut terminal, &mut app, metro_event_rx);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}
