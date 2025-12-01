use ratatui::style::Color;
use std::time::Instant;

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

pub const DEFAULT_ACTIVITY_HOLD_MS: f32 = 200.0;
pub const ACTIVITY_DECAY_MS: f32 = 300.0;

impl Theme {
    pub fn lerp_color(from: Color, to: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        match (from, to) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
                let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
                let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
                Color::Rgb(r, g, b)
            }
            _ => if t >= 0.5 { to } else { from }
        }
    }

    pub fn activity_color(&self, last_activity: Option<Instant>, is_selected: bool, hold_ms: f32) -> Color {
        let progress = match last_activity {
            Some(instant) => {
                let elapsed_ms = instant.elapsed().as_millis() as f32;
                if elapsed_ms < hold_ms {
                    0.0 // Fully lit during hold period
                } else {
                    let decay_elapsed = (elapsed_ms - hold_ms) / ACTIVITY_DECAY_MS;
                    // Cubic ease-out for smooth falloff
                    1.0 - (1.0 - decay_elapsed.min(1.0)).powi(3)
                }
            }
            None => 1.0,
        };
        // Use theme colors: foreground (bright) fading to secondary (dim)
        if is_selected {
            Self::lerp_color(self.highlight_bg, self.foreground, progress)
        } else {
            Self::lerp_color(self.foreground, self.secondary, progress)
        }
    }
}
