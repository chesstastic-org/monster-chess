use crate::{BitSet, Cols};

impl<const T: usize> BitSet<T> {
    pub fn up(&self, shift: u128, cols: Cols) -> BitSet<T> {
        *self >> shift * cols
    }

    pub fn down(&self, shift: u128, cols: Cols) -> BitSet<T> {
        *self << shift * cols
    }

    pub fn right(&self, shift: u128) -> BitSet<T> {
        *self << shift
    }

    pub fn left(&self, shift: u128) -> BitSet<T> {
        *self >> shift
    }

    pub fn up_mut(&mut self, shift: u128, cols: Cols) {
        *self >>= shift * cols;
    }

    pub fn down_mut(&mut self, shift: u128, cols: Cols) {
        *self <<= shift * cols;
    }

    pub fn right_mut(&mut self, shift: u128) {
        *self <<= shift;
    }

    pub fn left_mut(&mut self, shift: u128) {
        *self >>= shift;
    }
}