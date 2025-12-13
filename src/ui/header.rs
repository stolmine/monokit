use ratatui::{prelude::*, widgets::*};

use crate::types::{MeterData, Page, NAVIGABLE_PAGES};

// 8-level vertical bar characters for meter display
const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
// Header ASCII uses rounder, more readable chars than grid meters
const METER_CHARS_ASCII: [char; 9] = [' ', '.', 'o', 'O', '0', '@', '#', '#', '#'];

fn level_to_char(level: f32, ascii_mode: bool) -> char {
    let clamped = level.clamp(0.0, 1.0);
    let idx = (clamped * 8.0).round() as usize;
    let active_chars = if ascii_mode { &METER_CHARS_ASCII } else { &METER_CHARS };
    active_chars[idx.min(8)]
}

fn render_meters(meter_data: &MeterData, theme: &crate::theme::Theme, ascii_mode: bool) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    // Left channel meter character
    let l_color = if meter_data.clip_l { theme.error } else { theme.success };
    spans.push(Span::styled(
        level_to_char(meter_data.peak_l, ascii_mode).to_string(),
        Style::default().fg(l_color),
    ));

    // Right channel meter character (immediately adjacent)
    let r_color = if meter_data.clip_r { theme.error } else { theme.success };
    spans.push(Span::styled(
        level_to_char(meter_data.peak_r, ascii_mode).to_string(),
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

fn calculate_full_nav_width() -> usize {
    let mut width = 0;
    for (idx, page) in NAVIGABLE_PAGES.iter().enumerate() {
        let name = page.name();
        if idx == 0 {
            width += name.len() + 2;
        } else {
            width += name.len();
        }
        width += 1;
    }
    width
}

pub fn render_header(app: &crate::App, width: u16) -> Paragraph<'static> {
    let mut spans = vec![Span::raw(" ")];

    // Navigation label rendering based on header_level
    let full_nav_width = calculate_full_nav_width();

    if app.current_page == Page::Help {
        spans.push(Span::styled(
            "[HELP]",
            Style::default()
                .fg(app.theme.accent)
                .add_modifier(Modifier::BOLD),
        ));
        // Pad to match full nav width
        let current_width = 6;
        if current_width < full_nav_width {
            spans.push(Span::raw(" ".repeat(full_nav_width - current_width)));
        }
    } else if app.header_level <= 2 {
        // Levels 0-2: Show only current page name dynamically
        let activity = if app.show_activity {
            page_to_activity_index(&app.current_page)
                .and_then(|idx| app.script_activity[idx])
        } else {
            None
        };
        let color = app.theme.activity_color(activity, true, app.activity_hold_ms);
        let label = format!("[{}]", app.current_page.name());
        let label_width = label.len();
        spans.push(Span::styled(
            label,
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD),
        ));
        // Pad to match full nav width
        if label_width < full_nav_width {
            spans.push(Span::raw(" ".repeat(full_nav_width - label_width)));
        }
    } else {
        // Levels 3-4: Show full navigation
        for page in NAVIGABLE_PAGES.iter() {
            let is_selected = *page == app.current_page;
            let activity = if app.show_activity {
                page_to_activity_index(page)
                    .and_then(|idx| app.script_activity[idx])
            } else {
                None
            };
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

    // Build right-side content (C/P indicators + meters)
    let mut right_spans: Vec<Span<'static>> = Vec::new();
    let mut right_width = 0;

    // Multi-voice trigger indicators: show at level 2 and above if activity is enabled
    if app.show_activity && app.header_level >= 2 {
        // C = Complex oscillators (TR command)
        let c_color = app.theme.activity_color(app.trigger_activity, false, app.activity_hold_ms);
        right_spans.push(Span::styled(
            "C",
            Style::default().fg(c_color),
        ));
        right_width += 1;

        // P = Plaits (PLTR command)
        let p_color = app.theme.activity_color(app.plaits_trigger_activity, false, app.activity_hold_ms);
        right_spans.push(Span::styled(
            "P",
            Style::default().fg(p_color),
        ));
        right_spans.push(Span::raw(" "));
        right_width += 2; // "P" + space
    }

    // Meters: show at level 1 and above if header meters are enabled
    if app.show_meters_header && app.header_level >= 1 {
        right_spans.extend(render_meters(&app.meter_data, &app.theme, app.ascii_meters));
        right_width += 2; // Two meter characters
    }

    // Add padding to push right content to the right side
    if !right_spans.is_empty() {
        // Available width inside borders (subtract 2 for left and right border)
        let inner_width = width.saturating_sub(2) as usize;

        // Current nav width includes the leading space
        let nav_width = full_nav_width + 1;

        // Calculate padding needed
        // Leave 1 space after right content for visual separation from border
        let padding = inner_width.saturating_sub(nav_width + right_width + 1);

        if padding > 0 {
            spans.push(Span::raw(" ".repeat(padding)));
        }

        spans.extend(right_spans);
    }

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

    // Add BPM indicator: show at level 1 and above if show_bpm is enabled
    if app.show_bpm && app.header_level >= 1 {
        if let Ok(metro) = app.metro_state.try_lock() {
            // Add separator if REC is also showing
            if !title_parts.is_empty() {
                title_parts.push(Span::raw("  "));
            }

            let bpm = (15000.0 / metro.interval_ms as f32).round() as u32;

            let bpm_label = if let Some((_, scramble)) = app.ui_scrambles.iter().find(|(name, _)| name == "BPM") {
                scramble.current_display.clone()
            } else {
                "BPM".to_string()
            };

            title_parts.push(Span::styled(
                format!("{} {}", bpm_label, bpm),
                Style::default().fg(app.theme.secondary),
            ));
        }
    }

    // Add CPU indicator: show at level 4 OR if show_cpu is explicitly enabled
    if app.show_cpu || app.header_level >= 4 {
        // Add separator if REC or BPM is also showing
        if !title_parts.is_empty() {
            title_parts.push(Span::raw("  "));
        }

        let cpu_percent = app.cpu_data.avg_cpu as u32;
        let cpu_color = if app.cpu_data.avg_cpu >= 80.0 {
            app.theme.error
        } else {
            app.theme.secondary
        };

        let cpu_label = if let Some((_, scramble)) = app.ui_scrambles.iter().find(|(name, _)| name == "CPU") {
            scramble.current_display.clone()
        } else {
            "CPU".to_string()
        };

        title_parts.push(Span::styled(
            format!("{} {}%", cpu_label, cpu_percent),
            Style::default().fg(cpu_color),
        ));
    }

    let header_title = if let Some(scramble) = &app.header_scramble {
        format!(" {} ", scramble.current_display)
    } else {
        match app.title_mode {
            0 => " MONOKIT ".to_string(),
            1 => match &app.current_scene_name {
                Some(name) => {
                    let truncated = if name.len() > 15 {
                        format!(" {}... ", &name[..12])
                    } else {
                        format!(" {} ", name)
                    };
                    truncated
                }
                None => " [UNSAVED] ".to_string(),
            },
            _ => " MONOKIT ".to_string(),
        }
    };

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .title(header_title)
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
