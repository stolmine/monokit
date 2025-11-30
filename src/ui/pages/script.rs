use ratatui::{prelude::*, widgets::*};

pub fn render_script_page(app: &crate::App, num: u8) -> Paragraph<'static> {
    let script_index = (num - 1) as usize;
    let script = app.scripts.get_script(script_index);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)),
            ]));
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(app.theme.secondary)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().fg(app.theme.secondary)),
            ]));
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
