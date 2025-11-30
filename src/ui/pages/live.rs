use ratatui::{prelude::*, widgets::*};

pub fn render_live_page(app: &crate::App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    let start_idx = if app.output.len() > visible_lines {
        app.output.len() - visible_lines
    } else {
        0
    };

    let fg = app.theme.foreground;
    let text: Vec<Line> = app.output[start_idx..]
        .iter()
        .map(|line| Line::from(Span::styled(format!("  {}", line), Style::default().fg(fg))))
        .collect();

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" LIVE ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
