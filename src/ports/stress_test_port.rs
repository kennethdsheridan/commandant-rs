use common::ports::log_port::LoggerPort;

use crate::ports::database_port::DatabasePort;
use crate::StressNgArch;

/// `StressTestPort` Trait
///
/// Defines an interface for performing various stress tests on system resources.
/// This trait abstracts the functionality for running stress tests, allowing
/// different implementations for CPU load, memory pressure, I/O stress, etc.
///
/// Implementors can offer specific strategies for stress testing, adhering to
/// the Dependency Inversion Principle for flexibility and loose coupling in application design.
pub trait StressTestPort {
    /// Executes CPU stress tests.
    ///
    /// Implement this method to perform stress testing on the CPU. The implementation
    /// should effectively load the CPU and may include monitoring or logging mechanisms
    /// to track the progress and results of the tests.
    fn run_cpu_tests(&self);

    /// Retrieves the system's serial number.
    ///
    /// Implement this method to return the serial number of the system. The method
    /// should handle the retrieval process based on the operating system.
    ///
    /// # Returns
    /// The serial number as a `String`.
    fn get_system_serial_number(&self) -> String;

    /// Determines the appropriate `stress-ng` binary version based on the OS.
    ///
    /// This method should identify the operating system and return the corresponding
    /// `StressNgArch` enum variant, ensuring compatibility and proper functioning
    /// of stress tests across different platforms.
    ///
    /// # Returns
    /// An enum variant representing the architecture-specific version of `stress-ng`.
    fn decide_stress_ng_arch() -> StressNgArch;

    /// Retrieves the serial number of the system based on the operating system.
    ///
    /// This static method encapsulates the logic for fetching the serial number, varying
    /// with the target operating system. It is designed to be used internally by implementations
    /// of the `StressTestPort` trait.
    ///
    /// # Returns
    /// A `Result` containing the system's serial number as a `String` or an error.
    fn get_serial_number() -> Result<String, &'static str>;

    /// Executes the `stress-ng` command to perform stress tests.
    ///
    /// Implement this method to perform stress testing using the `stress-ng` command.
    /// The implementation should load the CPU and may include features for sorting, filtering,
    /// or limiting the output for specific monitoring requirements.
    ///
    /// # Arguments
    /// * `stress_ng_arch` - The architecture-specific version of `stress-ng` to use.
    /// * `stress_ng_args` - A vector of arguments to pass to the `stress-ng` command.
    /// * `output_file_path` - The path to the file where the command output will be saved.
    /// * `log_file_path` - The path to the file where the command output will be saved.
    /// * `db` - A reference to an object that implements the `DatabasePort` trait.
    /// * `logger` - A reference to an object that implements the `LoggerPort` trait.
    /// * `key` - The key to use for writing to the database.
    /// * `value` - The value to use for writing to the database.
    /// * `duration` - The duration of the stress test in seconds.
    /// * `interval` - The interval at which to write to the database in seconds.
    /// * `write_to_db` - A boolean indicating whether to write to the database.
    /// * `write_to_file` - A boolean indicating whether to write to a file.
    /// * `write_to_stdout` - A boolean indicating whether to write to stdout.
    /// * `write_to_logger` - A boolean indicating whether to write to the logger.
    ///
    /// # Returns
    /// A `Result` containing either the command output as a `String` or an error.
    fn execute_stress_ng_command(
        &self,
        stress_ng_arch: StressNgArch,
        stress_ng_args: Vec<String>,
        output_file_path: &str,
        log_file_path: &str,
        db: &dyn DatabasePort,
        logger: &dyn LoggerPort,
        key: &[u8],
        value: &[u8],
        duration: u64,
        interval: u64,
        write_to_db: bool,
        write_to_file: bool,
        write_to_stdout: bool,
        write_to_logger: bool,
    ) -> Result<String, String>;

    // Placeholder for additional stress test methods, e.g., memory or I/O tests.
    // fn run_memory_tests(&self);
    // fn run_io_tests(&self);
}
