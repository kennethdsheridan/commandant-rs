use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
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

    /// Executes a specific `stress-ng` command with given arguments.
    ///
    /// This function prepares and executes the `stress-ng` binary with the provided command-line arguments.
    /// It logs the execution process and results, and ensures that the `stress-ng` binary is removed
    /// from the filesystem after execution.
    ///
    /// # Arguments
    /// * `logger` - Logger implementation for logging messages.
    /// * `args` - Command-line arguments to pass to `stress-ng`.
    ///
    /// # Returns
    /// `Result<(), String>`: Ok(()) on success, or an error message wrapped in `Err` on failure.
    ///
    /// # Examples
    /// ```
    /// let logger = ...; // An instance of a Logger implementation
    /// let args = ["--cpu", "4", "--timeout", "60s"];
    /// StressNgAdapter::execute_stress_ng_command(&logger, &args).expect("Stress test failed");
    /// ```
    pub fn execute_stress_ng_command(logger: &dyn LoggerPort, args: &[&str]) -> Result<(), String> {
        let binary_path = "../stress-ng-binary".to_string();

        // Check if the binary exists and is executable
        if !Path::new(&binary_path).exists() {
            let error_msg = format!("Binary not found at path: {}", binary_path);
            logger.log_error(&error_msg);
            logger.log_debug("Attempting to prepare stress-ng binary");
            // write the binary to disk if using prepare_stress_ng_binary
            let binary_path = match StressNgAdapter::prepare_stress_ng_binary(logger) {
                Ok(path) => path,
                Err(e) => {
                    logger.log_error(&format!("Failed to prepare stress-ng binary: {}", e));
                    return Err(e);
                }
            };
            return Err(error_msg);
        }

        // Check if the file has execute permissions
        if let Ok(metadata) = fs::metadata(&binary_path) {
            if metadata.permissions().mode() & 0o111 == 0 {
                let error_msg = format!(
                    "Binary at {} does not have execute permissions",
                    binary_path
                );
                logger.log_error(&error_msg);
                return Err(error_msg);
            }
            logger.log_debug(&format!(
                "Binary at {} has execute permissions",
                binary_path
            ));
        } else {
            let error_msg = format!("Failed to read metadata for binary at {}", binary_path);
            logger.log_error(&error_msg);
            return Err(error_msg);
        }

        // Execute the stress-ng command with the specified arguments
        let mut command = Command::new(&binary_path);
        command.args(args);
        logger.log_debug(&format!("Binary path: {}", binary_path));
        // Show the file type of the binary
        match Command::new("file").arg(&binary_path).output() {
            Ok(output) => {
                logger.log_debug(&format!(
                    "Binary file type: {}",
                    str::from_utf8(&output.stdout).unwrap()
                ));
            }
            Err(e) => {
                logger.log_error(&format!("Failed to execute file command: {}", e));
                return Err(format!("Failed to execute file command: {}", e));
            }
        }
        logger.log_debug(&format!("Executing stress-ng with args: {:?}", args));

// Attempt to spawn the `stress-ng` command
        match command.spawn() {
            // If the command spawns successfully...
            Ok(mut child) => {
                // Wait for the command to complete and capture its output
                match child.wait_with_output() {
                    // If the command completes successfully...
                    Ok(output) => {
                        // Log a debug message indicating successful execution
                        logger.log_debug("stress-ng command executed successfully");

                        // If there's any standard error output, log it as an error
                        if !output.stderr.is_empty() {
                            logger.log_error(&format!(
                                "stress-ng encountered an error: {}",
                                String::from_utf8_lossy(&output.stderr)
                            ));
                        }

                        // If there's any standard output, log it; otherwise, log that there's no output
                        if !output.stdout.is_empty() {
                            logger.log_debug(&format!(
                                "stress-ng output: {}",
                                String::from_utf8_lossy(&output.stdout)
                            ));
                        } else {
                            logger.log_debug("No output captured from stress-ng command");
                        }

                        // Clean up by removing the `stress-ng` binary from the filesystem
                        match remove_stress_ng_binary(&binary_path) {
                            Ok(()) => logger.log_debug(&format!("Successfully cleaned up {}", binary_path)),
                            Err(e) => {
                                logger.log_error(&format!(
                                    "Failed to remove stress-ng binary at {}: {}",
                                    binary_path, e
                                ));
                                // Return an error if cleanup fails
                                return Err(e);
                            }
                        }

                        // Indicate successful execution of the whole process
                        Ok(())
                    },
                    // If there's an error in executing the command...
                    Err(e) => {
                        // Log an error message with the failure details
                        logger.log_error(&format!("Failed to execute stress-ng command: {}", e));
                        // Return an error
                        Err(e.to_string())
                    }
                }
            },
            // If there's an error in spawning the command...
            Err(e) => {
                // Log an error message with the failure details
                logger.log_error(&format!("Failed to spawn stress-ng command: {}", e));
                // Return an error
                Err(e.to_string())
            }
        }

        }
    }
    // Other additional methods specific to the StressNgAdapter...

    /// Removes the stress-ng binary from the filesystem with extensive logging.
    ///
    /// # Arguments
    /// * `logger` - Logger implementation for logging messages.
    /// * `binary_path` - The file path of the stress-ng binary to be removed.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the removal operation.
    pub fn remove_stress_ng_binary(binary_path: &str) -> Result<(), String> {
        let logger = FernLogger;
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
                    "Failed to remove stress-ng binary at {}: {:?}",
                    binary_path, e
                );
                logger.log_error(&error_msg);
                Err(error_msg)
            }
        }
    }

