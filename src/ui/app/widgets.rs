use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::text::{Span};
use tui::Frame;

use crate::core::process::ProcData;
use crate::ui::app::App;

pub fn draw<B>(rect: &mut Frame<B>, _app: &App)
where
    B: Backend,
{
    let size = rect.size();
    //check_size(&size);
    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(10)
            ].as_ref()
        )
        .split(size);

    // Title block
    let title = draw_title();
    rect.render_widget(title, chunks[0]);
    // Process block
    let process = draw_process(_app.data().to_vec());
    rect.render_widget(process, chunks[1]);
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

//fn check_size(rect: &Rect) {
//    if rect.width < 52 {
//        panic!("Require width >= 52, (got {})", rect.width);
//    }
//    if rect.height < 28 {
//        panic!("Require height >= 28, (got {})", rect.height);
//    }
//}


fn draw_process(data: Vec<ProcData>) -> Table<'static>{
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];

    for process in data.iter() {
        let row = Row::new(vec![
            Cell::from(Span::styled(process.pid.to_string(), key_style)),
            Cell::from(Span::styled(process.parent_pid.to_string(), help_style)),
            Cell::from(Span::styled(process.priority.to_string(), help_style)),
            Cell::from(Span::styled(process.round_cpu_usage_percent.to_string(), key_style)),
            Cell::from(Span::styled(process.round_mem_usage_percent.to_string(), help_style)),
            Cell::from(Span::styled(process.total_disk_read_bytes.unwrap_or(0).to_string(), key_style)),
            Cell::from(Span::styled(process.total_disk_write_bytes.unwrap_or(0).to_string(), help_style)),
            Cell::from(Span::styled(process.state.0.clone(), key_style)),
            Cell::from(Span::styled(process.uid.unwrap().to_string(), help_style)),
            Cell::from(Span::styled(process.name.to_string(), key_style)),
            Cell::from(Span::styled(process.command.to_string(), key_style))
        ]);
        rows.push(row);
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("All process"),
        )
        .header(
            Row::new(vec!["PID","PARENT_ID","PRIORITY",
                                "%CPU","%MEM","READ(KB)","WRITE(KB)",
                                "STATE","UID","NAME", "COMMAND"])
                .style(Style::default().fg(Color::LightGreen))
                .bottom_margin(1)
        )
        .widths(&[Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(30),
                Constraint::Min(10000)])
        .column_spacing(1)
}
