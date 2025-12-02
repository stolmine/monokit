use ratatui::{prelude::*, widgets::*};

pub fn render_variables_page(app: &crate::App) -> Paragraph<'static> {
    let mut lines = vec![];

    let global_label = Span::styled(" GLOBAL        COUNTERS", Style::default().fg(app.theme.label));
    lines.push(Line::from(global_label));

    let row1_spans = vec![
        Span::styled(" A: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.a), Style::default().fg(app.theme.foreground)),
        Span::styled(" X: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.x), Style::default().fg(app.theme.foreground)),
        Span::styled(" N1: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} ", app.counters.values[0]), Style::default().fg(app.theme.foreground)),
        Span::styled(format!("[{}..{}]", app.counters.min[0], app.counters.max[0]), Style::default().fg(app.theme.secondary)),
    ];
    lines.push(Line::from(row1_spans));

    let row2_spans = vec![
        Span::styled(" B: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.b), Style::default().fg(app.theme.foreground)),
        Span::styled(" Y: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.y), Style::default().fg(app.theme.foreground)),
        Span::styled(" N2: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} ", app.counters.values[1]), Style::default().fg(app.theme.foreground)),
        Span::styled(format!("[{}..{}]", app.counters.min[1], app.counters.max[1]), Style::default().fg(app.theme.secondary)),
    ];
    lines.push(Line::from(row2_spans));

    let row3_spans = vec![
        Span::styled(" C: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.c), Style::default().fg(app.theme.foreground)),
        Span::styled(" Z: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.z), Style::default().fg(app.theme.foreground)),
        Span::styled(" N3: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} ", app.counters.values[2]), Style::default().fg(app.theme.foreground)),
        Span::styled(format!("[{}..{}]", app.counters.min[2], app.counters.max[2]), Style::default().fg(app.theme.secondary)),
    ];
    lines.push(Line::from(row3_spans));

    let row4_spans = vec![
        Span::styled(" D: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.d), Style::default().fg(app.theme.foreground)),
        Span::styled(" T: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<4}", app.variables.t), Style::default().fg(app.theme.foreground)),
        Span::styled(" N4: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} ", app.counters.values[3]), Style::default().fg(app.theme.foreground)),
        Span::styled(format!("[{}..{}]", app.counters.min[3], app.counters.max[3]), Style::default().fg(app.theme.secondary)),
    ];
    lines.push(Line::from(row4_spans));

    lines.push(Line::from(""));

    let locals_label = Span::styled(" LOCALS (J, K)", Style::default().fg(app.theme.label));
    lines.push(Line::from(locals_label));

    // Row 1: scripts 1, 5, M
    let s1 = &app.scripts.scripts[0];
    let s5 = &app.scripts.scripts[4];
    let sm = &app.scripts.scripts[8];
    lines.push(Line::from(vec![
        Span::styled(" 1: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<3}{:<4}", s1.j, s1.k), Style::default().fg(app.theme.foreground)),
        Span::styled(" 5: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<3}{:<4}", s5.j, s5.k), Style::default().fg(app.theme.foreground)),
        Span::styled(" M: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} {}", sm.j, sm.k), Style::default().fg(app.theme.foreground)),
    ]));

    // Row 2: scripts 2, 6, I
    let s2 = &app.scripts.scripts[1];
    let s6 = &app.scripts.scripts[5];
    let si = &app.scripts.scripts[9];
    lines.push(Line::from(vec![
        Span::styled(" 2: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<3}{:<4}", s2.j, s2.k), Style::default().fg(app.theme.foreground)),
        Span::styled(" 6: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<3}{:<4}", s6.j, s6.k), Style::default().fg(app.theme.foreground)),
        Span::styled(" I: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} {}", si.j, si.k), Style::default().fg(app.theme.foreground)),
    ]));

    // Row 3: scripts 3, 7
    let s3 = &app.scripts.scripts[2];
    let s7 = &app.scripts.scripts[6];
    lines.push(Line::from(vec![
        Span::styled(" 3: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<3}{:<4}", s3.j, s3.k), Style::default().fg(app.theme.foreground)),
        Span::styled(" 7: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} {}", s7.j, s7.k), Style::default().fg(app.theme.foreground)),
    ]));

    // Row 4: scripts 4, 8
    let s4 = &app.scripts.scripts[3];
    let s8 = &app.scripts.scripts[7];
    lines.push(Line::from(vec![
        Span::styled(" 4: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{:<3}{:<4}", s4.j, s4.k), Style::default().fg(app.theme.foreground)),
        Span::styled(" 8: ", Style::default().fg(app.theme.secondary)),
        Span::styled(format!("{} {}", s8.j, s8.k), Style::default().fg(app.theme.foreground)),
    ]));

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" VARIABLES ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}
