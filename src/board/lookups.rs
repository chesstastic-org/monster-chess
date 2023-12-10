use crate::bitboard::BitBoard;

use super::{pieces::Piece, AttackLookup, Board, Cols, Rows};

pub fn generate_lookups<const T: usize>(
    board: &Board<T>,
    piece: &&'static dyn Piece<T>,
    _rows: Rows,
    _cols: Cols,
) -> AttackLookup<T> {
    let mut lookups = Vec::with_capacity(board.state.squares as usize);

    for i in 0..board.state.squares {
        let from = BitBoard::<T>::from_lsb(i);
        lookups.insert(i as usize, piece.generate_lookup_moves(board, from));
    }

    lookups
}

impl<'a, const T: usize> Board<'a, T> {
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
