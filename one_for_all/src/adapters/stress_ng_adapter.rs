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
        self.logger.log_debug("Deciding which stress-ng binary to use");

        let stress_ng_arch = decide_stress_ng_arch();
        let stress_str = match stress_ng_arch {
            Ok(path) => path,
            Err(e) => {
                self.logger.log_error(&format!("Error converting binary data to UTF-8: {:?}", e));
                return;
            }
        };

        if let Err(e) = write_binary_to_disk(stress_str, "stress-ng") {
            self.logger.log_error(&format!("Failed to write stress-ng binary to disk: {:?}", e));
            return;
        }

        if !Path::new(stress_str).exists() {
            self.logger.log_error(&format!("stress-ng binary not found at: {:?}", stress_str));
            return;
        }

        let cpu_test_command = Command::new(stress_str)
            .arg("--cpu")
            .arg("2")
            .arg("--timeout")
            .arg("30s")
            .spawn();

        let mut cpu_test_command = match cpu_test_command {
            Ok(command) => command,
            Err(e) => {
                self.logger.log_error(&format!("Failed to start stress-ng process: {:?}", e));
                return;
            }
        };

        self.logger.log_debug("Executing stress-ng command");

        let output = match cpu_test_command.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                self.logger.log_error(&format!("Failed to wait on stress-ng process: {:?}", e));
                return;
            }
        };

        self.logger.log_debug("Finished stress-ng process");
        self.logger.log_debug(&format!("stress-ng output: {:?}", output));
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
