use procfs::net::dev_status;

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
