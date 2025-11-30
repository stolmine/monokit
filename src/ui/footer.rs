use ratatui::{prelude::*, widgets::*};

pub fn render_footer(app: &crate::App) -> Paragraph<'static> {
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

    let footer_text = vec![
        input_line,
        Line::from(Span::styled(
            "[ ] PAGES  ESC HELP  F1-F12 NAV  'QUIT' TO EXIT",
            Style::default().fg(app.theme.secondary),
        )),
    ];

    Paragraph::new(footer_text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
        )
}
