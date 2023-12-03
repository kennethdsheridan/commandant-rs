use crate::domain::stress_test::StressTest;
use std::process::Command;
use crate::domain::logging::Logger;

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
        let stress: () = decide_stress_ng_arch();

        // Define the stress-ng command for CPU tests
        let cpu_test_command = Command::new(stress)
            .arg("--cpu")
            .arg("2") // Number of CPUs to stress. Consider parameterizing this.
            .arg("--timeout")
            .arg("30s") // Duration of the stress test. Consider parameterizing this.
            .spawn()
            .expect("Failed to start stress-ng process");

        // Log the start of the stress test
        self.logger.log_debug("Starting stress-ng process");

        // Handling the process output. In a real-world scenario, you might want
        // to parse this output and use it to monitor the results of the test.
        let output = cpu_test_command
            .wait_with_output()
            .expect("Failed to wait on stress-ng process");

        // Log the end of the stress test
        self.logger.log_debug("Finished stress-ng process");

        // Output the results of the stress test. Consider logging this instead
        // of merely printing it to stdout.
        self.logger.log_debug(&format!("stress-ng output: {:?}", output));
    }
}

// Use the binaries
pub fn decide_stress_ng_arch() -> &'static str {
    if cfg!(target_os = "linux") {
        "STRESS_NG_LINUX"
    } else if cfg!(target_os = "macos") {
        "STRESS_NG_MACOS"
    } else {
        panic!("Unsupported OS")
    }
}