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

impl ProcData {
    fn new(
        proc: Process,
        stat: Stat,
        cpu_usage: f64,
        cpu_fraction: f64,
        prev_cpu_time: u64,
        use_current_cpu_total: bool,
    ) -> (Self, u64) {
        let (command, name) = get_proc_cmd_and_name(&proc, &stat);
        let (cpu_usage_percent, new_process_time) = get_cpu_usage(
            &stat,
            cpu_usage,
            cpu_fraction,
            prev_cpu_time,
            use_current_cpu_total,
        );
        let data = ProcData {
            pid: proc.pid,
            parent_pid: stat.ppid,
            cpu_usage_percent,
            mem_usage_percent: 0.0,
            priority: 0,
            total_disk_read_bytes: 0,
            total_disk_write_bytes: 0,
            total_net_received_bytes: 0,
            total_net_sent_bytes: 0,
            name: name,
            command: command,
            state: (ProcessStatus::from(stat.state).to_string(), stat.state),
            uid: 0,
        };

        (data, new_process_time)
    }
}

pub fn read_process_data(
    prev_idle: &mut f64,
    prev_non_idle: &mut f64,
    cpu_times: &mut HashMap<Pid, u64>,
    use_current_cpu_total: bool,
) -> Result<Vec<ProcData>, RTopError> {
    let mut current_pids = HashSet::new();
    if let Ok((cpu_usage, cpu_percentage)) = cpu_usage_calculation(prev_idle, prev_non_idle) {
        let data = std::fs::read_dir("/proc")?
            .filter_map(|dir| {
                if let Ok(dir) = dir {
                    if let Ok(pid) = dir.file_name().to_string_lossy().trim().parse::<Pid>() {
                        if let Ok(proc) = Process::new(pid) {
                            if let Ok(stat) = proc.stat() {
                                let prev_proc_cpu_time = *cpu_times.get(&pid).unwrap_or(&0);
                                let (data, new_proc_cpu_time) = ProcData::new(
                                    proc,
                                    stat,
                                    cpu_usage,
                                    cpu_percentage,
                                    prev_proc_cpu_time,
                                    use_current_cpu_total,
                                );
                                cpu_times.insert(pid, new_proc_cpu_time);
                                current_pids.insert(pid);
                                return Some(data);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    }

                    return None;
                }

                None
            })
            .collect();

        let all_pids: HashSet<Pid> = cpu_times.keys().map(|k| *k).collect();
        all_pids.difference(&current_pids).for_each(|k| {
            cpu_times.remove(&k);
        });

        Ok(data)
    } else {
        Err(RTopError {
            err_msg: "Could not compute CPU usage.".to_string(),
        })
    }
}

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

    let active_time = if total_delta - idle_delta != 0_f64 {
        total_delta - idle_delta
    } else {
        1_f64
    };

    let cpu_percentage = if total_delta != 0_f64 {
        active_time / total_delta
    } else {
        0_f64
    };

    Ok((active_time, cpu_percentage))
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

fn get_cpu_usage(
    stat: &Stat,
    cpu_usage: f64,
    cpu_percentage: f64,
    prev_proc_time: u64,
    use_current_cpu_total: bool,
) -> (f64, u64) {
    let new_proc_time = stat.utime + stat.stime;
    let diff = (new_proc_time - prev_proc_time) as f64;

    if cpu_usage == 0.0 {
        (0.0, new_proc_time)
    } else if use_current_cpu_total {
        ((diff / cpu_usage) * 100_f64, new_proc_time)
    } else {
        ((diff / cpu_usage) * 100_f64 * cpu_percentage, new_proc_time)
    }
}
