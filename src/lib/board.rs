pub struct Board {
    pub positions: Vec<u16>,
}

pub fn init_board() -> Board {
    Board {
        positions: vec![0; 16],
    }
}

pub fn get_rows(board: &Board) -> Vec<Vec<u16>> {
    vec!(
        (&board.positions[0..4]).to_vec(),
        (&board.positions[4..8]).to_vec(),
        (&board.positions[8..12]).to_vec(),
        (&board.positions[12..16]).to_vec(),
    )
}
