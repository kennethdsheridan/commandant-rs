//! PS Adapter
//!
//! This module provides an adapter for the `ps` command, a tool for monitoring
//! process statuses and CPU usage on Unix-based systems.

use crate::ports::database_port::DatabasePort;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::ports::log_port::LoggerPort;
use crate::ports::ps_command_port::PsCommandPort;

// The ProcessData struct represents a single process and its CPU usage percentage.
// This struct is used to parse the output of the `ps` command and extract the CPU usage
// percentage for each process in the list. The `ps` command outputs a list of processes
// sorted by CPU usage, and this struct is used to parse each line of the output.
struct ProcessData {
    user: String,
    cpu_usage: f32,
    memory_usage: f32,
    command: String,
}

/// Represents the `ps` command adapter.
/// This struct is used to execute the `ps` command and manage its output.
pub struct PsAdapter {
    logger: Arc<dyn LoggerPort>, // inject the logger port
    db: Arc<dyn DatabasePort>,   // inject the database port
}

impl PsAdapter {
    /// Creates a new instance of `PsAdapter`.
    ///
    /// # Arguments
    /// * `logger` - A reference to an object that implements the `Logger` trait.
    ///
    /// # Returns
    /// An instance of `PsAdapter`.
    pub fn new(logger: Arc<dyn LoggerPort>, db: Arc<dyn DatabasePort>) -> Self {
        // Implement the `new` method for `PsAdapter`
        PsAdapter { logger, db } // Return a new instance of `PsAdapter` with the specified logger
    }
}

// Implement the `PsCommandPort` trait for `PsAdapter`. This allows the adapter to be used
// as a port in the application, and it also provides a concrete implementation of the
// `PsCommandPort` interface.
impl PsCommandPort for PsAdapter {
    /// Executes the `ps` command to gather CPU statistics and writes to a file.
    fn execute_ps_command(&self) -> Result<String, String> {
        // Execute the `ps` command
        let output = Command::new("sh") // Create a new `Command` instance
            .arg("-c") // Add an argument to the command
            .arg("ps aux | sort -nrk 3,3 | head -n 10") // Add an argument to the command
            .output() // output() returns a `std::process::Output` instance containing the command output
            .map_err(|e| e.to_string())?; // Convert the `std::io::Error` to a `String`

        if !output.status.success() {
            // Check if the command was successful
            let error_message = format!(
                "Failed to execute `ps` command: {}",
                String::from_utf8_lossy(&output.stderr) // Convert the byte array to a string slice
            );
            self.logger.log_error(&error_message);
            return Err(error_message); // Return `Err` to indicate failure
        }

        // Convert the byte array to a string and return it as a `Result`
        // The `?` operator will return an error if the conversion fails
        let output_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;

        Ok(output_str) // Return `Ok` to indicate success
    }

    /// Writes the output of the `ps` command to a specified file.
    fn write_to_file(&self, output: String, file_path: &str) -> Result<(), String> {
        match OpenOptions::new() // Create a new `OpenOptions` instance
            .create(true) // Create the file if it doesn't exist
            .write(true) // Allow writing to the file
            .append(true) // Append to the file instead of overwriting
            .open(file_path) // Open the file
        {
            Ok(mut file) => { // The file was successfully opened
                file.write_all(output.as_bytes()) // Write the output to the file
                    .map_err(|e| e.to_string())?; // Convert the `std::io::Error` to a `String`
                Ok(()) // Return `Ok` to indicate success
            }
            Err(e) => {
                let error_message = format!("Failed to open file: {}", e); // Create an error message
                self.logger.log_error(&error_message);
                Err(error_message) // Return `Err` to indicate failure
            }
        }
    }

    /// Periodically executes the `ps` command to gather CPU statistics and writes to a file.
    ///
    /// # Arguments
    /// * `output_file_path` - The path to the file where the command output will be saved.
    fn collect_cpu_statistics(&self, output_file_path: &str) {
        // Implement the `collect_cpu_statistics` method
        loop {
            // Loop forever
            match self.execute_ps_command() {
                // Execute the `ps` command
                Ok(output) => {
                    // The `ps` command was executed successfully
                    if let Err(e) = self.write_to_file(output, output_file_path) {
                        // An error occurred while writing to the file
                        self.logger.log_error(&e);
                        break;
                    }
                }
                Err(e) => {
                    // An error occurred while executing the `ps` command
                    self.logger.log_error(&e);
                    break; // Exit the loop
                }
            }
            thread::sleep(Duration::from_secs(2)); // Sleep for 2 seconds
        }
    }
}
