use procfs::net;
use procfs::net::{dev_status, TcpNetEntry, UdpNetEntry};
use procfs::process::{FDTarget, Process};

pub enum NetEntry {
    UDPEntry(UdpNetEntry),
    TCPEntry(TcpNetEntry),
}

pub fn get_net_entries() -> Vec<NetEntry> {
    get_tcp_net_entries()
        .into_iter()
        .chain(get_udp_net_entries())
        .collect()
}

pub fn get_net_ports(proc: &Process, net_list: &Vec<NetEntry>) -> (Vec<u16>, Vec<u16>) {
    use NetEntry::{TCPEntry, UDPEntry};

    let mut tcp_ports = Vec::new();
    let mut udp_ports = Vec::new();
    if let Ok(fds) = proc.fd() {
        for fd in fds {
            if let Ok(FDTarget::Socket(inode)) = fd.map(|v| v.target) {
                for entry in net_list.iter() {
                    match entry {
                        TCPEntry(entry) => {
                            if entry.inode == inode {
                                let local_addr = entry.local_address;
                                tcp_ports.push(local_addr.port());
                            }
                        }
                        UDPEntry(entry) => {
                            if entry.inode == inode {
                                let local_addr = entry.local_address;
                                udp_ports.push(local_addr.port());
                            }
                        }
                    }
                }
            }
        }

        (tcp_ports, udp_ports)
    } else {
        (tcp_ports, udp_ports)
    }
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

// Helpers
fn get_tcp_net_entries() -> Vec<NetEntry> {
    use NetEntry::TCPEntry;

    let tcp = net::tcp();
    let tcp6 = net::tcp6();

    match (tcp, tcp6) {
        (Ok(tcp), Ok(tcp6)) => tcp.into_iter().chain(tcp6).map(|v| TCPEntry(v)).collect(),
        (Ok(tcp), Err(_)) => tcp.into_iter().map(|v| TCPEntry(v)).collect(),
        (Err(_), Ok(tcp6)) => tcp6.into_iter().map(|v| TCPEntry(v)).collect(),
        (Err(_), Err(_)) => Vec::new(),
    }
}

fn get_udp_net_entries() -> Vec<NetEntry> {
    use NetEntry::UDPEntry;

    let udp = net::udp();
    let udp6 = net::udp6();

    match (udp, udp6) {
        (Ok(udp), Ok(udp6)) => udp.into_iter().chain(udp6).map(|v| UDPEntry(v)).collect(),
        (Ok(udp), Err(_)) => udp.into_iter().map(|v| UDPEntry(v)).collect(),
        (Err(_), Ok(udp6)) => udp6.into_iter().map(|v| UDPEntry(v)).collect(),
        (Err(_), Err(_)) => Vec::new(),
    }
}
