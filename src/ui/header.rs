use ratatui::{prelude::*, widgets::*};

use crate::types::{Page, NAVIGABLE_PAGES};

pub fn render_header(app: &crate::App) -> Paragraph<'static> {
    let mut spans = vec![Span::raw(" ")];

    if app.current_page == Page::Help {
        spans.push(Span::styled(
            "[HELP]",
            Style::default()
                .fg(app.theme.accent)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        for page in NAVIGABLE_PAGES.iter() {
            if *page == app.current_page {
                spans.push(Span::styled(
                    format!("[{}]", page.name()),
                    Style::default()
                        .fg(app.theme.accent)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    page.name().to_string(),
                    Style::default().fg(app.theme.secondary),
                ));
            }
            spans.push(Span::raw(" "));
        }
    }

    if app.recording {
        let duration = app.recording_start
            .map(|start| start.elapsed().as_secs())
            .unwrap_or(0);
        let mins = duration / 60;
        let secs = duration % 60;

        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            format!("● REC {:02}:{:02}", mins, secs),
            Style::default()
                .fg(app.theme.error)
                .add_modifier(Modifier::BOLD),
        ));
    }

    Paragraph::new(Line::from(spans))
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" MONOKIT ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
