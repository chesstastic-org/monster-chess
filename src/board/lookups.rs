use crate::{Piece, Rows, Cols, BitBoard, Board};

pub fn generate_lookups(board: &Board, piece: &Box<dyn Piece>, rows: Rows, cols: Cols) -> Vec<BitBoard> {
    let mut lookups = Vec::with_capacity((rows * cols) as usize);

    for i in 0..(rows * cols) {
        let from = !(BitBoard::max() << 1) << i;
        lookups.insert(i as usize, piece.generate_moves(board, from));
    }

    lookups
}

impl Board {
    pub fn generate_lookups(&mut self) {
        for (ind, piece) in self.pieces.iter().enumerate() {
            if !piece.can_lookup() {
                continue;
            }

            self.attack_lookup.insert(ind, generate_lookups(self, piece, self.state.rows, self.state.cols));
        }
    }   
}