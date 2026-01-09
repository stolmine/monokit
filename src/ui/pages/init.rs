use ratatui::{prelude::*, widgets::*};
use ratatui::widgets::block::{Title, Position};
use ratatui::layout::Alignment;

pub fn render_init_page(app: &crate::App) -> Paragraph<'static> {
    let mut lines = vec![Line::from("")];

    let rendered_lines = super::script_renderer::render_script_lines(
        app.scripts.get_script(9),
        9,
        app.selected_line,
        true,
        Some(crate::types::Page::Init),
        app,
        &app.patterns.toggle_state,
        &app.patterns.toggle_last_value,
        &app.patterns.direct_validation,
    );
    lines.extend(rendered_lines);

    let title = if app.script_mutes.muted[9] {
        " INIT [MUTED] "
    } else {
        " INIT "
    };

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .title(title)
        .title_style(Style::default().fg(app.theme.foreground));

    if let Some(error_msg) = &app.script_error {
        let display_msg = format!(" ERROR: {} ", error_msg.to_uppercase());
        block = block.title(
            Title::from(Span::styled(
                display_msg,
                Style::default().fg(app.theme.error)
            ))
            .alignment(Alignment::Left)
            .position(Position::Bottom)
        );
    }

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(block)
}
