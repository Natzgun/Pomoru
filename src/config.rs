use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_study")]
    pub study: [u8; 3],
    #[serde(default = "default_break")]
    pub r#break: [u8; 3],
    #[serde(default = "default_paused")]
    pub paused: [u8; 3],
    #[serde(default = "default_idle")]
    pub idle: [u8; 3],
    #[serde(default = "default_planned")]
    pub planned: [u8; 3],
    #[serde(default = "default_completed")]
    pub completed: [u8; 3],
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            study: default_study(),
            r#break: default_break(),
            paused: default_paused(),
            idle: default_idle(),
            planned: default_planned(),
            completed: default_completed(),
        }
    }
}

fn default_study() -> [u8; 3] { [120, 200, 140] }
fn default_break() -> [u8; 3] { [100, 180, 220] }
fn default_paused() -> [u8; 3] { [200, 190, 80] }
fn default_idle() -> [u8; 3] { [140, 140, 150] }
fn default_planned() -> [u8; 3] { [50, 80, 60] }
fn default_completed() -> [u8; 3] { [40, 160, 80] }

fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("pomoru");
    config_dir.join("config.toml")
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        if let Ok(content) = fs::read_to_string(&path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}
