use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::ui::app::App;

pub fn draw<B>(rect: &mut Frame<B>, _app: &App)
where
    B: Backend,
{
    let size = rect.size();
    // TODO check size
    check_size(&size);
    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    // Title block
    let title = draw_title();
    rect.render_widget(title, chunks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("RTop :)")
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}