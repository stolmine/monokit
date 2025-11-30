use ratatui::{prelude::*, widgets::*};

pub use super::help_content::{HelpCategory, HELP_CATEGORIES, HELP_LINES};

pub fn render_help_page(app: &crate::App, height: usize) -> Paragraph<'static> {
    let help_page = app.help_page;
    let scroll = app.help_scroll;
    let visible = if height > 2 { height - 2 } else { 1 };
    let fg = app.theme.foreground;
    let label = app.theme.label;

    let category = &HELP_CATEGORIES[help_page];
    let category_lines = category.lines;
    let total = category_lines.len();
    let start = scroll.min(total.saturating_sub(visible));

    let lines: Vec<Line> = category_lines
        .iter()
        .skip(start)
        .take(visible)
        .map(|&s| {
            if s.starts_with("# ") {
                let header_text = &s[2..];
                Line::from(Span::styled(
                    format!("  {}", header_text),
                    Style::default().fg(label).add_modifier(Modifier::BOLD)
                ))
            } else {
                Line::from(Span::styled(s, Style::default().fg(fg)))
            }
        })
        .collect();

    let title = format!(
        " HELP: {} ({}/{}) ",
        category.name,
        help_page + 1,
        HELP_CATEGORIES.len()
    );

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
