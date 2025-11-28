use crate::commands::process_command;
use crate::eval::{eval_condition, eval_expression};
use crate::types::{
    MetroCommand, MetroState, Page, Pattern, PatternStorage, Script, ScriptStorage, Variables,
    OSC_ADDR,
};
use anyhow::Result;
use rand::Rng;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::fs::OpenOptions;
use std::io::Write;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct App {
    pub current_page: Page,
    pub previous_page: Page,
    pub input: String,
    pub cursor_position: usize,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub output: Vec<String>,
    pub help_scroll: usize,
    pub metro_state: Arc<Mutex<MetroState>>,
    pub metro_tx: Sender<MetroCommand>,
    pub scripts: ScriptStorage,
    pub selected_line: Option<usize>,
    pub variables: Variables,
    pub patterns: PatternStorage,
    pub pattern_cursor: (usize, usize),
    pub pattern_input: String,
    pub ev_counters: [[u32; 8]; 10],
    pub if_else_condition: bool,
}

impl App {
    pub fn new(metro_tx: Sender<MetroCommand>, metro_state: Arc<Mutex<MetroState>>) -> Self {
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
            ev_counters: [[0; 8]; 10],
            if_else_condition: true,
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

    pub fn execute_script_with_depth(&mut self, script_index: usize, depth: usize) {
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

        if let Ok(mut f) = std::fs::OpenOptions::new()
            .append(true)
            .open("/tmp/monokit_debug.txt")
        {
            writeln!(f, "--- Executing ---").ok();
            for (i, l) in lines.iter().enumerate() {
                writeln!(
                    f,
                    "  Line {}: empty={} content='{}'",
                    i + 1,
                    l.is_empty(),
                    l
                )
                .ok();
            }
        }

        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Ok(mut f) = std::fs::OpenOptions::new()
                .append(true)
                .open("/tmp/monokit_debug.txt")
            {
                writeln!(f, "Processing line {}: '{}'", line_num + 1, line).ok();
            }

            if line.to_uppercase().starts_with("L ") {
                if let Some(colon_pos) = line.find(':') {
                    let loop_part = &line[2..colon_pos].trim();
                    let parts: Vec<&str> = loop_part.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let start = if let Some((val, _)) = eval_expression(
                            &parts,
                            0,
                            &self.variables,
                            &mut self.patterns,
                            &self.scripts,
                            script_index,
                        ) {
                            val
                        } else {
                            self.add_output(format!("Error: Failed to evaluate loop start value"));
                            continue;
                        };
                        let end = if let Some((val, _)) = eval_expression(
                            &parts,
                            1,
                            &self.variables,
                            &mut self.patterns,
                            &self.scripts,
                            script_index,
                        ) {
                            val
                        } else {
                            self.add_output(format!("Error: Failed to evaluate loop end value"));
                            continue;
                        };
                        let commands = line[colon_pos + 1..].trim();

                        let old_i = self.variables.i;

                        if start <= end {
                            for i_val in start..=end {
                                self.variables.i = i_val;
                                for sub_cmd in commands.split(';') {
                                    let sub_cmd = sub_cmd.trim();
                                    if sub_cmd.is_empty() {
                                        continue;
                                    }

                                    if sub_cmd.to_uppercase().starts_with("ELIF ") {
                                        if let Some(colon_pos) = sub_cmd.find(':') {
                                            let elif_cond = sub_cmd[5..colon_pos].trim();
                                            let cmd_to_run = sub_cmd[colon_pos + 1..].trim();

                                            if !self.if_else_condition {
                                                if eval_condition(
                                                    elif_cond,
                                                    &self.variables,
                                                    &mut self.patterns,
                                                    &self.scripts,
                                                    script_index,
                                                ) {
                                                    self.if_else_condition = true;

                                                    let mut output_messages = Vec::new();
                                                    let result = process_command(
                                                        &self.metro_tx,
                                                        &mut metro_interval,
                                                        &mut self.variables,
                                                        &mut self.patterns,
                                                        &mut self.scripts,
                                                        script_index,
                                                        cmd_to_run,
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
                                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                        if parts.len() >= 2 {
                                                            if let Some((val, _)) = eval_expression(
                                                                &parts,
                                                                1,
                                                                &self.variables,
                                                                &mut self.patterns,
                                                                &self.scripts,
                                                                script_index,
                                                            ) {
                                                                state.active = val != 0;
                                                            }
                                                        }
                                                    }
                                                    if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                        if parts.len() >= 2 {
                                                            if let Some((idx, _)) = eval_expression(
                                                                &parts,
                                                                1,
                                                                &self.variables,
                                                                &mut self.patterns,
                                                                &self.scripts,
                                                                script_index,
                                                            ) {
                                                                let idx = idx as usize;
                                                                if idx >= 1 && idx <= 8 {
                                                                    state.script_index = idx - 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            continue;
                                        }
                                    }

                                    if sub_cmd.to_uppercase().starts_with("ELSE:") {
                                        let cmd_to_run = sub_cmd[5..].trim();

                                        if !self.if_else_condition {
                                            self.if_else_condition = true;

                                            let mut output_messages = Vec::new();
                                            let result = process_command(
                                                &self.metro_tx,
                                                &mut metro_interval,
                                                &mut self.variables,
                                                &mut self.patterns,
                                                &mut self.scripts,
                                                script_index,
                                                cmd_to_run,
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
                                                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                if parts.len() >= 2 {
                                                    if let Some((val, _)) = eval_expression(
                                                        &parts,
                                                        1,
                                                        &self.variables,
                                                        &mut self.patterns,
                                                        &self.scripts,
                                                        script_index,
                                                    ) {
                                                        state.active = val != 0;
                                                    }
                                                }
                                            }
                                            if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                if parts.len() >= 2 {
                                                    if let Some((idx, _)) = eval_expression(
                                                        &parts,
                                                        1,
                                                        &self.variables,
                                                        &mut self.patterns,
                                                        &self.scripts,
                                                        script_index,
                                                    ) {
                                                        let idx = idx as usize;
                                                        if idx >= 1 && idx <= 8 {
                                                            state.script_index = idx - 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        continue;
                                    }

                                    let (condition, cmd_to_run) =
                                        if let Some(colon_pos) = sub_cmd.find(':') {
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
                                        if cond.trim().to_uppercase().starts_with("IF ") {
                                            self.if_else_condition = false;
                                            if eval_condition(
                                                cond,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                script_index,
                                            ) {
                                                self.if_else_condition = true;
                                            } else {
                                                continue;
                                            }
                                        } else if !eval_condition(
                                            cond,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            script_index,
                                        ) {
                                            continue;
                                        }
                                    }

                                    let mut output_messages = Vec::new();
                                    let result = process_command(
                                        &self.metro_tx,
                                        &mut metro_interval,
                                        &mut self.variables,
                                        &mut self.patterns,
                                        &mut self.scripts,
                                        script_index,
                                        cmd_to_run,
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
                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                        if parts.len() >= 2 {
                                            if let Some((val, _)) = eval_expression(
                                                &parts,
                                                1,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                script_index,
                                            ) {
                                                state.active = val != 0;
                                            }
                                        }
                                    }
                                    if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                        if parts.len() >= 2 {
                                            if let Some((idx, _)) = eval_expression(
                                                &parts,
                                                1,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                script_index,
                                            ) {
                                                let idx = idx as usize;
                                                if idx >= 1 && idx <= 8 {
                                                    state.script_index = idx - 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            for i_val in (end..=start).rev() {
                                self.variables.i = i_val;
                                for sub_cmd in commands.split(';') {
                                    let sub_cmd = sub_cmd.trim();
                                    if sub_cmd.is_empty() {
                                        continue;
                                    }

                                    if sub_cmd.to_uppercase().starts_with("ELIF ") {
                                        if let Some(colon_pos) = sub_cmd.find(':') {
                                            let elif_cond = sub_cmd[5..colon_pos].trim();
                                            let cmd_to_run = sub_cmd[colon_pos + 1..].trim();

                                            if !self.if_else_condition {
                                                if eval_condition(
                                                    elif_cond,
                                                    &self.variables,
                                                    &mut self.patterns,
                                                    &self.scripts,
                                                    script_index,
                                                ) {
                                                    self.if_else_condition = true;

                                                    let mut output_messages = Vec::new();
                                                    let result = process_command(
                                                        &self.metro_tx,
                                                        &mut metro_interval,
                                                        &mut self.variables,
                                                        &mut self.patterns,
                                                        &mut self.scripts,
                                                        script_index,
                                                        cmd_to_run,
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
                                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                        if parts.len() >= 2 {
                                                            if let Some((val, _)) = eval_expression(
                                                                &parts,
                                                                1,
                                                                &self.variables,
                                                                &mut self.patterns,
                                                                &self.scripts,
                                                                script_index,
                                                            ) {
                                                                state.active = val != 0;
                                                            }
                                                        }
                                                    }
                                                    if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                        if parts.len() >= 2 {
                                                            if let Some((idx, _)) = eval_expression(
                                                                &parts,
                                                                1,
                                                                &self.variables,
                                                                &mut self.patterns,
                                                                &self.scripts,
                                                                script_index,
                                                            ) {
                                                                let idx = idx as usize;
                                                                if idx >= 1 && idx <= 8 {
                                                                    state.script_index = idx - 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            continue;
                                        }
                                    }

                                    if sub_cmd.to_uppercase().starts_with("ELSE:") {
                                        let cmd_to_run = sub_cmd[5..].trim();

                                        if !self.if_else_condition {
                                            self.if_else_condition = true;

                                            let mut output_messages = Vec::new();
                                            let result = process_command(
                                                &self.metro_tx,
                                                &mut metro_interval,
                                                &mut self.variables,
                                                &mut self.patterns,
                                                &mut self.scripts,
                                                script_index,
                                                cmd_to_run,
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
                                                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                if parts.len() >= 2 {
                                                    if let Some((val, _)) = eval_expression(
                                                        &parts,
                                                        1,
                                                        &self.variables,
                                                        &mut self.patterns,
                                                        &self.scripts,
                                                        script_index,
                                                    ) {
                                                        state.active = val != 0;
                                                    }
                                                }
                                            }
                                            if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                if parts.len() >= 2 {
                                                    if let Some((idx, _)) = eval_expression(
                                                        &parts,
                                                        1,
                                                        &self.variables,
                                                        &mut self.patterns,
                                                        &self.scripts,
                                                        script_index,
                                                    ) {
                                                        let idx = idx as usize;
                                                        if idx >= 1 && idx <= 8 {
                                                            state.script_index = idx - 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        continue;
                                    }

                                    let (condition, cmd_to_run) =
                                        if let Some(colon_pos) = sub_cmd.find(':') {
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
                                        if cond.trim().to_uppercase().starts_with("IF ") {
                                            self.if_else_condition = false;
                                            if eval_condition(
                                                cond,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                script_index,
                                            ) {
                                                self.if_else_condition = true;
                                            } else {
                                                continue;
                                            }
                                        } else if !eval_condition(
                                            cond,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            script_index,
                                        ) {
                                            continue;
                                        }
                                    }

                                    let mut output_messages = Vec::new();
                                    let result = process_command(
                                        &self.metro_tx,
                                        &mut metro_interval,
                                        &mut self.variables,
                                        &mut self.patterns,
                                        &mut self.scripts,
                                        script_index,
                                        cmd_to_run,
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
                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                        if parts.len() >= 2 {
                                            if let Some((val, _)) = eval_expression(
                                                &parts,
                                                1,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                script_index,
                                            ) {
                                                state.active = val != 0;
                                            }
                                        }
                                    }
                                    if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                        if parts.len() >= 2 {
                                            if let Some((idx, _)) = eval_expression(
                                                &parts,
                                                1,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                script_index,
                                            ) {
                                                let idx = idx as usize;
                                                if idx >= 1 && idx <= 8 {
                                                    state.script_index = idx - 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        self.variables.i = old_i;
                        continue;
                    }
                }
            }

            let line_to_process = if line.to_uppercase().starts_with("EV ") {
                if let Some(colon_pos) = line.find(':') {
                    let ev_part = &line[3..colon_pos].trim();
                    let parts: Vec<&str> = ev_part.split_whitespace().collect();
                    if let Some((divisor, _)) = eval_expression(
                        &parts,
                        0,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        script_index,
                    ) {
                        if divisor > 0 {
                            let divisor = divisor as u32;
                            self.ev_counters[script_index][line_num] += 1;
                            if self.ev_counters[script_index][line_num] % divisor != 0 {
                                continue;
                            }
                            line[colon_pos + 1..].trim()
                        } else {
                            line
                        }
                    } else {
                        line
                    }
                } else {
                    line
                }
            } else if line.to_uppercase().starts_with("SKIP ") {
                if let Some(colon_pos) = line.find(':') {
                    let skip_part = &line[5..colon_pos].trim();
                    let parts: Vec<&str> = skip_part.split_whitespace().collect();
                    if let Some((divisor, _)) = eval_expression(
                        &parts,
                        0,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        script_index,
                    ) {
                        if divisor > 0 {
                            let divisor = divisor as u32;
                            self.ev_counters[script_index][line_num] += 1;
                            if self.ev_counters[script_index][line_num] % divisor == 0 {
                                continue;
                            }
                            line[colon_pos + 1..].trim()
                        } else {
                            line
                        }
                    } else {
                        line
                    }
                } else {
                    line
                }
            } else {
                line
            };

            for sub_cmd in line_to_process.split(';') {
                let sub_cmd = sub_cmd.trim();
                if sub_cmd.is_empty() {
                    continue;
                }

                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .append(true)
                    .open("/tmp/monokit_debug.txt")
                {
                    writeln!(f, "  sub_cmd: '{}'", sub_cmd).ok();
                }

                if sub_cmd.to_uppercase().starts_with("ELIF ") {
                    if let Some(colon_pos) = sub_cmd.find(':') {
                        let elif_cond = sub_cmd[5..colon_pos].trim();
                        let cmd_to_run = sub_cmd[colon_pos + 1..].trim();

                        if !self.if_else_condition {
                            if eval_condition(
                                elif_cond,
                                &self.variables,
                                &mut self.patterns,
                                &self.scripts,
                                script_index,
                            ) {
                                self.if_else_condition = true;

                                let mut output_messages = Vec::new();
                                let result = process_command(
                                    &self.metro_tx,
                                    &mut metro_interval,
                                    &mut self.variables,
                                    &mut self.patterns,
                                    &mut self.scripts,
                                    script_index,
                                    cmd_to_run,
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
                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Some((val, _)) = eval_expression(
                                            &parts,
                                            1,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            script_index,
                                        ) {
                                            state.active = val != 0;
                                        }
                                    }
                                }
                                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Some((idx, _)) = eval_expression(
                                            &parts,
                                            1,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            script_index,
                                        ) {
                                            let idx = idx as usize;
                                            if idx >= 1 && idx <= 8 {
                                                state.script_index = idx - 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        continue;
                    }
                }

                if sub_cmd.to_uppercase().starts_with("ELSE:") {
                    let cmd_to_run = sub_cmd[5..].trim();

                    if !self.if_else_condition {
                        self.if_else_condition = true;

                        let mut output_messages = Vec::new();
                        let result = process_command(
                            &self.metro_tx,
                            &mut metro_interval,
                            &mut self.variables,
                            &mut self.patterns,
                            &mut self.scripts,
                            script_index,
                            cmd_to_run,
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
                            let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                            if parts.len() >= 2 {
                                if let Some((val, _)) = eval_expression(
                                    &parts,
                                    1,
                                    &self.variables,
                                    &mut self.patterns,
                                    &self.scripts,
                                    script_index,
                                ) {
                                    state.active = val != 0;
                                }
                            }
                        }
                        if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                            let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                            if parts.len() >= 2 {
                                if let Some((idx, _)) = eval_expression(
                                    &parts,
                                    1,
                                    &self.variables,
                                    &mut self.patterns,
                                    &self.scripts,
                                    script_index,
                                ) {
                                    let idx = idx as usize;
                                    if idx >= 1 && idx <= 8 {
                                        state.script_index = idx - 1;
                                    }
                                }
                            }
                        }
                    }
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
                    if cond.trim().to_uppercase().starts_with("IF ") {
                        self.if_else_condition = false;
                        if eval_condition(
                            cond,
                            &self.variables,
                            &mut self.patterns,
                            &self.scripts,
                            script_index,
                        ) {
                            self.if_else_condition = true;
                        } else {
                            continue;
                        }
                    } else if !eval_condition(
                        cond,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        script_index,
                    ) {
                        continue;
                    }
                }

                let mut output_messages = Vec::new();
                let result = process_command(
                    &self.metro_tx,
                    &mut metro_interval,
                    &mut self.variables,
                    &mut self.patterns,
                    &mut self.scripts,
                    script_index,
                    cmd_to_run,
                    |msg| {
                        output_messages.push(msg);
                    },
                );

                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .append(true)
                    .open("/tmp/monokit_debug.txt")
                {
                    writeln!(
                        f,
                        "  cmd='{}' result={:?} output={:?}",
                        cmd_to_run,
                        result.is_ok(),
                        output_messages
                    )
                    .ok();
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
                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Some((val, _)) = eval_expression(
                            &parts,
                            1,
                            &self.variables,
                            &mut self.patterns,
                            &self.scripts,
                            10,
                        ) {
                            state.active = val != 0;
                        }
                    }
                }
                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Some((idx, _)) = eval_expression(
                            &parts,
                            1,
                            &self.variables,
                            &mut self.patterns,
                            &self.scripts,
                            10,
                        ) {
                            let idx = idx as usize;
                            if idx >= 1 && idx <= 8 {
                                state.script_index = idx - 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn execute_command(&mut self) {
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

        if cmd.to_uppercase().starts_with("L ") {
            if let Some(colon_pos) = cmd.find(':') {
                let loop_part = &cmd[2..colon_pos].trim();
                let parts: Vec<&str> = loop_part.split_whitespace().collect();
                if parts.len() >= 2 {
                    let start = if let Some((val, _)) = eval_expression(
                        &parts,
                        0,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        10,
                    ) {
                        val
                    } else {
                        self.add_output(format!("Error: Failed to evaluate loop start value"));
                        return;
                    };
                    let end = if let Some((val, _)) = eval_expression(
                        &parts,
                        1,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        10,
                    ) {
                        val
                    } else {
                        self.add_output(format!("Error: Failed to evaluate loop end value"));
                        return;
                    };
                    let commands = cmd[colon_pos + 1..].trim();

                    let old_i = self.variables.i;

                    if start <= end {
                        for i_val in start..=end {
                            self.variables.i = i_val;
                            for sub_cmd in commands.split(';') {
                                let sub_cmd = sub_cmd.trim();
                                if sub_cmd.is_empty() {
                                    continue;
                                }

                                if sub_cmd.to_uppercase().starts_with("ELIF ") {
                                    if let Some(colon_pos) = sub_cmd.find(':') {
                                        let elif_cond = sub_cmd[5..colon_pos].trim();
                                        let cmd_to_run = sub_cmd[colon_pos + 1..].trim();

                                        if !self.if_else_condition {
                                            if eval_condition(
                                                elif_cond,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                10,
                                            ) {
                                                self.if_else_condition = true;

                                                let mut output_messages = Vec::new();
                                                let result = process_command(
                                                    &self.metro_tx,
                                                    &mut metro_interval,
                                                    &mut self.variables,
                                                    &mut self.patterns,
                                                    &mut self.scripts,
                                                    10,
                                                    cmd_to_run,
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

                                                let mut state = self.metro_state.lock().unwrap();
                                                state.interval_ms = metro_interval;
                                                if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                    if parts.len() >= 2 {
                                                        if let Some((val, _)) = eval_expression(
                                                            &parts,
                                                            1,
                                                            &self.variables,
                                                            &mut self.patterns,
                                                            &self.scripts,
                                                            10,
                                                        ) {
                                                            state.active = val != 0;
                                                        }
                                                    }
                                                }
                                                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                    if parts.len() >= 2 {
                                                        if let Some((idx, _)) = eval_expression(
                                                            &parts,
                                                            1,
                                                            &self.variables,
                                                            &mut self.patterns,
                                                            &self.scripts,
                                                            10,
                                                        ) {
                                                            let idx = idx as usize;
                                                            if idx >= 1 && idx <= 8 {
                                                                state.script_index = idx - 1;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        continue;
                                    }
                                }

                                if sub_cmd.to_uppercase().starts_with("ELSE:") {
                                    let cmd_to_run = sub_cmd[5..].trim();

                                    if !self.if_else_condition {
                                        self.if_else_condition = true;

                                        let mut output_messages = Vec::new();
                                        let result = process_command(
                                            &self.metro_tx,
                                            &mut metro_interval,
                                            &mut self.variables,
                                            &mut self.patterns,
                                            &mut self.scripts,
                                            10,
                                            cmd_to_run,
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

                                        let mut state = self.metro_state.lock().unwrap();
                                        state.interval_ms = metro_interval;
                                        if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                            let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                            if parts.len() >= 2 {
                                                if let Some((val, _)) = eval_expression(
                                                    &parts,
                                                    1,
                                                    &self.variables,
                                                    &mut self.patterns,
                                                    &self.scripts,
                                                    10,
                                                ) {
                                                    state.active = val != 0;
                                                }
                                            }
                                        }
                                        if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                            let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                            if parts.len() >= 2 {
                                                if let Some((idx, _)) = eval_expression(
                                                    &parts,
                                                    1,
                                                    &self.variables,
                                                    &mut self.patterns,
                                                    &self.scripts,
                                                    10,
                                                ) {
                                                    let idx = idx as usize;
                                                    if idx >= 1 && idx <= 8 {
                                                        state.script_index = idx - 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    continue;
                                }

                                let (condition, cmd_to_run) =
                                    if let Some(colon_pos) = sub_cmd.find(':') {
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
                                    if cond.trim().to_uppercase().starts_with("IF ") {
                                        self.if_else_condition = false;
                                        if eval_condition(
                                            cond,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            10,
                                        ) {
                                            self.if_else_condition = true;
                                        } else {
                                            continue;
                                        }
                                    } else if !eval_condition(
                                        cond,
                                        &self.variables,
                                        &mut self.patterns,
                                        &self.scripts,
                                        10,
                                    ) {
                                        continue;
                                    }
                                }

                                let mut output_messages = Vec::new();
                                let result = process_command(
                                    &self.metro_tx,
                                    &mut metro_interval,
                                    &mut self.variables,
                                    &mut self.patterns,
                                    &mut self.scripts,
                                    10,
                                    cmd_to_run,
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

                                let mut state = self.metro_state.lock().unwrap();
                                state.interval_ms = metro_interval;
                                if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Some((val, _)) = eval_expression(
                                            &parts,
                                            1,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            10,
                                        ) {
                                            state.active = val != 0;
                                        }
                                    }
                                }
                                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Some((idx, _)) = eval_expression(
                                            &parts,
                                            1,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            10,
                                        ) {
                                            let idx = idx as usize;
                                            if idx >= 1 && idx <= 8 {
                                                state.script_index = idx - 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        for i_val in (end..=start).rev() {
                            self.variables.i = i_val;
                            for sub_cmd in commands.split(';') {
                                let sub_cmd = sub_cmd.trim();
                                if sub_cmd.is_empty() {
                                    continue;
                                }

                                if sub_cmd.to_uppercase().starts_with("ELIF ") {
                                    if let Some(colon_pos) = sub_cmd.find(':') {
                                        let elif_cond = sub_cmd[5..colon_pos].trim();
                                        let cmd_to_run = sub_cmd[colon_pos + 1..].trim();

                                        if !self.if_else_condition {
                                            if eval_condition(
                                                elif_cond,
                                                &self.variables,
                                                &mut self.patterns,
                                                &self.scripts,
                                                10,
                                            ) {
                                                self.if_else_condition = true;

                                                let mut output_messages = Vec::new();
                                                let result = process_command(
                                                    &self.metro_tx,
                                                    &mut metro_interval,
                                                    &mut self.variables,
                                                    &mut self.patterns,
                                                    &mut self.scripts,
                                                    10,
                                                    cmd_to_run,
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

                                                let mut state = self.metro_state.lock().unwrap();
                                                state.interval_ms = metro_interval;
                                                if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                    if parts.len() >= 2 {
                                                        if let Some((val, _)) = eval_expression(
                                                            &parts,
                                                            1,
                                                            &self.variables,
                                                            &mut self.patterns,
                                                            &self.scripts,
                                                            10,
                                                        ) {
                                                            state.active = val != 0;
                                                        }
                                                    }
                                                }
                                                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                                    if parts.len() >= 2 {
                                                        if let Some((idx, _)) = eval_expression(
                                                            &parts,
                                                            1,
                                                            &self.variables,
                                                            &mut self.patterns,
                                                            &self.scripts,
                                                            10,
                                                        ) {
                                                            let idx = idx as usize;
                                                            if idx >= 1 && idx <= 8 {
                                                                state.script_index = idx - 1;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        continue;
                                    }
                                }

                                if sub_cmd.to_uppercase().starts_with("ELSE:") {
                                    let cmd_to_run = sub_cmd[5..].trim();

                                    if !self.if_else_condition {
                                        self.if_else_condition = true;

                                        let mut output_messages = Vec::new();
                                        let result = process_command(
                                            &self.metro_tx,
                                            &mut metro_interval,
                                            &mut self.variables,
                                            &mut self.patterns,
                                            &mut self.scripts,
                                            10,
                                            cmd_to_run,
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

                                        let mut state = self.metro_state.lock().unwrap();
                                        state.interval_ms = metro_interval;
                                        if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                            let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                            if parts.len() >= 2 {
                                                if let Some((val, _)) = eval_expression(
                                                    &parts,
                                                    1,
                                                    &self.variables,
                                                    &mut self.patterns,
                                                    &self.scripts,
                                                    10,
                                                ) {
                                                    state.active = val != 0;
                                                }
                                            }
                                        }
                                        if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                            let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                            if parts.len() >= 2 {
                                                if let Some((idx, _)) = eval_expression(
                                                    &parts,
                                                    1,
                                                    &self.variables,
                                                    &mut self.patterns,
                                                    &self.scripts,
                                                    10,
                                                ) {
                                                    let idx = idx as usize;
                                                    if idx >= 1 && idx <= 8 {
                                                        state.script_index = idx - 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    continue;
                                }

                                let (condition, cmd_to_run) =
                                    if let Some(colon_pos) = sub_cmd.find(':') {
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
                                    if cond.trim().to_uppercase().starts_with("IF ") {
                                        self.if_else_condition = false;
                                        if eval_condition(
                                            cond,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            10,
                                        ) {
                                            self.if_else_condition = true;
                                        } else {
                                            continue;
                                        }
                                    } else if !eval_condition(
                                        cond,
                                        &self.variables,
                                        &mut self.patterns,
                                        &self.scripts,
                                        10,
                                    ) {
                                        continue;
                                    }
                                }

                                let mut output_messages = Vec::new();
                                let result = process_command(
                                    &self.metro_tx,
                                    &mut metro_interval,
                                    &mut self.variables,
                                    &mut self.patterns,
                                    &mut self.scripts,
                                    10,
                                    cmd_to_run,
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

                                let mut state = self.metro_state.lock().unwrap();
                                state.interval_ms = metro_interval;
                                if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Some((val, _)) = eval_expression(
                                            &parts,
                                            1,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            10,
                                        ) {
                                            state.active = val != 0;
                                        }
                                    }
                                }
                                if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                    let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Some((idx, _)) = eval_expression(
                                            &parts,
                                            1,
                                            &self.variables,
                                            &mut self.patterns,
                                            &self.scripts,
                                            10,
                                        ) {
                                            let idx = idx as usize;
                                            if idx >= 1 && idx <= 8 {
                                                state.script_index = idx - 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    self.variables.i = old_i;
                    return;
                }
            }
        }

        let cmd_to_process = if cmd.to_uppercase().starts_with("EV ") {
            if let Some(colon_pos) = cmd.find(':') {
                let ev_part = &cmd[3..colon_pos].trim();
                let parts: Vec<&str> = ev_part.split_whitespace().collect();
                if let Some((divisor, _)) = eval_expression(
                    &parts,
                    0,
                    &self.variables,
                    &mut self.patterns,
                    &self.scripts,
                    10,
                ) {
                    if divisor > 0 {
                        self.add_output("Warning: EV in interactive mode - counters not persisted".to_string());
                    }
                }
                cmd[colon_pos + 1..].trim().to_string()
            } else {
                cmd.clone()
            }
        } else if cmd.to_uppercase().starts_with("SKIP ") {
            if let Some(colon_pos) = cmd.find(':') {
                let skip_part = &cmd[5..colon_pos].trim();
                let parts: Vec<&str> = skip_part.split_whitespace().collect();
                if let Some((divisor, _)) = eval_expression(
                    &parts,
                    0,
                    &self.variables,
                    &mut self.patterns,
                    &self.scripts,
                    10,
                ) {
                    if divisor > 0 {
                        self.add_output("Warning: SKIP in interactive mode - counters not persisted".to_string());
                    }
                }
                cmd[colon_pos + 1..].trim().to_string()
            } else {
                cmd.clone()
            }
        } else {
            cmd.clone()
        };

        for sub_cmd in cmd_to_process.split(';') {
            let sub_cmd = sub_cmd.trim();
            if sub_cmd.is_empty() {
                continue;
            }

            if sub_cmd.to_uppercase().starts_with("ELIF ") {
                if let Some(colon_pos) = sub_cmd.find(':') {
                    let elif_cond = sub_cmd[5..colon_pos].trim();
                    let cmd_to_run = sub_cmd[colon_pos + 1..].trim();

                    if !self.if_else_condition {
                        if eval_condition(
                            elif_cond,
                            &self.variables,
                            &mut self.patterns,
                            &self.scripts,
                            10,
                        ) {
                            self.if_else_condition = true;

                            let mut output_messages = Vec::new();
                            let result = process_command(
                                &self.metro_tx,
                                &mut metro_interval,
                                &mut self.variables,
                                &mut self.patterns,
                                &mut self.scripts,
                                10,
                                cmd_to_run,
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

                            let mut state = self.metro_state.lock().unwrap();
                            state.interval_ms = metro_interval;
                            if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                if parts.len() >= 2 {
                                    if let Some((val, _)) = eval_expression(
                                        &parts,
                                        1,
                                        &self.variables,
                                        &mut self.patterns,
                                        &self.scripts,
                                        10,
                                    ) {
                                        state.active = val != 0;
                                    }
                                }
                            }
                            if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                                if parts.len() >= 2 {
                                    if let Some((idx, _)) = eval_expression(
                                        &parts,
                                        1,
                                        &self.variables,
                                        &mut self.patterns,
                                        &self.scripts,
                                        10,
                                    ) {
                                        let idx = idx as usize;
                                        if idx >= 1 && idx <= 8 {
                                            state.script_index = idx - 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }
            }

            if sub_cmd.to_uppercase().starts_with("ELSE:") {
                let cmd_to_run = sub_cmd[5..].trim();

                if !self.if_else_condition {
                    self.if_else_condition = true;

                    let mut output_messages = Vec::new();
                    let result = process_command(
                        &self.metro_tx,
                        &mut metro_interval,
                        &mut self.variables,
                        &mut self.patterns,
                        &mut self.scripts,
                        10,
                        cmd_to_run,
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

                    let mut state = self.metro_state.lock().unwrap();
                    state.interval_ms = metro_interval;
                    if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Some((val, _)) = eval_expression(
                                &parts,
                                1,
                                &self.variables,
                                &mut self.patterns,
                                &self.scripts,
                                10,
                            ) {
                                state.active = val != 0;
                            }
                        }
                    }
                    if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Some((idx, _)) = eval_expression(
                                &parts,
                                1,
                                &self.variables,
                                &mut self.patterns,
                                &self.scripts,
                                10,
                            ) {
                                let idx = idx as usize;
                                if idx >= 1 && idx <= 8 {
                                    state.script_index = idx - 1;
                                }
                            }
                        }
                    }
                }
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
                if cond.trim().to_uppercase().starts_with("IF ") {
                    self.if_else_condition = false;
                    if eval_condition(
                        cond,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        10,
                    ) {
                        self.if_else_condition = true;
                    } else {
                        continue;
                    }
                } else if !eval_condition(
                    cond,
                    &self.variables,
                    &mut self.patterns,
                    &self.scripts,
                    10,
                ) {
                    continue;
                }
            }

            let mut output_messages = Vec::new();
            let result = process_command(
                &self.metro_tx,
                &mut metro_interval,
                &mut self.variables,
                &mut self.patterns,
                &mut self.scripts,
                10,
                cmd_to_run,
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

            let mut state = self.metro_state.lock().unwrap();
            state.interval_ms = metro_interval;
            if cmd_to_run.to_uppercase().starts_with("M.ACT") {
                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some((val, _)) = eval_expression(
                        &parts,
                        1,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        10,
                    ) {
                        state.active = val != 0;
                    }
                }
            }
            if cmd_to_run.to_uppercase().starts_with("M.SCRIPT") {
                let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some((idx, _)) = eval_expression(
                        &parts,
                        1,
                        &self.variables,
                        &mut self.patterns,
                        &self.scripts,
                        10,
                    ) {
                        let idx = idx as usize;
                        if idx >= 1 && idx <= 8 {
                            state.script_index = idx - 1;
                        }
                    }
                }
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let byte_pos = self
            .input
            .char_indices()
            .nth(self.cursor_position)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(byte_pos, c);
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let byte_pos = self
                .input
                .char_indices()
                .nth(self.cursor_position - 1)
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(byte_pos);
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.chars().count() {
            self.cursor_position += 1;
        }
    }

    pub fn history_prev(&mut self) {
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

    pub fn history_next(&mut self) {
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

    pub fn delete_to_start(&mut self) {
        self.input = self.input.chars().skip(self.cursor_position).collect();
        self.cursor_position = 0;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn select_line_up(&mut self) {
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

    pub fn select_line_down(&mut self) {
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

    pub fn save_line(&mut self) {
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
