use crate::{BitSet, Cols};

impl<const T: usize> BitSet<T> {
    pub fn up(&self, shift: u32, cols: Cols) -> BitSet<T> {
        *self >> shift * (cols as u32)
    }

    pub fn down(&self, shift: u32, cols: Cols) -> BitSet<T> {
        *self << shift * (cols as u32)
    }

    pub fn right(&self, shift: u32) -> BitSet<T> {
        *self << shift
    }

    pub fn left(&self, shift: u32) -> BitSet<T> {
        *self >> shift
    }

    pub fn up_mut(&mut self, shift: u32, cols: Cols) {
        *self >>= shift * (cols as u32);
    }

    pub fn down_mut(&mut self, shift: u32, cols: Cols) {
        *self <<= shift * (cols as u32);
    }

    pub fn right_mut(&mut self, shift: u32) {
        *self <<= shift;
    }

    pub fn left_mut(&mut self, shift: u32) {
        *self >>= shift;
    }
}
