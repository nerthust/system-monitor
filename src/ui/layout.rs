use std::{io, cell::RefCell, rc::Rc};

use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use tui::{backend::CrosstermBackend, Terminal};

use crate::core::error::RTopError;
use crate::ui::app::widgets;
use crate::ui::app::App;


pub fn start_ui(app: Rc<RefCell<App>>) -> Result<(), RTopError> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();
        // Render
        terminal.draw(|rect| widgets::draw(rect, &app))?;
        // TODO handle inputs here
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}