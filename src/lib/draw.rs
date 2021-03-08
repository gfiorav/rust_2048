use std::io::Stdout;
use super::board;
use termion::raw::RawTerminal;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::Constraint;
use tui::layout::Layout;
use tui::style::Color;
use tui::style::Style;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Cell;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::widgets::TableState;

pub fn draw(
    terminal: &mut Terminal<TermionBackend<RawTerminal<Stdout>>>,
    board: &board::Board,
) {
    let mut state = TableState::default();

    let _ = terminal.draw(|f| {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(f.size());

        let rows: Vec<Row> = board::get_rows(board)
            .iter()
            .map(|row| {
                Row::new(
                    row.iter().map(|n| {
                        Cell::from(n.to_string())
                    })
                )
                    .height(10 as u16)

            })
            .collect();

        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Rusty 2048"))
            .style(Style::default().bg(Color::Black))
            .widths(&[
                Constraint::Percentage(33),
                Constraint::Length(30),
                Constraint::Max(10),
            ]);

        f.render_stateful_widget(table, rects[0], &mut state);
    });
}
