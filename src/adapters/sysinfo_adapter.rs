// sysinfo_adapter.rs

use crate::ports::SysInfoPort;
use sysinfo::System;
use sysinfo::Disks;

pub struct SysInfoAdapter;

impl SysInfoAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl SysInfoPort for SysInfoAdapter {
    fn get_cpu_usage(&self) -> f32 {
        let mut system = System::new_all();
        system.refresh_cpu_all();
        system.global_cpu_usage()
    }

    fn get_memory_usage(&self) -> (u64, u64) {
        let mut system = System::new_all();
        system.refresh_memory();
        (system.used_memory(), system.total_memory())
    }
    
    // update the disk information
    fn get_disk_usage(&self) -> u64 {
            todo!();
    }
}
