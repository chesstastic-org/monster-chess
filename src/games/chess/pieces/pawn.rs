use crate::{
    Action, BitBoard, Board, Cols, HistoryMove, IndexedPreviousBoard, Piece, PieceSymbol,
    PieceType, PreviousBoard, HistoryState, FenState,
};

const NORMAL_PAWN_MOVE: usize = 0;
const EN_PASSANT_MOVE: usize = 1;
fn promotion_move(piece_type: PieceType) -> usize {
    piece_type + 2
}

pub struct PawnPiece {
    pub piece_type: PieceType,
}

pub fn up(bitboard: &BitBoard, shift: u32, cols: Cols, team: u32) -> BitBoard {
    match team {
        0 => bitboard.up(shift, cols),
        1 => bitboard.down(shift, cols),
        _ => bitboard.up(shift, cols),
    }
}

pub fn down(bitboard: &BitBoard, shift: u32, cols: Cols, team: u32) -> BitBoard {
    match team {
        0 => bitboard.down(shift, cols),
        1 => bitboard.up(shift, cols),
        _ => bitboard.down(shift, cols),
    }
}

impl Piece for PawnPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self {
            piece_type: self.piece_type,
        })
    }

    fn can_lookup(&self) -> bool {
        false
    }

    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('p')
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let mut moves = BitBoard::new();
        let cols = board.state.cols;

        let single_moves = up(&from, 1, cols, team) & &!board.state.all_pieces;
        let first_move = (from & &board.state.first_move).is_set();

        moves |= &single_moves;

        if first_move {
            let double_moves = up(&single_moves, 1, cols, team) & &!board.state.all_pieces;
            moves |= &double_moves;
        }

        let up_one = from.up(1, cols);
        let mut captures = up_one.right(1);
        captures |= &up_one.left(1);

        let mut capture_requirements = board.state.all_pieces;
        if let Some(last_move) = board.state.history.last() {
            let conditions = last_move.action.piece_type == 0
                && (last_move.action.to.abs_diff(last_move.action.from) == (2 * (cols)));

            if conditions {
                capture_requirements |= &up(
                    &BitBoard::from_lsb(last_move.action.from),
                    1,
                    cols,
                    board.get_next_team(team),
                );
            }
        }

        captures &= &capture_requirements;

        moves |= &captures;

        moves
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

        let mut history_move = HistoryMove {
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

        board.state.teams[captured_color] ^= &to;
        board.state.teams[color] ^= &from;
        board.state.teams[color] |= &to;

        board.state.pieces[captured_piece_type] ^= &to;
        board.state.pieces[piece_type] ^= &from;
        if action.info < 2 {
            board.state.pieces[piece_type] |= &to;
        } else {
            let promotion_piece_type = action.info - 2;
            if let Some(state) = &mut history_move.state {
                state.pieces.push(IndexedPreviousBoard(
                    piece_type,
                    board.state.teams[promotion_piece_type],
                ));
            }
            board.state.pieces[promotion_piece_type] |= &to;
        }

        board.state.all_pieces ^= &from;

        board.state.first_move &= &!from;
        board.state.first_move &= &!to;
        // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.

        board.state.history.push(history_move);
    }

    fn make_normal_move(&self, board: &mut Board, action: &Action, from: BitBoard, to: BitBoard) {
        if action.info == EN_PASSANT_MOVE {
            let cols = board.state.cols;

            let color: usize = if (from & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let piece_type = self.get_piece_type();
            let en_passant_target = down(&to, 1, cols, color as u32);

            let en_passant_target_color: usize =
                if (en_passant_target & &board.state.teams[0]).is_set() {
                    0
                } else {
                    1
                };

            let history_move = HistoryMove {
                action: *action,
                state: Some(HistoryState {
                    teams: vec![
                        IndexedPreviousBoard(color, board.state.teams[color]),
                        IndexedPreviousBoard(
                            en_passant_target_color,
                            board.state.teams[en_passant_target_color],
                        ),
                    ],
                    pieces: vec![IndexedPreviousBoard(
                        piece_type,
                        board.state.pieces[piece_type],
                    )],
                    all_pieces: PreviousBoard(board.state.all_pieces),
                    first_move: PreviousBoard(board.state.first_move),
                })
            };

            board.state.teams[color] ^= &from;
            board.state.teams[color] |= &en_passant_target;
            board.state.teams[en_passant_target_color] ^= &en_passant_target;

            board.state.pieces[piece_type] ^= &from;

            board.state.all_pieces ^= &from;

            board.state.first_move &= &!from;
            board.state.first_move &= &!en_passant_target;

            board.state.history.push(history_move);
            return;
        }

        let color: usize = if (from & &board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let piece_type = self.get_piece_type();

        let mut history_move = HistoryMove {
            action: *action,
            state: Some(HistoryState {
                teams: vec![IndexedPreviousBoard(color, board.state.teams[color])],
                pieces: vec![IndexedPreviousBoard(
                    piece_type,
                    board.state.pieces[piece_type],
                )],
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
            })
        };

        board.state.teams[color] ^= &from;
        board.state.teams[color] |= &to;

        board.state.pieces[piece_type] ^= &from;
        if action.info < 1 {
            board.state.pieces[piece_type] |= &to;
        } else {
            let promotion_piece_type = action.info - 1;
            if let Some(state) = &mut history_move.state {
                state.pieces.push(IndexedPreviousBoard(
                    piece_type,
                    board.state.teams[promotion_piece_type],
                ));
            }
            board.state.pieces[promotion_piece_type] |= &to;
        }

        board.state.all_pieces ^= &from;
        board.state.all_pieces |= &to;

        board.state.first_move &= &!from;

        board.state.history.push(history_move);
    }

    fn add_actions(&self, actions: &mut Vec<Action>, board: &Board, from: u32, team: u32) {
        let piece_type = self.get_piece_type();
        let promotion_rows = board.state.edges[0].bottom | &board.state.edges[0].top;

        let from_board = BitBoard::from_lsb(from);
        let bit_actions =
            self.get_moves(board, from_board, team) & &!board.state.teams[team as usize];

        if bit_actions.is_empty() {
            return;
        }

        let rows = board.state.rows;
        let cols = board.state.cols;

        let piece_types = board.game.pieces.len();

        for bit in bit_actions.iter_one_bits((rows * cols) as u32) {
            if (BitBoard::from_lsb(bit) & &promotion_rows).is_set() {
                for promotion_piece_type in 0..piece_types {
                    actions.push(Action {
                        from,
                        to: bit,
                        info: promotion_move(promotion_piece_type),
                        piece_type,
                    });
                }
            } else {
                let mut en_passant = false;
                if let Some(last_move) = board.state.history.last() {
                    let conditions = last_move.action.piece_type == 0
                        && (last_move.action.to.abs_diff(last_move.action.from) == (2 * (cols)))
                        && (last_move.action.to.abs_diff(bit) == (cols));

                    if conditions {
                        en_passant = true;
                    }
                }

                actions.push(Action {
                    from,
                    to: bit,
                    info: if en_passant {
                        EN_PASSANT_MOVE
                    } else {
                        NORMAL_PAWN_MOVE
                    },
                    piece_type,
                });
            }
        }
    }
}
