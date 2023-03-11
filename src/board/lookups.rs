use super::{AttackLookup, Cols, Rows, Board, BitBoard, pieces::Piece};

pub fn generate_lookups(
    board: &Board,
    piece: &Box<dyn Piece>,
    rows: Rows,
    cols: Cols,
) -> AttackLookup {
    let mut lookups = Vec::with_capacity((rows * cols) as usize);

    for i in 0..(rows * cols) {
        let from = BitBoard::from_lsb(i as u32);
        lookups.insert(i as usize, piece.generate_lookup_moves(board, from));
    }

    lookups
}

impl Board {
    pub fn generate_lookups(&mut self) {
        for (ind, piece) in self.game.pieces.iter().enumerate() {
            if !piece.can_lookup() {
                self.attack_lookup.insert(ind, vec![]);
                continue;
            }

            self.attack_lookup.insert(
                ind,
                generate_lookups(self, piece, self.state.rows, self.state.cols),
            );
        }
    }
}
