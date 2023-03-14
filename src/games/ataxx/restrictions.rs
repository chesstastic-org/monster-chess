use crate::{board::{game::MoveRestrictions, Board, actions::Action}, bitboard::BitBoard};

pub struct AtaxxMoveRestrictions;
impl<const T: usize> MoveRestrictions<T> for AtaxxMoveRestrictions {
    fn is_legal(&self, board: &mut Board<T>, action: &Action) -> bool {
        true
    }
}
