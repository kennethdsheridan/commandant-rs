//! Log Adapter
//!
//! This module provides a logger with colored output, timestamps, and caller information.
//! It uses the `env_logger` and `colored` crates to achieve this functionality.

use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initializes the logger with the specified log level filter.
///
/// # Arguments
///
/// * `level_filter` - The minimum log level to display.
pub fn init(level_filter: LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                match record.level() {
                    log::Level::Error => record.args().to_string().red(),
                    log::Level::Warn => record.args().to_string().yellow(),
                    log::Level::Info => record.args().to_string().green(),
                    log::Level::Debug => record.args().to_string().blue(),
                    log::Level::Trace => record.args().to_string().cyan(),
                }
            )
        })
        .filter(None, level_filter)
        .init();
}