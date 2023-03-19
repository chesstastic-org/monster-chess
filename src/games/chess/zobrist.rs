use crate::board::{game::ZobristController, Board, zobrist::ZobristHashTable, actions::Move};

#[derive(Debug)]
pub struct ChessZobrist<const T: usize>;

impl<const T: usize> ZobristController<T> for ChessZobrist<T> {
    fn apply(&self, hash: &mut u64, zobrist: &mut ZobristHashTable<T>, board: &mut Board<T>) {
        let last_move = (&board.history).last();
        if let None = last_move {
            *hash ^= zobrist.table[zobrist.base_len];
            return;
        }

        let last_move =
            last_move.expect("The last move for exporting an en passant FEN must be Some.");

        match last_move {
            Move::Action(last_action) => {
                if last_action.piece_type != 0 {
                    *hash ^= zobrist.table[zobrist.base_len];
                    return;
                }

                match last_action.from {
                    Some(from) => {
                        if from.abs_diff(last_action.to) != (2 * board.state.cols) {
                            *hash ^= zobrist.table[zobrist.base_len];
                        } else {
                            *hash ^= zobrist.table[zobrist.base_len + (last_action.to as usize)];
                        }
                    }
                    None => {
                        *hash ^= zobrist.table[zobrist.base_len];
                    }
                }
            }
            Move::Pass => {
                *hash ^= zobrist.table[zobrist.base_len];
            }
        }
    }

    fn get_extra_hashes(&self) -> usize {
        65
    }
}