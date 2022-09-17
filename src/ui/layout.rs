use core::time;
use std::thread;
use std::{io, cell::RefCell, rc::Rc};

use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use tui::{backend::CrosstermBackend, Terminal};

use crate::core::error::RTopError;
use crate::core::system_reader::SystemReader;
use crate::ui::app::widgets;
use crate::ui::app::App;


pub fn start_ui(mut sys_data: SystemReader) -> Result<(), RTopError> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let delay = time::Duration::from_millis(1000);

    loop {
        //let app = app.borrow();
        let data = sys_data.read_process_data().unwrap();
        let app = Rc::new(RefCell::new(App::new(data)));
        let a = app.borrow();
        // Render
        terminal.draw(|rect| widgets::draw(rect, &a))?;
        // TODO handle inputs here


        thread::sleep(delay);
    }

    //terminal.clear()?;
    //execute!(stdout, LeaveAlternateScreen)?;
    //terminal.show_cursor()?;
    //crossterm::terminal::disable_raw_mode()?;

    //Ok(())
}