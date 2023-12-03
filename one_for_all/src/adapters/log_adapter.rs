use colored::*;
use fern::{Dispatch, log_file};
use log::LevelFilter;
use std::io;
use chrono::Local;

/// Initializes the logger with different log files for each level.
///
/// Log messages are directed to both the terminal and separate files for each log level.
/// The files are created in the specified directory.
///
/// # Arguments
///
/// * `log_dir_path` - The path to the directory where log files will be written.
///
/// # Panics
///
/// This function will panic if it fails to create or write to any of the log files.
pub fn init(log_dir_path: &str) {
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

    // Dispatcher for terminal output
    let stdout_config = base_config.chain(io::stdout());

    // Dispatchers for file output, one for each log level
    let error_file = log_file(&format!("{}/one_4_all_error.log", log_dir_path))
        .expect("Failed to open error log file");
    let warn_file = log_file(&format!("{}/one_4_all_warn.log", log_dir_path))
        .expect("Failed to open warn log file");
    let info_file = log_file(&format!("{}/one_4_all_info.log", log_dir_path))
        .expect("Failed to open info log file");
    let debug_file = log_file(&format!("{}/one_4_all_debug.log", log_dir_path))
        .expect("Failed to open debug log file");
    let trace_file = log_file(&format!("{}/one_4_all_trace.log", log_dir_path))
        .expect("Failed to open trace log file");

    // Combine all dispatches
    let combined_config = Dispatch::new()
        .chain(stdout_config)
        .chain(Dispatch::new().filter(|metadata| metadata.level() == log::Level::Error).chain(error_file))
        .chain(Dispatch::new().filter(|metadata| metadata.level() == log::Level::Warn).chain(warn_file))
        .chain(Dispatch::new().filter(|metadata| metadata.level() == log::Level::Info).chain(info_file))
        .chain(Dispatch::new().filter(|metadata| metadata.level() == log::Level::Debug).chain(debug_file))
        .chain(Dispatch::new().filter(|metadata| metadata.level() == log::Level::Trace).chain(trace_file));

    // Apply the logger configuration
    combined_config.apply()
        .expect("Failed to initialize logger.");
}