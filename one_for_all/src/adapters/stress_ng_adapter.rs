// src/adapters/stress_ng_adapter.rs

use crate::domain::stress_test::StressTest;
use std::process::Command;

/// An adapter that implements the `StressTest` trait using the `stress-ng` tool.
///
/// `StressNgAdapter` provides an implementation for running CPU stress tests
/// by executing the `stress-ng` command-line utility. This adapter abstracts
/// the details of how the stress tests are performed, allowing the rest of the
/// application to remain agnostic to the specifics of the `stress-ng` tool.
pub struct StressNgAdapter;

impl StressTest for StressNgAdapter {
    /// Runs CPU stress tests using `stress-ng`.
    ///
    /// This function launches the `stress-ng` process with predefined arguments
    /// to stress test the CPU. The number of CPUs to stress and the duration of the
    /// test are currently hardcoded but can be parameterized for flexibility.
    ///
    /// # Panics
    /// Panics if the `stress-ng` command fails to start or if waiting for the
    /// process output fails. In a production environment, consider using more
    /// robust error handling to gracefully handle these errors.
    fn run_cpu_tests(&self) {
        // Define the stress-ng command for CPU tests
        let cpu_test_command = Command::new("stress-ng")
            .arg("--cpu")
            .arg("2") // Number of CPUs to stress. Consider parameterizing this.
            .arg("--timeout")
            .arg("30s") // Duration of the stress test. Consider parameterizing this.
            .spawn()
            .expect("Failed to start stress-ng process");

        // Handling the process output. In a real-world scenario, you might want
        // to parse this output and use it to monitor the results of the test.
        let output = cpu_test_command
            .wait_with_output()
            .expect("Failed to wait on stress-ng process");

        // Output the results of the stress test. Consider logging this instead
        // of merely printing it to stdout.
        println!("stress-ng output: {:?}", output);
    }
}
