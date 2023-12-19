use crate::StressNgArch;

/// `StressTest` Trait
///
/// Defines an interface for performing various stress tests on system resources.
/// This trait abstracts the functionality for running stress tests, allowing
/// different implementations for CPU load, memory pressure, I/O stress, etc.
///
/// Implementors can offer specific strategies for stress testing, adhering to
/// the Dependency Inversion Principle for flexibility and loose coupling in application design.
pub trait StressTest {
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
    /// Returns the serial number as a `String`.
    fn get_system_serial_number(&self) -> String;

    /// Determines the appropriate `stress-ng` binary version based on the OS.
    ///
    /// This method should identify the operating system and return the corresponding
    /// `StressNgArch` enum variant, ensuring compatibility and proper functioning
    /// of stress tests across different platforms.
    ///
    /// Returns an enum variant representing the architecture-specific version of `stress-ng`.
    fn decide_stress_ng_arch() -> StressNgArch;

    /// Retrieves the serial number of the system based on the operating system.
    ///
    /// This static method encapsulates the logic for fetching the serial number, varying
    /// with the target operating system. It is designed to be used internally by implementations
    /// of the `StressTest` trait.
    ///
    /// Returns a `Result` containing the system's serial number as a `String` or an error.
    fn get_serial_number() -> Result<String, &'static str>;

    // Placeholder for additional stress test methods, e.g., memory or I/O tests.
    // fn run_memory_tests(&self);
    // fn run_io_tests(&self);
}
