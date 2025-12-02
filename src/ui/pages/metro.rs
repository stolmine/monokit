use ratatui::{prelude::*, widgets::*};
use crate::ui::state_highlight::highlight_stateful_operators;

pub fn render_metro_page(app: &crate::App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 15000.0 / state.interval_ms as f32; // interval is 16th note, so ×4 for quarter note
    let status = if state.active { "ON" } else { "OFF" };
    let status_color = if state.active {
        app.theme.success
    } else {
        app.theme.error
    };

    let label_color = app.theme.label;
    let fg = app.theme.foreground;
    let mut text = Vec::new();

    // Single info line: BPM, interval, and status
    text.push(Line::from(vec![
        Span::styled("  M ", Style::default().fg(label_color)),
        Span::styled(format!("{}MS", state.interval_ms), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(format!("{}BPM", bpm.round() as u32), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));

    let metro_script = app.scripts.get_script(8);
    for i in 0..8 {
        let line_content = &metro_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if line_content.is_empty() {
            if is_selected {
                text.push(Line::from(vec![
                    Span::styled("  ", Style::default().bg(app.theme.highlight_bg)),
                ]));
            } else {
                text.push(Line::from(vec![
                    Span::styled("  ", Style::default().fg(app.theme.secondary)),
                ]));
            }
        } else {
            let highlighted = highlight_stateful_operators(
                line_content,
                8,
                &app.patterns.toggle_state,
            );

            let (normal_color, highlight_color) = if is_selected {
                (app.theme.highlight_fg, app.theme.success)
            } else {
                (app.theme.secondary, app.theme.foreground)
            };

            let mut spans = vec![Span::raw("  ")];
            spans.extend(highlighted.to_spans(normal_color, highlight_color));

            let mut line = Line::from(spans);
            if is_selected {
                line = line.style(Style::default().bg(app.theme.highlight_bg));
            }
            text.push(line);
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
}
