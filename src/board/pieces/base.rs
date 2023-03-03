use crate::{BitBoard, Board, PieceType, AttackDirections, AttackLookup, Action, HistoryMove, IndexedPreviousBoard, PreviousBoard, NoHistoryMoves};

pub trait Piece {
    fn duplicate(&self) -> Box<dyn Piece>;

    fn get_piece_type(&self) -> PieceType;
    fn get_piece_symbol(&self) -> char;

    fn can_lookup(&self) -> bool;
    fn get_attack_lookup<'a>(&self, board: &'a Board, from: BitBoard) -> Option<&'a AttackLookup> {
        board.attack_lookup.get(self.get_piece_type())
    }

    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard;
    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        Vec::new()
    }

    fn make_move(&self, board: &mut Board, action: &Action) {
        let from = BitBoard::from_lsb(action.from);
        let to = BitBoard::from_lsb(action.to);

        if board.state.blockers.has_bit(action.to) {
            let color: usize = if (from & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let captured_color: usize = if (to & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let piece_type = self.get_piece_type();
            let mut captured_piece_type: usize = 0; 
            for i in 0..(board.pieces.len()) {
                if (board.state.pieces[i] & &to).is_set() {
                    captured_piece_type = i;
                    break;
                }
            }
        
            let history_move: HistoryMove = HistoryMove {
                action: *action,
                teams: vec![
                    IndexedPreviousBoard(color, board.state.teams[color]),
                    IndexedPreviousBoard(captured_color, board.state.teams[captured_color])
                ],
                pieces: vec![
                    IndexedPreviousBoard(piece_type, board.state.teams[piece_type]),
                    IndexedPreviousBoard(captured_piece_type, board.state.teams[captured_piece_type])
                ],
                blockers: PreviousBoard(board.state.blockers),
                first_move: PreviousBoard(board.state.first_move)
            };
            board.state.history.push(history_move);
        
            board.state.teams[captured_color] ^= &to;
            board.state.teams[color] ^= &from;
            board.state.teams[color] |= &to;
        
            board.state.pieces[captured_piece_type] ^= &to;
            board.state.pieces[piece_type] ^= &from;
            board.state.pieces[piece_type] |= &to;
        
            board.state.blockers ^= &from;
        
            board.state.first_move ^= &from;
            board.state.first_move ^= &to;
            // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
        } else {
            let color: usize = if (from & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let piece_type = self.get_piece_type();
        
            let history_move: HistoryMove = HistoryMove {
                action: *action,
                teams: vec![
                    IndexedPreviousBoard(color, board.state.teams[color])
                ],
                pieces: vec![
                    IndexedPreviousBoard(piece_type, board.state.teams[piece_type])
                ],
                blockers: PreviousBoard(board.state.blockers),
                first_move: PreviousBoard(board.state.first_move)
            };
            board.state.history.push(history_move);
        
            board.state.teams[color] ^= &from;
            board.state.teams[color] |= &to;
        
            board.state.pieces[piece_type] ^= &from;
            board.state.pieces[piece_type] |= &to;
        
            board.state.blockers ^= &from;
            board.state.blockers |= &to;
            
            board.state.first_move ^= &from;
        }
    }

    fn undo_move(&self, board: &mut Board) -> Result<(), NoHistoryMoves> {
        let history_move = board.state.history.pop();
        match history_move {
            Some(history_move) => {
                for IndexedPreviousBoard(index, bitboard) in history_move.teams {
                    board.state.teams[index] = bitboard;
                }

                for IndexedPreviousBoard(index, bitboard) in history_move.pieces {
                    board.state.pieces[index] = bitboard;
                }

                board.state.blockers = history_move.blockers.0;
                board.state.first_move = history_move.first_move.0;

                Ok(())
            }
            None => {
                Err(NoHistoryMoves)
            }
        }
    }

    fn add_actions(&self, actions: &mut Vec<Action>, board: &Board, from: u32, team: u32) {
        let from_board = BitBoard::from_lsb(from);
        let bit_actions = self.get_moves(board, from_board, team) & &!board.state.teams[team as usize];

        if bit_actions.is_empty() {
            return;
        }
        
        let rows = board.state.rows;
        let cols = board.state.cols;

        for bit in bit_actions.iter_one_bits((rows * cols) as u32) {
            actions.push(Action {
                from,
                to: bit,
                info: 0
            });
        }
    }
}