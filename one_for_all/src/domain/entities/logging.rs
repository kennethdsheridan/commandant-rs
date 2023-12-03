// Logger trait is a port in the hexagonal architecture.
pub trait Logger {
    fn log_info(&self, message: &str);
    fn log_warn(&self, message: &str);
    fn log_error(&self, message: &str);
    fn log_debug(&self, message: &str);
    fn log_trace(&self, message: &str);
}
