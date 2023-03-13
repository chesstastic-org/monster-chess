use super::{actions::{HistoryMove, HistoryUpdate, Action, UndoMoveError, IndexedPreviousBoard, HistoryState, PreviousBoard}, game::Game, Board, BitBoard, Rows, Cols, PieceType, AttackLookup, AttackDirections, BoardState};

pub enum PieceSymbol {
    Char(char),
    TeamSymbol(Vec<char>),
}

const NORMAL_MOVE: usize = 0;

pub trait Piece {
    fn get_piece_symbol(&self) -> PieceSymbol;

    fn format_info(&self, board: &Board, info: u32) -> String {
        "".to_string()
    }
    fn parse_info(&self, board: &Board, info: String) -> u32 {
        0
    }

    fn can_lookup(&self) -> bool;

    fn get_attack_lookup<'a>(&self, board: &'a Board, piece_type: usize) -> Option<&'a AttackLookup> {
        board.attack_lookup.get(piece_type)
    }

    fn get_moves(&self, board: &Board, from: BitBoard, piece_type: usize, team: u32, mode: u32) -> BitBoard;
    fn can_move_mask(&self, board: &Board, from: BitBoard, from_bit: u32, piece_type: usize, team: u32, mode: u32, to: BitBoard) -> BitBoard {
        self.get_moves(board, from, piece_type, team, mode)
    }

    #[allow(unused_variables)]
    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        Vec::new()
    }

    fn make_capture_move(&self, board: &mut Board, action: &Action, piece_type: usize, from: BitBoard, to: BitBoard) {
        let color: usize = action.team as usize;
        let captured_color: usize = if (to & board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let mut captured_piece_type: usize = 0;
        for i in 0..(board.game.pieces.len()) {
            if (board.state.pieces[i] & to).is_set() {
                captured_piece_type = i;
                break;
            }
        }

        let history_move = HistoryMove {
            action: *action,
            state: HistoryState::Any {
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
                updates: vec![                    
                    HistoryUpdate::Team(IndexedPreviousBoard(color, board.state.teams[color])),
                    HistoryUpdate::Team(IndexedPreviousBoard(captured_color, board.state.teams[captured_color])),
                    HistoryUpdate::Piece(IndexedPreviousBoard(piece_type, board.state.pieces[piece_type])),
                    HistoryUpdate::Piece(IndexedPreviousBoard(captured_piece_type, board.state.pieces[captured_piece_type]))
                ]
            }
        };
        board.history.push(history_move);

        board.state.teams[captured_color] ^= to;
        board.state.teams[color] ^= from;
        board.state.teams[color] |= to;

        board.state.pieces[captured_piece_type] ^= to;
        board.state.pieces[piece_type] ^= from;
        board.state.pieces[piece_type] |= to;

        board.state.all_pieces ^= from;

        board.state.first_move &= !from;
        board.state.first_move &= !to;
        // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
    }

    fn make_normal_move(&self, board: &mut Board, action: &Action, piece_type: usize, from: BitBoard, to: BitBoard) {
        let color: usize = action.team as usize;

        board.history.push(HistoryMove {
            action: *action,
            state: HistoryState::Single {
                team: IndexedPreviousBoard(color, board.state.teams[color]),
                piece: IndexedPreviousBoard(piece_type, board.state.pieces[piece_type]),
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move)
            }
        });

        board.state.teams[color] ^= from;
        board.state.teams[color] |= to;

        board.state.pieces[piece_type] ^= from;
        board.state.pieces[piece_type] |= to;

        board.state.all_pieces ^= from;
        board.state.all_pieces |= to;

        board.state.first_move &= !from;
    }

    fn make_move(&self, board: &mut Board, action: &Action) {
        let from = BitBoard::from_lsb(action.from);
        let to = BitBoard::from_lsb(action.to);

        if (board.state.all_pieces & to).is_empty() {
            self.make_normal_move(board, action, action.piece_type, from, to);
        } else {
            self.make_capture_move(board, action, action.piece_type, from, to);
        }

        self.update_turns(board);
    }

    fn update_turns(&self, board: &mut Board) {
        board.state.turns += 1;
        board.state.current_turn = board.state.turn_lookup[board.state.current_turn as usize];
        if board.state.current_turn == 0 {
            board.state.sub_moves += 1;

            if board.state.moving_team == 0 {
                board.state.full_moves += 1;
            }
            board.state.moving_team = board.state.team_lookup[board.state.moving_team as usize];
        };
    }

    fn undo_move(&self, state: &mut BoardState, game: &Game, history_move: &HistoryMove) {
        state.turns -= 1;
        state.current_turn = state.turn_reverse_lookup[state.current_turn as usize];
        if state.current_turn == game.turns - 1 {
            state.moving_team = state.team_reverse_lookup[state.moving_team as usize];
            state.sub_moves -= 1;
    
            if state.moving_team == 0 {
                state.full_moves -= 1;
            }
        }
    
        match &history_move.state {
            HistoryState::Single { all_pieces, first_move, team, piece } => {
                state.all_pieces = all_pieces.0;
                state.first_move = first_move.0;
                state.teams[team.0] = team.1;
                state.pieces[piece.0] = piece.1;
            }
            HistoryState::Any { first_move, all_pieces, updates } => {
                state.all_pieces = all_pieces.0;
                state.first_move = first_move.0;
                for change in updates {
                    match change {
                        HistoryUpdate::Team(team) => {
                            state.teams[team.0] = team.1;
                        }
                        HistoryUpdate::Piece(piece) => {
                            state.pieces[piece.0] = piece.1;
                        }
                    }
                }    
            }
            HistoryState::None => {}
        }
    }

    fn add_actions(
        &self,
        actions: &mut Vec<Action>,
        board: &Board, 
        piece_type: usize,
        from: u32,
        team: u32,
        mode: u32
    ) {
        let from_board = BitBoard::from_lsb(from);

        let bit_actions =
            self.get_moves(board, from_board, piece_type, team, mode) & !board.state.teams[team as usize];

        if bit_actions.is_empty() {
            return;
        }

        for bit in bit_actions.iter_one_bits(board.state.squares) {
            actions.push(Action {
                from,
                to: bit,
                team,
                info: NORMAL_MOVE,
                piece_type,
            });
        }
    }
}
