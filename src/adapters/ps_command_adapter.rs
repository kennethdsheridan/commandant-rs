//! PS Adapter
//!
//! This module provides an adapter for the `ps` command, a tool for monitoring
//! process statuses and CPU usage on Unix-based systems.

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use crate::ports::log_port::LoggerPort;
use crate::ports::ps_command_port::PsCommandPort;

/// Represents the `ps` command adapter.
/// This struct is used to execute the `ps` command and manage its output.
pub struct PsAdapter {
    logger: Arc<dyn LoggerPort>,
}

impl PsCommandPort for PsAdapter {
    /// Executes the `ps` command to gather CPU statistics and writes to a file.
    fn execute_ps_command(&self) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("ps aux | sort -nrk 3,3 | head -n 10")
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let error_message = format!(
                "Failed to execute `ps` command: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            self.logger.log_error(&error_message);
            return Err(error_message);
        }

        let output_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;

        Ok(output_str)
    }

    /// Writes the output of the `ps` command to a specified file.
    fn write_to_file(&self, output: String, file_path: &str) -> Result<(), String> {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
        {
            Ok(mut file) => {
                file.write_all(output.as_bytes())
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
            Err(e) => {
                let error_message = format!("Failed to open file: {}", e);
                self.logger.log_error(&error_message);
                Err(error_message)
            }
        }
    }
    /// Periodically executes the `ps` command to gather CPU statistics and writes to a file.
    ///
    /// # Arguments
    /// * `output_file_path` - The path to the file where the command output will be saved.
    fn collect_cpu_statistics(&self, output_file_path: &str) {
        loop {
            match self.execute_ps_command() {
                Ok(output) => {
                    if let Err(e) = self.write_to_file(output, output_file_path) {
                        self.logger.log_error(&e);
                        break;
                    }
                }
                Err(e) => {
                    self.logger.log_error(&e);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(2));
        }
    }
}
