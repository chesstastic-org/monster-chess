use super::Board;

pub struct ZobristHashTable<const T: usize> {
    pub table: Vec<u64>,
    pub positions: u32,
    pub teams: u32,
    pub pieces: u32,
    pub extra_len: u32
}

impl<const T: usize> ZobristHashTable<T> {
    fn get_gap_index(&self, position: u32) -> usize {
        (position + (self.positions)) as usize
    }

    fn get_first_move_index(&self, position: u32) -> usize {
        (position + (self.positions * 2)) as usize
    }

    fn get_piece_index(&self, position: u32, piece_type: u32, team: u32) -> usize {
        (position + (self.positions * (2 + (piece_type + (self.pieces * team))))) as usize
    }

    pub fn compute(&self, board: &Board<T>) -> u64 {
        let mut hash = 0;

        for gap_pos in board.state.gaps.iter_set_bits(board.state.squares) {
            hash ^= self.table[self.get_gap_index(gap_pos)];
        }

        for first_move_pos in board.state.first_move.iter_set_bits(board.state.squares) {
            hash ^= self.table[self.get_first_move_index(first_move_pos)];
        }

        for piece_type in 0..self.pieces {
            for team in 0..self.teams {
                let piece_team_board = board.state.pieces[piece_type as usize] & board.state.teams[team as usize];
                for piece_pos in piece_team_board.iter_set_bits(board.state.squares) {
                    hash ^= self.table[self.get_piece_index(piece_pos, piece_type, team)];
                }
            }
        }       

        hash
    }

    pub fn generate(positions: u32, pieces: u32, teams: u32, extra_len: u32, get_random: impl Fn() -> u64) -> ZobristHashTable<T> {
        let mut zobrist = ZobristHashTable {
            table: vec![ 0; (positions * (1 + (pieces * teams))) as usize ],
            positions,
            pieces,
            teams,
            extra_len
        };
    
        for position in 0..positions {
            let gap_pos = zobrist.get_gap_index(position);
            zobrist.table.insert(gap_pos, get_random());

            let first_move_pos = zobrist.get_first_move_index(position);
            zobrist.table.insert(first_move_pos, get_random());

            for piece_type in 0..pieces {
                for team in 0..teams {
                    let piece_pos = zobrist.get_piece_index(position, piece_type, team);
                    zobrist.table.insert(piece_pos, get_random());
                }
            }
        }
    
        zobrist
    }
}

#[cfg(test)]
mod tests {
    use fastrand::u64;

    use crate::games::chess::Chess;

    use super::ZobristHashTable;

    #[test]
    fn chess_zobrist_test() {
        let chess = Chess::create();
        let zobrist = ZobristHashTable::<1>::generate(
            chess.cols * chess.rows, chess.pieces.len() as u32, chess.teams, 0, || u64(0..u64::MAX)
        );
        
        let startpos = chess.default();
        let kiwipete = chess.from_fen("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1");

        assert_ne!(zobrist.compute(&startpos), zobrist.compute(&kiwipete), "Waaa?");
    }
}