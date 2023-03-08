use crate::{
    Action, AttackDirections, AttackLookup, BitBoard, Board, HistoryMove, IndexedPreviousBoard,
    PieceType, PreviousBoard, UndoMoveError, HistoryState,
};

pub enum PieceSymbol {
    Char(char),
    TeamSymbol(Vec<char>),
}

const NORMAL_MOVE: usize = 0;

pub trait Piece {
    fn duplicate(&self) -> Box<dyn Piece>;

    fn get_piece_type(&self) -> PieceType;
    fn get_piece_symbol(&self) -> PieceSymbol;

    fn can_lookup(&self) -> bool;

    fn get_attack_lookup<'a>(&self, board: &'a Board) -> Option<&'a AttackLookup> {
        board.attack_lookup.get(self.get_piece_type())
    }

    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard;

    #[allow(unused_variables)]
    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        Vec::new()
    }

    fn make_capture_move(&self, board: &mut Board, action: &Action, from: BitBoard, to: BitBoard) {
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
        for i in 0..(board.game.pieces.len()) {
            if (board.state.pieces[i] & &to).is_set() {
                captured_piece_type = i;
                break;
            }
        }

        let history_move = HistoryMove {
            action: *action,
            state: Some(HistoryState {
                teams: vec![
                    IndexedPreviousBoard(color, board.state.teams[color]),
                    IndexedPreviousBoard(captured_color, board.state.teams[captured_color]),
                ],
                pieces: vec![
                    IndexedPreviousBoard(piece_type, board.state.pieces[piece_type]),
                    IndexedPreviousBoard(captured_piece_type, board.state.pieces[captured_piece_type]),
                ],
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
            })
        };
        board.state.history.push(history_move);

        board.state.teams[captured_color] ^= &to;
        board.state.teams[color] ^= &from;
        board.state.teams[color] |= &to;

        board.state.pieces[captured_piece_type] ^= &to;
        board.state.pieces[piece_type] ^= &from;
        board.state.pieces[piece_type] |= &to;

        board.state.all_pieces ^= &from;

        board.state.first_move &= &!from;
        board.state.first_move &= &!to;
        // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
    }

    fn make_normal_move(&self, board: &mut Board, action: &Action, from: BitBoard, to: BitBoard) {
        let color: usize = if (from & &board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let piece_type = self.get_piece_type();

        let history_move = HistoryMove {
            action: *action,
            state: Some(HistoryState {
                teams: vec![IndexedPreviousBoard(color, board.state.teams[color])],
                pieces: vec![IndexedPreviousBoard(
                    piece_type,
                    board.state.pieces[piece_type],
                )],
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move)
            })
        };
        board.state.history.push(history_move);

        board.state.teams[color] ^= &from;
        board.state.teams[color] |= &to;

        board.state.pieces[piece_type] ^= &from;
        board.state.pieces[piece_type] |= &to;

        board.state.all_pieces ^= &from;
        board.state.all_pieces |= &to;

        board.state.first_move &= &!from;
    }

    fn make_move(&self, board: &mut Board, action: &Action) {
        let from = BitBoard::from_lsb(action.from);
        let to = BitBoard::from_lsb(action.to);

        if board.state.all_pieces.has_bit(action.to) {
            self.make_capture_move(board, action, from, to);
        } else {
            self.make_normal_move(board, action, from, to);
        }

        board.state.current_turn += 1;
        board.state.turns += 1;
        if board.state.current_turn >= board.game.turns {
            board.state.current_turn = 0;
            board.state.sub_moves += 1;

            if board.state.moving_team == 0 {
                board.state.full_moves += 1;
            }
            board.state.moving_team = board.get_next_team(board.state.moving_team);
        }
    }

    fn undo_move(&self, board: &mut Board) -> Result<(), UndoMoveError> {
        let previous_team = board.state.moving_team;

        // TODO: Update full moves, sub moves, turns

        board.state.current_turn -= 1;
        if board.state.current_turn == u32::MAX {
            board.state.moving_team = board.get_previous_team(previous_team);
            board.state.current_turn = board.game.turns - 1;
            board.state.sub_moves -= 1;

            if previous_team == 0 {
                board.state.full_moves -= 1;
            }
        }

        let history_move = board.state.history.pop();
        match history_move {
            Some(history_move) => {
                if let Some(history_state) = history_move.state {
                    for IndexedPreviousBoard(index, bitboard) in history_state.teams {
                        board.state.teams[index] = bitboard;
                    }

                    for IndexedPreviousBoard(index, bitboard) in history_state.pieces {
                        board.state.pieces[index] = bitboard;
                    }

                    board.state.all_pieces = history_state.all_pieces.0;
                    board.state.first_move = history_state.first_move.0;
                }

                Ok(())
            }
            None => Err(UndoMoveError::NoHistoryMoves),
        }
    }

    fn add_actions(&self, actions: &mut Vec<Action>, board: &Board, from: u32, team: u32) {
        let piece_type = self.get_piece_type();
        let from_board = BitBoard::from_lsb(from);
        let bit_actions =
            self.get_moves(board, from_board, team) & &!board.state.teams[team as usize];

        if bit_actions.is_empty() {
            return;
        }

        let rows = board.state.rows;
        let cols = board.state.cols;

        for bit in bit_actions.iter_one_bits((rows * cols) as u32) {
            actions.push(Action {
                from,
                to: bit,
                info: NORMAL_MOVE,
                piece_type,
            });
        }
    }
}
