use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;
use serde::Serialize;

use crate::config::get_default_config_file;



#[derive(Debug, Default, Parser, Serialize)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Print out the default configuration before running the application.
    #[arg(short, long, default_value_t = false)]
    pub print_config: bool,


    /// A path to a .toml configuration file for Gameput.
    #[arg(short, long, value_name = "FILE", default_value = get_default_config_path())]
    pub config_file: Option<PathBuf>,

    /// A log level to use. Valid values are: error, warn, info, debug, trace, off.
    /// 
    /// [default: info]
    #[arg(long, value_name = "LEVEL", alias = "log")]
    pub log_level: Option<LevelFilter>,

    /// Whether the application should print logging to stdio or a logfile. Overrides configuration option. 
    /// Defaults to false (don't display to stdio)
    #[arg(short, long, default_value_t = false)]
    pub log_to_stdio: bool,

}


/// Gets the default config file and converts to string. 
fn get_default_config_path() -> String {
    get_default_config_file().to_string_lossy().into_owned()
}

/// Reexports the function so it's less pain to use.
pub fn parse() -> Cli {
    Cli::parse()
}

