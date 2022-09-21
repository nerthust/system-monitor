use procfs::net::{dev_status, TcpNetEntry, UdpNetEntry};
use procfs::process::{FDTarget, Process};
use std::iter::Chain;
use std::vec::IntoIter;

pub fn get_tcp_port(
    proc: &Process,
    tcp_list: &mut Chain<IntoIter<TcpNetEntry>, IntoIter<TcpNetEntry>>,
) -> Vec<u16> {
    let mut tcp_ports: Vec<u16> = vec![];
    for entry in tcp_list {
        if let Ok(fds) = proc.fd() {
            for fd in fds {
                if let FDTarget::Socket(inode) = fd.unwrap().target {
                    if entry.inode == inode {
                        let local_addr = entry.local_address;
                        tcp_ports.push(local_addr.port());
                    }
                }
            }
        }
    }
    tcp_ports
}

pub fn get_udp_port(
    proc: &Process,
    tcp_list: &mut Chain<IntoIter<UdpNetEntry>, IntoIter<UdpNetEntry>>,
) -> Vec<u16> {
    let mut tcp_ports: Vec<u16> = vec![];
    for entry in tcp_list {
        if let Ok(fds) = proc.fd() {
            for fd in fds {
                if let FDTarget::Socket(inode) = fd.unwrap().target {
                    if entry.inode == inode {
                        let local_addr = entry.local_address;
                        tcp_ports.push(local_addr.port());
                    }
                }
            }
        }
    }
    tcp_ports
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
