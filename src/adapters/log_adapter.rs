// To filter log messages based on their severity level.
use std::fs;
// For filesystem operations like creating directories.
use std::fs::File;

// Custom trait for logging functionality.
use chrono::Local;
// For timestamping log messages with the current local time.
use colored::*;
// To colorize log messages based on their severity level.
use fern::{log_file, Dispatch};
// For setting up the logging infrastructure.
use log::LevelFilter;

// Import necessary modules and types from external crates and the standard library.
use crate::domain::logging::LoggerPort;

// For file operations like creating log files.

// Define a struct that will implement the LoggerPort trait.
pub struct FernLogger;

// Implement the LoggerPort trait for the FernLogger struct.
impl LoggerPort for FernLogger {
    // Define methods for logging messages at different severity levels.
    fn log_info(&self, message: &str) {
        log::info!("{}", message);
    }

    fn log_warn(&self, message: &str) {
        log::warn!("{}", message);
    }

    fn log_error(&self, message: &str) {
        log::error!("{}", message);
    }

    fn log_debug(&self, message: &str) {
        log::debug!("{}", message);
    }

    fn log_trace(&self, message: &str) {
        log::trace!("{}", message);
    }
}

// Define a function to initialize the logging system.
pub fn init(log_dir_path: &str, level_filter: LevelFilter) {
    // Ensure the log directory exists, creating it if necessary.
    fs::create_dir_all(log_dir_path).expect("Failed to create log directory");

    // Create or truncate log files for each log level.
    let log_levels = ["error", "warn", "info", "debug", "trace"];
    for level in &log_levels {
        let file_path = format!("{}/one_4_all_{}.log", log_dir_path, level);
        File::create(&file_path)
            .unwrap_or_else(|_| panic!("Failed to create or truncate log file: {}", file_path));
    }

    // Set up the base configuration for the logger, including message formatting.
    let base_config = Dispatch::new()
        .format(move |out, message, record| {
            // Colorize messages based on their log level.
            let color_message = match record.level() {
                log::Level::Error => message.to_string().red(),
                log::Level::Warn => message.to_string().yellow(),
                log::Level::Info => message.to_string().green(),
                log::Level::Debug => message.to_string().blue(),
                log::Level::Trace => message.to_string().cyan(),
            };
            // Format the log message with a timestamp, level, and the colorized message.
            out.finish(format_args!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                color_message
            ))
        })
        .level(level_filter); // Apply the specified level filter to the logger.

    // Set up individual log files for each log level.
    let error_log = log_file(&format!("{}/error.log", log_dir_path)).unwrap();
    let warn_log = log_file(&format!("{}/warn.log", log_dir_path)).unwrap();
    let info_log = log_file(&format!("{}/info.log", log_dir_path)).unwrap();
    let debug_log = log_file(&format!("{}/debug.log", log_dir_path)).unwrap();
    let trace_log = log_file(&format!("{}/trace.log", log_dir_path)).unwrap();

    // Create dispatch configurations for each log level, filtering and chaining to the respective log file.
    let error_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == log::Level::Error)
        .chain(error_log);
    let warn_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == log::Level::Warn)
        .chain(warn_log);
    let info_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == log::Level::Info)
        .chain(info_log);
    let debug_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == log::Level::Debug)
        .chain(debug_log);
    let trace_dispatch = Dispatch::new()
        .filter(|meta| meta.level() == log::Level::Trace)
        .chain(trace_log);

    // Combine all dispatch configurations into one.
    let combined_config = base_config
        .chain(error_dispatch)
        .chain(warn_dispatch)
        .chain(info_dispatch)
        .chain(debug_dispatch)
        .chain(trace_dispatch)
        .chain(std::io::stdout()); // Also log to standard output.

    // Apply the combined logger configuration.
    combined_config
        .apply()
        .expect("Failed to initialize logger.");
}
