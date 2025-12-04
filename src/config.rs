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
    #[serde(default = "default_header_level")]
    pub header_level: u8,
    #[serde(default)]
    pub load_rst: bool,
    #[serde(default = "default_debug_level")]
    pub debug_level: u8,
    #[serde(default)]
    pub show_cpu: bool,
    #[serde(default = "default_show_bpm")]
    pub show_bpm: bool,
    #[serde(default = "default_show_meters")]
    pub show_meters_header: bool,
    #[serde(default = "default_show_meters")]
    pub show_meters_grid: bool,
    #[serde(default = "default_show_spectrum")]
    pub show_spectrum: bool,
    #[serde(default = "default_show_activity")]
    pub show_activity: bool,
    #[serde(default = "default_show_grid")]
    pub show_grid: bool,
    #[serde(default)]
    pub show_grid_view: bool,
    #[serde(default = "default_show_seq_highlight")]
    pub show_seq_highlight: bool,
    #[serde(default = "default_show_conditional_highlight")]
    pub show_conditional_highlight: bool,
    #[serde(default = "default_grid_mode")]
    pub grid_mode: u8,
    #[serde(default = "default_limiter_enabled")]
    pub limiter_enabled: bool,
    #[serde(default = "default_activity_hold_ms")]
    pub activity_hold_ms: u32,
    #[serde(default)]
    pub title_mode: u8,
    #[serde(default)]
    pub title_timer_enabled: bool,
    #[serde(default = "default_title_timer_interval_secs")]
    pub title_timer_interval_secs: u16,
    #[serde(default = "default_scope_timespan_ms")]
    pub scope_timespan_ms: u32,
    #[serde(default)]
    pub scope_color_mode: u8,
    #[serde(default)]
    pub scope_display_mode: u8,
    #[serde(default)]
    pub scope_unipolar: bool,
    #[serde(default)]
    pub out_err: bool,
    #[serde(default)]
    pub out_ess: bool,
    #[serde(default)]
    pub out_qry: bool,
    #[serde(default)]
    pub out_cfm: bool,
    #[serde(default)]
    pub audio_out_device: Option<String>,
    #[serde(default = "default_true")]
    pub scramble_enabled: bool,
    #[serde(default = "default_scramble_mode")]
    pub scramble_mode: u8,
    #[serde(default = "default_scramble_speed")]
    pub scramble_speed: u8,
    #[serde(default)]
    pub scramble_curve: u8,
    #[serde(default)]
    pub vca_mode: bool,
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

fn default_true() -> bool {
    true
}

fn default_theme_mode() -> String {
    "dark".to_string()
}

fn default_header_level() -> u8 {
    4
}

fn default_debug_level() -> u8 {
    2
}

fn default_show_meters() -> bool {
    true
}

fn default_show_spectrum() -> bool {
    true
}

fn default_show_activity() -> bool {
    true
}

fn default_show_grid() -> bool {
    true
}

fn default_show_bpm() -> bool {
    true
}

fn default_show_seq_highlight() -> bool {
    true
}

fn default_show_conditional_highlight() -> bool {
    true
}

fn default_grid_mode() -> u8 {
    1
}

fn default_limiter_enabled() -> bool {
    true
}

fn default_activity_hold_ms() -> u32 {
    200
}

fn default_scope_timespan_ms() -> u32 {
    30
}

fn default_scramble_mode() -> u8 {
    2
}

fn default_scramble_speed() -> u8 {
    5
}

fn default_title_timer_interval_secs() -> u16 {
    5
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            theme: default_theme_mode(),
            header_level: default_header_level(),
            load_rst: false,
            debug_level: default_debug_level(),
            show_cpu: false,
            show_bpm: default_show_bpm(),
            show_meters_header: default_show_meters(),
            show_meters_grid: default_show_meters(),
            show_spectrum: default_show_spectrum(),
            show_activity: default_show_activity(),
            show_grid: default_show_grid(),
            show_grid_view: false,
            show_seq_highlight: default_show_seq_highlight(),
            show_conditional_highlight: default_show_conditional_highlight(),
            grid_mode: default_grid_mode(),
            limiter_enabled: default_limiter_enabled(),
            activity_hold_ms: default_activity_hold_ms(),
            title_mode: 0,
            title_timer_enabled: false,
            title_timer_interval_secs: default_title_timer_interval_secs(),
            scope_timespan_ms: default_scope_timespan_ms(),
            scope_color_mode: 0,
            scope_display_mode: 0,
            scope_unipolar: false,
            out_err: false,
            out_ess: false,
            out_qry: false,
            out_cfm: false,
            audio_out_device: None,
            scramble_enabled: default_true(),
            scramble_mode: default_scramble_mode(),
            scramble_speed: default_scramble_speed(),
            scramble_curve: 0,
            vca_mode: false,
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

pub fn save_header_level(level: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.header_level = level;
    save_config(&config)?;
    Ok(())
}

pub fn save_load_rst(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.load_rst = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_debug_level(level: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.debug_level = level;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_cpu(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_cpu = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_bpm(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_bpm = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_meters_header(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_meters_header = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_meters_grid(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_meters_grid = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_spectrum(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_spectrum = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_activity(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_activity = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_grid(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_grid = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_grid_view(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_grid_view = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_seq_highlight(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_seq_highlight = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_show_conditional_highlight(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.show_conditional_highlight = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_grid_mode(mode: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.grid_mode = mode;
    save_config(&config)?;
    Ok(())
}

pub fn save_limiter_enabled(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.limiter_enabled = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_activity_hold_ms(ms: u32) -> Result<()> {
    let mut config = load_config()?;
    config.display.activity_hold_ms = ms;
    save_config(&config)?;
    Ok(())
}

pub fn save_title_mode(mode: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.title_mode = mode;
    save_config(&config)?;
    Ok(())
}

pub fn save_scope_timespan_ms(ms: u32) -> Result<()> {
    let mut config = load_config()?;
    config.display.scope_timespan_ms = ms;
    save_config(&config)?;
    Ok(())
}

pub fn save_scope_color_mode(mode: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.scope_color_mode = mode;
    save_config(&config)?;
    Ok(())
}

pub fn save_scope_display_mode(mode: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.scope_display_mode = mode;
    save_config(&config)?;
    Ok(())
}

pub fn save_scope_unipolar(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.scope_unipolar = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_scope_settings(settings: &crate::types::ScopeSettings) -> Result<()> {
    let mut config = load_config()?;
    config.display.scope_timespan_ms = settings.timespan_ms;
    config.display.scope_color_mode = settings.color_mode;
    config.display.scope_display_mode = settings.display_mode;
    config.display.scope_unipolar = settings.unipolar;
    save_config(&config)?;
    Ok(())
}

pub fn save_out_err(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.out_err = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_out_ess(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.out_ess = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_out_qry(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.out_qry = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_out_cfm(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.out_cfm = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_audio_out_device(device: Option<String>) -> Result<()> {
    let mut config = load_config()?;
    config.display.audio_out_device = device;
    save_config(&config)?;
    Ok(())
}

pub fn save_scramble_enabled(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.scramble_enabled = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_scramble_mode(mode: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.scramble_mode = mode;
    save_config(&config)?;
    Ok(())
}

pub fn save_scramble_speed(speed: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.scramble_speed = speed;
    save_config(&config)?;
    Ok(())
}

pub fn save_scramble_curve(curve: u8) -> Result<()> {
    let mut config = load_config()?;
    config.display.scramble_curve = curve;
    save_config(&config)?;
    Ok(())
}

pub fn save_vca_mode(mode: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.vca_mode = mode;
    save_config(&config)?;
    Ok(())
}

pub fn save_title_timer_enabled(enabled: bool) -> Result<()> {
    let mut config = load_config()?;
    config.display.title_timer_enabled = enabled;
    save_config(&config)?;
    Ok(())
}

pub fn save_title_timer_interval_secs(secs: u16) -> Result<()> {
    let mut config = load_config()?;
    config.display.title_timer_interval_secs = secs;
    save_config(&config)?;
    Ok(())
}
