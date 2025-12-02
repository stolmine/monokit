use ratatui::{prelude::*, widgets::*};

pub fn render_live_page(app: &crate::App, height: usize) -> Paragraph<'static> {
    if app.show_grid_view {
        render_grid_view(app)
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

fn render_grid_view(app: &crate::App) -> Paragraph<'static> {
    use crate::types::GRID_ICONS;

    let mut lines = vec![];

    // Spacing: 4 chars between icons (icon + 3 spaces), 8 icons = 8 + 7*3 = 29 chars
    // Content area ~46 chars, left pad = (46 - 29) / 2 = 8
    let icon_spacing = "   ";  // 3 spaces between icons
    let left_pad = "        ";  // 8 spaces for centering

    // Add top padding for vertical centering (content ~10 lines, grid 6 lines)
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Render 6 rows of the grid
    for row in 0..6 {
        let mut spans = vec![Span::raw(left_pad)];
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

        // Add meter display to the right of the grid
        spans.push(Span::raw("  "));  // Space between grid and meters

        // Render meters for rows 0-7 (8 rows total for meter)
        if row < 6 {
            let meter_row = row;
            let (l_char, l_color) = get_meter_char_and_color(app.meter_data.peak_l, meter_row, app.meter_data.clip_l, &app.theme);
            let (r_char, r_color) = get_meter_char_and_color(app.meter_data.peak_r, meter_row, app.meter_data.clip_r, &app.theme);

            spans.push(Span::styled(l_char.to_string(), Style::default().fg(l_color)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(r_char.to_string(), Style::default().fg(r_color)));
        }

        lines.push(Line::from(spans));
    }

    // Add 2 more meter rows (rows 6-7) without grid icons
    for meter_row in 6..8 {
        let mut spans = vec![Span::raw(left_pad)];
        // Empty space where grid would be (29 chars for grid)
        spans.push(Span::raw("                             "));
        spans.push(Span::raw("  "));  // Space between grid and meters

        let (l_char, l_color) = get_meter_char_and_color(app.meter_data.peak_l, meter_row, app.meter_data.clip_l, &app.theme);
        let (r_char, r_color) = get_meter_char_and_color(app.meter_data.peak_r, meter_row, app.meter_data.clip_r, &app.theme);

        spans.push(Span::styled(l_char.to_string(), Style::default().fg(l_color)));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(r_char.to_string(), Style::default().fg(r_color)));

        lines.push(Line::from(spans));
    }

    // Add meter labels row
    let mut label_spans = vec![Span::raw(left_pad)];
    label_spans.push(Span::raw("                             "));
    label_spans.push(Span::raw("  "));
    label_spans.push(Span::styled("L", Style::default().fg(app.theme.label)));
    label_spans.push(Span::raw(" "));
    label_spans.push(Span::styled("R", Style::default().fg(app.theme.label)));
    lines.push(Line::from(label_spans));

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
