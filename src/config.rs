use anyhow::{Context, Result};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
  pub window: WindowConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowConfig {
  pub width: u32,
  pub height: u32,
  pub fullscreen: bool,
  pub resizable: bool,
}

impl Default for WindowConfig {
  fn default() -> Self {
    Self {
      width: 256,
      height: 256,
      fullscreen: false,
      resizable: false,
    }
  }
}

impl Config {
  const CONFIG_PATH: &'static str = "./settings.toml";

  pub fn load() -> Result<Self> {
    debug!("Reading confing file");
    let cfg = std::fs::read_to_string(Self::CONFIG_PATH)
      .context("Failed to read config file")?;
    toml::from_str(&cfg).context("Invalid config key")
  }
}
