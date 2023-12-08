use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use crate::ports::stress_test::StressTest;
use std::process::Command;
use crate::adapters::stress_ng_manager_adapter::{STRESS_NG_LINUX, STRESS_NG_MACOS};
use crate::domain::logging::Logger;
use crate::StressNgArch;


/// `StressNgAdapter` is a struct that provides functionality for performing stress tests
/// using the `stress-ng` utility. It is designed to work with any implementation of the `Logger` trait,
/// allowing for flexible logging.
///
/// The struct holds a reference to a logger, allowing it to log messages at various points
/// during the execution of stress tests.
///
/// Lifetime `'a` is used to denote that `StressNgAdapter` holds a reference to a logger
/// for the duration of its lifetime.
pub struct StressNgAdapter<'a> {
    // Logger reference: This allows `StressNgAdapter` to log messages.
    // The logger must implement the `Logger` trait.
    // The use of a dynamic reference (`dyn Logger`) allows for flexibility in
    // the specific logging implementation used.
    logger: &'a dyn Logger,
}

/// Implementation of `StressNgAdapter`.
impl<'a> StressNgAdapter<'a> {
    /// Creates a new instance of `StressNgAdapter`.
    ///
    /// This constructor function takes a reference to an object that implements the `Logger` trait.
    /// This allows `StressNgAdapter` to perform logging during its operations.
    ///
    /// # Arguments
    ///
    /// * `logger` - A reference to an object that implements the `Logger` trait.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance of `StressNgAdapter`.
    ///
    /// # Examples
    ///
    /// ```
    /// let logger = MyLogger::new();
    /// let adapter = StressNgAdapter::new(&logger);
    /// ```
    pub fn new(logger: &'a dyn Logger) -> Self {
        Self {
            logger,
        }
    }
}



/// Implementation of `StressTest` trait for `StressNgAdapter`.
/// This struct is responsible for running stress tests using the `stress-ng` utility.
impl<'a> StressTest for StressNgAdapter<'a> {
    /// Runs CPU stress tests.
    /// This method decides which `stress-ng` binary to use based on the operating system,
    /// writes the binary to a temporary file, sets the necessary permissions, and then executes the binary
    /// with specific arguments to stress test the CPU.
    fn run_cpu_tests(&self) {
        // Log the start of the decision process for choosing the appropriate binary
        self.logger.log_debug("Deciding which stress-ng binary to use");

        // Choose the appropriate binary data based on the target OS
        let binary_data = if cfg!(target_os = "linux") {
            STRESS_NG_LINUX
        } else {
            STRESS_NG_MACOS
        };

        // Define a path for the temporary file where the binary will be written
        let temp_file_path = format!("/tmp/{}", "stress-ng");

        // Attempt to create the file and write the binary data to it
        // If there is an error during this process, log the error and return early
        if let Err(e) = File::create(&temp_file_path)
            .and_then(|mut file| file.write_all(binary_data)) {
            self.logger.log_error(&format!("Failed to write stress-ng binary to disk: {:?}", e));
            return;
        }

        // Set the file permissions to make the binary executable
        // Permissions are set to 755 (read, write, and execute for owner; read and execute for others)
        let mut perms = fs::metadata(&temp_file_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_file_path, perms).unwrap();

        // Prepare the command to execute the stress test
        // Here, the stress test is configured to stress two CPUs for 30 seconds
        // These parameters could be made configurable
        let cpu_test_command = Command::new(temp_file_path)
            .arg("--cpu")
            .arg("2") // Number of CPUs to stress
            .arg("--timeout")
            .arg("30s") // Duration of the stress test
            .spawn();

        // Check if the command was successfully started
        match cpu_test_command {
            Ok(command) => {
                // Log the successful execution of the command
                self.logger.log_debug("Executing stress-ng command");

                // Wait for the command to complete and log the output or any errors encountered
                match command.wait_with_output() {
                    Ok(output) => {
                        // Log the successful completion and the output of the command
                        self.logger.log_debug("Finished stress-ng process");
                        self.logger.log_debug(&format!("stress-ng output: {:?}", output));
                    },
                    Err(e) => {
                        // Log any errors encountered while waiting for the command to complete
                        self.logger.log_error(&format!("Failed to wait on stress-ng process: {:?}", e));
                    }
                }
            },
            Err(e) => {
                // Log any errors encountered while starting the command
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
