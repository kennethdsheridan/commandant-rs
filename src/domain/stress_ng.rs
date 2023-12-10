//! Stress-ng Domain Entity
//!
//! This module provides a domain entity for stress-ng, a tool for stress testing
//! CPU and memory on Linux and macOS systems.

use std::collections::HashSet;

/// Represents the stress-ng configuration.
/// This struct is used to configure and manage the parameters for a stress-ng test.
/// It includes settings for CPU load, memory load, test duration, and other options.
pub struct StressNgConfig {
    /// The desired CPU load as a percentage.
    /// This value is used to set the amount of CPU stress to be applied during the test.
    pub cpu_load: u32,

    /// The desired memory load in megabytes.
    /// This value is used to set the amount of memory stress to be applied during the test.
    pub memory_load: u32,

    /// The duration of the stress test in seconds.
    /// This value defines how long the stress test will run.
    pub timeout: u32,

    /// Enables more aggressive file, cache, and memory stress options.
    /// When set to true, the test may apply more intense stress on the system.
    pub aggressive: bool,

    /// Keeps the process names the same as the parent process.
    /// This option can be useful for logging or monitoring purposes.
    pub keep_name: bool,

    /// Outputs messages without the program name, message type, and process ID.
    /// This option simplifies the log output for easier parsing or reading.
    pub log_brief: bool,

    /// Sets CPU affinity based on the list of CPUs provided.
    /// This option allows specifying which CPUs should be stressed, providing more control over the test.
    pub taskset: Option<HashSet<u32>>,
}

impl StressNgConfig {
    /// Creates a new stress-ng configuration with the specified parameters.
    ///
    /// This constructor initializes a `StressNgConfig` instance with detailed settings
    /// for running a stress test. Each parameter can be customized to tailor the stress test
    /// to specific needs or conditions.
    ///
    /// # Arguments
    ///
    /// * `cpu_load` - CPU load percentage.
    /// * `memory_load` - Memory load in megabytes.
    /// * `timeout` - Duration of the test in seconds.
    /// * `aggressive` - Enable more aggressive test options.
    /// * `keep_name` - Keep process names unchanged.
    /// * `log_brief` - Simplify log output format.
    /// * `taskset` - Set specific CPU affinity for the test.
    ///
    /// # Returns
    ///
    /// * `StressNgConfig` - A new instance with the provided configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = StressNgConfig::new(50, 1024, 60, true, false, true, None);
    /// // Use `config` to run a stress-ng test
    /// ```
    pub fn new(
        cpu_load: u32,
        memory_load: u32,
        timeout: u32,
        aggressive: bool,
        keep_name: bool,
        log_brief: bool,
        taskset: Option<HashSet<u32>>,
    ) -> Self {
        Self {
            cpu_load,
            memory_load,
            timeout,
            aggressive,
            keep_name,
            log_brief,
            taskset,
        }
    }

    // Additional methods to run the stress tests can be added here...
}
