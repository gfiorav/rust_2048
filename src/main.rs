use ncurses::getch;
use ncurses::initscr;
use ncurses::refresh;
use std::io::Stdout;
use std::io;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use tui::Terminal;
use tui::backend::TermionBackend;

mod lib;

fn main() -> Result<(), io::Error> {
    // Listen for ctrl-c and quit at any time.
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(1)
    })
    .expect("Error setting Ctrl-C handler");

    // Init ncurses screen (blank screen + setup).
    initscr();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal: Terminal<TermionBackend<RawTerminal<Stdout>>> =
        Terminal::new(backend)?;

    let board: lib::board::Board = lib::board::init_board();

    for _ in 0..10 {
        refresh();
        lib::draw::draw(&mut terminal, &board);
        println!("{:?}", getch());
    }

    Ok(())
}
