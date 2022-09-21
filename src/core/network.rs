use procfs::process::{FDTarget, Process};
use procfs::net::{dev_status, TcpNetEntry, UdpNetEntry};
use std::iter::Chain;
use std::vec::IntoIter;

use crate::Casting;

#[derive(Debug, Clone)]
struct NetInfo {
    local_addr: String,
    remote_addr: String,
    state: String,
}

fn get_tcp_info(
    proc: &Process,
    tcp_list: &mut Chain<IntoIter<TcpNetEntry>, IntoIter<TcpNetEntry>>,
    ) -> Vec<NetInfo> {

    let mut tcp_table: Vec<NetInfo> = vec![];

    for entry in tcp_list {
        if let Ok(fds) = proc.fd() {
            for fd in fds {
                if let FDTarget::Socket(inode) = fd.unwrap().target {
                    if entry.inode == inode {
                        let local_addr = entry.local_address.to_string();
                        let remote_addr = entry.remote_address.to_string();
                        let state = entry.state.clone();

                        tcp_table.push(
                                NetInfo {
                                    local_addr,
                                    remote_addr,
                                    state: state.to_string(),
                                }
                            );
                        }
                    }
                }
        }
    }

    return tcp_table;
}

fn get_udp_info(
    proc: &Process,
    udp_list: &mut Chain<IntoIter<UdpNetEntry>, IntoIter<UdpNetEntry>>,
    ) -> Vec<NetInfo> {

    let mut udp_table: Vec<NetInfo> = vec![];

    for entry in udp_list {
        if let Ok(fds) = proc.fd() {
            for fd in fds {
                if let FDTarget::Socket(inode) = fd.unwrap().target {
                    if entry.inode == inode {
                        let local_addr = entry.local_address.to_string();
                        let remote_addr = entry.remote_address.to_string();
                        let state = entry.state.clone();

                        udp_table.push(
                            NetInfo {
                                    local_addr,
                                    remote_addr,
                                    state: state.to_string(),
                                }
                            );
                        }
                    }
                }
        }
    }

    return udp_table;
}

pub fn get_system_network_stats() -> (u64, u64) {
    let mut recv_bytes = 0;
    let mut sent_bytes = 0;
    match dev_status() {
        Ok(devices) => {
            for dev in devices {
                let status = dev.1;
                recv_bytes += status.recv_bytes;
                sent_bytes += status.sent_bytes;
            }
            (recv_bytes, sent_bytes)
        }
        Err(_) => (0, 0),
    }
}
