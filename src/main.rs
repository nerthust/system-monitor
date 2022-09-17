use std::{thread, time};
use procfs::net::dev_status;

use rtop::core::system_reader::{SystemReader, calculate_general_bytes_network};

fn main() {
    let mut sys_data = SystemReader::new(false);
    let delay = time::Duration::from_millis(1000);

    loop {
        let data = sys_data.read_process_data().unwrap();
        data.iter().for_each(|proc| {
            if proc.name.contains("firefox") {
                //println!("({:?}, {:?})", proc.name, proc.mem_usage_percent);
            }
        });

        let dev_status = dev_status().unwrap();                                 
        let recv_bytes = calculate_general_bytes_network(true, &dev_status);    
        let sent_bytes = calculate_general_bytes_network(false, &dev_status);   

        println!("recv_bytes: {} bits -- sent_bytes: {} bits", recv_bytes, sent_bytes);

        thread::sleep(delay);
        println!("--");
    }
}
