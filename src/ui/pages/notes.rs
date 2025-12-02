use ratatui::{prelude::*, widgets::*};

pub fn render_notes_page(app: &crate::App) -> Paragraph<'static> {
    let lines: Vec<Line> = app
        .notes
        .lines()
        .map(|line| Line::from(Span::styled(format!("  {}", line), Style::default().fg(app.theme.foreground))))
        .collect();

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
