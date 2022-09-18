use std::borrow::BorrowMut;
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};
use std::{io, cell::RefCell, rc::Rc};

use crossterm::{
    event::{self, Event, KeyCode},
    execute
};

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use tui::widgets::TableState;
use tui::{backend::CrosstermBackend, Terminal};

use crate::core::error::RTopError;
use crate::core::system_reader::SystemReader;
use crate::ui::app::widgets;
use crate::ui::app::{App};

use super::inputs::InputEvent;




pub fn start_ui(mut sys_data: SystemReader) -> Result<(), RTopError> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(1000);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let Event::Key(key) = event::read().unwrap() {
                    tx.send(InputEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(InputEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut proc_table_state:TableState = TableState::default();
    proc_table_state.select(Some(0));
    
    let data = sys_data.read_process_data().unwrap();
    let app = Rc::new(RefCell::new(App::new(data)));
    
    loop {
        let a = app.borrow();
        let table_state = proc_table_state.borrow_mut();
        // Render
        terminal.draw(|rect| widgets::draw(rect, &a, table_state, &mut sys_data))?;
        
        match rx.recv()? {
            InputEvent::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char('c') => {
                    break;
                }
                KeyCode::Down => {
                    if let Some(selected) = table_state.selected() {
                        if selected >= 50 {
                            table_state.select(Some(0));
                        } else {
                            table_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = table_state.selected() {
                        if selected > 0 {
                            table_state.select(Some(selected - 1));
                        } else {
                            table_state.select(Some(50 - 1));
                        }
                    }

                }
                _ => {}
            },
            InputEvent::Tick => {}
        }
        //thread::sleep(Duration::from_millis(500))
    }

    terminal.clear()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}