

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
