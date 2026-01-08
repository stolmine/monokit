use ratatui::prelude::*;
use ratatui::style::{Color, Style};

use crate::types::LineSegmentActivity;
use crate::theme::Theme;

use super::types::HighlightedLine;

pub fn apply_conditional_activity(
    highlighted: HighlightedLine,
    conditional_activity: &LineSegmentActivity,
    theme: &Theme,
    hold_ms: f32,
    is_selected: bool,
) -> Vec<Span<'static>> {
    let (normal_color, highlight_color) = if is_selected {
        (theme.highlight_fg, theme.success)
    } else {
        (theme.secondary, theme.foreground)
    };

    if conditional_activity.segments.is_empty() {
        return highlighted.segments.iter().map(|seg| {
            let color = if seg.is_highlighted {
                highlight_color
            } else {
                normal_color
            };
            Span::styled(seg.text.clone(), Style::default().fg(color))
        }).collect();
    }

    let active_conds: Vec<_> = conditional_activity.segments.iter()
        .filter_map(|cond_seg| {
            let elapsed_ms = cond_seg.timestamp.elapsed().as_millis() as f32;
            let is_visible = elapsed_ms < hold_ms + crate::theme::ACTIVITY_DECAY_MS;
            if is_visible {
                let progress = if elapsed_ms < hold_ms {
                    0.0
                } else {
                    let decay_elapsed = (elapsed_ms - hold_ms) / crate::theme::ACTIVITY_DECAY_MS;
                    1.0 - (1.0 - decay_elapsed.min(1.0)).powi(3)
                };
                Some((cond_seg.start, cond_seg.end, progress))
            } else {
                None
            }
        })
        .collect();

    let mut result_spans = Vec::new();
    let mut char_pos = 0;

    for segment in &highlighted.segments {
        let segment_start = char_pos;
        let segment_end = char_pos + segment.text.len();
        let segment_text = &segment.text;

        let base_color = if segment.is_highlighted {
            highlight_color
        } else {
            normal_color
        };

        let overlapping: Vec<_> = active_conds.iter()
            .filter(|(start, end, _)| *start < segment_end && *end > segment_start)
            .collect();

        if overlapping.is_empty() {
            result_spans.push(Span::styled(segment_text.clone(), Style::default().fg(base_color)));
        } else {
            let mut pos = 0;
            let mut boundaries: Vec<usize> = Vec::new();

            for (cond_start, cond_end, _) in &overlapping {
                if *cond_start > segment_start && *cond_start < segment_end {
                    boundaries.push(*cond_start - segment_start);
                }
                if *cond_end > segment_start && *cond_end < segment_end {
                    boundaries.push(*cond_end - segment_start);
                }
            }
            boundaries.sort();
            boundaries.dedup();
            boundaries.push(segment_text.len());

            for boundary in boundaries {
                if boundary <= pos {
                    continue;
                }

                let slice = &segment_text[pos..boundary];
                let slice_abs_start = segment_start + pos;
                let slice_abs_end = segment_start + boundary;

                let cond_progress = overlapping.iter()
                    .filter(|(start, end, _)| *start <= slice_abs_start && *end >= slice_abs_end)
                    .map(|(_, _, progress)| *progress)
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

                let color = if let Some(progress) = cond_progress {
                    let active_color = if is_selected {
                        theme.success
                    } else {
                        theme.foreground
                    };
                    Theme::lerp_color(active_color, base_color, progress)
                } else {
                    base_color
                };

                result_spans.push(Span::styled(slice.to_string(), Style::default().fg(color)));
                pos = boundary;
            }
        }

        char_pos = segment_end;
    }

    result_spans
}
