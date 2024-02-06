// stress_ng_manager.rs

use std::fs::File;
use std::io::{self, Write};

use common::ports::log_port::LoggerPort;

// Required imports for file handling and IO operations.

// Constants holding the embedded binary data for stress-ng for different operating systems.
// These binaries are included at compile time and used for stress testing.
pub const STRESS_NG_LINUX: &'static [u8] = include_bytes!("linux/stress-ng");
pub const STRESS_NG_MACOS: &'static [u8] = include_bytes!("macOS/stress-ng");

/// Enumeration to represent the different architectures for stress-ng.
/// This helps in determining the correct binary to use based on the operating system.
#[derive(Debug)]
pub enum StressNgArch {
    Linux,
    MacOS,
}

/// Writes the stress-ng binary to disk based on the provided architecture.
///
/// # Arguments
///
/// * `stress_ng_arch` - The architecture for which to write the stress-ng binary.
/// * `logr` - Logger implementation for logging debug or error information.
///
/// # Returns
///
/// Returns `Ok(())` if the binary is written successfully, or an `Err` with the respective IO error.
///
pub fn write_stress_ng_to_disk(
    stress_ng_arch: StressNgArch,
    logr: &dyn LoggerPort,
) -> Result<(), std::io::Error> {
    // Determine which binary data and filename to use based on the specified architecture.
    let (binary_data, filename) = match stress_ng_arch {
        StressNgArch::Linux => (STRESS_NG_LINUX, "stress-ng-linux"),
        StressNgArch::MacOS => (STRESS_NG_MACOS, "stress-ng-macos"),
    };

    // Attempt to write the binary data to disk and log the outcome.
    match write_binary_to_disk(binary_data, filename) {
        Ok(_) => {
            logr.log_debug(&format!(
                "Successfully wrote stress-ng binary to disk: {}",
                filename
            ));
            Ok(())
        }
        Err(e) => {
            logr.log_error(&format!(
                "Failed to write stress-ng binary to disk: {:?}",
                e
            ));
            Err(e)
        }
    }
}

/// Helper function to write binary data to disk.
///
/// # Arguments
///
/// * `data` - The binary data to be written to the file.
/// * `filename` - The name of the file to which the binary data will be written.
///
/// # Returns
///
/// Returns `Ok(())` on successful write, or an `Err` with the respective IO error.
///
pub fn write_binary_to_disk(data: &[u8], filename: &str) -> io::Result<()> {
    // Create a new file with the specified filename.
    let mut file = File::create(filename)?;

    // Write the binary data to the newly created file.
    file.write_all(data)?;

    Ok(())
}
