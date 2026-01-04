use super::super::App;
use crate::commands::registry::validate::validate_from_registry;
use crate::eval::eval_expression;
use crate::output::OutputDecider;

impl App {
    pub fn execute_command(&mut self) {
        let cmd = self.input.trim().to_string();
        if cmd.is_empty() {
            return;
        }

        self.history.push(cmd.clone());
        self.history_index = None;
        self.input.clear();
        self.cursor_position = 0;

        let cmd_upper = cmd.to_uppercase();
        if cmd_upper == "Q" || cmd_upper == "QUIT" || cmd_upper == "EXIT" {
            // Check if quit confirmation is needed
            let metro_active = {
                let state = self.metro_state.lock().unwrap();
                state.active
            };
            let has_named_scene = self.current_scene_name.is_some()
                && self.current_scene_name.as_ref().map(|s| s.as_str()) != Some("[unsaved]");
            let needs_confirmation = (self.confirm_quit_unsaved && self.scene_modified)
                || (has_named_scene && metro_active);

            if needs_confirmation && self.pending_confirmation.is_none() {
                self.pending_confirmation = Some(crate::types::ConfirmAction::Quit);
                return;
            }

            if self.recording {
                let _ = self.metro_tx.send(crate::types::MetroCommand::StopRecording);
                self.recording = false;
                self.recording_start = None;
                self.add_output("RECORDING STOPPED (AUTO)".to_string());
            }
            self.should_quit = true;
            return;
        }

        if cmd_upper == "CLEAR" || cmd_upper == "CLR" {
            self.output.clear();
            return;
        }

        if cmd_upper.starts_with("REPL.DUMP") {
            let parts: Vec<&str> = cmd_upper.split_whitespace().collect();
            let filename = if parts.len() > 1 {
                parts[1].to_lowercase()
            } else {
                "repl_dump.txt".to_string()
            };
            match std::fs::File::create(&filename) {
                Ok(mut file) => {
                    use std::io::Write;
                    for line in &self.output {
                        if let Err(e) = writeln!(file, "{}", line) {
                            if self.should_output(crate::types::OutputCategory::Error) {
                                self.add_output(format!("WRITE FAILED: {}", e.to_string().to_uppercase()));
                            }
                            return;
                        }
                    }
                    self.add_output(format!("DUMPED {} LINES TO {}", self.output.len() - 1, filename));
                }
                Err(e) => {
                    if self.should_output(crate::types::OutputCategory::Error) {
                        self.add_output(format!("FILE CREATE FAILED: {}", e.to_string().to_uppercase()));
                    }
                }
            }
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
                    &mut self.counters,
                    &self.scripts,
                    10,
                    &self.scale,
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
                    &mut self.counters,
                    &self.scripts,
                    10,
                    &self.scale,
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

        for sub_cmd in super::split_respecting_quotes(&cmd_to_process) {
            let sub_cmd = sub_cmd.trim();
            if sub_cmd.is_empty() {
                continue;
            }

            if let Err(e) = validate_from_registry(sub_cmd) {
                if self.should_output(crate::types::OutputCategory::Error) {
                    self.add_output(format!("ERROR: {}", e.to_string().to_uppercase()));
                }
                continue;
            }

            if sub_cmd.eq_ignore_ascii_case("TR") {
                self.trigger_activity = Some(std::time::Instant::now());
            } else if sub_cmd.eq_ignore_ascii_case("PLTR") {
                self.plaits_trigger_activity = Some(std::time::Instant::now());
            } else if sub_cmd.to_uppercase().starts_with("STR") {
                self.sampler_trigger_activity = Some(std::time::Instant::now());
            }

            // Mark parameter activity
            let parts: Vec<&str> = sub_cmd.split_whitespace().collect();
            if let Some(cmd) = parts.get(0) {
                self.param_activity.mark(cmd);
            }

            // Check if this is a LOAD command
            let is_load_cmd = parts.get(0).map(|c| c.eq_ignore_ascii_case("LOAD")).unwrap_or(false);

            // Interactive mode (script_index=10) doesn't need highlighting
            self.process_sub_command(sub_cmd, 10, &mut metro_interval, None, 0, 0);

            // Clear output and undo stacks if LOAD command
            if is_load_cmd {
                self.clear_all_undo_stacks();
                if self.load_clr {
                    self.output.clear();
                }
            }
        }
    }
}
