use ratatui::{prelude::*, widgets::*};

use crate::types::SCOPE_SAMPLES;
use crate::ui::braille::{samples_to_braille, samples_to_blocks, samples_to_lines, samples_to_dots, samples_to_quadrants};

/// Render the oscilloscope page with centered waveform display
///
/// Takes the actual render area to properly center content
pub fn render_scope_page(app: &crate::App, area: Rect) -> Paragraph<'static> {
    // Content area inside borders
    let content_width = area.width.saturating_sub(2) as usize;
    let content_height = area.height.saturating_sub(2) as usize;

    // Use full content height for waveform
    let waveform_height = content_height.max(1);

    // No vertical padding needed - use full height
    let top_padding = 0;

    // Prepare samples - apply rectification if unipolar mode
    let samples: Vec<f32> = if app.scope_settings.unipolar {
        // Rectify: take absolute value, map 0-1 to full height
        // We map 0->1 to 1->-1 (top to bottom) for display
        app.scope_data.samples.iter()
            .map(|&s| s.abs() * 2.0 - 1.0)
            .collect()
    } else {
        app.scope_data.samples.to_vec()
    };

    let waveform_grid = match app.scope_settings.display_mode {
        1 => samples_to_blocks(&samples, content_width, waveform_height),
        2 => samples_to_lines(&samples, content_width, waveform_height),
        3 => samples_to_dots(&samples, content_width, waveform_height),
        4 => samples_to_quadrants(&samples, content_width, waveform_height),
        _ => samples_to_braille(&samples, content_width, waveform_height),
    };

    let waveform_color = app.scope_settings.color_mode.get_color(&app.theme);

    let mut lines: Vec<Line> = Vec::new();

    // Add top padding for vertical centering
    for _ in 0..top_padding {
        lines.push(Line::from(""));
    }

    // Add waveform rows
    for row in waveform_grid {
        let row_string: String = row.into_iter().collect();
        lines.push(Line::from(Span::styled(
            row_string,
            Style::default().fg(waveform_color),
        )));
    }

    // Create info title for bottom border
    let info_title = format!(" TIME: {}MS  SAMPLES: {} ", app.scope_settings.timespan_ms, SCOPE_SAMPLES);

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" SCOPE ")
                .title_style(Style::default().fg(app.theme.foreground))
                .title(
                    ratatui::widgets::block::Title::from(
                        Span::styled(info_title, Style::default().fg(app.theme.secondary))
                    )
                    .alignment(Alignment::Right)
                    .position(ratatui::widgets::block::Position::Bottom)
                )
        )
}
