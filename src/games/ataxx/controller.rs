use std::collections::HashSet;

use crate::{board::{game::{MoveController, NORMAL_MODE, get_theoretical_moves_bound, MoveLegalResponse}, Board, actions::{Move, TheoreticalMove, TurnUpdate, CounterUpdate}, BoardState}, bitboard::BitBoard};

use super::{is_single_move, pieces::STONE};

#[derive(Debug)]
pub struct AtaxxMoveController;
impl<const T: usize> MoveController<T> for AtaxxMoveController {
    fn is_legal(&self, _board: &mut Board<T>, _action: &Move, _make_moves: bool) -> MoveLegalResponse<T> {
        MoveLegalResponse {
            is_legal: true,
            made_move: None
        }
    }

    fn use_pseudolegal(&self) -> bool {
        return false;
    }

    fn transform_moves(&self, board: &mut Board<T>, _mode: u16, actions: Vec<Move>) -> Vec<Move> {
        // No Legal Moves
        if actions.len() == 0 {
            let board_mask = BitBoard::starting_at_lsb(0, 49);
            let filled_squares = board.state.pieces[STONE];
            let empty_squares = board_mask & !filled_squares;

            let team_squares = board.state.teams[board.state.moving_team as usize];

            let undo = board.make_move(&Move::Pass);
            let opposing_moves = board.generate_moves(NORMAL_MODE).len();
            board.undo_move(undo);
            if opposing_moves > 0 && empty_squares.count_ones() > 0 && team_squares.count_ones() > 0  {
                vec![ Move::Pass ]
            } else {
                actions
            }
        } else {
            let mut set = HashSet::<u16>::with_capacity(actions.len());
            let mut new_actions = Vec::with_capacity(actions.len());

            for action in actions {
                if let Move::Action(action) = action {
                    if is_single_move(&action) {
                        // Single Move

                        if set.contains(&action.to) {
                            continue;
                        }

                        set.insert(action.to);
                    }
                }

                new_actions.push(action);
            }

            new_actions
        }
    }

    fn encode_action(&self, board: &Board<T>, action: &Move) -> Vec<String> {
        vec![
            match action {
                Move::Action(action) => {
                    if is_single_move(&action) {
                        format!(
                            "{}{}",
                            board.encode_position(action.to),
                            board.game.pieces[action.piece_type as usize].format_info(board, action.info)
                        )
                    } else {
                        if let Some(from) = action.from {
                            format!(
                                "{}{}{}",
                                board.encode_position(from),
                                board.encode_position(action.to),
                                board.game.pieces[action.piece_type as usize].format_info(board, action.info)
                            )
                        } else {
                            "----".to_string()
                        }
                    }
                },
                Move::Pass => "0000".to_string()
            }   
        ]
    }

    fn update(&self, action: &Move, _state: &BoardState<T>) -> TurnUpdate {
        TurnUpdate {
            turns: CounterUpdate::Next,
            sub_moves: match action {
                Move::Action(action) => if is_single_move(action) {
                    CounterUpdate::To(0)
                } else {
                    CounterUpdate::Next
                },
                Move::Pass => CounterUpdate::Next
            },
            full_moves: CounterUpdate::Next
        }
    }

    fn get_theoretical_moves(&self, board: &Board<T>) -> Vec<TheoreticalMove> {
        get_theoretical_moves_bound(board, 0, true)
    }

    fn get_max_available_moves(&self) -> u32 {
        49 * 24
    }
}
