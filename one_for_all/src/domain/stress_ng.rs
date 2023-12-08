//! Stress-ng Domain Entity
//!
//! This module provides a domain entity for stress-ng, a tool for stress testing
//! CPU and memory on Linux and macOS systems.
// Include the stress-ng binaries

use std::collections::HashSet;


/// Represents the stress-ng configuration.
pub struct StressNgConfig {
    pub cpu_load: u32,
    pub memory_load: u32,
    pub timeout: u32,
    pub aggressive: bool,
    pub keep_name: bool,
    pub log_brief: bool,
    pub taskset: Option<HashSet<u32>>,
}

impl StressNgConfig {
    /// Creates a new stress-ng configuration with the specified CPU and memory load.
    ///
    /// # Arguments
    ///
    /// * `cpu_load` - The desired CPU load as a percentage.
    /// * `memory_load` - The desired memory load in megabytes.
    /// * `timeout` - The duration of the stress test in seconds.
    /// * `aggressive` - Enables more file, cache, and memory aggressive options.
    /// * `keep_name` - Keeps the process names to be the name of the parent process.
    /// * `log_brief` - Outputs messages without program name, message type, and process id.
    /// * `taskset` - Sets CPU affinity based on the list of CPUs provided.
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

    // Add methods to run the stress tests here...
}


