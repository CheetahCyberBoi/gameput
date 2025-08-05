use std::time::SystemTime;

use anyhow::{Result, Context};
use fern::Dispatch;
use log::LevelFilter;

use crate::config;

pub fn initialize_logging(log_to_stdio: bool) -> Result<()> {
    if !log_to_stdio {
        // Setup the log file before initializing logging
        let log_dir = config::get().log_dir.clone();
        // This creates the directories we need.
        std::fs::create_dir_all(log_dir.clone())?;
        let log_path = log_dir.join(format!("{}.log", env!("CARGO_PKG_NAME")));
        Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(config::get().log_level.unwrap_or(LevelFilter::Info))
        .chain(fern::log_file(log_path)?)
        .apply()?;
    } else {
        Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(config::get().log_level.unwrap_or(LevelFilter::Info))
        .chain(std::io::stdout())
        .apply()?;
    }
    Ok(())
}