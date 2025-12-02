use ratatui::{prelude::*, widgets::*};

pub fn render_footer(app: &crate::App) -> Paragraph<'static> {
    let footer_text = if app.search_mode {
        let query = &app.search_query;
        let pos = app.search_cursor;
        let fg = app.theme.foreground;
        let accent = app.theme.accent;

        let before: String = query.chars().take(pos).collect();
        let cursor_char = query.chars().nth(pos).unwrap_or(' ');
        let after: String = query.chars().skip(pos + 1).collect();

        let total_matches = app.search_matches.len();
        let current_index = if total_matches > 0 {
            app.search_current_match + 1
        } else {
            0
        };
        let match_count = format!("[{}/{}]", current_index, total_matches);

        let input_line = Line::from(vec![
            Span::styled("/ ", Style::default().fg(accent)),
            Span::styled(before, Style::default().fg(fg)),
            Span::styled(
                cursor_char.to_string(),
                Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg),
            ),
            Span::styled(after, Style::default().fg(fg)),
            Span::raw("  "),
            Span::styled(match_count, Style::default().fg(app.theme.secondary)),
        ]);

        vec![
            input_line,
            Line::from(Span::styled(
                "ENTER NEXT  SHIFT+ENTER PREV  ESC EXIT",
                Style::default().fg(app.theme.secondary),
            )),
        ]
    } else {
        let input = &app.input;
        let pos = app.cursor_position;
        let fg = app.theme.foreground;

        let before: String = input.chars().take(pos).collect();
        let cursor_char = input.chars().nth(pos).unwrap_or(' ');
        let after: String = input.chars().skip(pos + 1).collect();

        let input_line = Line::from(vec![
            Span::styled("> ", Style::default().fg(fg)),
            Span::styled(before, Style::default().fg(fg)),
            Span::styled(
                cursor_char.to_string(),
                Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg),
            ),
            Span::styled(after, Style::default().fg(fg)),
        ]);

        vec![
            input_line,
            Line::from(Span::styled(
                "[ ] NAV  ESC HELP  F1-F12  'Q' EXIT",
                Style::default().fg(app.theme.secondary),
            )),
        ]
    };

    Paragraph::new(footer_text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
        )
}
