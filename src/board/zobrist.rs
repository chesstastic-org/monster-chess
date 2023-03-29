use super::{Board, game::Game};

#[derive(Debug, Clone)]
pub struct ZobristHashTable<const T: usize> {
    pub table: Vec<u64>,
    pub squares: u16,
    pub teams: u16,
    pub pieces: u16,
    pub base_len: usize,
    pub extra_len: usize
}

impl<const T: usize> ZobristHashTable<T> {
    fn get_moving_team_index(&self, moving_team: u16) -> usize {
        moving_team as usize
    }

    fn get_gap_index(&self, moving_team: u16, position: u16) -> usize {
        (moving_team as usize) + position as usize
    }

    fn get_first_move_index(&self, moving_team: u16, position: u16) -> usize {
        (moving_team as usize) + (position + (self.squares)) as usize
    }

    fn get_piece_index(&self, moving_team: u16, position: u16, piece_type: u16, team: u16) -> usize {
        (moving_team as usize) + (position + (self.squares * (1 + (piece_type + (self.pieces * team))))) as usize
    }

    pub fn compute(&self, board: &Board<T>) -> u64 {
        let mut hash = 0;
        let moving_team = board.state.moving_team;

        hash ^= self.table[self.get_moving_team_index(moving_team)];

        for gap_pos in board.state.gaps.iter_set_bits(board.state.squares) {
            hash ^= self.table[self.get_gap_index(moving_team, gap_pos)];
        }

        for first_move_pos in board.state.first_move.iter_set_bits(board.state.squares) {
            hash ^= self.table[self.get_first_move_index(moving_team, first_move_pos)];
        }

        for piece_type in 0..self.pieces {
            for team in 0..self.teams {
                let piece_team_board = board.state.pieces[piece_type as usize] & board.state.teams[team as usize];
                for piece_pos in piece_team_board.iter_set_bits(board.state.squares) {
                    hash ^= self.table[self.get_piece_index(moving_team, piece_pos, piece_type, team)];
                }
            }
        }       

        hash
    }

    pub fn generate(squares: u16, teams: u16, pieces: u16, extra_hashes: usize, get_random: impl Fn() -> u64) -> ZobristHashTable<T> {
        let base_len = (
            (teams - 1) + (squares) + (squares * (1 + ((pieces - 1) + (pieces * (teams - 1))))) + 1
        ) as usize;
        let hashes = base_len + (extra_hashes as usize);
        let mut zobrist = ZobristHashTable {
            table: vec![ 0; hashes ],
            squares: squares,
            pieces,
            teams: teams,
            base_len,
            extra_len: extra_hashes
        };

        for hash in 0..hashes {
            zobrist.table[hash] = get_random();
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
        
        let startpos = chess.default();
        let kiwipete = chess.from_fen("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1");

        assert_ne!(chess.zobrist.compute(&startpos), chess.zobrist.compute(&kiwipete), "Waaa?");
    }
}