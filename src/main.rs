// Importing necessary modules and traits.
mod adapters;
mod domain;
mod ports;

use crate::adapters::stress_ng_adapter::{decide_stress_ng_arch, StressNgAdapter};
use crate::adapters::stress_ng_manager_adapter::{
    write_binary_to_disk, STRESS_NG_LINUX, STRESS_NG_MACOS,
};
use crate::ports::stress_test::StressTest;
use adapters::log_adapter::{init, FernLogger};
use domain::logging::LoggerPort;

/// Enumeration representing the supported architectures for the `stress-ng` binary.
/// This enum is used to select the correct binary for the running operating system.
enum StressNgArch {
    Linux,
    MacOS,
}

fn main() {
    // Initialize the logger for the application.
    // The logger is set up to write to "logs" with a level filter of Trace,
    // meaning all log messages at Trace level or higher will be recorded.
    init("logs", log::LevelFilter::Trace);

    // Create an instance of the logger.
    // `FernLogger` is an implementation of the `Logger` trait, which is used throughout the application.
    let logr = FernLogger;

    // Decide which `stress-ng` binary to use based on the operating system.
    // This function returns an enum variant indicating the suitable binary.
    let stress_ng_arch = decide_stress_ng_arch();

    // Determine the appropriate binary data and filename based on the chosen architecture.
    let (binary_data, filename) = match stress_ng_arch {
        StressNgArch::Linux => (STRESS_NG_LINUX, "stress-ng-linux"),
        StressNgArch::MacOS => (STRESS_NG_MACOS, "stress-ng-macos"),
    };

    // Attempt to write the binary data to disk.
    // Log success or error messages based on the outcome of the operation.
    match write_binary_to_disk(binary_data, filename) {
        Ok(_) => logr.log_debug(&format!(
            "Successfully wrote stress-ng binary to disk: {}",
            filename
        )),
        Err(e) => {
            logr.log_error(&format!(
                "Failed to write stress-ng binary to disk: {:?}",
                e
            ));
            return;
        }
    }

    // Create an instance of the StressNgAdapter.
    // This adapter is responsible for executing the stress tests using `stress-ng`.
    let stress_tester = StressNgAdapter::new(&logr);

    // Execute the CPU stress tests using the created instance of StressNgAdapter.
    stress_tester.run_cpu_tests();
}
