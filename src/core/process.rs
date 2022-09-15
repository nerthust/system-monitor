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
