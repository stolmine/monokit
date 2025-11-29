use super::App;
use crate::commands::process_command;
use crate::eval::{eval_condition, eval_expression};
use std::io::Write;

impl App {
    fn debug_log(&self, msg: String) {
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .append(true)
            .open("/tmp/monokit_debug.txt")
        {
            writeln!(f, "{}", msg).ok();
        }
    }

    fn process_elif(
        &mut self,
        elif_cond: &str,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) {
        if !self.if_else_condition {
            if eval_condition(
                elif_cond,
                &self.variables,
                &mut self.patterns,
                &self.scripts,
                script_index,
            ) {
                self.if_else_condition = true;
                self.execute_and_update_metro(cmd_to_run, script_index, metro_interval, depth);
            }
        }
    }

    fn process_else(
        &mut self,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) {
        if !self.if_else_condition {
            self.if_else_condition = true;
            self.execute_and_update_metro(cmd_to_run, script_index, metro_interval, depth);
        }
    }

    fn process_conditional(
        &mut self,
        condition: Option<&str>,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) -> bool {
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
                    return false;
                }
            } else if !eval_condition(
                cond,
                &self.variables,
                &mut self.patterns,
                &self.scripts,
                script_index,
            ) {
                return false;
            }
        }

        self.execute_and_update_metro(cmd_to_run, script_index, metro_interval, depth);
        true
    }

    fn execute_and_update_metro(
        &mut self,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) {
        let mut output_messages = Vec::new();
        let result = process_command(
            &self.metro_tx,
            metro_interval,
            &mut self.br_len,
            &mut self.variables,
            &mut self.patterns,
            &mut self.scripts,
            script_index,
            &mut self.theme,
            &mut self.debug_level,
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
                    if let Some(d) = depth {
                        self.execute_script_with_depth(script_idx, d + 1);
                    } else {
                        self.execute_script(script_idx);
                    }
                }
            }
            Err(e) => {
                output_messages.push(format!("Error: {}", e));
                for msg in output_messages {
                    self.add_output(msg);
                }
            }
        }

        self.update_metro_state(cmd_to_run, script_index, *metro_interval);
    }

    fn update_metro_state(&mut self, cmd_to_run: &str, script_index: usize, metro_interval: u64) {
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

        // Drop the lock before modifying self
        drop(state);

        // Update recording state
        let upper_cmd = cmd_to_run.to_uppercase();
        if upper_cmd == "REC" {
            self.recording = true;
            self.recording_start = Some(std::time::Instant::now());
        } else if upper_cmd == "REC.STOP" {
            self.recording = false;
            self.recording_start = None;
        }
    }

    fn process_sub_command(
        &mut self,
        sub_cmd: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) {
        if sub_cmd.to_uppercase().starts_with("ELIF ") {
            if let Some(colon_pos) = sub_cmd.find(':') {
                let elif_cond = sub_cmd[5..colon_pos].trim();
                let cmd_to_run = sub_cmd[colon_pos + 1..].trim();
                self.process_elif(elif_cond, cmd_to_run, script_index, metro_interval, depth);
                return;
            }
        }

        if sub_cmd.to_uppercase().starts_with("ELSE:") {
            let cmd_to_run = sub_cmd[5..].trim();
            self.process_else(cmd_to_run, script_index, metro_interval, depth);
            return;
        }

        let (condition, cmd_to_run) = if let Some(colon_pos) = sub_cmd.find(':') {
            let cond = &sub_cmd[..colon_pos];
            let cmd = sub_cmd[colon_pos + 1..].trim();
            (Some(cond), cmd)
        } else {
            (None, sub_cmd)
        };

        if !cmd_to_run.is_empty() {
            self.process_conditional(condition, cmd_to_run, script_index, metro_interval, depth);
        }
    }

    fn process_loop_commands(
        &mut self,
        commands: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) {
        for sub_cmd in commands.split(';') {
            let sub_cmd = sub_cmd.trim();
            if sub_cmd.is_empty() {
                continue;
            }
            self.process_sub_command(sub_cmd, script_index, metro_interval, depth);
        }
    }

    fn execute_loop(
        &mut self,
        line: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) -> bool {
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
                    return true;
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
                    return true;
                };
                let commands = line[colon_pos + 1..].trim();

                let old_i = self.variables.i;

                if start <= end {
                    for i_val in start..=end {
                        self.variables.i = i_val;
                        self.process_loop_commands(commands, script_index, metro_interval, depth);
                    }
                } else {
                    for i_val in (end..=start).rev() {
                        self.variables.i = i_val;
                        self.process_loop_commands(commands, script_index, metro_interval, depth);
                    }
                }

                self.variables.i = old_i;
                return true;
            }
        }
        false
    }

    fn process_ev_skip_prefix<'a>(
        &mut self,
        line: &'a str,
        script_index: usize,
        line_num: usize,
    ) -> &'a str {
        if line.to_uppercase().starts_with("EV ") {
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
                            return "";
                        }
                        return line[colon_pos + 1..].trim();
                    }
                }
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
                            return "";
                        }
                        return line[colon_pos + 1..].trim();
                    }
                }
            }
        }
        line
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

        self.debug_log("--- Executing ---".to_string());
        for (i, l) in lines.iter().enumerate() {
            self.debug_log(format!(
                "  Line {}: empty={} content='{}'",
                i + 1,
                l.is_empty(),
                l
            ));
        }

        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            self.debug_log(format!("Processing line {}: '{}'", line_num + 1, line));

            if line.to_uppercase().starts_with("L ") {
                if self.execute_loop(line, script_index, &mut metro_interval, Some(depth)) {
                    continue;
                }
            }

            let line_to_process = self.process_ev_skip_prefix(line, script_index, line_num);
            if line_to_process.is_empty() {
                continue;
            }

            for sub_cmd in line_to_process.split(';') {
                let sub_cmd = sub_cmd.trim();
                if sub_cmd.is_empty() {
                    continue;
                }

                self.debug_log(format!("  sub_cmd: '{}'", sub_cmd));
                self.process_sub_command(sub_cmd, script_index, &mut metro_interval, Some(depth));
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

        // Handle quit commands
        let cmd_upper = cmd.to_uppercase();
        if cmd_upper == "Q" || cmd_upper == "QUIT" || cmd_upper == "EXIT" {
            // Stop recording if in progress
            if self.recording {
                let _ = self.metro_tx.send(crate::types::MetroCommand::StopRecording);
                self.recording = false;
                self.recording_start = None;
                self.add_output("RECORDING STOPPED (AUTO)".to_string());
            }
            self.should_quit = true;
            return;
        }

        // Handle clear command
        if cmd_upper == "CLEAR" {
            self.output.clear();
            return;
        }

        let mut metro_interval = {
            let state = self.metro_state.lock().unwrap();
            state.interval_ms
        };

        if cmd.to_uppercase().starts_with("L ") {
            if self.execute_loop(&cmd, 10, &mut metro_interval, None) {
                return;
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

            self.process_sub_command(sub_cmd, 10, &mut metro_interval, None);
        }
    }
}
