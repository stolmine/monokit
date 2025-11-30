use super::super::App;
use crate::eval::eval_expression;

impl App {
    pub(super) fn process_loop_commands(
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

    pub(super) fn execute_loop(
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
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
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
                    &mut self.counters,
                    &self.scripts,
                    script_index,
                    &self.scale,
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
}
