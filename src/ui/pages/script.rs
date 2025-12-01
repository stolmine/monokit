use ratatui::{prelude::*, widgets::*};
use crate::ui::state_highlight::highlight_stateful_operators;

pub fn render_script_page(app: &crate::App, num: u8) -> Paragraph<'static> {
    let script_index = (num - 1) as usize;
    let script = app.scripts.get_script(script_index);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(app.theme.secondary)),
            ]));
        } else {
            let highlighted = highlight_stateful_operators(
                line_content,
                script_index,
                &app.patterns.toggle_state,
            );

            let (normal_color, highlight_color) = if is_selected {
                (app.theme.highlight_fg, app.theme.success)
            } else {
                (app.theme.secondary, app.theme.foreground)
            };

            let mut spans = vec![Span::styled("  ", Style::default().fg(normal_color))];
            spans.extend(highlighted.to_spans(normal_color, highlight_color));

            if is_selected {
                lines.push(Line::from(
                    spans.into_iter()
                        .map(|span| {
                            Span::styled(span.content, span.style.bg(app.theme.highlight_bg))
                        })
                        .collect::<Vec<_>>()
                ));
            } else {
                lines.push(Line::from(spans));
            }
        }
    }

    if let Some(error_msg) = &app.script_error {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(format!("  {}", error_msg), Style::default().fg(app.theme.error)),
        ]));
    }

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(format!(" SCRIPT {} ", num))
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
