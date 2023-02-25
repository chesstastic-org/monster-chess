use crate::{BitBoard, Rows, Cols};

pub type EdgeBuffer = u128;

pub struct Edges {
    pub top: BitBoard,
    pub bottom: BitBoard,
    pub right: BitBoard,
    pub left: BitBoard,
    pub all: BitBoard
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

pub fn generate_edge_list(rows: Rows, cols: Cols) -> Vec<Edges> {
    let mut max_edge = rows;
    if cols < max_edge {
        max_edge = cols;
    }

    max_edge /= 2;

    let mut edges = Vec::<Edges>::with_capacity(max_edge as usize);

    for buffer in 1..(max_edge + 1) {
        edges.push(generate_edges(buffer, rows, cols));
    }

    edges
}