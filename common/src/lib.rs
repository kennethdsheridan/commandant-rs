// common/src/lib.rs

// The `async_trait` crate is used to enable async functions in traits, which is not natively supported in Rust.
use async_trait::async_trait;

// The `LoggerPort` trait defines the logging interface that our `ConsoleLogger` struct will implement.
use crate::adapters::log_adapter::FernLogger;
use crate::ports::log_port::LoggerPort;

mod adapters;

// This module provides a console logger that can be used across the frontend, primary, and backends of the application.
// The logger is implemented as an async resource, which is a good practice for logging in concurrent applications.

/// A struct representing a console logger.
///
/// This struct implements the `LoggerPort` trait, providing methods for logging messages at various levels (info, warning, error, debug).
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
#[actix_rt::main]
async fn main() {
    // Create a new `ConsoleLogger` instance.
    let logger = ConsoleLogger::new();

    // Log an informational message.
    logger.log_info("Hello, world!");
}
