use std::collections::HashMap;
use sysinfo::{self, System, SystemExt};

use crate::core::error::RTopError;
use crate::core::network::get_system_network_stats;
use crate::core::process::{self, Pid, ProcData};

pub struct SystemReader {
    prev_idle: f64,
    prev_non_idle: f64,
    cpu_times: HashMap<Pid, u64>,
    use_current_cpu_total: bool,
    pub total_memory_bytes: u64,
}

pub struct SystemData {
    pub processes: Vec<ProcData>,
    pub net_received_bytes: u64,
    pub net_sent_bytes: u64,
}

impl SystemReader {
    pub fn new(use_current_cpu_total: bool) -> Self {
        let mut system = System::new_with_specifics(sysinfo::RefreshKind::new());
        system.refresh_memory();

        SystemReader {
            prev_idle: 0.0,
            prev_non_idle: 0.0,
            cpu_times: HashMap::new(),
            use_current_cpu_total,
            total_memory_bytes: system.total_memory(),
        }
    }

    pub fn read_process_data(&mut self) -> Result<SystemData, RTopError> {
        let processes = process::read_process_data(
            &mut self.prev_idle,
            &mut self.prev_non_idle,
            &mut self.cpu_times,
            self.use_current_cpu_total,
            self.total_memory_bytes,
        )?;

        let (net_received_bytes, net_sent_bytes) = get_system_network_stats();

        Ok(SystemData {
            processes,
            net_received_bytes,
            net_sent_bytes,
        })
    }
}
