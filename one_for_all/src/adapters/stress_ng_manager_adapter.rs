// stress_ng_manager.rs

use crate::domain::logging::Logger;
use std::fs::File;
use std::io::{self, Write};

pub const STRESS_NG_LINUX: &'static [u8] = include_bytes!("linux/stress-ng");
pub const STRESS_NG_MACOS: &'static [u8] = include_bytes!("macOS/stress-ng");

pub enum StressNgArch {
    Linux,
    MacOS,
}

pub fn write_stress_ng_to_disk(
    stress_ng_arch: StressNgArch,
    logr: &dyn Logger,
) -> Result<(), std::io::Error> {
    let (binary_data, filename) = match stress_ng_arch {
        StressNgArch::Linux => (STRESS_NG_LINUX, "stress-ng-linux"),
        StressNgArch::MacOS => (STRESS_NG_MACOS, "stress-ng-macos"),
    };

    match write_binary_to_disk(binary_data, filename) {
        Ok(_) => {
            logr.log_debug(&format!(
                "Successfully wrote stress-ng binary to disk: {}",
                filename
            ));
            Ok(())
        }
        Err(e) => {
            logr.log_error(&format!(
                "Failed to write stress-ng binary to disk: {:?}",
                e
            ));
            Err(e)
        }
    }
}

pub fn write_binary_to_disk(data: &[u8], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    // Write the binary data to the file
    file.write_all(data)?;

    Ok(())
}
