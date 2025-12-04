pub struct TerminalCapabilities {
    pub true_color: bool,
    pub term_program: Option<String>,
}

pub fn detect_capabilities() -> TerminalCapabilities {
    let colorterm = std::env::var("COLORTERM").ok();
    let true_color = colorterm.as_deref() == Some("truecolor")
                  || colorterm.as_deref() == Some("24bit");

    let term_program = std::env::var("TERM_PROGRAM").ok();

    TerminalCapabilities { true_color, term_program }
}

pub fn is_terminal_app(caps: &TerminalCapabilities) -> bool {
    caps.term_program.as_deref() == Some("Apple_Terminal")
}
