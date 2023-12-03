use colored::*;
use fern::{Dispatch, log_file};
use log::LevelFilter;
use std::fs;
use chrono::Local;

pub fn init(log_dir_path: &str, level_filter: LevelFilter) {
    // Create the log directory if it doesn't exist
    fs::create_dir_all(log_dir_path)
        .expect("Failed to create log directory");

    // Base configuration for formatting log messages
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

    // Create log file paths for each level
    let error_log = log_file(&format!("{}/one_4_all_error.log", log_dir_path)).unwrap();
    let warn_log = log_file(&format!("{}/one_4_all_warn.log", log_dir_path)).unwrap();
    let info_log = log_file(&format!("{}/one_4_all_info.log", log_dir_path)).unwrap();
    let debug_log = log_file(&format!("{}/one_4_all_debug.log", log_dir_path)).unwrap();
    let trace_log = log_file(&format!("{}/one_4_all_trace.log", log_dir_path)).unwrap();

    // Dispatchers for each log level
    let error_dispatch = Dispatch::new().filter(|meta| meta.level() == log::Level::Error).chain(error_log);
    let warn_dispatch = Dispatch::new().filter(|meta| meta.level() == log::Level::Warn).chain(warn_log);
    let info_dispatch = Dispatch::new().filter(|meta| meta.level() == log::Level::Info).chain(info_log);
    let debug_dispatch = Dispatch::new().filter(|meta| meta.level() == log::Level::Debug).chain(debug_log);
    let trace_dispatch = Dispatch::new().filter(|meta| meta.level() == log::Level::Trace).chain(trace_log);

    // Combine all dispatches
    let combined_config = base_config
        .chain(error_dispatch)
        .chain(warn_dispatch)
        .chain(info_dispatch)
        .chain(debug_dispatch)
        .chain(trace_dispatch)
        .chain(std::io::stdout());

    // Apply the logger configuration
    combined_config.apply().expect("Failed to initialize logger.");
}
