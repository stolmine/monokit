use ratatui::style::Color;
use std::time::Instant;

pub fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 { return 16; }
        if r > 248 { return 231; }
        return 232 + ((r - 8) / 10);
    }

    let r_idx = (r as u16 * 5 / 255) as u8;
    let g_idx = (g as u16 * 5 / 255) as u8;
    let b_idx = (b as u16 * 5 / 255) as u8;

    16 + 36 * r_idx + 6 * g_idx + b_idx
}

pub fn color_to_256(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Indexed(rgb_to_256(r, g, b)),
        other => other,
    }
}

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

    pub fn to_256_color(&self) -> Self {
        Self {
            name: format!("{}_256", self.name),
            background: color_to_256(self.background),
            foreground: color_to_256(self.foreground),
            secondary: color_to_256(self.secondary),
            highlight_bg: color_to_256(self.highlight_bg),
            highlight_fg: color_to_256(self.highlight_fg),
            border: color_to_256(self.border),
            error: color_to_256(self.error),
            accent: color_to_256(self.accent),
            success: color_to_256(self.success),
            label: color_to_256(self.label),
            font: self.font.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_256_black() {
        assert_eq!(rgb_to_256(0, 0, 0), 16);
    }

    #[test]
    fn test_rgb_to_256_white() {
        assert_eq!(rgb_to_256(255, 255, 255), 231);
    }

    #[test]
    fn test_rgb_to_256_grayscale() {
        assert_eq!(rgb_to_256(128, 128, 128), 244);
    }

    #[test]
    fn test_rgb_to_256_red() {
        assert_eq!(rgb_to_256(255, 0, 0), 196);
    }

    #[test]
    fn test_rgb_to_256_green() {
        assert_eq!(rgb_to_256(0, 255, 0), 46);
    }

    #[test]
    fn test_rgb_to_256_blue() {
        assert_eq!(rgb_to_256(0, 0, 255), 21);
    }

    #[test]
    fn test_color_to_256_rgb() {
        let color = Color::Rgb(255, 0, 0);
        assert_eq!(color_to_256(color), Color::Indexed(196));
    }

    #[test]
    fn test_color_to_256_indexed() {
        let color = Color::Indexed(42);
        assert_eq!(color_to_256(color), Color::Indexed(42));
    }

    #[test]
    fn test_theme_to_256_color() {
        let theme = Theme::dark();
        let theme_256 = theme.to_256_color();

        assert_eq!(theme_256.name, "dark_256");
        assert_eq!(theme_256.background, Color::Indexed(16));
        assert_eq!(theme_256.foreground, Color::Indexed(231));
    }
}
