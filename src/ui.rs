use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use std::sync::mpsc;
use std::time::Duration;

use crate::types::{MetroEvent, Page, NAVIGABLE_PAGES};

pub fn ui(f: &mut Frame, app: &super::App) {
    // Fill background by setting every cell - most reliable method for all terminals
    let area = f.area();
    let bg = app.theme.background;
    let fg = app.theme.foreground;
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = f.buffer_mut().get_mut(x, y);
            cell.set_bg(bg);
            cell.set_fg(fg);
        }
    }

    let is_help = app.current_page == Page::Help;
    let is_pattern = app.current_page == Page::Pattern;

    let chunks = if is_help || is_pattern {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(f.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area())
    };

    let header = render_header(app);
    f.render_widget(header, chunks[0]);

    let content = match app.current_page {
        Page::Live => render_live_page(app, chunks[1].height as usize),
        Page::Script1 => render_script_page(app, 1),
        Page::Script2 => render_script_page(app, 2),
        Page::Script3 => render_script_page(app, 3),
        Page::Script4 => render_script_page(app, 4),
        Page::Script5 => render_script_page(app, 5),
        Page::Script6 => render_script_page(app, 6),
        Page::Script7 => render_script_page(app, 7),
        Page::Script8 => render_script_page(app, 8),
        Page::Metro => render_metro_page(app),
        Page::Init => render_init_page(app),
        Page::Pattern => render_pattern_page(app),
        Page::Help => render_help_page(app, chunks[1].height as usize),
    };
    f.render_widget(content, chunks[1]);

    let is_pattern = app.current_page == Page::Pattern;
    if !is_help && !is_pattern {
        let footer = render_footer(app);
        f.render_widget(footer, chunks[2]);
    }
}

pub fn render_header(app: &super::App) -> Paragraph<'static> {
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
            if *page == app.current_page {
                spans.push(Span::styled(
                    format!("[{}]", page.name()),
                    Style::default()
                        .fg(app.theme.accent)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    page.name().to_string(),
                    Style::default().fg(app.theme.secondary),
                ));
            }
            spans.push(Span::raw(" "));
        }
    }

    // Add recording indicator on the right side
    if app.recording {
        let duration = app.recording_start
            .map(|start| start.elapsed().as_secs())
            .unwrap_or(0);
        let mins = duration / 60;
        let secs = duration % 60;

        // Add spacer to push recording indicator to the right
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            format!("● REC {:02}:{:02}", mins, secs),
            Style::default()
                .fg(app.theme.error)
                .add_modifier(Modifier::BOLD),
        ));
    }

    Paragraph::new(Line::from(spans))
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" MONOKIT ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

pub fn render_metro_page(app: &super::App) -> Paragraph<'static> {
    let state = app.metro_state.lock().unwrap();
    let bpm = 60000.0 / state.interval_ms as f32;
    let status = if state.active { "ON" } else { "OFF" };
    let status_color = if state.active {
        app.theme.success
    } else {
        app.theme.error
    };

    let label_color = app.theme.label;
    let fg = app.theme.foreground;
    let mut text = Vec::new();
    text.push(Line::from(vec![
        Span::styled("  BPM: ", Style::default().fg(label_color)),
        Span::styled(format!("{:.1}", bpm), Style::default().fg(fg)),
        Span::raw("  "),
        Span::styled("INTERVAL: ", Style::default().fg(label_color)),
        Span::styled(format!("{}MS", state.interval_ms), Style::default().fg(fg)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  STATUS: ", Style::default().fg(label_color)),
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));
    text.push(Line::from(""));
    text.push(Line::from(vec![
        Span::styled("  M SCRIPT LINES:", Style::default().fg(label_color)),
    ]));

    let metro_script = app.scripts.get_script(8);
    for i in 0..8 {
        let line_content = &metro_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            text.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)),
            ]));
        } else if line_content.is_empty() {
            text.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(app.theme.secondary)),
            ]));
        } else {
            text.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().fg(app.theme.secondary)),
            ]));
        }
    }

    if let Some(error_msg) = &app.script_error {
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled(format!("  {}", error_msg), Style::default().fg(app.theme.error)),
        ]));
    }

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" METRO ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
        .wrap(Wrap { trim: false })
}

pub fn render_live_page(app: &super::App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    let start_idx = if app.output.len() > visible_lines {
        app.output.len() - visible_lines
    } else {
        0
    };

    let fg = app.theme.foreground;
    let text: Vec<Line> = app.output[start_idx..]
        .iter()
        .map(|line| Line::from(Span::styled(format!("  {}", line), Style::default().fg(fg))))
        .collect();

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" LIVE ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

pub fn render_script_page(app: &super::App, num: u8) -> Paragraph<'static> {
    let script_index = (num - 1) as usize;
    let script = app.scripts.get_script(script_index);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)),
            ]));
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(app.theme.secondary)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().fg(app.theme.secondary)),
            ]));
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
                .title(format!(" SCRIPT {} ", num))
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

pub fn render_init_page(app: &super::App) -> Paragraph<'static> {
    let init_script = app.scripts.get_script(9);

    let mut lines = vec![Line::from("")];

    for i in 0..8 {
        let line_content = &init_script.lines[i];
        let is_selected = app.selected_line == Some(i);

        if is_selected {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)),
            ]));
        } else if line_content.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(app.theme.secondary)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("  {}", line_content), Style::default().fg(app.theme.secondary)),
            ]));
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

pub fn render_pattern_page(app: &super::App) -> Paragraph<'static> {
    let (cursor_pattern, cursor_step) = app.pattern_cursor;

    let visible_rows = 16;
    let scroll_offset = if cursor_step < visible_rows / 2 {
        0
    } else if cursor_step >= 64 - visible_rows / 2 {
        64 - visible_rows
    } else {
        cursor_step.saturating_sub(visible_rows / 2)
    };

    let mut lines = vec![];

    let mut header_spans = vec![Span::raw("     ")];
    for pattern_idx in 0..4 {
        let label = format!("P{}", pattern_idx);
        let style = if pattern_idx == app.patterns.working {
            Style::default().fg(app.theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.secondary)
        };
        header_spans.push(Span::styled(format!(" {:^5} ", label), style));
    }
    lines.push(Line::from(header_spans));

    let mut len_spans = vec![Span::styled(" LEN ", Style::default().fg(app.theme.secondary))];
    for pattern_idx in 0..4 {
        let pattern = &app.patterns.patterns[pattern_idx];
        len_spans.push(Span::styled(
            format!(" {:^5} ", pattern.length),
            Style::default().fg(app.theme.secondary),
        ));
    }
    lines.push(Line::from(len_spans));

    for step in scroll_offset..(scroll_offset + visible_rows).min(64) {
        let mut row_spans = vec![
            Span::styled(format!("{:3}: ", step), Style::default().fg(app.theme.secondary)),
        ];

        for pattern_idx in 0..4 {
            let pattern = &app.patterns.patterns[pattern_idx];
            let value = pattern.data[step];
            let is_cursor = cursor_pattern == pattern_idx && cursor_step == step;
            let is_playhead = pattern.index == step;
            let is_beyond_length = step >= pattern.length;

            let display = if is_cursor && !app.pattern_input.is_empty() {
                format!(" {:>5} ", app.pattern_input)
            } else {
                format!(" {:>5} ", value)
            };

            let style = if is_cursor {
                Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg)
            } else if is_playhead && !is_beyond_length {
                Style::default().bg(app.theme.secondary).fg(app.theme.background)
            } else if is_beyond_length {
                Style::default().fg(app.theme.secondary)
            } else {
                Style::default()
            };

            row_spans.push(Span::styled(display, style));
        }

        lines.push(Line::from(row_spans));
    }

    let title = format!(" PATTERN ({}/64) ", cursor_step);
    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

pub const HELP_LINES: &[&str] = &[
    "",
    "  NAVIGATION",
    "  [ ]           CYCLE PAGES",
    "  ESC           TOGGLE HELP",
    "  F1-F8         SCRIPT 1-8",
    "  F9            LIVE PAGE",
    "  F10           METRO PAGE",
    "  F11           INIT PAGE",
    "  F12           PATTERN PAGE",
    "",
    "  EDITING (SCRIPT PAGES)",
    "  UP/DOWN       SELECT LINE",
    "  ENTER         SAVE LINE",
    "  CTRL+D        DUPLICATE LINE",
    "  CTRL+K        DELETE LINE",
    "  CTRL+C        COPY LINE",
    "  CTRL+X        CUT LINE",
    "  CTRL+V        PASTE LINE",
    "  CTRL+L/R      WORD MOVEMENT",
    "",
    "  TRIGGER & VOLUME",
    "  TR            TRIGGER VOICE",
    "  VOL 0-1       MASTER VOLUME",
    "  RST           RESET TO DEFAULTS",
    "  Q/QUIT/EXIT   QUIT APPLICATION",
    "",
    "  REPL UTILITIES",
    "  CLEAR         CLEAR OUTPUT HISTORY",
    "  DEBUG <0-2>   SET VERBOSITY LEVEL",
    "  DEBUG 0       SILENT (ERRORS ONLY)",
    "  DEBUG 1       IMPORTANT (METRO, PRINT)",
    "  DEBUG 2       VERBOSE (ALL PARAMS)",
    "  PRINT \"TEXT\"  OUTPUT LITERAL STRING",
    "  PRINT <EXPR>  EVALUATE & PRINT",
    "  EXAMPLE: PRINT A, PRINT ADD 1 2",
    "",
    "  VARIABLES",
    "  A B C D       GLOBAL ACCUMULATORS",
    "  X Y Z T       GLOBAL ACCUMULATORS",
    "  J K           PER-SCRIPT LOCAL VARS",
    "  I             LOOP COUNTER (IN L)",
    "  EXAMPLE: A 100 (SET), PF A (USE)",
    "",
    "  MATH (COMMAND & EXPRESSION)",
    "  ADD/+ <A> <B>   ADD",
    "  SUB/- <A> <B>   SUBTRACT",
    "  MUL/* <A> <B>   MULTIPLY",
    "  DIV// <A> <B>   DIVIDE",
    "  MOD/% <A> <B>   MODULO",
    "  MAP <V> <I1> <I2> <O1> <O2>",
    "    MAP VALUE FROM INPUT RANGE TO OUTPUT",
    "  EXAMPLE: PF MAP A 0 127 200 2000",
    "",
    "  RANDOM",
    "  RND <MAX>        0 TO MAX",
    "  RRND <MIN> <MAX> MIN TO MAX",
    "  TOSS             COIN FLIP (0/1)",
    "  EITH <A> <B>     RANDOM CHOICE A OR B",
    "  TOG <A> <B>      ALTERNATE BETWEEN A AND B",
    "",
    "  NOTE/PITCH",
    "  N <SEMI>      SEMITONES TO HZ",
    "  N 0 = C3 (131 HZ)",
    "  N 12 = C4, N 21 = A4 (440)",
    "  EXAMPLE: PF N 12, PF N ADD A 7",
    "",
    "  CONTROL FLOW (PRE OPERATORS)",
    "  IF <X>: <CMD>        IF X != 0 (TRUTHY)",
    "  IF <COND>: <CMD>     WITH COMPARISON",
    "  ELIF <COND>: <CMD>   ELSE-IF",
    "  ELSE: <CMD>          ELSE BRANCH",
    "  PROB <0-100>: <CMD>  PROBABILITY %",
    "  EV <N>: <CMD>        EVERY NTH TICK",
    "  SKIP <N>: <CMD>      SKIP EVERY NTH",
    "  L <S> <E>: <CMDS>    LOOP S TO E",
    "  CMD1; CMD2           SUB-COMMANDS",
    "",
    "  COMPARISONS (RETURN 1/0)",
    "  EZ <X>       X == 0 (EQUALS ZERO)",
    "  NZ <X>       X != 0 (NOT ZERO)",
    "  EQ <A> <B>   A == B",
    "  NE <A> <B>   A != B",
    "  GT <A> <B>   A > B",
    "  LT <A> <B>   A < B",
    "  GTE <A> <B>  A >= B",
    "  LTE <A> <B>  A <= B",
    "  ALSO: > < >= <= == != IN COND",
    "",
    "  SCENES",
    "  SAVE <NAME>   SAVE SCRIPTS+PATTERNS",
    "  LOAD <NAME>   LOAD SCENE",
    "  SCENES        LIST SAVED SCENES",
    "  DELETE <NAME> DELETE SCENE",
    "",
    "  RECORDING",
    "  REC           START RECORDING (WAV)",
    "  REC.STOP      STOP RECORDING",
    "  REC.PATH <P>  SET PATH PREFIX",
    "",
    "  BEAT REPEAT",
    "  BR.ACT <0|1>  ENABLE BEAT REPEAT",
    "  BR.LEN <0-7>  LOOP DIV (0=1/16...7=8X)",
    "  BR.REV <0|1>  REVERSE PLAYBACK",
    "  BR.WIN <1-50> WINDOW SIZE (MS)",
    "  BR.MIX <0-16383> BEAT REPEAT MIX",
    "",
    "  PITCH SHIFT",
    "  PS.MODE <0|1>   MODE (NORMAL/GRANULAR)",
    "  PS.SEMI <-24-24> SHIFT (SEMITONES)",
    "  PS.GRAIN <5-100> GRAIN SIZE (MS)",
    "  PS.MIX <0-16383> PITCH SHIFT MIX",
    "  PS.TARG <0|1>    TARGET (INPUT/OUTPUT)",
    "",
    "  OSCILLATORS",
    "  PF <HZ>       PRIMARY FREQ (20-20000)",
    "  PW <0-2>      PRIMARY WAVE (SIN/TRI/SAW)",
    "  MF <HZ>       MOD FREQ (20-20000)",
    "  MW <0-2>      MOD WAVE (SIN/TRI/SAW)",
    "",
    "  FM SYNTHESIS",
    "  FM <0-16383>  FM INDEX",
    "  FA <0-16383>  FM ENV AMOUNT",
    "  FD <MS>       FM ENV DECAY",
    "",
    "  FEEDBACK FM",
    "  FB <0-16383>  FEEDBACK AMOUNT",
    "  FBA <0-16383> FEEDBACK ENV AMOUNT",
    "  FBD <MS>      FEEDBACK ENV DECAY",
    "",
    "  DISCONTINUITY",
    "  DC <0-16383>  DISCONTINUITY AMOUNT",
    "  DA <0-16383>  DC ENV AMOUNT",
    "  DD <MS>       DC ENV DECAY",
    "  DM <0-6>      MODE (FOLD/TANH/SOFT/HARD/ASYM/RECT/CRUSH)",
    "",
    "  LO-FI",
    "  LB <1-16>     BIT DEPTH (16=CLEAN)",
    "  LS <100-48K>  SAMPLE RATE (HZ)",
    "  LM <0-16383>  LO-FI MIX",
    "",
    "  SVF FILTER",
    "  FC <HZ>       CUTOFF FREQ (20-20000)",
    "  FQ <0-16383>  RESONANCE",
    "  FT <0-3>      TYPE (LP/HP/BP/NOTCH)",
    "  FE <0-16383>  FILTER ENV AMOUNT",
    "  FED <MS>      FILTER ENV DECAY",
    "  FK <0-16383>  KEY TRACKING",
    "  MF.F <0-1>    MOD BUS -> FILTER",
    "",
    "  RING MOD",
    "  RGF <20-2000> RING MOD FREQ (HZ)",
    "  RGW <0-3>     WAVEFORM (SIN/TRI/SAW/SQR)",
    "  RGM <0-16383> RING MOD MIX",
    "",
    "  COMB RESONATOR",
    "  RF <HZ>       RESONATOR FREQ",
    "  RD <MS>       RESONATOR DECAY",
    "  RM <0-16383>  RESONATOR MIX",
    "  RK <0-16383>  RESONATOR KEY TRACK",
    "",
    "  COMPRESSOR",
    "  CT <0-16383>  THRESHOLD",
    "  CR <1-20>     RATIO",
    "  CA <1-500>    ATTACK (MS)",
    "  CL <10-2000>  RELEASE (MS)",
    "  CM <0-16383>  MAKEUP GAIN",
    "",
    "  PAN",
    "  PAN <-16K-16K> STEREO POSITION",
    "",
    "  ENVELOPES",
    "  AD <MS>       AMP DECAY",
    "  PD <MS>       PITCH DECAY",
    "  PA <0-16>     PITCH ENV AMOUNT",
    "",
    "  MOD BUS",
    "  MB <0-16383>  MOD BUS AMOUNT",
    "  TK <0-16383>  TRACKING AMOUNT",
    "  MP <0|1>      MOD -> PRIMARY FREQ",
    "  MD <0|1>      MOD -> DISCONTINUITY",
    "  MT <0|1>      MOD -> TRACKING",
    "  MA <0|1>      MOD -> AMPLITUDE",
    "",
    "  MIX",
    "  MX <0-16383>  MIX AMOUNT",
    "  MM <0|1>      MOD BUS -> MIX",
    "  ME <0|1>      ENVELOPE -> MIX",
    "",
    "  STEREO DELAY",
    "  DT <MS>       DELAY TIME (1-2000)",
    "  DF <0-16383>  DELAY FEEDBACK",
    "  DLP <HZ>      DELAY LP FILTER",
    "  DW <0-16383>  DELAY WET MIX",
    "  DS <0-16383>  DELAY STEREO WIDTH",
    "",
    "  DELAY ROUTING",
    "  D.MODE <0-2>  BYPASS/INSERT/SEND",
    "  D.TAIL <0-2>  CUT/RING/FREEZE",
    "",
    "  3-BAND EQ",
    "  EL <-24-24>   LOW SHELF (DB)",
    "  EM <-24-24>   MID PEAK (DB)",
    "  EF <200-8000> MID FREQ (HZ)",
    "  EQ <0.1-10>   MID Q",
    "  EH <-24-24>   HIGH SHELF (DB)",
    "",
    "  PLATE REVERB",
    "  RV <0-16383>  REVERB DECAY",
    "  RP <MS>       REVERB PRE-DELAY",
    "  RH <0-16383>  REVERB DAMPING (HI)",
    "  RW <0-16383>  REVERB WET MIX",
    "",
    "  REVERB ROUTING",
    "  R.MODE <0-2>  BYPASS/INSERT/SEND",
    "  R.TAIL <0-2>  CUT/RING/FREEZE",
    "",
    "  METRO",
    "  M             SHOW INTERVAL",
    "  M <MS>        SET INTERVAL",
    "  M.BPM <BPM>   SET BPM",
    "  M.ACT <0|1>   START/STOP",
    "  M.SCRIPT <1-8>  SCRIPT FOR METRO",
    "",
    "  SCRIPTS",
    "  SCRIPT <1-8>  EXECUTE STORED SCRIPT",
    "",
    "  COUNTERS",
    "  N1-N4         READ VALUE + AUTO-INCREMENT",
    "  N1.MIN <N>    SET MINIMUM (DEFAULT 0)",
    "  N1.MAX <N>    SET MAXIMUM (WRAPS BACK)",
    "  N1.RST        RESET TO MIN",
    "  EXAMPLE: N1.MIN 10; N1.MAX 14",
    "  CYCLES: 10, 11, 12, 13, 14, 10...",
    "",
    "  PATTERNS (WORKING - P.N)",
    "  P.N           GET WORKING PATTERN",
    "  P.N <0-3>     SET WORKING PATTERN",
    "  P.L / P.L <N> GET/SET LENGTH",
    "  P.I / P.I <N> GET/SET INDEX",
    "  P.HERE        VALUE AT INDEX",
    "  P.NEXT        ADVANCE, GET VALUE",
    "  P.PREV        REVERSE, GET VALUE",
    "  P <I> / P <I> <V>  GET/SET AT INDEX",
    "",
    "  PATTERNS (EXPLICIT - PN)",
    "  PN.L <P> / PN.L <P> <N>  LENGTH",
    "  PN.I <P> / PN.I <P> <N>  INDEX",
    "  PN.HERE <P>              VALUE",
    "  PN.NEXT <P>              ADVANCE",
    "  PN.PREV <P>              REVERSE",
    "  PN <P> <I> / PN <P> <I> <V>",
    "",
    "  EXPRESSIONS",
    "  ALL NUMERIC ARGS ACCEPT EXPRESSIONS:",
    "  PF N ADD A 7",
    "  DC MUL PN.NEXT 0 100",
    "  A RRND 0 127",
    "",
];

pub fn render_help_page(app: &super::App, height: usize) -> Paragraph<'static> {
    let scroll = app.help_scroll;
    let visible = if height > 2 { height - 2 } else { 1 };
    let total = HELP_LINES.len();
    let start = scroll.min(total.saturating_sub(visible));
    let fg = app.theme.foreground;
    let label = app.theme.label;

    let lines: Vec<Line> = HELP_LINES
        .iter()
        .skip(start)
        .take(visible)
        .map(|&s| {
            if s.starts_with("  ") && s.chars().nth(2).map_or(false, |c| c.is_uppercase()) && !s.contains('<') && !s.contains("0-") {
                Line::from(Span::styled(s, Style::default().fg(label).add_modifier(Modifier::BOLD)))
            } else {
                Line::from(Span::styled(s, Style::default().fg(fg)))
            }
        })
        .collect();

    let title = if total > visible {
        format!(" HELP ({}/{}) ", start + 1, total.saturating_sub(visible) + 1)
    } else {
        " HELP ".to_string()
    };

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

pub fn render_footer(app: &super::App) -> Paragraph<'static> {
    let input = &app.input;
    let pos = app.cursor_position;
    let fg = app.theme.foreground;

    let before: String = input.chars().take(pos).collect();
    let cursor_char = input.chars().nth(pos).unwrap_or(' ');
    let after: String = input.chars().skip(pos + 1).collect();

    let input_line = Line::from(vec![
        Span::styled("> ", Style::default().fg(fg)),
        Span::styled(before, Style::default().fg(fg)),
        Span::styled(
            cursor_char.to_string(),
            Style::default().bg(app.theme.highlight_bg).fg(app.theme.highlight_fg),
        ),
        Span::styled(after, Style::default().fg(fg)),
    ]);

    let footer_text = vec![
        input_line,
        Line::from(Span::styled(
            "[ ] PAGES  ESC HELP  F1-F12 NAV  'QUIT' TO EXIT",
            Style::default().fg(app.theme.secondary),
        )),
    ];

    Paragraph::new(footer_text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
        )
}

pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut super::App,
    metro_event_rx: mpsc::Receiver<MetroEvent>,
) -> Result<()> {
    loop {
        app.clear_expired_error();
        terminal.draw(|f| ui(f, app))?;

        while let Ok(event) = metro_event_rx.try_recv() {
            match event {
                MetroEvent::ExecuteScript(index) => {
                    app.execute_script(index);
                }
            }
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let is_help = app.current_page == Page::Help;
                let has_alt = key.modifiers.contains(KeyModifiers::ALT);

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) && app.is_script_page() => {
                        app.copy_line();
                    }
                    KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) && app.is_script_page() => {
                        app.cut_line();
                    }
                    KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) && app.is_script_page() => {
                        app.paste_line();
                    }
                    KeyCode::Char('[') => {
                        app.prev_page();
                    }
                    KeyCode::Char(']') => {
                        app.next_page();
                    }
                    // Function keys (work in all terminals)
                    KeyCode::F(1) => {
                        app.go_to_page(Page::Script1);
                    }
                    KeyCode::F(2) => {
                        app.go_to_page(Page::Script2);
                    }
                    KeyCode::F(3) => {
                        app.go_to_page(Page::Script3);
                    }
                    KeyCode::F(4) => {
                        app.go_to_page(Page::Script4);
                    }
                    KeyCode::F(5) => {
                        app.go_to_page(Page::Script5);
                    }
                    KeyCode::F(6) => {
                        app.go_to_page(Page::Script6);
                    }
                    KeyCode::F(7) => {
                        app.go_to_page(Page::Script7);
                    }
                    KeyCode::F(8) => {
                        app.go_to_page(Page::Script8);
                    }
                    KeyCode::F(9) => {
                        app.go_to_page(Page::Live);
                    }
                    KeyCode::F(10) => {
                        app.go_to_page(Page::Metro);
                    }
                    KeyCode::F(11) => {
                        app.go_to_page(Page::Init);
                    }
                    KeyCode::F(12) => {
                        app.go_to_page(Page::Pattern);
                    }
                    KeyCode::Esc => {
                        app.toggle_help();
                    }
                    // Alt+key (requires iTerm2: Preferences > Profiles > Keys > Left Option = Esc+)
                    KeyCode::Char('h') if has_alt => {
                        app.toggle_help();
                    }
                    KeyCode::Char('l') if has_alt => {
                        app.go_to_page(Page::Live);
                    }
                    KeyCode::Char('m') if has_alt => {
                        app.go_to_page(Page::Metro);
                    }
                    KeyCode::Char('i') if has_alt => {
                        app.go_to_page(Page::Init);
                    }
                    KeyCode::Char('p') if has_alt => {
                        app.go_to_page(Page::Pattern);
                    }
                    KeyCode::Char('1') if has_alt => {
                        app.go_to_page(Page::Script1);
                    }
                    KeyCode::Char('2') if has_alt => {
                        app.go_to_page(Page::Script2);
                    }
                    KeyCode::Char('3') if has_alt => {
                        app.go_to_page(Page::Script3);
                    }
                    KeyCode::Char('4') if has_alt => {
                        app.go_to_page(Page::Script4);
                    }
                    KeyCode::Char('5') if has_alt => {
                        app.go_to_page(Page::Script5);
                    }
                    KeyCode::Char('6') if has_alt => {
                        app.go_to_page(Page::Script6);
                    }
                    KeyCode::Char('7') if has_alt => {
                        app.go_to_page(Page::Script7);
                    }
                    KeyCode::Char('8') if has_alt => {
                        app.go_to_page(Page::Script8);
                    }
                    KeyCode::Up if is_help => {
                        app.help_scroll = app.help_scroll.saturating_sub(1);
                    }
                    KeyCode::Down if is_help => {
                        app.help_scroll = app.help_scroll.saturating_add(1).min(HELP_LINES.len().saturating_sub(1));
                    }
                    KeyCode::Up if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.1 > 0 {
                            app.pattern_cursor.1 -= 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Down if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.1 < 63 {
                            app.pattern_cursor.1 += 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Left if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.0 > 0 {
                            app.pattern_cursor.0 -= 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Right if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_cursor.0 < 3 {
                            app.pattern_cursor.0 += 1;
                        }
                        app.pattern_input.clear();
                    }
                    KeyCode::Char('-') if !is_help && app.current_page == Page::Pattern => {
                        if app.pattern_input.is_empty() {
                            app.pattern_input.push('-');
                        }
                    }
                    KeyCode::Char(c) if !is_help && app.current_page == Page::Pattern && c.is_ascii_digit() => {
                        app.pattern_input.push(c);
                    }
                    KeyCode::Backspace if !is_help && app.current_page == Page::Pattern => {
                        app.pattern_input.pop();
                    }
                    KeyCode::Esc if !is_help && app.current_page == Page::Pattern => {
                        app.pattern_input.clear();
                    }
                    KeyCode::Enter if !is_help && app.current_page == Page::Pattern => {
                        if !app.pattern_input.is_empty() {
                            if let Ok(value) = app.pattern_input.parse::<i16>() {
                                let (pattern_idx, step_idx) = app.pattern_cursor;
                                app.patterns.patterns[pattern_idx].data[step_idx] = value;
                                app.pattern_input.clear();
                            }
                        }
                    }
                    KeyCode::Up if !is_help && app.is_script_page() => {
                        app.select_line_up();
                    }
                    KeyCode::Down if !is_help && app.is_script_page() => {
                        app.select_line_down();
                    }
                    KeyCode::Char('d') if !is_help && app.is_script_page() && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.duplicate_line();
                    }
                    KeyCode::Enter if !is_help && app.is_script_page() => {
                        app.save_line();
                    }
                    KeyCode::Enter if !is_help && app.current_page != Page::Pattern => {
                        app.execute_command();
                        if app.should_quit {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('k') if !is_help && app.is_script_page() && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_entire_line();
                    }
                    KeyCode::Backspace if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.clear_input();
                    }
                    KeyCode::Delete if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::SHIFT) => {
                        app.clear_input();
                    }
                    KeyCode::Char('u') if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.delete_to_start();
                    }
                    KeyCode::Char(c) if !is_help && app.current_page != Page::Pattern => {
                        app.insert_char(c);
                    }
                    KeyCode::Backspace if !is_help && app.current_page != Page::Pattern => {
                        app.delete_char();
                    }
                    KeyCode::Left if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.move_cursor_word_left();
                    }
                    KeyCode::Right if !is_help && app.current_page != Page::Pattern && key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.move_cursor_word_right();
                    }
                    KeyCode::Left if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right if !is_help && app.current_page != Page::Pattern => {
                        app.move_cursor_right();
                    }
                    KeyCode::Up if !is_help && app.current_page != Page::Pattern => {
                        app.history_prev();
                    }
                    KeyCode::Down if !is_help && app.current_page != Page::Pattern => {
                        app.history_next();
                    }
                    _ => {}
                }
            }
        }
    }
}
