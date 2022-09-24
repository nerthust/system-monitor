use std::env;

use rtop::core::system_reader::SystemReader;
use rtop::ui::layout::start_ui;

fn check_args(args: Vec<String>) -> Result<bool, &'static str> {
    if args.len() == 2 {
        if args[1].eq("-u") || args[1].eq("--current_usage") {
            return Ok(true);
        } else {
            return Err("USAGE: rtop [FLAG]?\nFLAG: -u, --current_usage: Sets process CPU% usage to be based on the current system CPU% rather than total CPU usage.");
        }
    } else if args.len() > 2 {
        return Err("Too many arguments");
    }

    Ok(false)
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    match check_args(args) {
        Ok(use_current_cpu_total) => {
            let sys_data = SystemReader::new(use_current_cpu_total);
            start_ui(sys_data).unwrap();
        }
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(1);
        }
    }

    Ok(())
}
