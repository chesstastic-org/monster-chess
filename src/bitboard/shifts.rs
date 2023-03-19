use crate::board::Cols;

use super::BitBoard;

impl<const T: usize> BitBoard<T> {
    pub fn up(&self, shift: u16, cols: Cols) -> BitBoard<T> {
        *self >> shift * (cols)
    }

    pub fn down(&self, shift: u16, cols: Cols) -> BitBoard<T> {
        *self << shift * (cols)
    }

    pub fn right(&self, shift: u16) -> BitBoard<T> {
        *self << shift
    }

    pub fn left(&self, shift: u16) -> BitBoard<T> {
        *self >> shift
    }

    pub fn up_mut(&mut self, shift: u16, cols: Cols) {
        *self >>= shift * (cols);
    }

    pub fn down_mut(&mut self, shift: u16, cols: Cols) {
        *self <<= shift * (cols);
    }

    pub fn right_mut(&mut self, shift: u16) {
        *self <<= shift;
    }

    pub fn left_mut(&mut self, shift: u16) {
        *self >>= shift;
    }
}
