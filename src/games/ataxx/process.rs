use crate::{board::{fen::PostProcess, Board}};

#[derive(Debug)]
pub struct AtaxxPostProcess;
impl<const T: usize> PostProcess<T> for AtaxxPostProcess {
    fn apply(&self, _board: &mut Board<T>) {}
}