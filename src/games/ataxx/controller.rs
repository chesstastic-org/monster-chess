use crate::{board::{game::MoveController, Board, actions::Action}, bitboard::BitBoard};

pub struct AtaxxMoveController;
impl<const T: usize> MoveController<T> for AtaxxMoveController {
    fn transform_moves(&self, board: &mut Board<T>, mode: u32, actions: Vec<Option<Action>>) -> Vec<Option<Action>> {
        actions
    }
}
