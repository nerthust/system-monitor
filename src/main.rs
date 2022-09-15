use std::{thread, time};

use rtop::core::system_reader::SystemReader;

fn main() {
    let mut sys_data = SystemReader::new(false);
    let delay = time::Duration::from_millis(1000);

    loop {
        let data = sys_data.read_process_data().unwrap();
        data.iter().for_each(|proc| {
            if proc.command.contains("teams") {
                println!("({:?}, {:?}, {:?})", proc.name, proc.total_disk_read_bytes, proc.total_disk_write_bytes);
            }
        });

        thread::sleep(delay);
        println!("--");
    }
}
