use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub secondary: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
    pub border: Color,
    pub error: Color,
    pub font: Option<String>,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            background: Color::Black,
            foreground: Color::White,
            secondary: Color::Gray,
            highlight_bg: Color::White,
            highlight_fg: Color::Black,
            border: Color::White,
            error: Color::Red,
            font: Some("Menlo".to_string()),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".to_string(),
            background: Color::White,
            foreground: Color::Black,
            secondary: Color::DarkGray,
            highlight_bg: Color::Black,
            highlight_fg: Color::White,
            border: Color::Black,
            error: Color::Red,
            font: Some("Menlo".to_string()),
        }
    }

    pub fn system() -> Self {
        match dark_light::detect() {
            dark_light::Mode::Dark => Self::dark(),
            dark_light::Mode::Light => Self::light(),
            dark_light::Mode::Default => Self::dark(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
