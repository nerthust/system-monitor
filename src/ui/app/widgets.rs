use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, List, ListItem, Row, Table, TableState};
use tui::Frame;

use crate::core::process::ProcData;
use crate::ui::app::App;

pub fn draw<B>(rect: &mut Frame<B>, _app: &mut App, proc_state: &mut TableState)
where
    B: Backend,
{
    let size = rect.size();

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(10)].as_ref())
        .split(size);

    //General network
    let net_list = draw_network_general(_app.tx_bits_n, _app.rx_bits_n);
    rect.render_widget(net_list, chunks[0]);

    // Process table
    let process = draw_process(_app.data().to_vec());
    rect.render_stateful_widget(process, chunks[1], proc_state);
}

fn draw_network_general<'a>(tx: u64, rx: u64) -> List<'static> {
    let rx_style = Style::default().fg(Color::LightMagenta);
    let tx_style = Style::default().fg(Color::LightCyan);

    let spans = Spans::from(vec![
        Span::styled("Total RX: ", rx_style),
        Span::styled(tx.to_string() + " Bytes", rx_style),
        Span::raw("      "),
        Span::styled("Total TX: ", tx_style),
        Span::styled(rx.to_string() + " Bytes", tx_style),
    ]);

    let title = Span::styled("  Rtop  ", Style::default().fg(Color::LightGreen));

    let list_items = [ListItem::new(vec![spans])];
    List::new(list_items)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
}

fn draw_process(data: Vec<ProcData>) -> Table<'static> {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];

    for process in data.iter() {
        let row = Row::new(vec![
            Cell::from(Span::styled(process.pid.to_string(), key_style)),
            Cell::from(Span::styled(process.parent_pid.to_string(), help_style)),
            Cell::from(Span::styled(process.priority.to_string(), help_style)),
            Cell::from(Span::styled(
                process.cpu_usage_percent.to_string(),
                key_style,
            )),
            Cell::from(Span::styled(
                process.mem_usage_percent.to_string(),
                help_style,
            )),
            Cell::from(Span::styled(
                process.total_disk_read_bytes.unwrap_or(0).to_string(),
                key_style,
            )),
            Cell::from(Span::styled(
                process.total_disk_write_bytes.unwrap_or(0).to_string(),
                help_style,
            )),
            Cell::from(Span::styled(process.state.0.clone(), key_style)),
            Cell::from(Span::styled(process.uid.unwrap().to_string(), help_style)),
            Cell::from(Span::styled(process.name.to_string(), key_style)),
            Cell::from(Span::styled(process.command.to_string(), key_style)),
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
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .header(
            Row::new(vec![
                "PID",
                "PARENT_ID",
                "PRIORITY",
                "%CPU",
                "%MEM",
                "READ(KB)",
                "WRITE(KB)",
                "STATE",
                "UID",
                "NAME",
                "COMMAND",
            ])
            .style(Style::default().fg(Color::LightGreen))
            .bottom_margin(1),
        )
        .widths(&[
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(30),
            Constraint::Min(10000),
        ])
        .column_spacing(1)
}
