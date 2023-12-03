use colored::*;
use fern::{Dispatch, log_file};
use log::LevelFilter;
use std::io;
use std::fs;
use chrono::Local;

/// Initializes the logger with different log files for each level.
///
/// Log messages are directed to both the terminal and separate files for each log level.
/// The files are created in the specified directory. If the directory does not exist,
/// it is created.
///
/// # Arguments
///
/// * `log_dir_path` - The path to the directory where log files will be written.
///
/// # Panics
///
/// This function will panic if it fails to create or write to any of the log files
/// or if it cannot create the log directory.
pub fn init(log_dir_path: &str) {
    // Create log directory if it doesn't exist
    fs::create_dir_all(log_dir_path)
        .unwrap_or_else(|_| panic!("Failed to create log directory: {}", log_dir_path));

    // Create a base configuration for formatting log messages
    let base_config = Dispatch::new()
        .format(move |out, message, record| {
            let color_message = match record.level() {
                log::Level::Error => message.to_string().red(),
                log::Level::Warn => message.to_string().yellow(),
                log::Level::Info => message.to_string().green(),
                log::Level::Debug => message.to_string().blue(),
                log::Level::Trace => message.to_string().cyan(),
            };
            writeln!(
                out,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                color_message
            )
        });

    // Rest of the logger setup
    // ...

    // Apply the logger configuration
    combined_config.apply()
        .expect("Failed to initialize logger.");
}
