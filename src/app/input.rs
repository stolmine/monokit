use super::App;

impl App {
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

    pub fn duplicate_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                if selected < 7 {
                    let script = self.scripts.get_script(script_idx);
                    let line_content = script.lines[selected].clone();
                    let script = self.scripts.get_script_mut(script_idx);
                    script.lines[selected + 1] = line_content;
                    self.selected_line = Some(selected + 1);
                }
            }
        }
    }

    pub fn delete_entire_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                let script = self.scripts.get_script_mut(script_idx);
                script.lines[selected].clear();
                self.input.clear();
                self.cursor_position = 0;
            }
        }
    }

    pub fn move_cursor_word_left(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        let chars: Vec<char> = self.input.chars().collect();
        let mut pos = self.cursor_position;

        while pos > 0 && chars[pos - 1] == ' ' {
            pos -= 1;
        }

        while pos > 0 && chars[pos - 1] != ' ' {
            pos -= 1;
        }

        self.cursor_position = pos;
    }

    pub fn move_cursor_word_right(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();
        let len = chars.len();

        if self.cursor_position >= len {
            return;
        }

        let mut pos = self.cursor_position;

        while pos < len && chars[pos] != ' ' {
            pos += 1;
        }

        while pos < len && chars[pos] == ' ' {
            pos += 1;
        }

        self.cursor_position = pos;
    }
}
