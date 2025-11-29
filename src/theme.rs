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
    pub accent: Color,      // Selected items, active indicators
    pub success: Color,     // Positive states (metro on, etc.)
    pub label: Color,       // Section labels, headers
    pub font: Option<String>,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            background: Color::Rgb(0, 0, 0),
            foreground: Color::Rgb(255, 255, 255),
            secondary: Color::Rgb(128, 128, 128),
            highlight_bg: Color::Rgb(255, 255, 255),
            highlight_fg: Color::Rgb(0, 0, 0),
            border: Color::Rgb(255, 255, 255),
            error: Color::Rgb(255, 80, 80),
            accent: Color::Rgb(255, 255, 255),    // Bright white for selection
            success: Color::Rgb(80, 255, 80),     // Green for active states
            label: Color::Rgb(180, 180, 180),     // Light gray for labels
            font: Some("Menlo".to_string()),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".to_string(),
            background: Color::Rgb(255, 255, 255),
            foreground: Color::Rgb(0, 0, 0),
            secondary: Color::Rgb(96, 96, 96),
            highlight_bg: Color::Rgb(0, 0, 0),
            highlight_fg: Color::Rgb(255, 255, 255),
            border: Color::Rgb(0, 0, 0),
            error: Color::Rgb(200, 0, 0),
            accent: Color::Rgb(0, 0, 0),          // Black for selection
            success: Color::Rgb(0, 160, 0),       // Dark green for active states
            label: Color::Rgb(64, 64, 64),        // Dark gray for labels
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
