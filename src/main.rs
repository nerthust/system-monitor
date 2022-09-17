use std::env;

use rtop::core::system_reader::SystemReader;
use rtop::ui::layout::start_ui;

fn check_args(args: Vec<String>) -> Result<bool, &'static str> {
    if args.len() == 3 {
        if args[2].eq("-u") {
            return Ok(true);
        } else {
            return Err("Not a valid argument");
        }
    }else if args.len() > 3 {
        return Err("Too many arguments");
    }

    Ok(false)
}

fn main() -> Result<(), &'static str>{
    let args: Vec<String> = env::args().collect();
    let use_current_cpu_total = check_args(args)?;

    let sys_data = SystemReader::new(use_current_cpu_total);
    let _ui = start_ui(sys_data);

    Ok(())
}
