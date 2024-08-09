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
    // update cpu information
    fn get_cpu_usage(&self) -> f32 {
        let mut system = System::new_all();
        system.refresh_cpu_all();
        system.global_cpu_usage()
    }
    
    // update memory information
    fn get_memory_usage(&self) -> (u64, u64) {
        let mut system = System::new_all();
        system.refresh_memory();
        (system.used_memory(), system.total_memory())
    }
    
    // update the disk information
    fn get_disk_usage(&self) -> (u64, u64) {
       let disks = Disks::new_with_refreshed_list();
       
       let total_space: u64 = disks.list().iter()
           .map(|disk| disk.total_space()).sum();
       
       let used_space: u64 = disks.list().iter()
           .map(|disk| disk.total_space() - disk.available_space()).sum();

       (used_space, total_space) // return the tuple
        
    }
}
