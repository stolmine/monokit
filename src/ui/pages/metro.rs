use ratatui::{prelude::*, widgets::*};

pub fn render_metro_page(app: &crate::App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 60000.0 / state.interval_ms as f32;
    let status = if state.active { "ON" } else { "OFF" };
    let status_color = if state.active {
        app.theme.success
    } else {
        app.theme.error
    };

    let label_color = app.theme.label;
    let fg = app.theme.foreground;
    let mut text = Vec::new();
    text.push(Line::from(vec![
        Span::styled("  BPM: ", Style::default().fg(label_color)),
        Span::styled(format!("{:.1}", bpm), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled("INTERVAL: ", Style::default().fg(label_color)),
        Span::styled(format!("{}MS", state.interval_ms), Style::default().fg(fg)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  STATUS: ", Style::default().fg(label_color)),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  M SCRIPT LINES:", Style::default().fg(label_color)),
    ]));

    let metro_script = app.scripts.get_script(8);
    for i in 0..8 {
        let line_content = &metro_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            text.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)),
            ]));
        } else if line_content.is_empty() {
            text.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(app.theme.secondary)),
            ]));
        } else {
            text.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().fg(app.theme.secondary)),
            ]));
        }
    }

    if let Some(error_msg) = &app.script_error {
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled(format!("  {}", error_msg), Style::default().fg(app.theme.error)),
        ]));
    }

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" METRO ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
        .wrap(Wrap { trim: false })
}
