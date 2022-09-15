use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};

use rtop::core::system_reader::SystemReader;
use rtop::ui::app::App;
use rtop::ui::widget::start_ui;

fn main() {
    let mut sys_data = SystemReader::new(false);
    let delay = time::Duration::from_millis(1000);
    let proc = thread::spawn(move || {
        loop {
            let data = sys_data.read_process_data().unwrap();
            data.iter().for_each(|proc| {
                if proc.name.contains("spotify") {
                    //println!("({:?}, {:?})", proc.name, proc.mem_usage_percent);
                }
            });
            thread::sleep(delay);
            //println!("--");
        }
    });

    let ui = thread::spawn(move || {
        let app = Rc::new(RefCell::new(App::new()));
        let ui = start_ui(app);
    });

    proc.join();
    ui.join();
}
