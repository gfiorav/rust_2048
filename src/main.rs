use std::io::Stdout;
use std::io;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use tui::Terminal;
use tui::backend::TermionBackend;

mod lib;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal: Terminal<TermionBackend<RawTerminal<Stdout>>> =
        Terminal::new(backend)?;

    let board: lib::board::Board = lib::board::init_board();

    lib::draw::draw(&mut terminal, &board);

    Ok(())
}
