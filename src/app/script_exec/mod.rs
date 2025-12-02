mod control_flow;
mod interactive;
mod loops;

use super::App;
use crate::commands::process_command;
use crate::eval::eval_expression;
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

    fn execute_and_update_metro(
        &mut self,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
    ) {
        if cmd_to_run.trim().eq_ignore_ascii_case("TR") {
            self.trigger_activity = Some(std::time::Instant::now());
        }

        // Mark parameter activity
        let parts: Vec<&str> = cmd_to_run.split_whitespace().collect();
        if let Some(cmd) = parts.get(0) {
            self.param_activity.mark(cmd);
        }

        let mut output_messages = Vec::new();
        let result = process_command(
            &self.metro_tx,
            metro_interval,
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
            &mut self.limiter_enabled,
            &mut self.notes,
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
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
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
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
                ) {
                    let idx = idx as usize;
                    if idx >= 1 && idx <= 8 {
                        state.script_index = idx - 1;
                    }
                }
            }
        }

        drop(state);

        let upper_cmd = cmd_to_run.to_uppercase();
        if upper_cmd == "REC" {
            self.recording = true;
            self.recording_start = Some(std::time::Instant::now());
        } else if upper_cmd == "REC.STOP" {
            self.recording = false;
            self.recording_start = None;
        }
    }

    pub fn execute_script_with_depth(&mut self, script_index: usize, depth: usize) {
        if script_index >= 10 {
            self.add_output(format!("Error: Invalid script index {}", script_index));
            return;
        }

        // Record activity for all script executions, including nested calls
        self.script_activity[script_index] = Some(std::time::Instant::now());

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

}
