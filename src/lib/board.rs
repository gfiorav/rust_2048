pub struct Board {
    positions: Vec<u16>,
}

pub fn init_board() -> Board {
    Board {
        positions: vec![0; 16],
    }
}
