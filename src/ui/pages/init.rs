use ratatui::{prelude::*, widgets::*};
use crate::ui::state_highlight::{highlight_stateful_operators, apply_conditional_activity};
use crate::ui::search_highlight::highlight_matches_in_line;
use crate::types::{Page, SearchScope};

pub fn render_init_page(app: &crate::App) -> Paragraph<'static> {
    let init_script = app.scripts.get_script(9);

    let should_highlight_search = app.search_mode && !app.search_query.is_empty();
    let script_index = 9;
    let current_match_line_col = if should_highlight_search && !app.search_matches.is_empty() {
        let current_match = &app.search_matches[app.search_current_match];
        if matches!(current_match.scope, SearchScope::Script)
            && current_match.page == Page::Init
            && current_match.page_index == script_index
        {
            Some((current_match.line_index, current_match.column_start))
        } else {
            None
        }
    } else {
        None
    };

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &init_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if line_content.is_empty() {
            if is_selected {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default().bg(app.theme.highlight_bg)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default().fg(app.theme.secondary)),
                ]));
            }
        } else {
            let (normal_color, highlight_color) = if is_selected {
                (app.theme.highlight_fg, app.theme.success)
            } else {
                (app.theme.secondary, app.theme.foreground)
            };

            let mut span_vec = vec![Span::styled("  ", Style::default().fg(normal_color))];

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
                        );

                        for segment_span in segment_highlighted.to_spans(normal_color, highlight_color) {
                            let mut style = segment_span.style;
                            if is_current {
                                style = style.bg(app.theme.highlight_bg).fg(app.theme.highlight_fg);
                            } else if is_match {
                                style = style.fg(app.theme.accent);
                            }
                            span_vec.push(Span::styled(segment_span.content, style));
                        }
                    } else {
                        let mut style = Style::default().fg(normal_color);
                        if is_current {
                            style = style.bg(app.theme.highlight_bg).fg(app.theme.highlight_fg);
                        } else if is_match {
                            style = style.fg(app.theme.accent);
                        }
                        span_vec.push(Span::styled(segment_text, style));
                    }
                }
            } else if app.show_seq_highlight {
                let highlighted = highlight_stateful_operators(
                    line_content,
                    script_index,
                    &app.patterns.toggle_state,
                );

                if app.show_conditional_highlight {
                    let conditional_spans = apply_conditional_activity(
                        highlighted,
                        &app.conditional_segments[script_index][i],
                        &app.theme,
                        app.activity_hold_ms,
                        is_selected,
                    );
                    span_vec.extend(conditional_spans);
                } else {
                    let content_spans = highlighted.to_spans(normal_color, highlight_color);
                    span_vec.extend(content_spans);
                }
            } else {
                span_vec.push(Span::styled(line_content.to_string(), Style::default().fg(normal_color)));
            }

            if is_selected {
                let styled_line = Line::from(
                    span_vec.into_iter()
                        .map(|span| {
                            Span::styled(span.content, span.style.bg(app.theme.highlight_bg))
                        })
                        .collect::<Vec<_>>()
                );
                lines.push(styled_line);
            } else {
                lines.push(Line::from(span_vec));
            }
        }
    }

    if let Some(error_msg) = &app.script_error {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(format!("  {}", error_msg), Style::default().fg(app.theme.error)),
        ]));
    }

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" INIT ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
