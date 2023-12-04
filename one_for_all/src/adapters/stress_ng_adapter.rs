use std::path::Path;
use crate::ports::stress_test::StressTest;
use std::process::Command;
use crate::domain::logging::Logger;
use crate::domain::stress_ng::{STRESS_NG_LINUX, STRESS_NG_MACOS, write_binary_to_disk};

pub struct StressNgAdapter<'a> {
    logger: &'a dyn Logger,
}

impl<'a> StressNgAdapter<'a> {
    pub fn new(logger: &'a dyn Logger) -> Self {
        Self { logger }
    }
}


impl<'a> StressTest for StressNgAdapter<'a> {
    fn run_cpu_tests(&self) {
        // Log the decision of which stress-ng binary to use
        self.logger.log_debug("Deciding which stress-ng binary to use");

        // Determine the stress-ng binary path
        let stress_ng_arch = decide_stress_ng_arch();
        match stress_ng_arch {
            Ok(stress_str) => {
                self.logger.log_debug(&format!("Using stress-ng binary: {}", stress_str));

                // Write the binary to disk
                if let Err(e) = write_binary_to_disk(stress_str, "stress-ng") {
                    self.logger.log_error(&format!("Failed to write stress-ng binary to disk: {}", e));
                    return;
                }

                // Check if the binary exists
                if !Path::new(stress_str).exists() {
                    self.logger.log_error(&format!("stress-ng binary not found at: {:?}", stress_str));
                    return;
                }

                // Prepare and execute the CPU test command
                let cpu_test_command = Command::new(stress_str)
                    .arg("--cpu")
                    .arg("2") // Number of CPUs to stress. Consider parameterizing this.
                    .arg("--timeout")
                    .arg("30s") // Duration of the stress test. Consider parameterizing this.
                    .spawn();

                // Check if the command was successfully started
                let mut cpu_test_command = match cpu_test_command {
                    Ok(command) => command,
                    Err(e) => {
                        self.logger.log_error(&format!("Failed to start stress-ng process: {}", e));
                        return;
                    }
                };

                // Log the execution of the stress-ng command
                self.logger.log_debug("Executing stress-ng command");

                // Start the stress-ng process and handle any errors
                let output = match cpu_test_command.wait_with_output() {
                    Ok(output) => output,
                    Err(e) => {
                        self.logger.log_error(&format!("Failed to wait on stress-ng process: {}", e));
                        return;
                    }
                };

                // Log the completion of the stress test
                self.logger.log_debug("Finished stress-ng process");

                // Log the output of the stress test
                self.logger.log_debug(&format!("stress-ng output: {:?}", output));
            },
            Err(e) => {
                self.logger.log_error(&format!("Error converting binary data to UTF-8: {}", e));
                return;
            }
        }
    }
}

// Use the binaries
pub fn decide_stress_ng_arch() -> Result<&'static str, std::str::Utf8Error> {
    if cfg!(target_os = "linux") {
        std::str::from_utf8(STRESS_NG_LINUX)
    } else {
        std::str::from_utf8(STRESS_NG_MACOS)
    }
}
