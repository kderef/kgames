use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct UI {
    pub background: String,
    pub foreground: String,
    pub border: String,
    pub background_hover: String,
    pub background_click: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ui: UI,
}

impl Config {
    pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let s = toml::from_str(&contents)?;
        Ok(s)
    }
}
