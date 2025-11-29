use crate::theme::Theme;
use anyhow::{Context, Result};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub themes: HashMap<String, CustomTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_theme_mode")]
    pub theme: String,
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
    #[serde(default = "default_accent")]
    pub accent: String,
    #[serde(default = "default_success")]
    pub success: String,
    #[serde(default = "default_label")]
    pub label: String,
    #[serde(default)]
    pub font: Option<String>,
}

fn default_accent() -> String {
    "#ffffff".to_string()
}

fn default_success() -> String {
    "#50ff50".to_string()
}

fn default_label() -> String {
    "#b4b4b4".to_string()
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

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig::default(),
            themes: HashMap::new(),
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
    load_theme_by_name(&config.display.theme, config)
}

pub fn load_theme_by_name(name: &str, config: &Config) -> Result<Theme> {
    // Check built-in themes first
    match name.to_lowercase().as_str() {
        "dark" => return Ok(Theme::dark()),
        "light" => return Ok(Theme::light()),
        "system" => return Ok(Theme::system()),
        _ => {}
    }

    // Look up custom theme by name (case-insensitive)
    let name_lower = name.to_lowercase();
    if let Some(custom) = config.themes.get(&name_lower)
        .or_else(|| config.themes.iter().find(|(k, _)| k.to_lowercase() == name_lower).map(|(_, v)| v))
    {
        Ok(Theme {
            name: name.to_string(),
            background: parse_hex_color(&custom.background)?,
            foreground: parse_hex_color(&custom.foreground)?,
            secondary: parse_hex_color(&custom.secondary)?,
            highlight_bg: parse_hex_color(&custom.highlight_bg)?,
            highlight_fg: parse_hex_color(&custom.highlight_fg)?,
            border: parse_hex_color(&custom.border)?,
            error: parse_hex_color(&custom.error)?,
            accent: parse_hex_color(&custom.accent)?,
            success: parse_hex_color(&custom.success)?,
            label: parse_hex_color(&custom.label)?,
            font: custom.font.clone(),
        })
    } else {
        anyhow::bail!("Unknown theme: {}. Available: dark, light, system{}",
            name,
            if config.themes.is_empty() {
                String::new()
            } else {
                format!(", {}", config.themes.keys().cloned().collect::<Vec<_>>().join(", "))
            }
        )
    }
}

pub fn list_themes(config: &Config) -> Vec<String> {
    let mut themes = vec!["dark".to_string(), "light".to_string(), "system".to_string()];
    themes.extend(config.themes.keys().cloned());
    themes
}

pub fn save_theme_mode(mode: &str) -> Result<()> {
    let mut config = load_config()?;
    config.display.theme = mode.to_string();
    save_config(&config)?;
    Ok(())
}
