use std::collections::HashSet;

use crate::{board::{game::MoveController, Board, actions::Action}, bitboard::BitBoard};

pub struct AtaxxMoveController;
impl<const T: usize> MoveController<T> for AtaxxMoveController {
    fn transform_moves(&self, board: &mut Board<T>, mode: u32, actions: Vec<Option<Action>>) -> Vec<Option<Action>> {
        // No Legal Moves
        if actions.len() == 0 {
            let board_mask = BitBoard::starting_at_lsb(0, 49);
            let filled_squares = board.state.pieces[0];
            let empty_squares = board_mask & !filled_squares;

            if empty_squares.count_ones() > 0 && filled_squares.count_ones() > 0 {
                vec![ None ]
            } else {
                actions
            }
        } else {
            let mut set = HashSet::<u32>::with_capacity(actions.len());
            let mut new_actions = Vec::with_capacity(actions.len());

            for action in actions {
                if let Some(action) = action {
                    let dif = action.from.abs_diff(action.to);
                    if dif == 1 || dif == 7 || dif == 6 || dif == 8 {
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
}
