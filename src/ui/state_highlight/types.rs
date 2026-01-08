use ratatui::prelude::*;
use ratatui::style::{Color, Style};

#[derive(Debug, Clone)]
pub struct HighlightedSegment {
    pub text: String,
    pub is_highlighted: bool,
}

#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<HighlightedSegment>,
}

impl HighlightedLine {
    pub fn to_spans(&self, normal_color: Color, highlight_color: Color) -> Vec<Span<'static>> {
        self.segments
            .iter()
            .map(|seg| {
                let color = if seg.is_highlighted {
                    highlight_color
                } else {
                    normal_color
                };
                Span::styled(seg.text.clone(), Style::default().fg(color))
            })
            .collect()
    }
}
