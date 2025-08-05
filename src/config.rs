use std::{sync::OnceLock, path::PathBuf, env};

use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use figment::{Figment, providers::{Format, Env, Toml, Serialized}};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::cli::Cli;


const CONFIG_DEFAULT: &str = include_str!("../.config/config.default.toml");
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Stores the configuration for the whole application.
/// Includes color palette, layers, controller deadzones, etc.
/// Serializes to and from different formats.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// File where configuration is stored.
    pub config_file: PathBuf,

    /// Directory where configuration is stored.
    pub config_dir: PathBuf,

    /// Directory to record logs to. Initializes usually to $XDG_DATA_HOME/gameput or AppData on windows.
    pub log_dir: PathBuf,

    /// The log level to use. Valid values are: error, warn, info, debug, trace. Default is info.
    pub log_level: Option<LevelFilter>,



}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_file: get_default_config_file(),
            config_dir: get_default_config_dir(),
            log_level: Some(LevelFilter::Info), // TODO: remove this for release
            log_dir: get_default_logging_dir(),
        }
    }
}


/// Initializes the configuration for the program.
/// MUST be called before the rest of the program initializes.
pub fn initialize_config(arguments: &Cli) -> Result<()>{
    println!("wowie we did a config");
    let config_file = arguments.config_file.clone().unwrap_or_else(get_default_config_file);
    let config = Figment::new()
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::string(CONFIG_DEFAULT))
        .merge(Toml::file(config_file))
        .merge(Env::prefixed("GAMEPUT_"))
        .merge(Serialized::defaults(arguments))
        .extract::<Config>()?;
    CONFIG.set(config).map_err(|config| anyhow!("failed to existificate configuration {config:?}"))
}

/// Returns the default file for storing configurations.
pub fn get_default_config_file() -> PathBuf {
    get_default_config_dir().join("config.toml")
}


/// Returns the default directory for storing configurations.
pub fn get_default_config_dir() -> PathBuf {
    env::var("GAMEPUT_CONFIG_HOME")
    .map(PathBuf::from)
    .or_else(|_| project_dirs().map(|dirs| dirs.config_dir().to_path_buf()))
    .unwrap_or(PathBuf::from(".").join(".config"))
}

/// Returns the default directory for logging files. 
pub fn get_default_logging_dir() -> PathBuf {
    env::var("GAMEPUT_LOG_HOME")
    .map(PathBuf::from)
    .or_else(|_| project_dirs().map(|dirs| dirs.data_dir().to_path_buf()))
    .unwrap_or(PathBuf::from(".").join("log"))
}

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("rs", "goobers", "gameput").context("Failed to locate config directory.")
}

/// Gets the configuration.
pub fn get() -> &'static Config {
    CONFIG.get().expect("Configuration not initialized")
}
