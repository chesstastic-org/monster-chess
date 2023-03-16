use crate::{board::{fen::PostProcess, Board, actions::Action}, bitboard::BitBoard};

#[derive(Debug)]
pub struct AtaxxPostProcess;
impl<const T: usize> PostProcess<T> for AtaxxPostProcess {
    fn apply(&self, board: &mut Board<T>) {}
}