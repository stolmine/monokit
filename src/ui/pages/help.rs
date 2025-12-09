use ratatui::{prelude::*, widgets::*};
use crate::types::SearchScope;
use crate::ui::search_highlight::highlight_matches_in_line;

pub use super::help_content::{HelpCategory, HELP_CATEGORIES, HELP_LINES};

pub fn render_help_page(app: &crate::App, height: usize) -> Paragraph<'static> {
    let help_page = app.help_page;
    let scroll = app.help_scroll;
    let visible = if height > 2 { height - 2 } else { 1 };
    let fg = app.theme.foreground;
    let label = app.theme.label;
    let accent = app.theme.accent;
    let highlight_bg = app.theme.highlight_bg;
    let highlight_fg = app.theme.highlight_fg;

    let category = &HELP_CATEGORIES[help_page];
    let category_lines = category.lines;
    let total = category_lines.len();
    let start = scroll.min(total.saturating_sub(visible));

    let should_highlight = app.search_mode && !app.search_query.is_empty();

    let current_match_line_col = if should_highlight && !app.search_matches.is_empty() {
        let current_match = &app.search_matches[app.search_current_match];
        if matches!(current_match.scope, SearchScope::Help)
            && current_match.page_index == help_page
        {
            Some((current_match.line_index, current_match.column_start))
        } else {
            None
        }
    } else {
        None
    };

    let lines: Vec<Line> = category_lines
        .iter()
        .enumerate()
        .skip(start)
        .take(visible)
        .map(|(line_idx, &s)| {
            if s.starts_with("# ") {
                let header_text = &s[2..];
                Line::from(Span::styled(
                    format!("  {}", header_text),
                    Style::default().fg(label).add_modifier(Modifier::BOLD)
                ))
            } else if should_highlight {
                let current_col = if current_match_line_col.map(|(l, _)| l) == Some(line_idx) {
                    current_match_line_col.map(|(_, c)| c)
                } else {
                    None
                };

                let segments = highlight_matches_in_line(s, &app.search_query, current_col);
                let spans: Vec<Span> = segments
                    .into_iter()
                    .map(|(text, is_match, is_current)| {
                        if is_current {
                            Span::styled(text, Style::default().bg(highlight_bg).fg(highlight_fg))
                        } else if is_match {
                            Span::styled(text, Style::default().fg(accent))
                        } else {
                            Span::styled(text, Style::default().fg(fg))
                        }
                    })
                    .collect();
                Line::from(spans)
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
    let version = format!(" v{} ", env!("CARGO_PKG_VERSION"));

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
                .title_bottom(Line::from(version).right_aligned())
        )
}
