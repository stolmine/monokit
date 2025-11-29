use crate::theme::Theme;
use anyhow::{Context, Result};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_theme_mode")]
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default)]
    pub custom: Option<CustomTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    pub background: String,
    pub foreground: String,
    pub secondary: String,
    pub highlight_bg: String,
    pub highlight_fg: String,
    pub border: String,
    pub error: String,
    #[serde(default)]
    pub font: Option<String>,
}

fn default_theme_mode() -> String {
    "dark".to_string()
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            theme: default_theme_mode(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self { custom: None }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::home_dir()
        .context("Failed to find home directory")?
        .join(".monokit");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }

    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(Config::default());
    }

    let contents = fs::read_to_string(&path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&contents).context("Failed to parse config file")?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    let contents = toml::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(&path, contents).context("Failed to write config file")?;

    Ok(())
}

fn parse_hex_color(hex: &str) -> Result<Color> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        anyhow::bail!("Invalid hex color: must be 6 characters");
    }

    let r = u8::from_str_radix(&hex[0..2], 16).context("Invalid red component")?;
    let g = u8::from_str_radix(&hex[2..4], 16).context("Invalid green component")?;
    let b = u8::from_str_radix(&hex[4..6], 16).context("Invalid blue component")?;

    Ok(Color::Rgb(r, g, b))
}

pub fn load_theme(config: &Config) -> Result<Theme> {
    match config.display.theme.as_str() {
        "dark" => Ok(Theme::dark()),
        "light" => Ok(Theme::light()),
        "system" => Ok(Theme::system()),
        "custom" => {
            if let Some(custom) = &config.theme.custom {
                Ok(Theme {
                    name: "custom".to_string(),
                    background: parse_hex_color(&custom.background)?,
                    foreground: parse_hex_color(&custom.foreground)?,
                    secondary: parse_hex_color(&custom.secondary)?,
                    highlight_bg: parse_hex_color(&custom.highlight_bg)?,
                    highlight_fg: parse_hex_color(&custom.highlight_fg)?,
                    border: parse_hex_color(&custom.border)?,
                    error: parse_hex_color(&custom.error)?,
                    font: custom.font.clone(),
                })
            } else {
                anyhow::bail!("Custom theme selected but no custom theme defined in config");
            }
        }
        _ => {
            anyhow::bail!("Unknown theme mode: {}", config.display.theme);
        }
    }
}

pub fn save_theme_mode(mode: &str) -> Result<()> {
    let mut config = load_config()?;
    config.display.theme = mode.to_string();
    save_config(&config)?;
    Ok(())
}
