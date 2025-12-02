use ratatui::{prelude::*, widgets::*};

pub fn render_notes_page(app: &crate::App) -> Paragraph<'static> {
    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &app.notes.lines[i];
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
            let (normal_color, highlight_color) = if is_selected {
                (app.theme.highlight_fg, app.theme.highlight_fg)
            } else {
                (app.theme.secondary, app.theme.foreground)
            };

            let mut spans = vec![Span::styled("  ", Style::default().fg(normal_color))];
            spans.push(Span::styled(line_content.clone(), Style::default().fg(highlight_color)));

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

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" NOTES ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
