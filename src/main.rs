use rtop::core::system_reader::SystemReader;
use rtop::ui::layout::start_ui;
fn main() {
    let sys_data = SystemReader::new(false);

    let _ui = start_ui(sys_data);
}
