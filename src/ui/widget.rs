use std::{io, cell::RefCell, rc::Rc};

use tui::{backend::CrosstermBackend, Terminal};

use crate::core::error::RTopError;
use crate::ui::app::layout;
use crate::ui::app::App;


pub fn start_ui(app: Rc<RefCell<App>>) -> Result<(), RTopError> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();
        // Render
        terminal.draw(|rect| layout::draw(rect, &app))?;
        // TODO handle inputs here
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}