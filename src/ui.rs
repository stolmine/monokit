use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use std::sync::mpsc;
use std::time::Duration;

use crate::types::{MetroEvent, Page, NAVIGABLE_PAGES};

pub fn ui(f: &mut Frame, app: &super::App) {
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
        Page::Live => render_live_page(app, chunks[1].height as usize),
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
        Page::Help => render_help_page(app.help_scroll, chunks[1].height as usize),
    };
    f.render_widget(content, chunks[1]);

    let is_pattern = app.current_page == Page::Pattern;
    if !is_help && !is_pattern {
        let footer = render_footer(app);
        f.render_widget(footer, chunks[2]);
    }
}

pub fn render_header(app: &super::App) -> Paragraph<'static> {
    let mut spans = vec![Span::raw(" ")];

    if app.current_page == Page::Help {
        spans.push(Span::styled(
            "[HELP]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        for page in NAVIGABLE_PAGES.iter() {
            if *page == app.current_page {
                spans.push(Span::styled(
                    format!("[{}]", page.name()),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    page.name().to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
            }
            spans.push(Span::raw(" "));
        }
    }

    Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title(" MONOKIT "))
}

pub fn render_metro_page(app: &super::App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 60000.0 / state.interval_ms as f32;
    let status = if state.active { "ON" } else { "OFF" };
    let status_color = if state.active {
        Color::Green
    } else {
        Color::Red
    };

    let mut text = Vec::new();
    text.push(Line::from(vec![
        Span::styled("  BPM: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{:.1}", bpm)),
        Span::raw("  "),
        Span::styled("Interval: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{}ms", state.interval_ms)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  Status: ", Style::default().fg(Color::Cyan)),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  M Script Lines:", Style::default().fg(Color::Cyan)),
    ]));

    let metro_script = app.scripts.get_script(8);
    for i in 0..8 {
        let line_num = i + 1;
        let line_content = &metro_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            if line_content.is_empty() {
                text.push(Line::from(vec![
                    Span::styled(format!("  > {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                text.push(Line::from(vec![
                    Span::styled(format!("  > {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(line_content.clone(), Style::default().fg(Color::Yellow)),
                ]));
            }
        } else if line_content.is_empty() {
            text.push(Line::from(vec![
                Span::styled(format!("    {}: ", line_num), Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            text.push(Line::from(vec![
                Span::styled(format!("    {}: ", line_num), Style::default().fg(Color::Cyan)),
                Span::raw(line_content.clone()),
            ]));
        }
    }

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Metro "))
        .wrap(Wrap { trim: false })
}

pub fn render_live_page(app: &super::App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    let start_idx = if app.output.len() > visible_lines {
        app.output.len() - visible_lines
    } else {
        0
    };

    let text: Vec<Line> = app.output[start_idx..]
        .iter()
        .map(|line| Line::from(format!("  {}", line)))
        .collect();

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Live "))
}

pub fn render_script_page(app: &super::App, num: u8) -> Paragraph<'static> {
    let script_index = (num - 1) as usize;
    let script = app.scripts.get_script(script_index);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_num = i + 1;
        let line_content = &script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            if line_content.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(line_content.clone(), Style::default().fg(Color::Yellow)),
                ]));
            }
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::Cyan)),
                Span::raw(line_content.clone()),
            ]));
        }
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(format!(" Script {} ", num)))
}

pub fn render_init_page(app: &super::App) -> Paragraph<'static> {
    let init_script = app.scripts.get_script(9);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_num = i + 1;
        let line_content = &init_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            if line_content.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled(format!("> {}: ", line_num), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(line_content.clone(), Style::default().fg(Color::Yellow)),
                ]));
            }
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}: ", line_num), Style::default().fg(Color::Cyan)),
                Span::raw(line_content.clone()),
            ]));
        }
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Init "))
}

pub fn render_pattern_page(app: &super::App) -> Paragraph<'static> {
    let (cursor_pattern, cursor_step) = app.pattern_cursor;

    let visible_rows = 16;
    let scroll_offset = if cursor_step < visible_rows / 2 {
        0
    } else if cursor_step >= 64 - visible_rows / 2 {
        64 - visible_rows
    } else {
        cursor_step.saturating_sub(visible_rows / 2)
    };

    let mut lines = vec![];

    let mut header_spans = vec![Span::raw("     ")];
    for pattern_idx in 0..4 {
        let label = format!("P{}", pattern_idx);
        let style = if pattern_idx == app.patterns.working {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        header_spans.push(Span::styled(format!(" {:^5} ", label), style));
    }
    lines.push(Line::from(header_spans));

    let mut len_spans = vec![Span::styled(" len ", Style::default().fg(Color::DarkGray))];
    for pattern_idx in 0..4 {
        let pattern = &app.patterns.patterns[pattern_idx];
        len_spans.push(Span::styled(
            format!(" {:^5} ", pattern.length),
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::from(len_spans));

    for step in scroll_offset..(scroll_offset + visible_rows).min(64) {
        let mut row_spans = vec![
            Span::styled(format!("{:3}: ", step), Style::default().fg(Color::DarkGray)),
        ];

        for pattern_idx in 0..4 {
            let pattern = &app.patterns.patterns[pattern_idx];
            let value = pattern.data[step];
            let is_cursor = cursor_pattern == pattern_idx && cursor_step == step;
            let is_playhead = pattern.index == step;
            let is_beyond_length = step >= pattern.length;

            let display = if is_cursor && !app.pattern_input.is_empty() {
                format!(" {:>5} ", app.pattern_input)
            } else {
                format!(" {:>5} ", value)
            };

            let style = if is_cursor {
                Style::default().bg(Color::White).fg(Color::Black)
            } else if is_playhead && !is_beyond_length {
                Style::default().bg(Color::Cyan).fg(Color::Black)
            } else if is_beyond_length {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };

            row_spans.push(Span::styled(display, style));
        }

        lines.push(Line::from(row_spans));
    }

    let title = format!(" Pattern ({}/64) ", cursor_step);
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
}

pub const HELP_LINES: &[&str] = &[
    "",
    "  NAVIGATION",
    "  [ ]         Cycle pages",
    "  Alt+L       Live page",
    "  Alt+1-8     Script 1-8",
    "  Alt+M       Metro page",
    "  Alt+I       Init page",
    "  Alt+P       Pattern page",
    "  Alt+H       Toggle help",
    "  q           Quit",
    "",
    "  TRIGGER & VOLUME",
    "  TR          Trigger voice",
    "  VOL 0-1     Master volume",
    "  RST         Reset to defaults",
    "",
    "  OSCILLATORS",
    "  PF <hz>     Primary freq (20-20000)",
    "  PW <0-2>    Primary wave (sin/tri/saw)",
    "  MF <hz>     Mod freq (20-20000)",
    "  MW <0-2>    Mod wave (sin/tri/saw)",
    "",
    "  FM SYNTHESIS",
    "  FM <0-16383>  FM index",
    "  FA <0-16383>  FM env amount",
    "  FD <ms>       FM env decay",
    "",
    "  DISCONTINUITY",
    "  DC <0-16383>  Discontinuity amount",
    "  DA <0-16383>  DC env amount",
    "  DD <ms>       DC env decay",
    "  DM <0-2>      Mode (fold/tanh/soft)",
    "",
    "  ENVELOPES",
    "  AD <ms>       Amp decay",
    "  PD <ms>       Pitch decay",
    "  PA <0-16>     Pitch env amount",
    "",
    "  MOD BUS",
    "  MB <0-16383>  Mod bus amount",
    "  TK <0-16383>  Tracking amount",
    "  MP <0|1>      Mod -> primary freq",
    "  MD <0|1>      Mod -> discontinuity",
    "  MT <0|1>      Mod -> tracking",
    "  MA <0|1>      Mod -> amplitude",
    "",
    "  MIX",
    "  MX <0-16383>  Mix amount",
    "  MM <0|1>      Mod bus -> mix",
    "  ME <0|1>      Envelope -> mix",
    "",
    "  METRO",
    "  M             Show interval",
    "  M <ms>        Set interval",
    "  M.BPM <bpm>   Set BPM",
    "  M.ACT <0|1>     Start/stop",
    "  M.SCRIPT <1-8>  Set script to call on each tick",
    "",
    "  SCRIPTS",
    "  SCRIPT <1-8>  Execute stored script",
    "",
    "  PATTERNS (Working Pattern)",
    "  P.N           Show working pattern",
    "  P.N <0-3>     Set working pattern",
    "  P.L           Show pattern length",
    "  P.L <1-64>    Set pattern length",
    "  P.I           Show pattern index",
    "  P.I <0-63>    Set pattern index",
    "  P.HERE        Get value at index",
    "  P.NEXT        Advance index, return value",
    "  P.PREV        Reverse index, return value",
    "  P <idx>       Get value at index",
    "  P <idx> <val> Set value at index",
    "",
    "  PATTERNS (Explicit Pattern)",
    "  PN.L <pat>           Get pattern length",
    "  PN.L <pat> <len>     Set pattern length",
    "  PN.I <pat>           Get pattern index",
    "  PN.I <pat> <idx>     Set pattern index",
    "  PN.HERE <pat>        Get value at index",
    "  PN.NEXT <pat>        Advance index, return value",
    "  PN.PREV <pat>        Reverse index, return value",
    "  PN <pat> <idx>       Get value at index",
    "  PN <pat> <idx> <val> Set value at index",
    "",
];

pub fn render_help_page(scroll: usize, height: usize) -> Paragraph<'static> {
    let visible = if height > 2 { height - 2 } else { 1 };
    let total = HELP_LINES.len();
    let start = scroll.min(total.saturating_sub(visible));

    let lines: Vec<Line> = HELP_LINES
        .iter()
        .skip(start)
        .take(visible)
        .map(|&s| {
            if s.starts_with("  ") && s.chars().nth(2).map_or(false, |c| c.is_uppercase()) && !s.contains('<') && !s.contains("0-") {
                Line::from(Span::styled(s, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            } else {
                Line::from(s)
            }
        })
        .collect();

    let title = if total > visible {
        format!(" Help ({}/{}) ", start + 1, total.saturating_sub(visible) + 1)
    } else {
        " Help ".to_string()
    };

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
}

pub fn render_footer(app: &super::App) -> Paragraph<'static> {
    let input = &app.input;
    let pos = app.cursor_position;

    let before: String = input.chars().take(pos).collect();
    let cursor_char = input.chars().nth(pos).unwrap_or(' ');
    let after: String = input.chars().skip(pos + 1).collect();

    let input_line = Line::from(vec![
        Span::raw("> "),
        Span::raw(before),
        Span::styled(
            cursor_char.to_string(),
            Style::default().bg(Color::White).fg(Color::Black),
        ),
        Span::raw(after),
    ]);

    let footer_text = vec![
        input_line,
        Line::from(Span::styled(
            "[ ] pages  Alt+H help  q quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL))
}

pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut super::App,
    metro_event_rx: mpsc::Receiver<MetroEvent>,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        while let Ok(event) = metro_event_rx.try_recv() {
            match event {
                MetroEvent::ExecuteScript(index) => {
                    app.execute_script(index);
                }
            }
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let is_help = app.current_page == Page::Help;
                let has_alt = key.modifiers.contains(KeyModifiers::ALT);

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if !has_alt => {
                        return Ok(());
                    }
                    KeyCode::Char('[') => {
                        app.prev_page();
                    }
                    KeyCode::Char(']') => {
                        app.next_page();
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
                        app.help_scroll = app.help_scroll.saturating_add(1).min(HELP_LINES.len().saturating_sub(1));
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
                        if app.pattern_cursor.0 < 3 {
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
                    KeyCode::Enter if !is_help && app.is_script_page() => {
                        app.save_line();
                    }
                    KeyCode::Enter if !is_help && app.current_page != Page::Pattern => {
                        app.execute_command();
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
                    KeyCode::Left if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_right();
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
