// common/src/lib.rs

// Importing the `async_trait` crate. This crate is used to enable async functions in traits,
// which is not natively supported in Rust.
use async_trait::async_trait;

/// This module provides a console logger that can be used across the frontend, primary, and backends of the application.
/// The logger is implemented as an async resource, which is a good practice for logging in concurrent applications.

/// A struct representing a console logger.
///
/// This struct implements the `LoggerPort` trait, providing methods for logging messages at various levels (info, warning, error, debug).
/// It uses an instance of `FernLogger` for the actual logging, allowing it to leverage all the capabilities of `FernLogger`.
pub struct ConsoleLogger {
    fern_logger: FernLogger,
}

impl ConsoleLogger {
    /// Creates a new instance of `ConsoleLogger`.
    ///
    /// # Returns
    ///
    /// * `ConsoleLogger` - The new `ConsoleLogger` instance.
    pub fn new() -> ConsoleLogger {
        ConsoleLogger {
            fern_logger: FernLogger::new(),
        }
    }
}

// Implementing the `LoggerPort` trait for `ConsoleLogger`.
#[async_trait]
impl LoggerPort for ConsoleLogger {
    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_info(&self, message: &str) {
        self.fern_logger.log_info(message);
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_warn(&self, message: &str) {
        self.fern_logger.log_warn(message);
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_error(&self, message: &str) {
        self.fern_logger.log_error(message);
    }

    /// Logs a debug message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log.
    fn log_debug(&self, message: &str) {
        self.fern_logger.log_debug(message);
    }
}

// The main function where the `ConsoleLogger` is used.
#[actix_rt::main]
async fn main() {
    // Create a new `ConsoleLogger` instance.
    let logger = ConsoleLogger::new();

    // Log an informational message.
    logger.log_info("Hello, world!");
}
