use crate::{BitBoard, Rows, Cols};

pub type EdgeBuffer = u128;

pub struct Edges {
    top: BitBoard,
    bottom: BitBoard,
    right: BitBoard,
    left: BitBoard,
    all: BitBoard
}

pub fn generate_edges(buffer: EdgeBuffer, rows: Rows, cols: Cols) -> Edges {
    let top = !(BitBoard::max() << (buffer * cols));
    let bottom = BitBoard::max() << ((rows - buffer) * cols);

    let mut left = BitBoard::max() & &(!(BitBoard::max() << buffer));
    for _ in 1..rows {
        left |= &(left << cols);
    }

    let right = left << (cols - buffer);

    let edges = top | &bottom | &left | &right;

    Edges {
        top,
        bottom,
        right,
        left,
        all: edges
    }
}