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
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::io;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const OSC_ADDR: &str = "127.0.0.1:57120";

#[derive(Debug, Clone)]
enum MetroCommand {
    SetInterval(u64),
    SetActive(bool),
    SetScript(Vec<ScriptCommand>),
    SendParam(String, OscType),
    SendTrigger,
    SendVolume(f32),
}

#[derive(Debug, Clone)]
struct ScriptCommand {
    param_name: String,
    value: OscType,
}

#[derive(Debug, Clone)]
struct MetroState {
    interval_ms: u64,
    active: bool,
    script: String,
}

impl Default for MetroState {
    fn default() -> Self {
        Self {
            interval_ms: 500,
            active: false,
            script: String::new(),
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
        }
    }

    fn go_to_page(&mut self, page: Page) {
        if page != Page::Help {
            self.previous_page = page;
        }
        self.current_page = page;
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
    }

    fn prev_page(&mut self) {
        self.current_page = self.current_page.prev();
    }

    fn add_output(&mut self, msg: String) {
        self.output.push(msg);
        if self.output.len() > 100 {
            self.output.remove(0);
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

        let mut output_messages = Vec::new();
        if let Err(e) = process_command(&self.metro_tx, &mut metro_interval, &cmd, |msg| {
            output_messages.push(msg);
        }) {
            output_messages.push(format!("Error: {}", e));
        }

        for msg in output_messages {
            self.add_output(msg);
        }

        let mut state = self.metro_state.lock().unwrap();
        state.interval_ms = metro_interval;
        if let Some(script_prefix) = cmd.strip_prefix("M: ").or_else(|| cmd.strip_prefix("m: ")) {
            state.script = script_prefix.trim().to_string();
        }
        if cmd.to_uppercase().starts_with("M.ACT") {
            if let Some(parts) = cmd.split_whitespace().nth(1) {
                if let Ok(val) = parts.parse::<i32>() {
                    state.active = val != 0;
                }
            }
        }
    }

    fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.input.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
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

fn metro_thread(rx: mpsc::Receiver<MetroCommand>, _state: Arc<Mutex<MetroState>>) {
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
    let mut script: Vec<ScriptCommand> = Vec::new();
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
                MetroCommand::SetScript(s) => {
                    script = s;
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
            let msg = OscMessage {
                addr: "/monokit/trigger".to_string(),
                args: vec![],
            };
            let packet = OscPacket::Message(msg);
            if let Ok(buf) = encoder::encode(&packet) {
                let _ = socket.send(&buf);
            }

            for cmd in &script {
                let msg = OscMessage {
                    addr: "/monokit/param".to_string(),
                    args: vec![
                        OscType::String(cmd.param_name.clone()),
                        cmd.value.clone(),
                    ],
                };
                let packet = OscPacket::Message(msg);
                if let Ok(buf) = encoder::encode(&packet) {
                    let _ = socket.send(&buf);
                }
            }

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

fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    input: &str,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(());
    }

    if let Some(script) = trimmed.strip_prefix("M: ").or_else(|| trimmed.strip_prefix("m: ")) {
        let script = script.trim().to_string();

        let mut script_commands = Vec::new();

        for cmd in script.split(';') {
            let cmd = cmd.trim();
            if cmd.is_empty() {
                continue;
            }
            if let Err(e) = validate_script_command(cmd) {
                output(format!("Error: Invalid command in script: {}", e));
                return Ok(());
            }

            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let command = parts[0].to_uppercase();

            if command == "TR" {
                continue;
            }

            let param_name = command.to_lowercase();

            if parts.len() > 1 {
                let value_str = parts[1];
                let value = if let Ok(int_val) = value_str.parse::<i32>() {
                    OscType::Int(int_val)
                } else if let Ok(float_val) = value_str.parse::<f32>() {
                    OscType::Float(float_val)
                } else {
                    output(format!("Error: Cannot parse value for {}", command));
                    return Ok(());
                };

                script_commands.push(ScriptCommand { param_name, value });
            }
        }

        metro_tx
            .send(MetroCommand::SetScript(script_commands))
            .context("Failed to send script to metro thread")?;
        output(format!("Set M script: {}", script));
        return Ok(());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "TR" => {
            metro_tx
                .send(MetroCommand::SendTrigger)
                .context("Failed to send trigger to metro thread")?;
            output("Sent trigger".to_string());
        }
        "VOL" => {
            if parts.len() < 2 {
                output("Error: VOL requires a value (0.0-1.0)".to_string());
                return Ok(());
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
                    return Ok(());
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
                return Ok(());
            }
            let bpm: f32 = parts[1]
                .parse()
                .context("Failed to parse BPM value as number")?;
            if bpm <= 0.0 {
                output("Error: BPM must be greater than 0".to_string());
                return Ok(());
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
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse M.ACT value")?;
            if !(0..=1).contains(&value) {
                output("Error: M.ACT value must be 0 or 1".to_string());
                return Ok(());
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
        "PF" => {
            if parts.len() < 2 {
                output("Error: PF requires a frequency value (20-20000)".to_string());
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse frequency value")?;
            if !(20.0..=20000.0).contains(&value) {
                output("Error: Frequency must be between 20 and 20000 Hz".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set primary frequency to {} Hz", value));
        }
        "PW" => {
            if parts.len() < 2 {
                output("Error: PW requires a waveform value (0-2)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse waveform value")?;
            if !(0..=2).contains(&value) {
                output("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set primary waveform to {}", value));
        }
        "MF" => {
            if parts.len() < 2 {
                output("Error: MF requires a frequency value (20-20000)".to_string());
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse frequency value")?;
            if !(20.0..=20000.0).contains(&value) {
                output("Error: Frequency must be between 20 and 20000 Hz".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod frequency to {} Hz", value));
        }
        "MW" => {
            if parts.len() < 2 {
                output("Error: MW requires a waveform value (0-2)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse waveform value")?;
            if !(0..=2).contains(&value) {
                output("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod waveform to {}", value));
        }
        "DC" => {
            if parts.len() < 2 {
                output("Error: DC requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse discontinuity amount")?;
            if !(0..=16383).contains(&value) {
                output("Error: Discontinuity amount must be between 0 and 16383".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("dc".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity amount to {}", value));
        }
        "DM" => {
            if parts.len() < 2 {
                output("Error: DM requires a mode value (0-2)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse discontinuity mode")?;
            if !(0..=2).contains(&value) {
                output("Error: Mode must be 0 (fold), 1 (tanh), or 2 (softclip)".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("dm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity mode to {}", value));
        }
        "TK" => {
            if parts.len() < 2 {
                output("Error: TK requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse tracking amount")?;
            if !(0..=16383).contains(&value) {
                output("Error: Tracking amount must be between 0 and 16383".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("tk".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set tracking amount to {}", value));
        }
        "MB" => {
            if parts.len() < 2 {
                output("Error: MB requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod bus amount")?;
            if !(0..=16383).contains(&value) {
                output("Error: Mod bus amount must be between 0 and 16383".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mb".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod bus amount to {}", value));
        }
        "MP" => {
            if parts.len() < 2 {
                output("Error: MP requires a value (0 or 1)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> primary value")?;
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mp".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> primary freq to {}", value));
        }
        "MD" => {
            if parts.len() < 2 {
                output("Error: MD requires a value (0 or 1)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> discontinuity value")?;
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("md".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> discontinuity to {}", value));
        }
        "MT" => {
            if parts.len() < 2 {
                output("Error: MT requires a value (0 or 1)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> tracking value")?;
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mt".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> tracking to {}", value));
        }
        "MA" => {
            if parts.len() < 2 {
                output("Error: MA requires a value (0 or 1)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> amplitude value")?;
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("ma".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod -> amplitude to {}", value));
        }
        "FM" => {
            if parts.len() < 2 {
                output("Error: FM requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse FM index")?;
            if !(0..=16383).contains(&value) {
                output("Error: FM index must be between 0 and 16383".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("fm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM index to {}", value));
        }
        "AD" => {
            if parts.len() < 2 {
                output("Error: AD requires a time value (1-10000 ms)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse amp decay time")?;
            if !(1..=10000).contains(&value) {
                output("Error: Amp decay must be between 1 and 10000 ms".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set amp decay to {} ms", value));
        }
        "PD" => {
            if parts.len() < 2 {
                output("Error: PD requires a time value (1-10000 ms)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse pitch decay time")?;
            if !(1..=10000).contains(&value) {
                output("Error: Pitch decay must be between 1 and 10000 ms".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set pitch decay to {} ms", value));
        }
        "FD" => {
            if parts.len() < 2 {
                output("Error: FD requires a time value (1-10000 ms)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse FM decay time")?;
            if !(1..=10000).contains(&value) {
                output("Error: FM decay must be between 1 and 10000 ms".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM decay to {} ms", value));
        }
        "PA" => {
            if parts.len() < 2 {
                output("Error: PA requires a multiplier value (0-16)".to_string());
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse pitch env amount")?;
            if !(0.0..=16.0).contains(&value) {
                output("Error: Pitch env amount must be between 0 and 16".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set pitch env amount to {}", value));
        }
        "DD" => {
            if parts.len() < 2 {
                output("Error: DD requires a time value (1-10000 ms)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse discontinuity decay time")?;
            if !(1..=10000).contains(&value) {
                output("Error: Discontinuity decay must be between 1 and 10000 ms".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("dd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set discontinuity decay to {} ms", value));
        }
        "MX" => {
            if parts.len() < 2 {
                output("Error: MX requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mix amount")?;
            if !(0..=16383).contains(&value) {
                output("Error: Mix amount must be between 0 and 16383".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mx".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mix amount to {}", value));
        }
        "MM" => {
            if parts.len() < 2 {
                output("Error: MM requires a value (0 or 1)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod bus -> mix value")?;
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set mod bus -> mix to {}", value));
        }
        "ME" => {
            if parts.len() < 2 {
                output("Error: ME requires a value (0 or 1)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse envelope -> mix value")?;
            if !(0..=1).contains(&value) {
                output("Error: Value must be 0 or 1".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("me".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set envelope -> mix to {}", value));
        }
        "FA" => {
            if parts.len() < 2 {
                output("Error: FA requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse FM envelope amount")?;
            if !(0..=16383).contains(&value) {
                output("Error: FM envelope amount must be between 0 and 16383".to_string());
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            output(format!("Set FM envelope amount to {}", value));
        }
        "DA" => {
            if parts.len() < 2 {
                output("Error: DA requires a value (0-16383)".to_string());
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse DC envelope amount")?;
            if !(0..=16383).contains(&value) {
                output("Error: DC envelope amount must be between 0 and 16383".to_string());
                return Ok(());
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
            output("M.ACT <0|1>   Start/stop".to_string());
            output("M: <script>   Set M script".to_string());
        }
        _ => {
            output(format!("Unknown command: {}", cmd));
        }
    }

    Ok(())
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

    let chunks = if is_help {
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
        Page::Script1 => render_script_page(1),
        Page::Script2 => render_script_page(2),
        Page::Script3 => render_script_page(3),
        Page::Script4 => render_script_page(4),
        Page::Script5 => render_script_page(5),
        Page::Script6 => render_script_page(6),
        Page::Script7 => render_script_page(7),
        Page::Script8 => render_script_page(8),
        Page::Metro => render_metro_page(app),
        Page::Init => render_init_page(),
        Page::Pattern => render_pattern_page(),
        Page::Help => render_help_page(app.help_scroll, chunks[1].height as usize),
    };
    f.render_widget(content, chunks[1]);

    if !is_help {
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
        Span::styled("  M Script: ", Style::default().fg(Color::Cyan)),
    ]));
    if state.script.is_empty() {
        text.push(Line::from(Span::styled(
            "    (none)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        text.push(Line::from(format!("    {}", state.script)));
    }

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Metro Status "))
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

fn render_script_page(num: u8) -> Paragraph<'static> {
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  (script storage not yet implemented)",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    for i in 1..=8 {
        lines.push(Line::from(Span::styled(
            format!("  {}: ", i),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(format!(" Script {} ", num)))
}

fn render_init_page() -> Paragraph<'static> {
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  (init script not yet implemented)",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    for i in 1..=8 {
        lines.push(Line::from(Span::styled(
            format!("  {}: ", i),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Init "))
}

fn render_pattern_page() -> Paragraph<'static> {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  (pattern storage not yet implemented)",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  4 patterns × 64 steps",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Pattern "))
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
    "  M.ACT <0|1>   Start/stop",
    "  M: <script>   Set M script (;-separated)",
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
    let input_display = format!("> {}", app.input);

    let footer_text = vec![
        Line::from(input_display),
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
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

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
                    // Normal input handling (non-Help pages)
                    KeyCode::Enter if !is_help => {
                        app.execute_command();
                    }
                    KeyCode::Char(c) if !is_help => {
                        app.insert_char(c);
                    }
                    KeyCode::Backspace if !is_help => {
                        app.delete_char();
                    }
                    KeyCode::Left if !is_help => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right if !is_help => {
                        app.move_cursor_right();
                    }
                    KeyCode::Up if !is_help => {
                        app.history_prev();
                    }
                    KeyCode::Down if !is_help => {
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

    let metro_state_clone = metro_state.clone();
    thread::spawn(move || {
        metro_thread(metro_rx, metro_state_clone);
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(metro_tx, metro_state);
    app.add_output("MONOKIT - Teletype-style scripting for complex oscillator".to_string());
    app.add_output("Type commands and press Enter. Use [ ] to navigate pages.".to_string());

    let res = run_app(&mut terminal, &mut app);

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
