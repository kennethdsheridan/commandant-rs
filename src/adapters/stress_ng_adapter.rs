use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, str};

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
            _ => {
                let error_msg = "Unsupported OS for stress-ng binary".to_string();
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
        };

        let temp_file_path = "../stress-ng-binary".to_string();
        logger.log_debug(&format!(
            "Attempting to write stress-ng binary to {}",
            temp_file_path
        ));

        match File::create(&temp_file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(binary_data) {
                    let error_msg = format!(
                        "Failed to write stress-ng binary to {}: {:?}",
                        temp_file_path, e
                    );
                    logger.log_error(&error_msg);
                    return Err(error_msg);
                }

                logger.log_debug("stress-ng binary successfully written to disk");

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
                        let error_msg =
                            format!("Failed to read metadata for {}: {:?}", temp_file_path, e);
                        logger.log_error(&error_msg);
                        return Err(error_msg);
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to create file at {}: {:?}", temp_file_path, e);
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

    pub fn execute_stress_ng_command(logger: &dyn LoggerPort, args: &[&str]) -> Result<(), String> {
        let binary_path = "../stress-ng-binary".to_string();

        // Check if the binary exists and is executable
        if !Path::new(&binary_path).exists() {
            let error_msg = format!("Binary not found at path: {}", binary_path);
            logger.log_error(&error_msg);
            // Additional logic for handling binary preparation
            // ...
            return Err(error_msg);
        }

        // Check if the file has execute permissions
        if let Ok(metadata) = fs::metadata(&binary_path) {
            if metadata.permissions().mode() & 0o111 == 0 {
                let error_msg = format!("Binary at {} does not have execute permissions", binary_path);
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
            logger.log_debug(&format!("Binary at {} has execute permissions", binary_path));
        } else {
            let error_msg = format!("Failed to read metadata for binary at {}", binary_path);
            logger.log_error(&error_msg);
            return Err(error_msg);
        }

        // Define the output file path
        let output_file_path = "stress_ng_output.txt";

        // Create or open the file to capture the command's output
        let output_file = match File::create(&output_file_path) {
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
        command.stdout(Stdio::from(output_file.try_clone().map_err(|e| e.to_string())?));
        command.stderr(Stdio::from(output_file));

        // Execute the stress-ng command
        match command.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(_) => {
                        logger.log_debug("stress-ng command executed successfully");
                        // Clean up by removing the `stress-ng` binary from the filesystem
                        // Correct the call to remove_stress_ng_binary in your existing code
                        match remove_stress_ng_binary(logger, &binary_path) {
                            Ok(()) => logger.log_debug(&format!("Successfully cleaned up {}", binary_path)),
                            Err(e) => {
                                logger.log_error(&format!(
                                    "Failed to remove stress-ng binary at {}: {}",
                                    binary_path, e
                                ));
                                return Err(e);
                            }
                        }
                        Ok(())
                    },
                    Err(e) => {
                        logger.log_error(&format!("Failed to execute stress-ng command: {}", e));
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
pub fn remove_stress_ng_binary(logger: &dyn LoggerPort, binary_path: &str) -> Result<(), String> {
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

