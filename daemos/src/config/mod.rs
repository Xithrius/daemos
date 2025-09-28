pub mod core;
pub mod general;
pub mod playback;
pub mod search;
pub mod ui;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use color_eyre::{Result, eyre::Context};
use config::{Config, File};
use toml;
use tracing::warn;

use crate::{BINARY_NAME, config::core::CoreConfig};

const CONFIG_FILE_NAME: &str = "config.toml";

pub fn get_config_path() -> Result<PathBuf> {
    let partial_path = match env::consts::OS {
        "linux" | "macos" => {
            let home = env::var("HOME").context("HOME environment variable not found")?;

            Path::new(&home).join(".config")
        }
        "windows" => {
            let appdata = env::var("APPDATA").context("APPDATA environment variable not found")?;

            Path::new(&appdata).to_path_buf()
        }
        _ => unimplemented!(),
    };

    let full_path = partial_path.join(BINARY_NAME).join(CONFIG_FILE_NAME);

    Ok(full_path)
}

pub fn load_config() -> Result<CoreConfig> {
    let path = get_config_path()?;

    let config = match Config::builder()
        .add_source(File::with_name(&path.to_string_lossy()))
        .build()
    {
        Ok(config) => match config.try_deserialize() {
            Ok(cfg) => cfg,
            Err(err) => {
                warn!("Failed to deserialize config: {err}. Falling back to default config.");
                CoreConfig::default()
            }
        },
        Err(err) => {
            warn!("Failed to build config: {err}. Falling back to default config.");
            CoreConfig::default()
        }
    };

    Ok(config)
}

pub fn save_config(config: &CoreConfig) -> Result<()> {
    let path = get_config_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    let toml_string =
        toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;

    fs::write(&path, toml_string).context("Failed to write config file")?;

    Ok(())
}
