use ratatui::{prelude::*, widgets::*};
use crate::types::SPECTRUM_BANDS;

pub fn render_live_page(app: &crate::App, width: usize, height: usize) -> Paragraph<'static> {
    if app.show_grid_view {
        render_grid_view(app, width, height)
    } else {
        render_repl_view(app, height)
    }
}

fn render_repl_view(app: &crate::App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    // Calculate the window of output to show based on scroll offset
    // output_scroll=0 means show the most recent (bottom), higher values scroll up
    let total_lines = app.output.len();
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll_offset = app.output_scroll.min(max_scroll);

    let end_idx = total_lines.saturating_sub(scroll_offset);
    let start_idx = end_idx.saturating_sub(visible_lines);

    let fg = app.theme.foreground;
    let text: Vec<Line> = app.output[start_idx..end_idx]
        .iter()
        .map(|line| Line::from(Span::styled(format!("  {}", line), Style::default().fg(fg))))
        .collect();

    // Show scroll indicator in title if scrolled up
    let title = if scroll_offset > 0 {
        format!(" LIVE [↑{}] ", scroll_offset)
    } else {
        " LIVE ".to_string()
    };

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

fn render_grid_view(app: &crate::App, _width: usize, height: usize) -> Paragraph<'static> {
    use crate::types::GRID_ICONS;

    let mut lines = vec![];

    // Content height (excluding borders)
    let content_height = height.saturating_sub(2);

    // Total content height: 6 grid rows + 1 meter label + 2 spectrum rows + 1 spectrum label = 10
    let total_content_height = 10;

    // Calculate vertical padding for centering
    let top_pad_count = content_height.saturating_sub(total_content_height) / 2;

    let icon_spacing = "   ";  // 3 spaces between icons

    // Add top padding for vertical centering
    for _ in 0..top_pad_count {
        lines.push(Line::from(""));
    }

    // Render 6 rows of parameter grid with 6-row meters on right
    let meter_rows = 6;

    for row in 0..6 {
        let mut spans = vec![];

        // Render grid icons
        for col in 0..8 {
            let idx = row * 8 + col;
            let activity = app.param_activity.timestamps[idx];
            let color = app.theme.activity_color(activity, false, app.activity_hold_ms);
            let icon = GRID_ICONS[idx];
            spans.push(Span::styled(format!("{}", icon), Style::default().fg(color)));
            if col < 7 {
                spans.push(Span::raw(icon_spacing));
            }
        }

        // Space before meters
        spans.push(Span::raw("  "));

        // Add 2-char wide L/R meters (full height, matching grid)
        let (l_char, l_color) = get_meter_char_and_color_scaled(app.meter_data.peak_l, row, meter_rows, app.meter_data.clip_l, &app.theme);
        let (r_char, r_color) = get_meter_char_and_color_scaled(app.meter_data.peak_r, row, meter_rows, app.meter_data.clip_r, &app.theme);

        spans.push(Span::styled(format!("{}{}", l_char, l_char), Style::default().fg(l_color)));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(format!("{}{}", r_char, r_char), Style::default().fg(r_color)));

        lines.push(Line::from(spans).alignment(Alignment::Center));
    }

    // Meter labels row
    let mut meter_label = vec![];
    meter_label.push(Span::raw("                             "));  // Grid space (29 chars)
    meter_label.push(Span::raw("  "));
    meter_label.push(Span::styled("L ", Style::default().fg(app.theme.label)));
    meter_label.push(Span::raw(" "));
    meter_label.push(Span::styled("R ", Style::default().fg(app.theme.label)));
    lines.push(Line::from(meter_label).alignment(Alignment::Center));

    // Multi-row spectrum (2 rows tall)
    // 2 rows × 8 sub-levels per char = 16 levels of resolution
    // Grid+meters width = 36 chars, spectrum = 30 chars, CPU = 2 chars, spacing = 1 char
    let spectrum_rows = 2;
    let spectrum_chars: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // CPU percentage and color
    let cpu_percent = app.cpu_data.avg_cpu as u32;
    let cpu_color = if app.cpu_data.avg_cpu >= 80.0 {
        app.theme.error
    } else {
        app.theme.secondary
    };

    for spec_row in 0..spectrum_rows {
        let mut spectrum_spans = vec![];

        for i in 0..SPECTRUM_BANDS {
            // Apply logarithmic scaling for better visual response
            // sqrt gives a moderate curve between linear and full log
            let raw = app.spectrum_data.bands[i];
            let scaled = (raw * 3.0).sqrt().min(1.0);

            let (spec_char, spec_color) = get_spectrum_char_for_row(
                scaled,
                spec_row,
                spectrum_rows,
                &spectrum_chars,
                &app.theme
            );

            // Double each character for 2-char width
            spectrum_spans.push(Span::styled(
                format!("{}{}", spec_char, spec_char),
                Style::default().fg(spec_color)
            ));
        }

        // Right column: CPU percentage on bottom row only (aligned with spectrum bottom)
        // Layout: spectrum (30) + gap (1) + right-aligned percentage (5) = 36
        if spec_row == spectrum_rows - 1 {
            // Bottom row: show CPU percentage, right-aligned
            let cpu_text = format!("{:>5}%", cpu_percent);
            spectrum_spans.push(Span::styled(cpu_text, Style::default().fg(cpu_color)));
        } else {
            // Top row: empty space
            spectrum_spans.push(Span::raw("      "));  // 6 spaces
        }

        lines.push(Line::from(spectrum_spans).alignment(Alignment::Center));
    }

    // Label row: SPECTRUM on left, CPU on right (same row)
    let mut spec_label = vec![];
    spec_label.push(Span::styled("SPECTRUM", Style::default().fg(app.theme.label)));
    // Pad between labels: 36 - 8 (SPECTRUM) - 3 (CPU) = 25 spaces
    spec_label.push(Span::raw("                         "));  // 25 spaces
    spec_label.push(Span::styled("CPU", Style::default().fg(app.theme.label)));
    lines.push(Line::from(spec_label).alignment(Alignment::Center));

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" LIVE ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}


fn get_spectrum_char_for_row(
    level: f32,
    row: usize,
    total_rows: usize,
    chars: &[char; 9],
    theme: &crate::theme::Theme
) -> (char, ratatui::style::Color) {
    // Row 0 = top, row (total_rows-1) = bottom
    // Each row covers 1/total_rows of the range
    let row_height = 1.0 / total_rows as f32;
    let row_bottom = (total_rows - 1 - row) as f32 * row_height;
    let row_top = row_bottom + row_height;

    if level >= row_top {
        // Full block - level is above this row
        (chars[8], theme.success)
    } else if level > row_bottom {
        // Partial block - level is within this row
        let fill = (level - row_bottom) / row_height;
        let char_idx = ((fill * 8.0).round() as usize).clamp(1, 8);
        (chars[char_idx], theme.success)
    } else {
        // Empty
        (' ', theme.secondary)
    }
}

fn get_meter_char_and_color_scaled(
    peak: f32,
    meter_row: usize,
    total_rows: usize,
    is_clipping: bool,
    theme: &crate::theme::Theme
) -> (char, ratatui::style::Color) {
    const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // Each row covers 1/total_rows of the range
    let row_height = 1.0 / total_rows as f32;
    let row_bottom = (total_rows - 1 - meter_row) as f32 * row_height;
    let row_top = row_bottom + row_height;

    let color = if is_clipping { theme.error } else { theme.success };

    if peak >= row_top {
        (METER_CHARS[8], color)
    } else if peak > row_bottom {
        let fill = (peak - row_bottom) / row_height;
        let char_idx = ((fill * 8.0).round() as usize).clamp(1, 8);
        (METER_CHARS[char_idx], color)
    } else {
        (' ', theme.secondary)
    }
}

#[allow(dead_code)]
fn get_meter_char_and_color(peak: f32, meter_row: usize, is_clipping: bool, theme: &crate::theme::Theme) -> (char, ratatui::style::Color) {
    const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // 8 rows: row 0 = top (87.5%-100%), row 7 = bottom (0%-12.5%)
    // Each row covers 12.5% of the range
    let row_bottom = (7 - meter_row) as f32 / 8.0;  // row 7 -> 0.0, row 0 -> 0.875
    let row_top = row_bottom + 0.125;               // row 7 -> 0.125, row 0 -> 1.0

    let color = if is_clipping { theme.error } else { theme.success };

    if peak >= row_top {
        // Full block - level is above this row
        (METER_CHARS[8], color)
    } else if peak > row_bottom {
        // Partial block - level is within this row
        let fill = (peak - row_bottom) / 0.125;  // 0.0 to 1.0 within row
        let char_idx = ((fill * 8.0).round() as usize).clamp(1, 8);
        (METER_CHARS[char_idx], color)
    } else {
        // Empty - level is below this row
        (' ', theme.secondary)
    }
}

