use clap::{Parser, Subcommand};

use adapters::log_adapter::{init, FernLogger};
use domain::logging::LoggerPort;

use crate::adapters::stress_ng_adapter::{decide_stress_ng_arch, StressNgAdapter};
use crate::adapters::stress_ng_manager_adapter::{
    write_binary_to_disk, STRESS_NG_LINUX, STRESS_NG_MACOS,
};
use crate::ports::stress_test::StressTest;

// Importing necessary modules and traits.
mod adapters;
mod domain;
mod ports;

// Enumeration representing the supported architectures for the `stress-ng` binary.
// This enum is used to select the correct binary for the running operating system.
enum StressNgArch {
    Linux,
    MacOS,
}

// OneForAll CLI Application
// This struct represents the command-line interface of the application,
// defining the available subcommands and their respective functionalities.
// Main application description.
#[derive(Parser, Debug)]
#[clap(author = "Kenny Sheridan", version = "0.1 (Dev)", about = "OneForAll - An advanced tool for hardware performance testing and diagnostics.", long_about = long_description())]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

// Enum representing the different subcommands available in the CLI.
// Each variant corresponds to a specific functionality of the application.
#[derive(Subcommand, Debug)]
enum Commands {
    // Runs benchmark tests
    Benchmark,

    // Executes stress tests
    Stress,

    // Scans and analyzes hardware
    Discover,

    // Monitors hardware performance in real-time
    Overwatch,
}

fn long_description() -> &'static str {
    "\n\n\nOneForAll is a comprehensive tool designed for in-depth hardware performance analysis and diagnostics. \
    It leverages advanced testing methodologies to provide users with detailed insights into their system's capabilities \
    and bottlenecks. With OneForAll, you can run various tests, including benchmarks, stress tests, and hardware scans, \
    to understand the full scope of your hardware's performance.\n\n\
    The tool is structured into several modules, each targeting a specific aspect of hardware performance:\n\
    - Benchmark: Run extensive benchmarks to measure the speed and efficiency of your CPU, GPU, memory, and storage devices.\n\
    - Stress: Put your system under intense stress to test stability and endurance under heavy loads.\n\
    - Scan: Analyze and report on the configuration and current state of your hardware components.\n\
    - Overwatch: Monitor your system's performance in real-time, capturing critical metrics and providing live feedback.\n\n\
    OneForAll is designed with both simplicity and power in mind, making it suitable for both casual users looking to \
    check their system's performance and professionals requiring detailed hardware analysis."
}

fn main() {
    // Initialize the logger for the application.
    // The logger is set up to write to the "logs" directory with a level filter of Trace,
    // which means all log messages at Trace level or higher will be recorded.
    init("logs", log::LevelFilter::Trace);
    let logr = FernLogger;

    // Parse the command-line arguments into the Cli struct using clap.
    let cli = Cli::parse();

    // Decide which `stress-ng` binary to use based on the operating system.
    // This decision is made using the decide_stress_ng_arch function,
    // which returns an enum variant indicating the suitable binary.
    let stress_ng_arch = decide_stress_ng_arch();
    let (binary_data, filename) = match stress_ng_arch {
        StressNgArch::Linux => (STRESS_NG_LINUX, "stress-ng-linux"),
        StressNgArch::MacOS => (STRESS_NG_MACOS, "stress-ng-macos"),
    };

    // Write the binary data to disk and log the outcome.
    match write_binary_to_disk(binary_data, filename) {
        Ok(_) => logr.log_debug(&format!(
            "Successfully wrote stress-ng binary to disk: {}",
            filename
        )),
        Err(e) => {
            logr.log_error(&format!(
                "Failed to write stress-ng binary to disk: {:?}",
                e
            ));
            return;
        }
    }

    // Create an instance of the StressNgAdapter.
    // This adapter is responsible for executing the stress tests using `stress-ng`.
    let stress_tester = StressNgAdapter::new(&logr);

    // Handle the parsed subcommands and execute the corresponding functionality.
    match cli.command {
        Commands::Benchmark => {
            // Implement benchmark functionality.
            logr.log_info("Benchmarking functionality not yet implemented.");
        }
        Commands::Stress => {
            // Execute the CPU stress tests using the StressNgAdapter.
            stress_tester.run_cpu_tests();
        }
        Commands::Discover => {
            // Implement discovery functionality.
            logr.log_info("System discovery functionality not yet implemented.");
        }
        Commands::Overwatch => {
            // Implement system overwatch functionality.
            logr.log_info("System overwatch functionality not yet implemented.");
        }
    }
}
