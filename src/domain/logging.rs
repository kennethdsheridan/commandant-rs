/// The `LoggerPort` trait acts as a port in the hexagonal architecture.
/// It defines a standard interface for logging functionality.
/// This abstraction allows for different logging implementations
/// that can be plugged into the application as needed.
pub trait LoggerPort {
    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `message` - The informational message to be logged.
    fn log_info(&self, message: &str);

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `message` - The warning message to be logged.
    fn log_warn(&self, message: &str);

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to be logged.
    fn log_error(&self, message: &str);

    /// Logs a debug message.
    ///
    /// # Arguments
    ///
    /// * `message` - The debug message to be logged.
    fn log_debug(&self, message: &str);

    /// Logs a trace message.
    ///
    /// # Arguments
    ///
    /// * `message` - The trace message to be logged.
    fn log_trace(&self, message: &str);
}

/// `MyLogger` is a simple implementation of the `LoggerPort` trait.
/// This struct provides basic logging functionality to the console.
pub struct MyLogger;

impl LoggerPort for MyLogger {
    /// Logs an informational message to the console with [INFO] level.
    fn log_info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    /// Logs a warning message to the console with [WARN] level.
    fn log_warn(&self, message: &str) {
        println!("[WARN] {}", message);
    }

    /// Logs an error message to the console with [ERROR] level.
    fn log_error(&self, message: &str) {
        println!("[ERROR] {}", message);
    }

    /// Logs a debug message to the console with [DEBUG] level.
    /// Typically used for detailed debugging information.
    fn log_debug(&self, message: &str) {
        println!("[DEBUG] {}", message);
    }

    /// Logs a trace message to the console with [TRACE] level.
    /// Used for low-level functionality tracing.
    fn log_trace(&self, message: &str) {
        println!("[TRACE] {}", message);
    }
}
