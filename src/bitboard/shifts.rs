use crate::board::Cols;

use super::BitBoard;

impl<const T: usize> BitBoard<T> {
    /// Shifts the `BitBoard` upwards by a given amount.
    pub fn up(&self, shift: u16, cols: Cols) -> BitBoard<T> {
        *self >> shift * (cols)
    }

    /// Shifts the `BitBoard` downwards by a given amount.
    pub fn down(&self, shift: u16, cols: Cols) -> BitBoard<T> {
        *self << shift * (cols)
    }

    /// Shifts the `BitBoard` right by a given amount.
    pub fn right(&self, shift: u16) -> BitBoard<T> {
        *self << shift
    }

    /// Shifts the `BitBoard` left by a given amount.
    pub fn left(&self, shift: u16) -> BitBoard<T> {
        *self >> shift
    }

    /// Shifts the given `BitBoard` upwards by a given amount, mutating it.
    pub fn up_mut(&mut self, shift: u16, cols: Cols) {
        *self >>= shift * (cols);
    }

    /// Shifts the given `BitBoard` upwards by a given amount, mutating it.
    pub fn down_mut(&mut self, shift: u16, cols: Cols) {
        *self <<= shift * (cols);
    }

    /// Shifts the given `BitBoard` right by a given amount, mutating it.
    pub fn right_mut(&mut self, shift: u16) {
        *self <<= shift;
    }

    /// Shifts the given `BitBoard` left by a given amount, mutating it.
    pub fn left_mut(&mut self, shift: u16) {
        *self >>= shift;
    }
}
