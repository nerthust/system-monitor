use std::borrow::BorrowMut;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Instant, Duration};
use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute
};

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use tui::widgets::TableState;
use tui::{backend::CrosstermBackend, Terminal};

use crate::core::error::RTopError;
use crate::core::system_reader::SystemReader;
use crate::ui::app::widgets;
use crate::ui::app::App;

use super::inputs::InputEvent;


pub fn start_ui(mut sys_data: SystemReader) -> Result<(), RTopError> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    //read input thread
    let rxinput = input_thread(Duration::from_millis(1000));
    
    let mut proc_table_state:TableState = TableState::default();
    proc_table_state.select(Some(0));

    //let (data, (tx_b_n,rx_b_n))= rxproc.recv().unwrap();

    let data = sys_data.read_process_data().unwrap();
    let mut app = App::new(data, 0,0);

    //let mut app = Rc::new(RefCell::new(app));

    loop {
        //App state
        let a = app.borrow_mut();                            
        let table_state = proc_table_state.borrow_mut();

        //Wait for input
        match rxinput.recv()? {
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
            InputEvent::Tick => {
                //Update data
                let new_data = sys_data.read_process_data().unwrap();
                a.update_data(&new_data);
                //Update tx/rx network bits
            }
        }

        // Render
        terminal.draw(|rect| widgets::draw(rect, a, table_state))?;
    }

    terminal.clear()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

pub fn input_thread(tick_rate: Duration) -> Receiver<InputEvent<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    tx.send(InputEvent::Input(key)).unwrap();
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(InputEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    rx
}
