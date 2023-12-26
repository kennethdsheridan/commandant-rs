use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, io, str};

use crate::adapters::log_adapter::FernLogger;
// Assuming these are defined elsewhere in your project
use crate::adapters::stress_ng_manager_adapter::{STRESS_NG_LINUX, STRESS_NG_MACOS};
use crate::domain::logging::LoggerPort;
use crate::ports::stress_test::StressTest;
use crate::StressNgArch;

pub struct StressNgAdapter<'a> {
    logger: &'a dyn LoggerPort,
}

impl<'a> StressNgAdapter<'a> {
    /// Creates a new instance of `StressNgAdapter`.
    ///
    /// # Arguments
    /// * `logger` - A reference to an object that implements the `Logger` trait.
    ///
    /// # Returns
    /// An instance of `StressNgAdapter`.
    pub fn new(logger: &'a dyn LoggerPort) -> Self {
        Self { logger }
    }

    /// Decides which `stress-ng` binary to use based on the operating system.
    ///
    /// # Arguments
    /// * `logger` - Logger implementation for logging messages.
    ///
    /// # Returns
    /// `StressNgArch`: The architecture-specific enum variant for `stress-ng`.
    pub fn decide_stress_ng_arch(logger: &dyn LoggerPort) -> StressNgArch {
        let arch = if cfg!(target_os = "linux") {
            logger.log_debug("Selected stress-ng binary for Linux");
            StressNgArch::Linux
        } else if cfg!(target_os = "macos") {
            logger.log_debug("Selected stress-ng binary for macOS");
            StressNgArch::MacOS
        } else {
            // Defaulting to Linux for other operating systems
            logger.log_debug("Defaulted to stress-ng binary for Linux");
            StressNgArch::Linux
        };

        arch
    }

    /// Prepares and writes the stress-ng binary to a temporary file.
    ///
    /// # Arguments
    /// * `logger` - Logger implementation for logging messages.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the operation.
    pub fn prepare_stress_ng_binary(logger: &dyn LoggerPort) -> Result<String, String> {
        logger.log_debug("Starting preparation of stress-ng binary");

        let arch = StressNgAdapter::decide_stress_ng_arch(logger);
        logger.log_debug(&format!("Determined system architecture: {:?}", arch));

        let binary_data = match arch {
            StressNgArch::Linux => {
                logger.log_debug("Selected stress-ng binary for Linux");
                STRESS_NG_LINUX
            }
            StressNgArch::MacOS => {
                logger.log_debug("Selected stress-ng binary for MacOS");
                STRESS_NG_MACOS
            }
        };

        let temp_file_path = "../stress-ng-binary".to_string();
        logger.log_debug(&format!(
            "Attempting to write stress-ng binary to {}",
            temp_file_path
        ));

        // Use write_binary function to write data to the file
        match StressNgAdapter::write_binary(&temp_file_path, binary_data) {
            Ok(_) => logger.log_debug("stress-ng binary successfully written to disk"),
            Err(e) => {
                let error_msg = format!("Failed to write stress-ng binary: {:?}", e);
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
        }

        // Setting file permissions
        match fs::metadata(&temp_file_path) {
            Ok(metadata) => {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                if let Err(e) = fs::set_permissions(&temp_file_path, perms) {
                    let error_msg = format!(
                        "Failed to set executable permissions on {}: {:?}",
                        temp_file_path, e
                    );
                    logger.log_error(&error_msg);
                    return Err(error_msg);
                }
                logger.log_debug("Executable permissions set on stress-ng binary");
            }
            Err(e) => {
                let error_msg = format!("Failed to read metadata for {}: {:?}", temp_file_path, e);
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
        }

        logger.log_debug(&format!(
            "Successfully prepared stress-ng binary at {}",
            temp_file_path
        ));
        Ok(temp_file_path)
    }

    pub fn execute_stress_ng_command(logger: &dyn LoggerPort, args: &[&str])
                                     -> Result<(), String> {
        let binary_path = "../stress-ng-binary".to_string();

        // Check if the binary exists and is executable
        if !Path::new(&binary_path).exists() {
            logger.log_debug(&format!(
                "Binary not found at {}. Preparing binary...",
                binary_path
            ));

            // Call the function to prepare the stress-ng binary
            if let Err(e) = StressNgAdapter::prepare_stress_ng_binary(logger) {
                return Err(e);
            }
        }

        // Check if the file has execute permissions
        if let Ok(metadata) = fs::metadata(&binary_path) {
            if metadata.permissions().mode() & 0o111 == 0 {
                let error_msg = format!("Binary at {} does not have execute \
            permissions", binary_path);
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
            logger.log_debug(&format!("Binary at {} has execute permissions",
                                      binary_path));
        } else {
            let error_msg = format!("Failed to read metadata for binary at \
        {}", binary_path);
            logger.log_error(&error_msg);
            return Err(error_msg);
        }

        // Define the output file path
        let output_file_path = "stress_ng_output.txt";

        // Create or open the file to capture the command's output
        let output_file = match File::create(output_file_path) {
            Ok(file) => file,
            Err(e) => {
                let error_msg = format!("Failed to create output file: {}", e);
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
        };

        // Prepare the stress-ng command with redirection of output to file
        let mut command = Command::new(&binary_path);
        command.args(args);
        logger.log_debug(&format!("Preparing stress-ng command with args: {:?}", args));

// Attempt to set up output redirection
        match output_file.try_clone() {
            Ok(output_file_clone) => {
                command.stdout(Stdio::from(output_file_clone));
                command.stderr(Stdio::from(output_file));
            },
            Err(e) => {
                logger.log_warn(&format!("Failed to clone output file handle: {}", e));
            }
        }

// Execute the stress-ng command
        match command.spawn() {
            Ok(mut child) => {
                logger.log_debug("stress-ng command spawned, waiting for it to finish");

                match child.wait() {
                    Ok(_) => {
                        logger.log_debug("stress-ng command finished successfully");

                        // Attempting to clean up the binary
                        match StressNgAdapter::remove_stress_ng_binary(logger,
                                                          &binary_path) {
                            Ok(()) => logger.log_debug(&format!("Cleaned up binary at {}", binary_path)),
                            Err(e) => {
                                logger.log_error(&format!("Cleanup failed for binary at {}: {}", binary_path, e));
                                return Err(e);
                            }
                        }
                        Ok(())
                    },
                    Err(e) => {
                        logger.log_error(&format!("Execution failed for stress-ng command: {}", e));
                        Err(e.to_string())
                    }
                }
            },
            Err(e) => {
                logger.log_error(&format!("Failed to spawn stress-ng command: {}", e));
                Err(e.to_string())
            }
        }
    }


/// Removes the stress-ng binary from the filesystem with extensive logging.
///
/// This function attempts to remove the stress-ng binary file specified by the `binary_path`.
/// It logs the process and outcome, providing debug information on successful removal and
/// error details in case of failure.
///
/// # Arguments
/// * `logger` - Logger implementation for logging messages.
/// * `binary_path` - The file path of the stress-ng binary to be removed.
///
/// # Returns
/// A `Result<(), String>` indicating the success (`Ok(())`) or failure (`Err(String)`)
/// of the removal operation.
pub fn remove_stress_ng_binary(logger: &dyn LoggerPort, binary_path: &str)
    ->Result<(), String> {
    logger.log_debug(&format!(
        "Attempting to remove stress-ng binary at {}",
        binary_path
    ));

    match std::fs::remove_file(binary_path) {
        Ok(_) => {
            logger.log_debug(&format!(
                "Successfully removed stress-ng binary at {}",
                binary_path
            ));
            Ok(())
        }
        Err(e) => {
            let error_msg = format!(
                "Failed to remove stress-ng binary at {}: {}",
                binary_path, e
            );
            logger.log_error(&error_msg);
            Err(error_msg)
        }
    }
}

/// Writes binary data to a file if it doesn't already exist.
///
/// # Arguments
///
/// * `file_path` - The path to the file to write to.
/// * `data` - The binary data to write to the file.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation.
pub fn write_binary(file_path: &str, data: &[u8]) -> io::Result<()> {
    let path = Path::new(file_path);

    // Check if the file already exists
    if !path.exists() {
        // Open the file in write-only mode, create it if it doesn't exist
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true) // Ensures a new file is created
            // it exists
            .open(file_path)?;

        // Write the binary data to the file
        file.write_all(data)?;
    }

    Ok(())
}}

