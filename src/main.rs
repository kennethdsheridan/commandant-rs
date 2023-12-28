mod adapters;
mod ports;

use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use adapters::log_adapter::{init, FernLogger};

use crate::adapters::stress_ng_adapter::StressNgAdapter;
use crate::adapters::web_server_adapter::WebServerAdapter;
use crate::ports::log_port::LoggerPort;
use crate::ports::web_server_port::WebServerPort;

// Enumeration representing the supported architectures for the `stress-ng`
// binary.
// This enum is used to select the correct binary for the running operating
// system.
#[derive(Debug)]
enum StressNgArch {
    Linux,
    MacOS,
}

// OneForAll CLI Application
// This struct represents the command-line interface of the application,
// defining the available subcommands and their respective functionalities.
#[derive(Parser, Debug)]
#[clap(author = "Kenny Sheridan", version = "0.1 (Dev)", about = "OneForAll -\
 An advanced tool for hardware performance testing and diagnostics.",
long_about = long_description())]
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
    "\n\n\nOneForAll is a comprehensive tool designed for in-depth hardware \
    performance analysis and diagnostics. \
    It leverages advanced testing methodologies to provide users with \
    detailed insights into their system's capabilities \
    and bottlenecks. With OneForAll, you can run various tests, including \
    benchmarks, stress tests, and hardware discovery, \
    to understand the full scope of your hardware's performance.\n\n\
    The tool is structured into several modules, each targeting a specific \
    aspect of hardware performance:\n\n\
    \
    - Benchmark: Run extensive benchmarks to measure the speed and efficiency\
     of your CPU, GPU, memory, and storage devices.\n\
    
    - Stress: Put your system under intense stress to test stability and \
    endurance under heavy loads.\n\
    \
    - Discover: Analyze and report on the configuration and current state of \
    your hardware components.\n\
    \
    - Overwatch: Watch your system's performance in real-time, capturing \
    critical metrics and providing live feedback.\n
   
    OneForAll is designed with both simplicity and power in mind, making it \
    suitable for both casual users looking to \
    check their system's performance and professionals requiring detailed \
    hardware analysis."
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logging system.
    let logger: Arc<FernLogger> = Arc::new(init("logs", log::LevelFilter::Trace));
    let logger_as_port: Arc<dyn LoggerPort> = logger.clone();

    // Start an asynchronous web server on port 8000
    let web_server = WebServerAdapter::new(logger.clone());
    let server = web_server.start_server();

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
    server = server => {
        match server {
          Ok(_) => logger.log_info("Server started successfully."),
          Err(e) => logger.log_error(&format!("Failed to start the web server: {:?}", e)),
        }
    }
    _ = ctrl_c => {
        logger.log_info("Received Ctrl+C, shutting down.");
          }
      }

    // Parse the command-line arguments into the Cli struct using clap.
    let cli = Cli::parse();

    // Create an instance of the StressNgAdapter.
    // This adapter is responsible for executing the stress tests using
    // `stress-ng`.
    let stress_tester = StressNgAdapter::new(logger_as_port);

    // Handle the parsed subcommands and execute the corresponding
    // functionality.
    match cli.command {
        Commands::Benchmark => {
            // Implement benchmark functionality.
            logger.log_info("Benchmarking functionality not yet implemented.");
        }
        Commands::Stress => {
            // Define the arguments for the stress-ng command
            let args = ["--cpu", "2", "--timeout", "60s"];

            // Number of retries in case stress command fails
            let mut retries = 2;

            // Execute the stress test using the stress_tester instance
            while retries >= 0 {
                // log the attempt
                logger.log_info(&format!(
                    "Executing CPU stress test. Attempts remaining: {}",
                    retries,
                ));

                match StressNgAdapter::execute_stress_ng_command(logger.clone(), &args).await {
                    Ok(()) => {
                        logger.log_info("CPU stress test executed successfully.");
                        break; // Exit the loop on successful execution
                    }
                    Err(e) => {
                        if retries > 0 {
                            logger.log_warn(&format!(
                                "Retrying CPU stress test.\
                             Attempts remaining: {}",
                                retries
                            ));
                            sleep(Duration::from_secs(10)).await; // Add an awaitable delay here
                        } else {
                            logger.log_error(&format!(
                                "Error executing CPU \
                            stress test: {}",
                                e
                            ));
                        }
                    }
                }
                retries -= 1; // Decrement the retry counter
            }
        }

        Commands::Discover => {
            // Implement discovery functionality.
            logger.log_info("Discovery functionality not yet implemented.");
        }
        Commands::Overwatch => {
            // Implement system overwatch functionality.
            logger.log_info("System overwatch functionality not yet implemented.");
        }
    } // End of match cli.command

    Ok(()) // Return a successful result
} // End of main()
