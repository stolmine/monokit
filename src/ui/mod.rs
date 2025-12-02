mod footer;
mod header;
pub mod pages;
pub mod state_highlight;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{prelude::*, widgets::Block};
use std::sync::mpsc;
use std::time::Duration;

use crate::types::{MetroEvent, Page};

use footer::render_footer;
use header::render_header;
use pages::{
    render_help_page, render_init_page, render_live_page, render_metro_page, render_notes_page,
    render_pattern_page, render_script_page, render_variables_page, HELP_CATEGORIES,
};

pub fn ui(f: &mut Frame, app: &crate::App) {
    let area = f.area();

    // Clear and set background for entire terminal area
    f.render_widget(
        Block::default().style(Style::default().bg(app.theme.background).fg(app.theme.foreground)),
        area
    );

    let is_help = app.current_page == Page::Help;
    let is_pattern = app.current_page == Page::Pattern;

    let chunks = if is_help || is_pattern {
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
        Page::Live => render_live_page(app, chunks[1].width as usize, chunks[1].height as usize),
        Page::Script1 => render_script_page(app, 1),
        Page::Script2 => render_script_page(app, 2),
        Page::Script3 => render_script_page(app, 3),
        Page::Script4 => render_script_page(app, 4),
        Page::Script5 => render_script_page(app, 5),
        Page::Script6 => render_script_page(app, 6),
        Page::Script7 => render_script_page(app, 7),
        Page::Script8 => render_script_page(app, 8),
        Page::Metro => render_metro_page(app),
        Page::Init => render_init_page(app),
        Page::Pattern => render_pattern_page(app),
        Page::Variables => render_variables_page(app),
        Page::Notes => render_notes_page(app),
        Page::Help => render_help_page(app, chunks[1].height as usize),
    };
    f.render_widget(content, chunks[1]);

    let is_pattern = app.current_page == Page::Pattern;
    if !is_help && !is_pattern {
        let footer = render_footer(app);
        f.render_widget(footer, chunks[2]);
    }
}

pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut crate::App,
    metro_event_rx: mpsc::Receiver<MetroEvent>,
) -> Result<()> {
    loop {
        app.clear_expired_error();

        // Process metro events BEFORE rendering so activity indicators update immediately
        while let Ok(event) = metro_event_rx.try_recv() {
            match event {
                MetroEvent::ExecuteScript(index) => {
                    app.execute_script(index);
                }
                MetroEvent::ExecuteDelayed(command, script_index) => {
                    app.execute_delayed_command(&command, script_index);
                }
                MetroEvent::MeterUpdate(meter_data) => {
                    app.meter_data = meter_data;
                }
                MetroEvent::SpectrumUpdate(spectrum_data) => {
                    app.spectrum_data = spectrum_data;
                }
                MetroEvent::CpuUpdate(cpu_data) => {
                    app.cpu_data = cpu_data;
                }
            }
        }

        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                let is_help = app.current_page == Page::Help;
                let has_alt = key.modifiers.contains(KeyModifiers::ALT);

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) && app.is_script_page() => {
                        app.copy_line();
                    }
                    KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) && app.is_script_page() => {
                        app.cut_line();
                    }
                    KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) && app.is_script_page() => {
                        app.paste_line();
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) && app.current_page == Page::Notes => {
                        app.copy_notes_line();
                    }
                    KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) && app.current_page == Page::Notes => {
                        app.cut_notes_line();
                    }
                    KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) && app.current_page == Page::Notes => {
                        app.paste_notes_line();
                    }
                    KeyCode::Tab => {
                        if app.current_page == Page::Live {
                            app.show_grid_view = !app.show_grid_view;
                        }
                    }
                    KeyCode::Char('[') if is_help => {
                        app.prev_help_page();
                    }
                    KeyCode::Char(']') if is_help => {
                        app.next_help_page();
                    }
                    KeyCode::Char('[') => {
                        app.prev_page();
                    }
                    KeyCode::Char(']') => {
                        app.next_page();
                    }
                    KeyCode::F(1) => {
                        app.go_to_page(Page::Script1);
                    }
                    KeyCode::F(2) => {
                        app.go_to_page(Page::Script2);
                    }
                    KeyCode::F(3) => {
                        app.go_to_page(Page::Script3);
                    }
                    KeyCode::F(4) => {
                        app.go_to_page(Page::Script4);
                    }
                    KeyCode::F(5) => {
                        app.go_to_page(Page::Script5);
                    }
                    KeyCode::F(6) => {
                        app.go_to_page(Page::Script6);
                    }
                    KeyCode::F(7) => {
                        app.go_to_page(Page::Script7);
                    }
                    KeyCode::F(8) => {
                        app.go_to_page(Page::Script8);
                    }
                    KeyCode::F(9) => {
                        app.go_to_page(Page::Live);
                    }
                    KeyCode::F(10) => {
                        app.go_to_page(Page::Metro);
                    }
                    KeyCode::F(11) => {
                        app.go_to_page(Page::Init);
                    }
                    KeyCode::F(12) => {
                        app.go_to_page(Page::Pattern);
                    }
                    KeyCode::Esc if is_help || app.current_page != Page::Pattern => {
                        app.toggle_help();
                    }
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
                    KeyCode::Char('v') if has_alt => {
                        app.go_to_page(Page::Variables);
                    }
                    KeyCode::Char('n') if has_alt => {
                        app.go_to_page(Page::Notes);
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
                    KeyCode::Up if is_help => {
                        app.help_scroll = app.help_scroll.saturating_sub(1);
                    }
                    KeyCode::Down if is_help => {
                        let current_page_lines = HELP_CATEGORIES[app.help_page].lines.len();
                        app.help_scroll = app.help_scroll.saturating_add(1).min(current_page_lines.saturating_sub(1));
                    }
                    KeyCode::Up if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.1 > 0 {
                            app.pattern_cursor.1 -= 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Down if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.1 < 63 {
                            app.pattern_cursor.1 += 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Left if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.0 > 0 {
                            app.pattern_cursor.0 -= 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Right if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.0 < 5 {
                            app.pattern_cursor.0 += 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Char('-') if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_input.is_empty() {
                            app.pattern_input.push('-');
                        }
                    }
                    KeyCode::Char(c) if !is_help && app.current_page == Page::Pattern && c.is_ascii_digit() => {
                        app.pattern_input.push(c);
                    }
                    KeyCode::Backspace if !is_help && app.current_page == Page::Pattern => {
                        app.pattern_input.pop();
                    }
                    KeyCode::Esc if !is_help && app.current_page == Page::Pattern => {
                        app.pattern_input.clear();
                    }
                    KeyCode::Enter if !is_help && app.current_page == Page::Pattern => {
                        if !app.pattern_input.is_empty() {
                            if let Ok(value) = app.pattern_input.parse::<i16>() {
                                let (pattern_idx, step_idx) = app.pattern_cursor;
                                app.patterns.patterns[pattern_idx].data[step_idx] = value;
                                app.pattern_input.clear();
                            }
                        }
                    }
                    KeyCode::Up if !is_help && app.is_script_page() => {
                        app.select_line_up();
                    }
                    KeyCode::Down if !is_help && app.is_script_page() => {
                        app.select_line_down();
                    }
                    KeyCode::Up if !is_help && app.current_page == Page::Notes => {
                        app.select_notes_line_up();
                    }
                    KeyCode::Down if !is_help && app.current_page == Page::Notes => {
                        app.select_notes_line_down();
                    }
                    KeyCode::Char('d') if !is_help && app.is_script_page() && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.duplicate_line();
                    }
                    KeyCode::Char('d') if !is_help && app.current_page == Page::Notes && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.duplicate_notes_line();
                    }
                    KeyCode::Enter if !is_help && app.is_script_page() => {
                        app.save_line();
                    }
                    KeyCode::Enter if !is_help && app.current_page == Page::Notes => {
                        app.save_notes_line();
                    }
                    KeyCode::Enter if !is_help && app.current_page != Page::Pattern => {
                        app.execute_command();
                        if app.should_quit {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('k') if !is_help && app.is_script_page() && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_entire_line();
                    }
                    KeyCode::Char('k') if !is_help && app.current_page == Page::Notes && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_notes_line();
                    }
                    KeyCode::Backspace if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.clear_input();
                    }
                    KeyCode::Delete if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.clear_input();
                    }
                    KeyCode::Char('u') if !is_help && app.current_page != Page::Pattern && app.current_page != Page::Notes && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_to_start();
                    }
                    KeyCode::Char(c) if !is_help && app.current_page != Page::Pattern => {
                        app.insert_char(c);
                    }
                    KeyCode::Backspace if !is_help && app.current_page != Page::Pattern => {
                        app.delete_char();
                    }
                    KeyCode::Left if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.move_cursor_word_left();
                    }
                    KeyCode::Right if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.move_cursor_word_right();
                    }
                    KeyCode::Left if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_right();
                    }
                    // REPL output scrolling with Ctrl+Up/Down
                    KeyCode::Up if !is_help && app.current_page == Page::Live && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.output_scroll = app.output_scroll.saturating_add(1);
                    }
                    KeyCode::Down if !is_help && app.current_page == Page::Live && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.output_scroll = app.output_scroll.saturating_sub(1);
                    }
                    KeyCode::Up if !is_help && app.current_page != Page::Pattern => {
                        app.history_prev();
                    }
                    KeyCode::Down if !is_help && app.current_page != Page::Pattern => {
                        app.history_next();
                    }
                    _ => {}
                }
            }
        }
    }
}
