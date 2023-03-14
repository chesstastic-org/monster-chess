use crate::board::Cols;

use super::BitBoard;

impl<const T: usize> BitBoard<T> {
    pub fn up(&self, shift: u32, cols: Cols) -> BitBoard<T> {
        *self >> shift * (cols)
    }

    pub fn down(&self, shift: u32, cols: Cols) -> BitBoard<T> {
        *self << shift * (cols)
    }

    pub fn right(&self, shift: u32) -> BitBoard<T> {
        *self << shift
    }

    pub fn left(&self, shift: u32) -> BitBoard<T> {
        *self >> shift
    }

    pub fn up_mut(&mut self, shift: u32, cols: Cols) {
        *self >>= shift * (cols);
    }

    pub fn down_mut(&mut self, shift: u32, cols: Cols) {
        *self <<= shift * (cols);
    }

    pub fn right_mut(&mut self, shift: u32) {
        *self <<= shift;
    }

    pub fn left_mut(&mut self, shift: u32) {
        *self >>= shift;
    }
}
