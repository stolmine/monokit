use ratatui::{prelude::*, widgets::*};
use ratatui::widgets::block::{Title, Position};
use ratatui::layout::Alignment;
use crate::ui::state_highlight::{highlight_stateful_operators, apply_conditional_activity};
use crate::ui::search_highlight::highlight_matches_in_line;
use crate::types::{Page, SearchScope};

pub fn render_metro_page(app: &crate::App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 15000.0 / state.interval_ms as f32;
    let status = if state.active { "ON" } else { "OFF" };
    let status_color = if state.active {
        app.theme.success
    } else {
        app.theme.error
    };

    let label_color = app.theme.label;
    let fg = app.theme.foreground;
    let mut text = Vec::new();

    text.push(Line::from(vec![
        Span::styled("  M ", Style::default().fg(label_color)),
        Span::styled(format!("{}MS", state.interval_ms), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(format!("{}BPM", bpm.round() as u32), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));

    let should_highlight_search = app.search_mode && !app.search_query.is_empty();
    let script_index = 8;
    let current_match_line_col = if should_highlight_search && !app.search_matches.is_empty() {
        let current_match = &app.search_matches[app.search_current_match];
        if matches!(current_match.scope, SearchScope::Script)
            && current_match.page == Page::Metro
            && current_match.page_index == script_index
        {
            Some((current_match.line_index, current_match.column_start))
        } else {
            None
        }
    } else {
        None
    };

    let metro_script = app.scripts.get_script(script_index);
    for i in 0..8 {
        let line_content = &metro_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if line_content.is_empty() {
            if is_selected {
                text.push(Line::from(vec![
                    Span::styled("  ", Style::default().bg(app.theme.highlight_bg)),
                ]));
            } else {
                text.push(Line::from(vec![
                    Span::styled("  ", Style::default().fg(app.theme.secondary)),
                ]));
            }
        } else {
            let (normal_color, highlight_color) = if is_selected {
                (app.theme.highlight_fg, app.theme.success)
            } else {
                (app.theme.secondary, app.theme.foreground)
            };

            let mut spans = vec![Span::raw("  ")];

            if should_highlight_search {
                let current_col = if current_match_line_col.map(|(l, _)| l) == Some(i) {
                    current_match_line_col.map(|(_, c)| c)
                } else {
                    None
                };

                let search_segments = highlight_matches_in_line(line_content, &app.search_query, current_col);

                for (segment_text, is_match, is_current) in search_segments {
                    if app.show_seq_highlight {
                        let segment_highlighted = highlight_stateful_operators(
                            &segment_text,
                            script_index,
                            &app.patterns.toggle_state,
                            &app.patterns.toggle_last_value,
                        );

                        for segment_span in segment_highlighted.to_spans(normal_color, highlight_color) {
                            let mut style = segment_span.style;
                            if is_current {
                                style = style.bg(app.theme.highlight_bg).fg(app.theme.highlight_fg);
                            } else if is_match {
                                style = style.fg(app.theme.accent);
                            }
                            spans.push(Span::styled(segment_span.content, style));
                        }
                    } else {
                        let mut style = Style::default().fg(normal_color);
                        if is_current {
                            style = style.bg(app.theme.highlight_bg).fg(app.theme.highlight_fg);
                        } else if is_match {
                            style = style.fg(app.theme.accent);
                        }
                        spans.push(Span::styled(segment_text, style));
                    }
                }
            } else if app.show_seq_highlight {
                let highlighted = highlight_stateful_operators(
                    line_content,
                    script_index,
                    &app.patterns.toggle_state,
                    &app.patterns.toggle_last_value,
                );

                if app.show_conditional_highlight {
                    let conditional_spans = apply_conditional_activity(
                        highlighted,
                        &app.conditional_segments[script_index][i],
                        &app.theme,
                        app.activity_hold_ms,
                        is_selected,
                    );
                    spans.extend(conditional_spans);
                } else {
                    spans.extend(highlighted.to_spans(normal_color, highlight_color));
                }
            } else {
                spans.push(Span::styled(line_content.to_string(), Style::default().fg(normal_color)));
            }

            let mut line = Line::from(spans);
            if is_selected {
                line = line.style(Style::default().bg(app.theme.highlight_bg));
            }
            text.push(line);
        }
    }

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .title(" METRO ")
        .title_style(Style::default().fg(app.theme.foreground));

    if let Some(error_msg) = &app.script_error {
        block = block.title(
            Title::from(Span::styled(
                format!(" {} ", error_msg),
                Style::default().fg(app.theme.error)
            ))
            .alignment(Alignment::Left)
            .position(Position::Bottom)
        );
    }

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(block)
}
