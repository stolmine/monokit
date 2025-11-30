use ratatui::{prelude::*, widgets::*};

pub fn render_live_page(app: &crate::App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    // Calculate the window of output to show based on scroll offset
    // output_scroll=0 means show the most recent (bottom), higher values scroll up
    let total_lines = app.output.len();
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll_offset = app.output_scroll.min(max_scroll);

    let end_idx = total_lines.saturating_sub(scroll_offset);
    let start_idx = end_idx.saturating_sub(visible_lines);

    let fg = app.theme.foreground;
    let text: Vec<Line> = app.output[start_idx..end_idx]
        .iter()
        .map(|line| Line::from(Span::styled(format!("  {}", line), Style::default().fg(fg))))
        .collect();

    // Show scroll indicator in title if scrolled up
    let title = if scroll_offset > 0 {
        format!(" LIVE [↑{}] ", scroll_offset)
    } else {
        " LIVE ".to_string()
    };

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
