mod control_flow;
mod interactive;
mod loops;

use super::App;
use crate::commands::process_command;
use crate::eval::eval_expression;
use crate::utils::{split_respecting_quotes, split_whitespace_respecting_quotes};
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
        let trimmed = cmd_to_run.trim();
        if trimmed.eq_ignore_ascii_case("TR") {
            self.trigger_activity = Some(std::time::Instant::now());
        } else if trimmed.eq_ignore_ascii_case("PLTR") {
            self.plaits_trigger_activity = Some(std::time::Instant::now());
        }

        // Mark parameter activity
        let parts_owned = split_whitespace_respecting_quotes(cmd_to_run);
        if let Some(cmd) = parts_owned.get(0) {
            self.param_activity.mark(cmd);
        }

        let mut output_messages = Vec::new();

        // Construct ExecutionContext
        let mut ctx = crate::commands::context::ExecutionContext {
            metro_tx: &self.metro_tx,
            metro_interval,
            variables: &mut self.variables,
            patterns: &mut self.patterns,
            counters: &mut self.counters,
            scripts: &mut self.scripts,
            script_index,
            scale: &mut self.scale,
            debug_level: &mut self.debug_level,
            out_err: &mut self.out_err,
            out_ess: &mut self.out_ess,
            out_qry: &mut self.out_qry,
            out_cfm: &mut self.out_cfm,
            theme: &mut self.theme,
            activity_hold_ms: &mut self.activity_hold_ms,
            show_cpu: &mut self.show_cpu,
            show_bpm: &mut self.show_bpm,
            header_level: &mut self.header_level,
            limiter_enabled: &mut self.limiter_enabled,
            show_meters_header: &mut self.show_meters_header,
            show_meters_grid: &mut self.show_meters_grid,
            show_spectrum: &mut self.show_spectrum,
            show_activity: &mut self.show_activity,
            show_grid: &mut self.show_grid,
            show_grid_view: &mut self.show_grid_view,
            show_seq_highlight: &mut self.show_seq_highlight,
            grid_mode: &mut self.grid_mode,
            scope_settings: &mut self.scope_settings,
            br_len: &mut self.br_len,
            sync_mode: &mut self.sync_mode,
            midi_connection: &mut self.midi_connection,
            midi_timing_stats: &self.midi_timing_stats,
            notes: &mut self.notes,
            load_rst: &mut self.load_rst,
            load_clr: &mut self.load_clr,
            vca_mode: &mut self.vca_mode,
            show_conditional_highlight: &mut self.show_conditional_highlight,
            current_scene_name: &mut self.current_scene_name,
            title_mode: &mut self.title_mode,
            title_timer_enabled: &mut self.title_timer_enabled,
            title_timer_interval_secs: &mut self.title_timer_interval_secs,
            title_timer_last_toggle: &mut self.title_timer_last_toggle,
            audio_devices: &self.audio_devices,
            header_scramble: &mut self.header_scramble,
            scramble_enabled: &mut self.scramble_enabled,
            scramble_mode: &mut self.scramble_mode,
            scramble_speed: &mut self.scramble_speed,
            scramble_curve: &mut self.scramble_curve,
            ascii_meters: &mut self.ascii_meters,
            autoload: &mut self.autoload,
            terminal_caps: &self.terminal_caps,
            color_mode: self.color_mode,
            script_break: &mut self.script_break,
            ev_counters: &mut self.ev_counters,
        };

        let result = process_command(
            &mut ctx,
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
                    // Reset break flag after nested script - BRK only affects the script it's in
                    self.script_break = false;
                }
            }
            Err(e) => {
                if self.should_output(crate::types::OutputCategory::Error) {
                    output_messages.push(e.to_string().to_uppercase());
                    for msg in output_messages {
                        self.add_output(msg);
                    }
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
            if self.should_output(crate::types::OutputCategory::Error) {
                self.add_output(format!("INVALID SCRIPT INDEX {}", script_index));
            }
            return;
        }

        // Record activity for all script executions, including nested calls
        self.script_activity[script_index] = Some(std::time::Instant::now());

        if depth > 10 {
            if self.should_output(crate::types::OutputCategory::Error) {
                self.add_output("SCRIPT RECURSION DEPTH EXCEEDED (MAX 10)".to_string());
            }
            return;
        }

        self.script_break = false;

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
            if self.script_break {
                break;
            }

            let original_line = line.trim();
            if original_line.is_empty() {
                continue;
            }

            self.debug_log(format!("Processing line {}: '{}'", line_num + 1, original_line));

            if original_line.to_uppercase().starts_with("L ") {
                if self.execute_loop(original_line, script_index, &mut metro_interval, Some(depth)) {
                    if self.script_break {
                        break;
                    }
                    continue;
                }
            }

            let line_to_process = self.process_ev_skip_prefix(original_line, script_index, line_num);
            if line_to_process.is_empty() {
                continue;
            }

            let mut search_start = 0;
            for sub_cmd in split_respecting_quotes(&line_to_process) {
                let sub_cmd_trimmed = sub_cmd.trim();
                if sub_cmd_trimmed.is_empty() {
                    search_start += sub_cmd.len() + 1; // +1 for semicolon
                    continue;
                }

                // Find where this sub_cmd starts in the original line
                let sub_cmd_offset = if let Some(pos) = original_line[search_start..].find(sub_cmd_trimmed) {
                    search_start + pos
                } else {
                    search_start
                };

                self.debug_log(format!("  sub_cmd: '{}'", sub_cmd_trimmed));
                self.process_sub_command(sub_cmd_trimmed, script_index, &mut metro_interval, Some(depth), line_num, sub_cmd_offset);

                if self.script_break {
                    break;
                }

                search_start += sub_cmd.len() + 1; // +1 for semicolon
            }
        }
    }

}
