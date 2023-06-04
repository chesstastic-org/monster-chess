use crate::board::{Rows, Cols};
use std::cmp::Ordering;

use super::BitBoard;

/// Generate `BitBoard`s for each rank of the board.
pub fn generate_ranks<const T: usize>(cols: Cols, rows: Rows) -> Vec<BitBoard<T>> {
    let mut ranks: Vec<BitBoard<T>> = Vec::with_capacity(rows as usize);
    let mut rank = BitBoard::<T>::starting_at_lsb(0, cols);
    ranks.push(rank);
    for _ in 1..rows {
        rank = rank.down(1, cols);
        ranks.push(rank);
    }

    ranks
}

/// Generate `BitBoard`s for each file of the board.
pub fn generate_files<const T: usize>(cols: Cols, rows: Rows) -> Vec<BitBoard<T>> {
    let mut files: Vec<BitBoard<T>> = Vec::with_capacity(rows as usize);
    let mut file = BitBoard::<T>::from_lsb(0);
    for _ in 1..rows {
        file |= file.down(1, cols);
    }

    files.push(file);
    for _ in 1..rows {
        file = file.right(1);
        files.push(file);
    }

    files
}

impl<const T: usize> BitBoard<T> {
    /// Flip the `BitBoard` vertically.
    pub fn flip_vertically(self, ranks: &[BitBoard<T>], cols: Cols, rows: Rows) -> BitBoard<T> {
        let mut new_board = BitBoard::<T>::new();
        let max_rank = rows - 1;

        for row in 0..rows {
            let previous_rank = ranks[row as usize] & self;
            let new_ind = max_rank - row;
            let diff = new_ind.abs_diff(row);

            new_board |= match new_ind.cmp(&row) {
                Ordering::Greater => {
                    previous_rank.down(diff, cols)
                }
                Ordering::Equal => {
                    previous_rank
                }
                Ordering::Less => {
                    previous_rank.up(diff, cols)
                }
            }
        }

        new_board
    }

    /// Flip the `BitBoard` horizontally.
    pub fn flip_horizontally(self, files: &[BitBoard<T>], cols: Cols) -> BitBoard<T> {
        let mut new_board = BitBoard::<T>::new();

        let max_file = cols - 1;

        for col in 0..cols {
            let previous_file = files[col as usize] & self;
            let new_ind = max_file - col;
            let diff = new_ind.abs_diff(col);

            new_board |= match new_ind.cmp(&col) {
                Ordering::Greater => {
                    previous_file.right(diff)
                }
                Ordering::Equal => {
                    previous_file
                }
                Ordering::Less => {
                    previous_file.left(diff)
                }
            }
        }

        new_board
    }
}