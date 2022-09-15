use procfs::process::{Process, Stat};
use std::collections::HashMap;
use std::collections::HashSet;
use sysinfo::ProcessStatus;

use crate::core::error::RTopError;

pub type Pid = libc::pid_t;
pub type Pri = libc::priority_t;
pub type Uid = libc::uid_t;

#[derive(Debug, Clone, Default)]
pub struct ProcData {
    // Pid of the process.
    pub pid: Pid,

    // Parent PID of the process.
    pub parent_pid: Pid,

    // CPU usage as a percentage.
    pub cpu_usage_percent: f64,

    // Memory usage as a percentage.
    pub mem_usage_percent: f64,

    // Total number of bytes read by the process on disk.
    pub total_disk_read_bytes: u64,

    // Total number of bytes written by the process on disk.
    pub total_disk_write_bytes: u64,

    // Total number of bytes received by the process over the network.
    pub total_net_received_bytes: u64,

    // Total number of bytes sent by the process over the network.
    pub total_net_sent_bytes: u64,

    // Name of the process.
    pub name: String,

    // Exact command for the process.
    pub command: String,

    // Current state of the process (e.g. zombie, asleep)
    pub state: (String, char),

    // Process' user ID.
    pub uid: Uid,

    // Process' priority
    pub priority: Pri,
}


// Helpers
const MAX_STAT_NAME_LEN: usize = 15;

fn get_proc_cmd_and_name(proc: &Process, stat: &Stat) -> (String, String) {
    let (command, name) = {
        let truncated_name = stat.comm.as_str();
        if let Ok(cmdline) = proc.cmdline() {
            if cmdline.is_empty() {
                return (format!("[{}]", truncated_name), truncated_name.to_string());
            } else {
                let name = if truncated_name.len() >= MAX_STAT_NAME_LEN {
                    if let Some(first_part) = cmdline.first() {
                        first_part
                            .rsplit_once('/')
                            .map(|(_prefix, suffix)| suffix)
                            .unwrap_or(truncated_name)
                            .to_string()
                    } else {
                        truncated_name.to_string()
                    }
                } else {
                    truncated_name.to_string()
                };

                return (cmdline.join(" "), name);
            }
        } else {
            (truncated_name.to_string(), truncated_name.to_string())
        }
    };

    return (command, name);
}

fn cpu_usage_calculation(
    prev_idle: &mut f64,
    prev_non_idle: &mut f64,
) -> Result<(f64, f64), RTopError> {
    use std::io::prelude::*;
    use std::io::BufReader;

    let mut reader = BufReader::new(std::fs::File::open("/proc/stat")?);
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let (idle, non_idle) = compute_idle_and_non_idle_values(first_line);

    let total = idle + non_idle;
    let prev_total = *prev_idle + *prev_non_idle;

    let total_delta: f64 = total - prev_total;
    let idle_delta: f64 = idle - *prev_idle;

    *prev_idle = idle;
    *prev_non_idle = non_idle;

    let result = if total_delta - idle_delta != 0_f64 {
        total_delta - idle_delta
    } else {
        1_f64
    };

    let cpu_percentage = if total_delta != 0_f64 {
        result / total_delta
    } else {
        0_f64
    };

    Ok((result, cpu_percentage))
}

fn compute_idle_and_non_idle_values(line: String) -> (f64, f64) {
    fn str_to_f64(val: Option<&str>) -> f64 {
        val.and_then(|v| v.trim().parse::<f64>().ok())
            .unwrap_or(0_f64)
    }

    let mut val = line.split_whitespace();
    let prefix = val.next().map(|s| s.trim());
    assert!(prefix == Some("cpu"));
    let userm = str_to_f64(val.next());
    let nice: f64 = str_to_f64(val.next());
    let kernelm: f64 = str_to_f64(val.next());
    let idle: f64 = str_to_f64(val.next());
    let iowait: f64 = str_to_f64(val.next());
    let irq: f64 = str_to_f64(val.next());
    let softirq: f64 = str_to_f64(val.next());
    let steal: f64 = str_to_f64(val.next());

    let idle = idle + iowait;
    let non_idle = userm + nice + kernelm + irq + softirq + steal;

    (idle, non_idle)
}
