use ratatui::{prelude::*, widgets::*};

pub fn render_live_page(app: &crate::App, height: usize) -> Paragraph<'static> {
    if app.show_grid_view {
        render_grid_view(app)
    } else {
        render_repl_view(app, height)
    }
}

fn render_repl_view(app: &crate::App, height: usize) -> Paragraph<'static> {
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

fn render_grid_view(app: &crate::App) -> Paragraph<'static> {
    use crate::types::GRID_ICONS;

    let mut lines = vec![];

    // Spacing: 4 chars between icons (icon + 3 spaces), 8 icons = 8 + 7*3 = 29 chars
    // Content area ~46 chars, left pad = (46 - 29) / 2 = 8
    let icon_spacing = "   ";  // 3 spaces between icons
    let left_pad = "        ";  // 8 spaces for centering

    // Add top padding for vertical centering (content ~10 lines, grid 6 lines)
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    for row in 0..6 {
        let mut spans = vec![Span::raw(left_pad)];
        for col in 0..8 {
            let idx = row * 8 + col;
            let activity = app.param_activity.timestamps[idx];
            let color = app.theme.activity_color(activity, false, app.activity_hold_ms);
            let icon = GRID_ICONS[idx];
            spans.push(Span::styled(format!("{}", icon), Style::default().fg(color)));
            if col < 7 {
                spans.push(Span::raw(icon_spacing));
            }
        }
        lines.push(Line::from(spans));
    }

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" LIVE ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
