use ratatui::{prelude::*, widgets::*};

use crate::types::{MeterData, Page, NAVIGABLE_PAGES};

// 8-level vertical bar characters for meter display
const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

fn level_to_char(level: f32) -> char {
    let clamped = level.clamp(0.0, 1.0);
    let idx = (clamped * 8.0).round() as usize;
    METER_CHARS[idx.min(8)]
}

fn render_meters(meter_data: &MeterData, theme: &crate::theme::Theme) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    // Left channel
    let l_color = if meter_data.clip_l { theme.error } else { theme.success };
    spans.push(Span::styled("L ", Style::default().fg(theme.secondary)));
    spans.push(Span::styled(
        level_to_char(meter_data.peak_l).to_string(),
        Style::default().fg(l_color),
    ));

    spans.push(Span::raw("  "));

    // Right channel
    let r_color = if meter_data.clip_r { theme.error } else { theme.success };
    spans.push(Span::styled("R ", Style::default().fg(theme.secondary)));
    spans.push(Span::styled(
        level_to_char(meter_data.peak_r).to_string(),
        Style::default().fg(r_color),
    ));

    spans
}

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

    // Add meters after TR
    spans.push(Span::raw("  "));
    spans.extend(render_meters(&app.meter_data, &app.theme));

    // Build right-aligned border title parts
    let mut title_parts = Vec::new();

    // Add REC indicator if recording
    if app.recording {
        let duration = app.recording_start
            .map(|start| start.elapsed().as_secs())
            .unwrap_or(0);
        let mins = duration / 60;
        let secs = duration % 60;
        title_parts.push(Span::styled(
            format!("● REC {:02}:{:02}", mins, secs),
            Style::default().fg(app.theme.error).add_modifier(Modifier::BOLD),
        ));
    }

    // Add CPU indicator if enabled
    if app.show_cpu {
        // Add separator if REC is also showing
        if !title_parts.is_empty() {
            title_parts.push(Span::raw("  "));
        }

        let cpu_percent = app.cpu_data.avg_cpu as u32;
        let cpu_color = if app.cpu_data.avg_cpu >= 80.0 {
            app.theme.error
        } else {
            app.theme.secondary
        };

        title_parts.push(Span::styled(
            format!("CPU {}%", cpu_percent),
            Style::default().fg(cpu_color),
        ));
    }

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .title(" MONOKIT ")
        .title_style(Style::default().fg(app.theme.foreground));

    // Add right-aligned title if there are parts to show
    if !title_parts.is_empty() {
        // Add leading space for visual separation
        let mut title_spans = vec![Span::raw(" ")];
        title_spans.extend(title_parts);
        title_spans.push(Span::raw(" "));

        block = block.title(
            ratatui::widgets::block::Title::from(Line::from(title_spans))
                .alignment(Alignment::Right)
        );
    }

    Paragraph::new(Line::from(spans))
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(block)
}
