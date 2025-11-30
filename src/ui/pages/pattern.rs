use ratatui::{prelude::*, widgets::*};

pub fn render_pattern_page(app: &crate::App) -> Paragraph<'static> {
    let (cursor_pattern, cursor_step) = app.pattern_cursor;

    let visible_rows = 16;
    let scroll_offset = if cursor_step < visible_rows / 2 {
        0
    } else if cursor_step >= 64 - visible_rows / 2 {
        64 - visible_rows
    } else {
        cursor_step.saturating_sub(visible_rows / 2)
    };

    let mut lines = vec![];

    let mut header_spans = vec![Span::raw("     ")];
    for pattern_idx in 0..6 {
        let label = format!("P{}", pattern_idx);
        let style = if pattern_idx == app.patterns.working {
            Style::default().fg(app.theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.secondary)
        };
        header_spans.push(Span::styled(format!(" {:^5} ", label), style));
    }
    lines.push(Line::from(header_spans));

    let mut len_spans = vec![Span::styled(" LEN ", Style::default().fg(app.theme.secondary))];
    for pattern_idx in 0..6 {
        let pattern = &app.patterns.patterns[pattern_idx];
        len_spans.push(Span::styled(
            format!(" {:^5} ", pattern.length),
            Style::default().fg(app.theme.secondary),
        ));
    }
    lines.push(Line::from(len_spans));

    for step in scroll_offset..(scroll_offset + visible_rows).min(64) {
        let mut row_spans = vec![
            Span::styled(format!("{:3}: ", step), Style::default().fg(app.theme.secondary)),
        ];

        for pattern_idx in 0..6 {
            let pattern = &app.patterns.patterns[pattern_idx];
            let value = pattern.data[step];
            let is_cursor = cursor_pattern == pattern_idx && cursor_step == step;
            let is_playhead = pattern.index == step;
            let is_beyond_length = step >= pattern.length;

            let display = if is_cursor && !app.pattern_input.is_empty() {
                format!(" {:>5} ", app.pattern_input)
            } else {
                format!(" {:>5} ", value)
            };

            let style = if is_cursor {
                Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)
            } else if is_playhead && !is_beyond_length {
                Style::default().bg(app.theme.secondary).fg(app.theme.background)
            } else if is_beyond_length {
                Style::default().fg(app.theme.secondary)
            } else {
                Style::default()
            };

            row_spans.push(Span::styled(display, style));
        }

        lines.push(Line::from(row_spans));
    }

    let title = format!(" PATTERN ({}/64) ", cursor_step);
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
