pub mod braille;
pub mod eq_curve;
mod footer;
mod header;
pub mod pages;
pub mod search;
pub mod search_highlight;
pub mod state_highlight;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{prelude::*, widgets::Block};
use std::sync::mpsc;
use std::time::Duration;

use crate::output::OutputDecider;
use crate::types::{MetroEvent, Page};

use footer::render_footer;
use header::render_header;
use pages::{
    render_help_page, render_init_page, render_live_page, render_metro_page, render_notes_page,
    render_pattern_page, render_scope_page, render_script_page, render_variables_page, HELP_CATEGORIES,
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
    let needs_footer = app.search_mode || (!is_help && !is_pattern);

    let chunks = if needs_footer {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(f.area())
    };

    let header = render_header(app, chunks[0].width);
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
        Page::Scope => render_scope_page(app, chunks[1]),
        Page::Help => render_help_page(app, chunks[1].height as usize),
    };
    f.render_widget(content, chunks[1]);

    if needs_footer {
        let footer = render_footer(app);
        f.render_widget(footer, chunks[2]);
    }

    // Render confirmation dialog overlay if pending
    if let Some(ref action) = app.pending_confirmation {
        render_confirmation_dialog(f, app, action);
    }
}

fn render_confirmation_dialog(f: &mut Frame, app: &crate::App, action: &crate::types::ConfirmAction) {
    use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
    use ratatui::layout::{Alignment, Constraint, Direction, Layout};

    let message = match action {
        crate::types::ConfirmAction::Quit => "CONFIRM QUIT? (Y/N)",
        crate::types::ConfirmAction::SaveOverwrite(name) => {
            return render_save_overwrite_dialog(f, app, name);
        }
    };

    let area = f.area();
    let dialog_width = message.len() as u16 + 4;
    let dialog_height = 3;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(dialog_height)) / 2),
            Constraint::Length(dialog_height),
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(dialog_width)) / 2),
            Constraint::Length(dialog_width),
            Constraint::Min(0),
        ])
        .split(vertical[1]);

    let dialog_area = horizontal[1];

    // Clear the area first to prevent underlying content from bleeding through
    f.render_widget(Clear, dialog_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.error))
        .style(Style::default().bg(app.theme.background));

    let paragraph = Paragraph::new(message)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.error).bg(app.theme.background));

    f.render_widget(paragraph, dialog_area);
}

fn render_save_overwrite_dialog(f: &mut Frame, app: &crate::App, name: &str) {
    use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
    use ratatui::layout::{Alignment, Constraint, Direction, Layout};

    let message = format!("OVERWRITE SCENE '{}'? (Y/N)", name);
    let area = f.area();
    let dialog_width = message.len().min(area.width as usize - 4) as u16 + 4;
    let dialog_height = 3;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(dialog_height)) / 2),
            Constraint::Length(dialog_height),
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(dialog_width)) / 2),
            Constraint::Length(dialog_width),
            Constraint::Min(0),
        ])
        .split(vertical[1]);

    let dialog_area = horizontal[1];

    // Clear the area first to prevent underlying content from bleeding through
    f.render_widget(Clear, dialog_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.error))
        .style(Style::default().bg(app.theme.background));

    let paragraph = Paragraph::new(message)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.error).bg(app.theme.background));

    f.render_widget(paragraph, dialog_area);
}

pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut crate::App,
    metro_event_rx: mpsc::Receiver<MetroEvent>,
    sc_process: std::sync::Arc<std::sync::Mutex<crate::sc_process::ScProcess>>,
) -> Result<()> {
    loop {
        app.clear_expired_error();

        // Check title timer and toggle if needed
        if app.title_timer_enabled {
            if let Some(last_toggle) = app.title_timer_last_toggle {
                if last_toggle.elapsed().as_secs() >= app.title_timer_interval_secs as u64 {
                    // Toggle title mode
                    app.title_mode = if app.title_mode == 0 { 1 } else { 0 };
                    app.title_timer_last_toggle = Some(std::time::Instant::now());

                    // Trigger scramble animation for new title
                    let text = if app.title_mode == 0 {
                        "MONOKIT"
                    } else {
                        app.current_scene_name.as_ref().map(|s| s.as_str()).unwrap_or("[UNSAVED]")
                    };
                    if app.scramble_enabled {
                        let mode = crate::scramble::ScrambleMode::from_u8(app.scramble_mode);
                        let curve = crate::scramble::ScrambleCurve::from_u8(app.scramble_curve);
                        app.header_scramble = Some(crate::scramble::ScrambleAnimation::new_with_options(text, mode, app.scramble_speed, curve));
                    }
                }
            }
        }

        // Update header scramble animation
        if let Some(scramble) = &mut app.header_scramble {
            scramble.update();
            if scramble.complete {
                app.header_scramble = None;
            }
        }

        // Update UI scrambles
        app.ui_scrambles.retain_mut(|(_, scramble)| {
            scramble.update();
            !scramble.complete
        });

        // Update grid scrambles
        for scramble in &mut app.grid_scrambles {
            scramble.update();
        }
        app.grid_scrambles.retain(|s| !s.complete);

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
                MetroEvent::ScopeUpdate(scope_data) => {
                    app.scope_data = scope_data;
                }
                MetroEvent::CpuUpdate(cpu_data) => {
                    app.cpu_data = cpu_data;
                }
                MetroEvent::CompressorUpdate(comp_data) => {
                    app.compressor_data = comp_data;
                }
                MetroEvent::ScReady => {
                    if app.awaiting_audio_restart {
                        app.add_output("AUDIO ENGINE ONLINE".to_string());
                        app.awaiting_audio_restart = false;
                    }
                }
                MetroEvent::AudioDeviceList { current, devices } => {
                    // Store for numbered selection
                    app.audio_device_current = current.clone();
                    app.audio_devices = devices.clone();

                    app.add_output(format!("CURRENT: {}", current.to_uppercase()));
                    app.add_output("AVAILABLE DEVICES:".to_string());
                    for (i, device) in devices.iter().enumerate() {
                        app.add_output(format!("  {}: {}", i + 1, device));
                    }
                }
                MetroEvent::RestartScWithDevice(device) => {
                    let mut sc = sc_process.lock().unwrap();
                    if let Err(e) = sc.restart_with_device(&device) {
                        if app.should_output(crate::types::OutputCategory::Error) {
                            app.add_output(format!("ERROR: {}", e));
                        }
                    } else {
                        if let Err(e) = crate::config::save_audio_out_device(Some(device.clone())) {
                            if app.should_output(crate::types::OutputCategory::Error) {
                                app.add_output(format!("ERROR SAVING CONFIG: {}", e));
                            }
                        }
                        app.awaiting_audio_restart = true;
                    }
                }
                MetroEvent::Error(msg) => {
                    if app.should_output(crate::types::OutputCategory::Error) {
                        app.add_output(msg);
                    }
                }
                MetroEvent::StartRecordingDirect(dir) => {
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let mut sc = sc_process.lock().unwrap();
                        match sc.start_recording(&dir, None) {
                            Ok(_) => {
                                if app.should_output(crate::types::OutputCategory::Confirm) {
                                    app.add_output("RECORDING STARTED".to_string());
                                }
                            }
                            Err(e) => {
                                if app.should_output(crate::types::OutputCategory::Error) {
                                    app.add_output(format!("ERROR: {}", e));
                                }
                            }
                        }
                    }
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let _ = dir;
                    }
                }
                MetroEvent::StopRecordingDirect => {
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let mut sc = sc_process.lock().unwrap();
                        match sc.stop_recording() {
                            Ok(_) => {
                                if app.should_output(crate::types::OutputCategory::Confirm) {
                                    app.add_output("RECORDING STOPPED".to_string());
                                }
                            }
                            Err(e) => {
                                if app.should_output(crate::types::OutputCategory::Error) {
                                    app.add_output(format!("ERROR: {}", e));
                                }
                            }
                        }
                    }
                }
                MetroEvent::SetRecordingPathDirect(path) => {
                    #[cfg(feature = "scsynth-direct")]
                    {
                        let mut sc = sc_process.lock().unwrap();
                        sc.set_recording_path_prefix(path);
                    }
                    #[cfg(not(feature = "scsynth-direct"))]
                    {
                        let _ = path;
                    }
                }
            }
        }

        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                let is_help = app.current_page == Page::Help;
                let has_alt = key.modifiers.contains(KeyModifiers::ALT);

                if key.code == KeyCode::Char('f') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.enter_search_mode();
                    continue;
                }

                if app.search_mode {
                    let is_nav_hotkey = match key.code {
                        KeyCode::F(_) => true,
                        KeyCode::Char(_) if has_alt => true,
                        _ => false,
                    };

                    if is_nav_hotkey {
                        app.exit_search_mode();
                    } else {
                        match key.code {
                            KeyCode::Esc => {
                                app.exit_search_mode();
                            }
                            KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
                                app.prev_search_match();
                            }
                            KeyCode::Enter => {
                                app.next_search_match();
                            }
                            KeyCode::Backspace => {
                                app.search_delete_char();
                            }
                            KeyCode::Left => {
                                app.search_move_cursor_left();
                            }
                            KeyCode::Right => {
                                app.search_move_cursor_right();
                            }
                            KeyCode::Char(c) => {
                                app.search_insert_char(c);
                            }
                            _ => {}
                        }
                        continue;
                    }
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        let metro_active = {
                            let state = app.metro_state.lock().unwrap();
                            state.active
                        };
                        let has_named_scene = app.current_scene_name.is_some()
                            && app.current_scene_name.as_ref().map(|s| s.as_str()) != Some("[unsaved]");
                        let needs_confirmation = (app.confirm_quit_unsaved && app.scene_modified)
                            || (has_named_scene && metro_active);

                        if needs_confirmation && app.pending_confirmation.is_none() {
                            app.pending_confirmation = Some(crate::types::ConfirmAction::Quit);
                        } else {
                            app.should_quit = true;
                            // Stop SuperCollider before quitting
                            if let Ok(mut sc) = sc_process.lock() {
                                sc.stop();
                            }
                            return Ok(());
                        }
                    }
                    KeyCode::Char('y') | KeyCode::Char('Y') if app.pending_confirmation.is_some() => {
                        match app.pending_confirmation.take() {
                            Some(crate::types::ConfirmAction::Quit) => {
                                app.should_quit = true;
                                if let Ok(mut sc) = sc_process.lock() {
                                    sc.stop();
                                }
                                return Ok(());
                            }
                            Some(crate::types::ConfirmAction::SaveOverwrite(name)) => {
                                // Directly save the scene (bypass confirmation check)
                                let scene = crate::scene::Scene::from_app_state(
                                    &app.scripts,
                                    &app.patterns,
                                    &app.notes,
                                    &app.script_mutes,
                                );
                                match crate::scene::save_scene(&name, &scene) {
                                    Ok(()) => {
                                        app.current_scene_name = Some(name.clone());
                                        app.scene_modified = false;
                                        if app.scramble_enabled {
                                            let mode = crate::scramble::ScrambleMode::from_u8(app.scramble_mode);
                                            let curve = crate::scramble::ScrambleCurve::from_u8(app.scramble_curve);
                                            app.header_scramble = Some(crate::scramble::ScrambleAnimation::new_with_options(
                                                &name, mode, app.scramble_speed, curve
                                            ));
                                        }
                                        app.add_output(format!("SAVED SCENE: {}", name));
                                    }
                                    Err(e) => app.add_output(format!("ERROR: {:?}", e)),
                                }
                            }
                            None => {}
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') if app.pending_confirmation.is_some() => {
                        app.pending_confirmation = None;
                    }
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
                    KeyCode::Char('z') | KeyCode::Char('Z') if key.modifiers.contains(KeyModifiers::CONTROL)
                        && !key.modifiers.contains(KeyModifiers::SHIFT)
                        && (app.is_script_page() || app.current_page == Page::Notes) => {
                        app.undo();
                    }
                    KeyCode::Char('z') | KeyCode::Char('Z') if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                        && (app.is_script_page() || app.current_page == Page::Notes) => {
                        app.redo();
                    }
                    KeyCode::Tab => {
                        if app.current_page == Page::Live {
                            app.show_grid_view = !app.show_grid_view;
                        } else {
                            app.go_to_page(Page::Live);
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
                    KeyCode::Char('s') if has_alt => {
                        app.go_to_page(Page::Scope);
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
                    KeyCode::Char('1') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(0);
                    }
                    KeyCode::Char('2') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(1);
                    }
                    KeyCode::Char('3') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(2);
                    }
                    KeyCode::Char('4') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(3);
                    }
                    KeyCode::Char('5') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(4);
                    }
                    KeyCode::Char('6') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(5);
                    }
                    KeyCode::Char('7') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(6);
                    }
                    KeyCode::Char('8') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(7);
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(8);
                    }
                    KeyCode::Char('i') | KeyCode::Char('I') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                        app.toggle_script_mute(9);
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
                            // Stop SuperCollider before quitting
                            if let Ok(mut sc) = sc_process.lock() {
                                sc.stop();
                            }
                            return Ok(());
                        }
                        // Re-render immediately to show updated state (fixes TOG/SEQ/EITH highlight delay)
                        terminal.draw(|f| ui(f, app))?;
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
                    KeyCode::Char('u') if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
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
