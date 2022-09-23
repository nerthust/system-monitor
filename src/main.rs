use std::{thread, time};

use rtop::core::system_reader::SystemReader;

fn main() {
    let mut sys_data = SystemReader::new(false);
    let delay = time::Duration::from_millis(1000);

    loop {
        let data = sys_data.read_process_data().unwrap();
        //println!("Network received bytes = {:?}", data.net_received_bytes);
        //println!("Network sent bytes {:?}", data.net_sent_bytes);

        data.processes.iter().for_each(|proc| {
            if proc.name.contains("firefox") {
                for tcp in &proc.tcp_ports {
                    println!("tcp: {}", tcp);
                }

                for udp in &proc.udp_ports {
                    println!("udp: {}", udp);
                }
            }
        });

        thread::sleep(delay);
        println!("--");
    }
}
