use colored::*;
use fern::{Dispatch};
use log::LevelFilter;
use chrono::Local;

pub fn init(log_dir_path: &str) {
    // Create the log directory if it doesn't exist
    std::fs::create_dir_all(log_dir_path)
        .expect("Failed to create log directory");

    // Base configuration
    let base_config = Dispatch::new()
        .format(move |out, message, record| {
            let color_message = match record.level() {
                log::Level::Error => message.to_string().red(),
                log::Level::Warn => message.to_string().yellow(),
                log::Level::Info => message.to_string().green(),
                log::Level::Debug => message.to_string().blue(),
                log::Level::Trace => message.to_string().cyan(),
            };
            out.finish(format_args!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                color_message
            ))
        })
        .level(level_filter);

    // Define log file paths
    let error_log = format!("{}/one_4_all_error.log", log_dir_path);
    // ... define other log files similarly

    // Combined configuration
    let combined_config = base_config
        .chain(Dispatch::new().filter(|meta| meta.level() <= log::Level::Error).chain(fern::log_file(error_log).unwrap()))
        // ... chain other log levels similarly
        .chain(std::io::stdout()); // Add this to log to stdout

    // Apply the logger configuration
    combined_config.apply().expect("Failed to initialize logger.");
}
