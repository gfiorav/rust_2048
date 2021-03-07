use tui::Terminal;
use tui::widgets::{Block, Borders};
use termion::raw::RawTerminal;
use tui::backend::TermionBackend;
use std::io::Stdout;
use super::board;


pub fn draw(
    terminal: &mut Terminal<TermionBackend<RawTerminal<Stdout>>>,
    board: &board::Board,
) {
    let _ = terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    });
}
