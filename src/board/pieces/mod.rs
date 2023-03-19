use std::fmt::Debug;

use crate::bitboard::BitBoard;

use super::{
    actions::{
        Action, HistoryMove, HistoryState, HistoryUpdate, IndexedPreviousBoard, PreviousBoard,
        UndoMoveError, Move, ActionInfo,
    },
    game::Game,
    AttackDirections, AttackLookup, Board, BoardState, Cols, PieceType, Rows, update_turns, reverse_turns,
};

pub enum PieceSymbol {
    Char(char),
    TeamSymbol(Vec<char>),
}

const NORMAL_MOVE: u16 = 0;

pub trait Piece<const T: usize> : Debug + Send + Sync {
    fn get_piece_symbol(&self) -> PieceSymbol;

    fn format_info(&self, board: &Board<T>, info: ActionInfo) -> String {
        "".to_string()
    }
    fn parse_info(&self, board: &Board<T>, info: String) -> u32 {
        0
    }

    fn can_lookup(&self) -> bool;

    fn get_attack_lookup<'a>(
        &self,
        board: &'a Board<T>,
        piece_type: PieceType,
    ) -> Option<&'a AttackLookup<T>> {
        board.attack_lookup.get(piece_type as usize)
    }

    fn get_moves(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        piece_type: PieceType,
        team: u16,
        mode: u16,
    ) -> BitBoard<T>;
    fn can_move_mask(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        from_bit: u16,
        piece_type: PieceType,
        team: u16,
        mode: u16,
        to: BitBoard<T>,
    ) -> BitBoard<T> {
        self.get_moves(board, from, piece_type, team, mode)
    }

    #[allow(unused_variables)]
    fn generate_lookup_moves(&self, board: &Board<T>, from: BitBoard<T>) -> AttackDirections<T> {
        Vec::new()
    }

    fn make_capture_move(
        &self,
        board: &mut Board<T>,
        action: &Action,
        piece_type: PieceType,
        from: BitBoard<T>,
        to: BitBoard<T>,
    ) -> Option<HistoryMove<T>> {
        let color = action.team as usize;
        let piece_type = piece_type as usize;
        let captured_color: usize = if (to & board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let mut captured_piece_type: PieceType = 0;
        for i in 0..(board.game.pieces.len()) {
            if (board.state.pieces[i] & to).is_set() {
                captured_piece_type = i as u16;
                break;
            }
        }
        
        

        let history_move = HistoryMove {
            action: Move::Action(*action),
            first_history_move: board.retrieve_first_history_move(Move::Action(*action)),
            turn_info: board.get_turn_info(),
            state: HistoryState::Any {
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
                updates: vec![
                    HistoryUpdate::Team(IndexedPreviousBoard(color, board.state.teams[color])),
                    HistoryUpdate::Team(IndexedPreviousBoard(
                        captured_color,
                        board.state.teams[captured_color],
                    )),
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        piece_type,
                        board.state.pieces[piece_type],
                    )),
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        captured_piece_type as usize,
                        board.state.pieces[captured_piece_type as usize],
                    )),
                ],
            }
        };

        board.state.teams[captured_color] ^= to;
        board.state.teams[color] ^= from;
        board.state.teams[color] |= to;

        board.state.pieces[captured_piece_type as usize] ^= to;
        board.state.pieces[piece_type] ^= from;
        board.state.pieces[piece_type] |= to;

        board.state.all_pieces ^= from;

        board.state.first_move &= !from;
        board.state.first_move &= !to;
        // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.

        Some(history_move)
    }

    fn make_normal_move(
        &self,
        board: &mut Board<T>,
        action: &Action,
        piece_type: PieceType,
        from: BitBoard<T>,
        to: BitBoard<T>,
    ) -> Option<HistoryMove<T>> {
        let color = action.team as usize;
        let piece_type = piece_type as usize;

        let history_move = HistoryMove {
            action: Move::Action(*action),
            first_history_move: board.retrieve_first_history_move(Move::Action(*action)),
            turn_info: board.get_turn_info(),
            state: HistoryState::Single {
                team: IndexedPreviousBoard(color, board.state.teams[color]),
                piece: IndexedPreviousBoard(piece_type, board.state.pieces[piece_type]),
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
            },
        };

        board.state.teams[color] ^= from;
        board.state.teams[color] |= to;

        board.state.pieces[piece_type] ^= from;
        board.state.pieces[piece_type] |= to;

        board.state.all_pieces ^= from;
        board.state.all_pieces |= to;

        board.state.first_move &= !from;

        Some(history_move)
    }

    fn make_move(&self, board: &mut Board<T>, action: &Action) -> Option<HistoryMove<T>> {
        if let Some(from) = action.from {
            let from = BitBoard::from_lsb(from);
            let to = BitBoard::from_lsb(action.to);

            let history_move = if (board.state.all_pieces & to).is_empty() {
                self.make_normal_move(board, action, action.piece_type, from, to)
            } else {
                self.make_capture_move(board, action, action.piece_type, from, to)
            };

            update_turns(&mut board.state, &board.game, &Move::Action(*action));

            history_move
        } else {
            None
        }
    }

    fn undo_move(&self, state: &mut BoardState<T>, game: &Game<T>, history_move: &HistoryMove<T>) {
        reverse_turns(state, game, &history_move);

        match &history_move.state {
            HistoryState::Single {
                all_pieces,
                first_move,
                team,
                piece,
            } => {
                state.all_pieces = all_pieces.0;
                state.first_move = first_move.0;
                state.teams[team.0] = team.1;
                state.pieces[piece.0] = piece.1;
            }
            HistoryState::Any {
                first_move,
                all_pieces,
                updates,
            } => {
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
        actions: &mut Vec<Move>,
        board: &Board<T>,
        piece_type: PieceType,
        from: u16,
        team: u16,
        mode: u16,
    ) {
        let from_board = BitBoard::from_lsb(from);

        let bit_actions = self.get_moves(board, from_board, piece_type, team, mode)
            & !board.state.teams[team as usize] & !board.state.gaps;

        if bit_actions.is_empty() {
            return;
        }

        for bit in bit_actions.iter_set_bits(board.state.squares) {
            actions.push(Move::Action(Action {
                from: Some(from),
                to: bit,
                team,
                info: NORMAL_MOVE,
                move_type: NORMAL_MOVE,
                piece_type,
            }));
        }
    }
}
