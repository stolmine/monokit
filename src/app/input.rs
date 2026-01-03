use super::{App, EditAction};
use crate::commands::registry::validate::validate_from_registry;

impl App {
    pub fn insert_char(&mut self, c: char) {
        let c = c.to_ascii_uppercase();
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

            if !self.input.trim().is_empty() {
                if let Err(e) = validate_from_registry(&self.input) {
                    self.script_error = Some(format!("{}", e));
                    self.script_error_time = Some(std::time::Instant::now());
                    return;
                }
            }

            // Record undo action before modifying
            let old = self.scripts.get_script(script_idx).lines[line_idx].clone();
            let new = self.input.clone();
            self.script_undo_stacks[script_idx].push(EditAction::SaveLine {
                line_idx,
                old,
                new,
            });

            let script = self.scripts.get_script_mut(script_idx);
            script.lines[line_idx] = self.input.clone();
            self.scene_modified = true;
            let next_line = if line_idx < 7 { line_idx + 1 } else { 0 };
            self.selected_line = Some(next_line);
            self.input.clear();
            self.cursor_position = 0;
            // Clear any previous error on successful save
            self.script_error = None;
            self.script_error_time = None;
        }
    }

    pub fn duplicate_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                if selected < 7 {
                    let script = self.scripts.get_script(script_idx);
                    let line_content = script.lines[selected].clone();

                    // Record lines that will be shifted for undo
                    let shifted_lines: Vec<String> = (selected + 1..=7)
                        .map(|i| script.lines[i].clone())
                        .collect();
                    self.script_undo_stacks[script_idx].push(EditAction::DuplicateLine {
                        line_idx: selected,
                        shifted_lines,
                    });

                    let script = self.scripts.get_script_mut(script_idx);
                    for i in (selected + 2..=7).rev() {
                        script.lines[i] = script.lines[i - 1].clone();
                    }
                    script.lines[selected + 1] = line_content;
                    self.scene_modified = true;
                    self.selected_line = Some(selected + 1);
                }
            }
        }
    }

    pub fn delete_entire_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                // Record undo action before modifying
                let content = self.scripts.get_script(script_idx).lines[selected].clone();
                if !content.is_empty() {
                    self.script_undo_stacks[script_idx].push(EditAction::DeleteLine {
                        line_idx: selected,
                        content,
                    });
                }

                let script = self.scripts.get_script_mut(script_idx);
                script.lines[selected].clear();
                self.scene_modified = true;
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

    pub fn copy_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                let script = self.scripts.get_script(script_idx);
                self.clipboard = script.lines[selected].clone();
            }
        }
    }

    pub fn cut_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                let content = self.scripts.get_script(script_idx).lines[selected].clone();
                self.clipboard = content.clone();

                if !content.is_empty() {
                    self.script_undo_stacks[script_idx].push(EditAction::CutLine {
                        line_idx: selected,
                        content,
                    });
                }

                let script = self.scripts.get_script_mut(script_idx);
                script.lines[selected].clear();
                self.scene_modified = true;
                self.input.clear();
                self.cursor_position = 0;
            }
        }
    }

    pub fn paste_line(&mut self) {
        if let Some(script_idx) = self.current_script_index() {
            if let Some(selected) = self.selected_line {
                if !self.clipboard.trim().is_empty() {
                    if let Err(e) = validate_from_registry(&self.clipboard) {
                        self.script_error = Some(format!("{}", e));
                        self.script_error_time = Some(std::time::Instant::now());
                        return;
                    }
                }

                // Record undo action before modifying
                let old = self.scripts.get_script(script_idx).lines[selected].clone();
                let new = self.clipboard.clone();
                self.script_undo_stacks[script_idx].push(EditAction::PasteLine {
                    line_idx: selected,
                    old,
                    new,
                });

                let script = self.scripts.get_script_mut(script_idx);
                script.lines[selected] = self.clipboard.clone();
                self.scene_modified = true;
                self.input = self.clipboard.clone();
                self.cursor_position = self.input.len();
                self.script_error = None;
                self.script_error_time = None;
            }
        }
    }

    pub fn select_notes_line_up(&mut self) {
        let new_selection = match self.selected_line {
            None => 7,
            Some(0) => 0,
            Some(n) => n - 1,
        };
        self.selected_line = Some(new_selection);
        self.input = self.notes.lines[new_selection].clone();
        self.cursor_position = self.input.len();
    }

    pub fn select_notes_line_down(&mut self) {
        let new_selection = match self.selected_line {
            None => 0,
            Some(7) => 7,
            Some(n) => n + 1,
        };
        self.selected_line = Some(new_selection);
        self.input = self.notes.lines[new_selection].clone();
        self.cursor_position = self.input.len();
    }

    pub fn save_notes_line(&mut self) {
        let line_idx = if let Some(selected) = self.selected_line {
            selected
        } else {
            let mut first_empty = None;
            for i in 0..8 {
                if self.notes.lines[i].is_empty() {
                    first_empty = Some(i);
                    break;
                }
            }
            first_empty.unwrap_or(7)
        };

        // Record undo action before modifying
        let old = self.notes.lines[line_idx].clone();
        let new = self.input.clone();
        self.notes_undo_stack.push(EditAction::SaveNotesLine {
            line_idx,
            old,
            new,
        });

        self.notes.lines[line_idx] = self.input.clone();
        let next_line = if line_idx < 7 { line_idx + 1 } else { 0 };
        self.selected_line = Some(next_line);
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn duplicate_notes_line(&mut self) {
        if let Some(selected) = self.selected_line {
            if selected < 7 {
                let line_content = self.notes.lines[selected].clone();

                // Record lines that will be shifted for undo
                let shifted_lines: Vec<String> = (selected + 1..=7)
                    .map(|i| self.notes.lines[i].clone())
                    .collect();
                self.notes_undo_stack.push(EditAction::DuplicateNotesLine {
                    line_idx: selected,
                    shifted_lines,
                });

                for i in (selected + 2..=7).rev() {
                    self.notes.lines[i] = self.notes.lines[i - 1].clone();
                }
                self.notes.lines[selected + 1] = line_content;
                self.selected_line = Some(selected + 1);
            }
        }
    }

    pub fn delete_notes_line(&mut self) {
        if let Some(selected) = self.selected_line {
            // Record undo action before modifying
            let content = self.notes.lines[selected].clone();
            if !content.is_empty() {
                self.notes_undo_stack.push(EditAction::DeleteNotesLine {
                    line_idx: selected,
                    content,
                });
            }

            self.notes.lines[selected].clear();
            self.input.clear();
            self.cursor_position = 0;
        }
    }

    pub fn copy_notes_line(&mut self) {
        if let Some(selected) = self.selected_line {
            self.clipboard = self.notes.lines[selected].clone();
        }
    }

    pub fn cut_notes_line(&mut self) {
        if let Some(selected) = self.selected_line {
            let content = self.notes.lines[selected].clone();
            self.clipboard = content.clone();

            if !content.is_empty() {
                self.notes_undo_stack.push(EditAction::CutNotesLine {
                    line_idx: selected,
                    content,
                });
            }

            self.notes.lines[selected].clear();
            self.input.clear();
            self.cursor_position = 0;
        }
    }

    pub fn paste_notes_line(&mut self) {
        if let Some(selected) = self.selected_line {
            // Record undo action before modifying
            let old = self.notes.lines[selected].clone();
            let new = self.clipboard.clone();
            self.notes_undo_stack.push(EditAction::PasteNotesLine {
                line_idx: selected,
                old,
                new,
            });

            self.notes.lines[selected] = self.clipboard.clone();
            self.input = self.clipboard.clone();
            self.cursor_position = self.input.len();
        }
    }
}
