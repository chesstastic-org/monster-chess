use crate::bitboard::BitBoard;

use super::{
    actions::{Action, HistoryMove, UndoMoveError},
    game::Game,
    pieces::Piece,
    Board, Cols, Rows,
};

pub type EdgeBuffer = u32;

#[derive(Copy, Clone, Debug)]
pub struct Edges<const T: usize> {
    pub top: BitBoard<T>,
    pub bottom: BitBoard<T>,
    pub right: BitBoard<T>,
    pub left: BitBoard<T>,
    pub all: BitBoard<T>,
}

pub fn generate_edges<const T: usize>(buffer: EdgeBuffer, rows: Rows, cols: Cols) -> Edges<T> {
    let top = !(BitBoard::max() << (buffer * cols) as u32);
    let bottom = BitBoard::max() << ((rows - buffer) * cols) as u32;

    let mut left = BitBoard::max() & (!(BitBoard::max() << (buffer as u32)));
    for _ in 1..rows {
        left |= (left << (cols));
    }

    let right = left << (cols - buffer) as u32;

    let edges = top | bottom | left | right;

    Edges {
        top,
        bottom,
        right,
        left,
        all: edges,
    }
}

pub fn generate_edge_list<const T: usize>(rows: Rows, cols: Cols) -> Vec<Edges<T>> {
    let mut max_edge = rows;
    if cols < max_edge {
        max_edge = cols;
    }

    max_edge /= 2;

    let mut edges = Vec::with_capacity(max_edge as usize);

    for buffer in 1..(max_edge + 1) {
        edges.push(generate_edges(buffer, rows, cols));
    }

    edges
}
