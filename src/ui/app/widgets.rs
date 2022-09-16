use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::text::{Span, Spans};
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
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(size);

    // Title block
    let title = draw_title();
    rect.render_widget(title, chunks[0]);
    // Body & Help
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(0), Constraint::Min(10)].as_ref())
        .split(chunks[1]);

    let help = draw_process(&size);
    rect.render_widget(help, body_chunks[1]);
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

fn draw_process(rect: &Rect) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];

    let row = Row::new(vec![
            Cell::from(Span::styled("puid", key_style)),
            Cell::from(Span::styled("command", help_style)),
        ]);
        rows.push(row);

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("All process"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}