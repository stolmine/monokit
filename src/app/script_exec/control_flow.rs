use super::super::App;
use crate::commands::registry::validate::validate_from_registry;
use crate::eval::{eval_condition, eval_expression};
use crate::output::OutputDecider;
use crate::types::ConditionalSegment;
use std::time::Instant;

impl App {
    pub(super) fn mark_conditional_segment(&mut self, script_index: usize, line_num: usize, start: usize, end: usize) {
        if !self.show_conditional_highlight {
            return;
        }

        if script_index < 10 && line_num < 8 {
            let segment = ConditionalSegment {
                start,
                end,
                timestamp: Instant::now(),
            };
            self.conditional_segments[script_index][line_num].segments.push(segment);
        }
    }
    pub(super) fn process_elif(
        &mut self,
        elif_cond: &str,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
        line_num: usize,
        sub_cmd: &str,
        sub_cmd_offset: usize,
    ) {
        if !self.if_else_condition {
            if eval_condition(
                elif_cond,
                &self.variables,
                &mut self.patterns,
                &mut self.counters,
                &self.scripts,
                script_index,
                &self.scale,
            ) {
                self.if_else_condition = true;
                if let Some(colon_pos) = sub_cmd.find(':') {
                    self.mark_conditional_segment(script_index, line_num, sub_cmd_offset, sub_cmd_offset + colon_pos + 1);
                }
                self.execute_and_update_metro(cmd_to_run, script_index, metro_interval, depth);
            }
        }
    }

    pub(super) fn process_else(
        &mut self,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
        line_num: usize,
        sub_cmd: &str,
        sub_cmd_offset: usize,
    ) {
        if !self.if_else_condition {
            self.if_else_condition = true;
            if let Some(colon_pos) = sub_cmd.find(':') {
                self.mark_conditional_segment(script_index, line_num, sub_cmd_offset, sub_cmd_offset + colon_pos + 1);
            }
            self.execute_and_update_metro(cmd_to_run, script_index, metro_interval, depth);
        }
    }

    pub(super) fn process_conditional(
        &mut self,
        condition: Option<&str>,
        cmd_to_run: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
        line_num: usize,
        sub_cmd: &str,
        sub_cmd_offset: usize,
    ) -> bool {
        if let Some(cond) = condition {
            let cond_upper = cond.trim().to_uppercase();

            // Handle nested conditionals (e.g., "IF GT I 2: IF LT I 5")
            if let Some(colon_pos) = cond.find(':') {
                let first_cond = &cond[..colon_pos];
                let rest = &cond[colon_pos + 1..].trim();

                // Evaluate first condition
                if !eval_condition(
                    first_cond,
                    &self.variables,
                    &mut self.patterns,
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
                ) {
                    return false;
                }

                // Recursively process remaining nested conditions
                if !rest.is_empty() {
                    return self.process_conditional(
                        Some(rest),
                        cmd_to_run,
                        script_index,
                        metro_interval,
                        depth,
                        line_num,
                        sub_cmd,
                        sub_cmd_offset,
                    );
                }

                // If we get here, all conditions passed
                self.if_else_condition = true;
                if let Some(colon_pos) = sub_cmd.find(':') {
                    self.mark_conditional_segment(script_index, line_num, sub_cmd_offset, sub_cmd_offset + colon_pos + 1);
                }
            } else if cond_upper.starts_with("IF ") {
                self.if_else_condition = false;
                if eval_condition(
                    cond,
                    &self.variables,
                    &mut self.patterns,
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
                ) {
                    self.if_else_condition = true;
                    if let Some(colon_pos) = sub_cmd.find(':') {
                        self.mark_conditional_segment(script_index, line_num, sub_cmd_offset, sub_cmd_offset + colon_pos + 1);
                    }
                } else {
                    return false;
                }
            } else if cond_upper.starts_with("PROB ") {
                if eval_condition(
                    cond,
                    &self.variables,
                    &mut self.patterns,
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
                ) {
                    if let Some(colon_pos) = sub_cmd.find(':') {
                        self.mark_conditional_segment(script_index, line_num, sub_cmd_offset, sub_cmd_offset + colon_pos + 1);
                    }
                } else {
                    return false;
                }
            } else if !eval_condition(
                cond,
                &self.variables,
                &mut self.patterns,
                &mut self.counters,
                &self.scripts,
                script_index,
                &self.scale,
            ) {
                return false;
            }
        }

        self.execute_and_update_metro(cmd_to_run, script_index, metro_interval, depth);
        true
    }

    pub(super) fn process_ev_skip_prefix<'a>(
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
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
                ) {
                    if divisor > 0 {
                        let divisor = divisor as u32;
                        self.ev_counters[script_index][line_num] += 1;
                        if self.ev_counters[script_index][line_num] % divisor != 0 {
                            return "";
                        }
                        self.mark_conditional_segment(script_index, line_num, 0, colon_pos + 1);
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
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
                ) {
                    if divisor > 0 {
                        let divisor = divisor as u32;
                        self.ev_counters[script_index][line_num] += 1;
                        if self.ev_counters[script_index][line_num] % divisor == 0 {
                            return "";
                        }
                        self.mark_conditional_segment(script_index, line_num, 0, colon_pos + 1);
                        return line[colon_pos + 1..].trim();
                    }
                }
            }
        }
        line
    }

    pub(super) fn process_sub_command(
        &mut self,
        sub_cmd: &str,
        script_index: usize,
        metro_interval: &mut u64,
        depth: Option<usize>,
        line_num: usize,
        sub_cmd_offset: usize,
    ) {
        if let Err(e) = validate_from_registry(sub_cmd) {
            if self.should_output(crate::types::OutputCategory::Error) {
                self.add_output(e.to_string().to_uppercase());
            }
            return;
        }

        // Handle REPL.DUMP in script context (needs access to self.output)
        if sub_cmd.to_uppercase().starts_with("REPL.DUMP") {
            let parts: Vec<&str> = sub_cmd.split_whitespace().collect();
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
                    self.add_output(format!("DUMPED {} LINES TO {}", self.output.len(), filename));
                }
                Err(e) => {
                    if self.should_output(crate::types::OutputCategory::Error) {
                        self.add_output(format!("FILE CREATE FAILED: {}", e.to_string().to_uppercase()));
                    }
                }
            }
            return;
        }

        if sub_cmd.to_uppercase().starts_with("ELIF ") {
            if let Some(colon_pos) = sub_cmd.find(':') {
                let elif_cond = sub_cmd[5..colon_pos].trim();
                let cmd_to_run = sub_cmd[colon_pos + 1..].trim();
                self.process_elif(elif_cond, cmd_to_run, script_index, metro_interval, depth, line_num, sub_cmd, sub_cmd_offset);
                return;
            }
        }

        if sub_cmd.to_uppercase().starts_with("ELSE:") {
            let cmd_to_run = sub_cmd[5..].trim();
            self.process_else(cmd_to_run, script_index, metro_interval, depth, line_num, sub_cmd, sub_cmd_offset);
            return;
        }

        // Handle DEL, DEL.X, DEL.R commands specially - they use colon syntax but aren't conditionals
        let upper = sub_cmd.to_uppercase();
        if upper.starts_with("DEL ") || upper.starts_with("DEL.X ") || upper.starts_with("DEL.R ") {
            self.execute_and_update_metro(sub_cmd, script_index, metro_interval, depth);
            return;
        }

        let (condition, cmd_to_run) = if let Some(colon_pos) = sub_cmd.rfind(':') {
            let cond = &sub_cmd[..colon_pos];
            let cmd = sub_cmd[colon_pos + 1..].trim();
            (Some(cond), cmd)
        } else {
            (None, sub_cmd)
        };

        if !cmd_to_run.is_empty() {
            self.process_conditional(condition, cmd_to_run, script_index, metro_interval, depth, line_num, sub_cmd, sub_cmd_offset);
        }
    }
}
