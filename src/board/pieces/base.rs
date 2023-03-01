use crate::{BitBoard, Board, PieceType, AttackDirections, AttackLookup, Action, HistoryMove, IndexedPreviousBoard, PreviousBoard, NoHistoryMoves};

pub trait Piece {
    fn get_piece_type(&self) -> PieceType;
    fn get_piece_symbol(&self) -> &str;

    fn can_lookup(&self) -> bool;
    fn get_attack_lookup<'a>(&self, board: &'a Board, from: BitBoard) -> Option<&'a AttackLookup> {
        board.attack_lookup.get(self.get_piece_type())
    }

    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard;
    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        Vec::new()
    }

    fn make_move(&self, board: &mut Board, action: &Action) {
        if action.capture {
            let color: usize = if (action.from & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let captured_color: usize = if (action.to & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let piece_type = self.get_piece_type();
            let mut captured_piece_type: usize = 0; 
            for i in 0..(board.pieces.len()) {
                if (board.state.pieces[i] & &action.to).is_set() {
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

            board.state.teams[captured_color] ^= &action.to;
            board.state.teams[color] ^= &action.from;
            board.state.teams[color] |= &action.to;

            board.state.pieces[captured_piece_type] ^= &action.to;
            board.state.pieces[piece_type] ^= &action.from;
            board.state.pieces[piece_type] |= &action.to;

            board.state.blockers ^= &action.from;
            // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
        } else {
            let color: usize = if (action.from & &board.state.teams[0]).is_set() {
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

            board.state.teams[color] ^= &action.from;
            board.state.teams[color] |= &action.to;

            board.state.pieces[piece_type] ^= &action.from;
            board.state.pieces[piece_type] |= &action.to;

            board.state.blockers ^= &action.from;
            board.state.blockers |= &action.to;
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
}