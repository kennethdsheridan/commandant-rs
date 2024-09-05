//! PS Adapter
//!
//! This module provides an adapter for the `ps` command, a tool for monitoring
//! process statuses and CPU usage on Unix-based systems. It executes the command
//! processes the output through WebAssembly for the browser, then stores it within
//! SurrealDB using the RockDB engine.

use async_trait::async_trait;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use wasm_bindgen::prelude::*;

use common::adapters::ps_wasm_adapter;
use common::ports::log_port::LoggerPort;

use crate::ports::database_port::DatabasePort;
use crate::ports::ps_command_port::PsCommandPort;

/// WebAssembly binding for browsers console.log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// The ProcessData struct represents a single process and its CPU usage percentage.
/// This struct is used to parse the output of the `ps` command and extract the CPU usage
/// percentage for each process in the list. The `ps` command outputs a list of processes
/// sorted by CPU usage, and this struct is used to parse each line of the output.
struct ProcessData {
    user: String,
    /// The user who started the process
    cpu_usage: f32,
    /// The percentage of CPU used by the process
    memory_usage: f32,
    /// The percentage of memory used by the process
    command: String, // The command used to start the process
}

/// Represents the linux `ps` command adapter.
/// This struct is used to execute the `ps` command and manage its output.
#[derive(Clone)]
pub struct PsAdapter {
    db: Arc<dyn DatabasePort>,
    logger: Arc<dyn LoggerPort>, // inject the database port
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
        PsAdapter { db, logger } // Return a new instance of `PsAdapter` with the specified logger
    }
}

// Implement the `PsCommandPort` trait for `PsAdapter`. This allows the adapter to be used
// as a port in the application, and it also provides a concrete implementation of the
// `PsCommandPort` interface.
#[async_trait]
impl PsCommandPort for PsAdapter {
    fn execute_ps_command(&self) -> Result<String, String> {
        // Execute the `ps` command
        let output = Command::new("sh")
            .arg("-c")
            .arg("ps aux | sort -nrk 3,3 | head -n 10")
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        // Convert the output to a string
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        // Call the WASM serialization function from ps_wasm_adapter.rs
        let wasm_output = ps_wasm_adapter::write_to_wasm(output_str)
            .map_err(|e| format!("Failed to serialize for WASM: {:?}", e))?;

        // Convert the JsValue to string
        match wasm_output.as_string() {
            Some(s) => Ok(s),
            None => Err("Failed to convert WASM output to string".to_string()),
        }
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
    async fn collect_cpu_statistics(&self, key: &str) {
        loop {
            // Loop forever
            match self.execute_ps_command() {
                Ok(output) => {
                    // write to the database if the command was successful and the output is not
                    // empty (i.e., the command returned no results)
                    if let Err(e) = self.write_to_db(output, key.as_bytes(), "database").await {
                        self.logger.log_error(&e);
                        break; // Break out of the loop if an error occurs
                    }
                }
                Err(e) => {
                    self.logger.log_error(&e);
                    break; // Break out of the loop if an error occurs
                }
            }
            thread::sleep(Duration::from_secs(2));
        }
    }

    /// Writes the output of the `ps` command to a database.
    ///
    /// # Arguments
    /// * `output` - The output string from the `ps` command.
    /// * `key` - The key under which the command output will be saved in the database.
    /// * `db_identifier` - A string that identifies the database location.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the write operation.
    /// This method writes the output of the `ps` command to a database.
    /// This can be useful for logging, analysis, or real-time monitoring purposes.
    async fn write_to_db(
        &self,
        output: String,
        key: &[u8],
        db_identifier: &str,
    ) -> Result<(), String> {
        // Convert the output to bytes
        let output_bytes = output;

        // Write the output to the database
        match self.db.insert(key, &output_bytes).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_message =
                    format!("Failed to write to database at {}: {}", db_identifier, e);
                self.logger.log_error(&error_message);
                Err(error_message)
            }
        }
    }
}
