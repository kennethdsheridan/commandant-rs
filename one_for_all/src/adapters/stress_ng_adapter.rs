use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use crate::ports::stress_test::StressTest;
use std::process::Command;
use crate::adapters::stress_ng_manager_adapter::{STRESS_NG_LINUX, STRESS_NG_MACOS};
use crate::domain::logging::Logger;
use crate::StressNgArch;


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

        let binary_data = if cfg!(target_os = "linux") {
            STRESS_NG_LINUX
        } else {
            STRESS_NG_MACOS
        };

        // Generate a unique temporary file name
        let temp_file_path = format!("/tmp/stress-ng-{}", uuid::Uuid::new_v4());

        // Write the binary data to a file
        if let Err(e) = File::create(&temp_file_path)
            .and_then(|mut file| file.write_all(binary_data)) {
            self.logger.log_error(&format!("Failed to write stress-ng binary to disk: {:?}", e));
            return;
        }

        // Set executable permissions
        let mut perms = fs::metadata(&temp_file_path).unwrap().permissions();
        perms.set_mode(0o755); // Read, write, and execute for owner; read and execute for others
        fs::set_permissions(&temp_file_path, perms).unwrap();


        // Prepare and execute the CPU test command
        let cpu_test_command = Command::new(temp_file_path)
            .arg("--cpu")
            .arg("2") // Number of CPUs to stress, consider parameterizing this
            .arg("--timeout")
            .arg("30s") // Duration of the stress test, consider parameterizing this
            .spawn();

        // Check if the command was successfully started
        match cpu_test_command {
            Ok(mut command) => {
                self.logger.log_debug("Executing stress-ng command");

                // Start the stress-ng process and handle any errors
                match command.wait_with_output() {
                    Ok(output) => {
                        self.logger.log_debug("Finished stress-ng process");
                        self.logger.log_debug(&format!("stress-ng output: {:?}", output));
                    },
                    Err(e) => {
                        self.logger.log_error(&format!("Failed to wait on stress-ng process: {:?}", e));
                    }
                }
            },
            Err(e) => {
                self.logger.log_error(&format!("Failed to start stress-ng process: {:?}", e));
            }
        }
    }
}


// Use the binaries
// Use the binaries
pub fn decide_stress_ng_arch() -> StressNgArch {
    if cfg!(target_os = "linux") {
        StressNgArch::Linux
    } else {
        StressNgArch::MacOS
    }
}
