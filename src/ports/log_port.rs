/// The `LoggerPort` trait acts as a port in the hexagonal architecture.
/// It defines a standard interface for logging functionality.
/// This abstraction allows for different logging implementations
/// that can be plugged into the application as needed.
pub trait LoggerPort: Sync + Send {
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
