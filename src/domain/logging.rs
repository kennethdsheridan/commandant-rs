// Logger trait is a port in the hexagonal architecture.
pub trait LoggerPort {
    fn log_info(&self, message: &str);
    fn log_warn(&self, message: &str);
    fn log_error(&self, message: &str);
    fn log_debug(&self, message: &str);
    fn log_trace(&self, message: &str);
}

pub struct MyLogger;

impl LoggerPort for MyLogger {
    fn log_info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    fn log_warn(&self, message: &str) {
        println!("[WARN] {}", message);
    }

    fn log_error(&self, message: &str) {
        println!("[ERROR] {}", message);
    }

    fn log_debug(&self, message: &str) {
        println!("[DEBUG] {}", message);
    }

    fn log_trace(&self, message: &str) {
        println!("[TRACE] {}", message);
    }
}
