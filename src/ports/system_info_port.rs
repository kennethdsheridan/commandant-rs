// system_info_port.rs

pub trait SystemInfoPort {
    fn get_cpu_usage(&self) -> f32;
    fn get_memory_usage(&self) -> (u64, u64); // used, total
    fn get_disk_usage(&self) -> (u64, u64); // (used, total)
}
