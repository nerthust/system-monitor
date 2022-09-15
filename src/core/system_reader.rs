use std::collections::HashMap;

use crate::core::error::RTopError;
use crate::core::process::{self, Pid, ProcData};

pub struct SystemReader {
    prev_idle: f64,
    prev_non_idle: f64,
    cpu_times: HashMap<Pid, u64>,
    use_current_cpu_total: bool,
}

impl SystemReader {
    pub fn new(use_current_cpu_total: bool) -> Self {
        SystemReader {
            prev_idle: 0.0,
            prev_non_idle: 0.0,
            cpu_times: HashMap::new(),
            use_current_cpu_total,
        }
    }

    pub fn read_process_data(&mut self) -> Result<Vec<ProcData>, RTopError> {
        process::read_process_data(
            &mut self.prev_idle,
            &mut self.prev_non_idle,
            &mut self.cpu_times,
            self.use_current_cpu_total,
        )
    }
}
