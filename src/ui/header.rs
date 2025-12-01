use ratatui::{prelude::*, widgets::*};

use crate::types::{Page, NAVIGABLE_PAGES};

fn page_to_activity_index(page: &Page) -> Option<usize> {
    match page {
        Page::Script1 => Some(0),
        Page::Script2 => Some(1),
        Page::Script3 => Some(2),
        Page::Script4 => Some(3),
        Page::Script5 => Some(4),
        Page::Script6 => Some(5),
        Page::Script7 => Some(6),
        Page::Script8 => Some(7),
        Page::Metro => Some(8),
        Page::Init => Some(9),
        _ => None,
    }
}

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
            let is_selected = *page == app.current_page;
            let activity = page_to_activity_index(page)
                .and_then(|idx| app.script_activity[idx]);
            let color = app.theme.activity_color(activity, is_selected, app.activity_hold_ms);

            if is_selected {
                spans.push(Span::styled(
                    format!("[{}]", page.name()),
                    Style::default()
                        .fg(color)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    page.name().to_string(),
                    Style::default().fg(color),
                ));
            }
            spans.push(Span::raw(" "));
        }
    }

    let tr_color = app.theme.activity_color(app.trigger_activity, false, app.activity_hold_ms);
    spans.push(Span::raw("  "));
    spans.push(Span::styled(
        "TR",
        Style::default().fg(tr_color),
    ));

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
