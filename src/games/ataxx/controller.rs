use std::collections::HashSet;

use crate::{board::{game::{MoveController, NORMAL_MODE}, Board, actions::Action}, bitboard::BitBoard};

use super::is_single_move;

#[derive(Debug)]
pub struct AtaxxMoveController;
impl<const T: usize> MoveController<T> for AtaxxMoveController {
    fn is_legal(&self, board: &mut Board<T>, action: &Option<Action>) -> bool {
        return true;
    }

    fn use_psuedolegal(&self) -> bool {
        return false;
    }

    fn transform_moves(&self, board: &mut Board<T>, mode: u32, actions: Vec<Option<Action>>) -> Vec<Option<Action>> {
        // No Legal Moves
        if actions.len() == 0 {
            let board_mask = BitBoard::starting_at_lsb(0, 49);
            let filled_squares = board.state.pieces[0];
            let empty_squares = board_mask & !filled_squares;

            let team_squares = board.state.teams[board.state.moving_team as usize];

            board.make_move(&None);
            let opposing_moves = board.generate_moves(NORMAL_MODE).len();
            board.undo_move();
            if opposing_moves > 0 && empty_squares.count_ones() > 0 && team_squares.count_ones() > 0  {
                vec![ None ]
            } else {
                actions
            }
            
        } else {
            let mut set = HashSet::<u32>::with_capacity(actions.len());
            let mut new_actions = Vec::with_capacity(actions.len());

            for action in actions {
                if let Some(action) = action {
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

    fn encode_action(&self, board: &Board<T>, action: &Option<Action>) -> Vec<String> {
        vec![
            match action {
                Some(action) => {
                    if is_single_move(&action) {
                        format!(
                            "{}{}",
                            board.encode_position(action.to),
                            board.game.pieces[action.piece_type].format_info(board, action.info)
                        )
                    } else {
                        if let Some(from) = action.from {
                            format!(
                                "{}{}{}",
                                board.encode_position(from),
                                board.encode_position(action.to),
                                board.game.pieces[action.piece_type].format_info(board, action.info)
                            )
                        } else {
                            "----".to_string()
                        }
                    }
                },
                None => "0000".to_string()
            }   
        ]
    }
}
