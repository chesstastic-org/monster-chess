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
            actions
        }
    }
}
