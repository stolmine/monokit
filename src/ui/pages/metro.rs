use ratatui::{prelude::*, widgets::*};
use ratatui::widgets::block::{Title, Position};
use ratatui::layout::Alignment;

pub fn render_metro_page(app: &crate::App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 15000.0 / state.interval_ms as f32;
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
        Span::styled("  M ", Style::default().fg(label_color)),
        Span::styled(format!("{}MS", state.interval_ms), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(format!("{}BPM", bpm.round() as u32), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));

    let rendered_lines = super::script_renderer::render_script_lines(
        app.scripts.get_script(8),
        8,
        app.selected_line,
        true,
        Some(crate::types::Page::Metro),
        app,
        &app.patterns.toggle_state,
        &app.patterns.toggle_last_value,
        &app.patterns.direct_validation,
    );
    text.extend(rendered_lines);

    let title = if app.script_mutes.muted[8] {
        " METRO [MUTED] "
    } else {
        " METRO "
    };

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .title(title)
        .title_style(Style::default().fg(app.theme.foreground));

    if let Some(error_msg) = &app.script_error {
        let display_msg = format!(" ERROR: {} ", error_msg.to_uppercase());
        block = block.title(
            Title::from(Span::styled(
                display_msg,
                Style::default().fg(app.theme.error)
            ))
            .alignment(Alignment::Left)
            .position(Position::Bottom)
        );
    }

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(block)
}
