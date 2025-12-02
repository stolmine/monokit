use ratatui::{prelude::*, widgets::*};
use crate::ui::state_highlight::highlight_stateful_operators;

pub fn render_init_page(app: &crate::App) -> Paragraph<'static> {
    let init_script = app.scripts.get_script(9);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &init_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if line_content.is_empty() {
            if is_selected {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default().bg(app.theme.highlight_bg)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default().fg(app.theme.secondary)),
                ]));
            }
        } else {
            let highlighted = highlight_stateful_operators(
                line_content,
                9,
                &app.patterns.toggle_state,
            );

            let (normal_color, highlight_color) = if is_selected {
                (app.theme.highlight_fg, app.theme.success)
            } else {
                (app.theme.secondary, app.theme.foreground)
            };

            let mut span_vec = vec![Span::styled("  ", Style::default().fg(normal_color))];

            let content_spans = highlighted.to_spans(normal_color, highlight_color);
            span_vec.extend(content_spans);

            if is_selected {
                let styled_line = Line::from(
                    span_vec.into_iter()
                        .map(|span| {
                            Span::styled(span.content, span.style.bg(app.theme.highlight_bg))
                        })
                        .collect::<Vec<_>>()
                );
                lines.push(styled_line);
            } else {
                lines.push(Line::from(span_vec));
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
                .title(" INIT ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
