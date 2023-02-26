use crate::{Piece, BitBoard, PieceType, Board, Edges, Cols};

pub struct KnightPiece {
    pub piece_type: PieceType
}

fn north_north_east(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard { (b << (2 * cols + 1)) & &!edges.left & &!deep_edges.top }
fn north_east_east(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b << (cols + 2)) & &!deep_edges.left & &!edges.top }
fn south_east_east(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >>  (cols - 2)) & &!deep_edges.left & &!edges.bottom }
fn south_south_east(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >> (2 * cols - 1)) & &!edges.left & &!deep_edges.bottom }
fn north_north_west(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b << (2 * cols - 1)) & &!edges.right & &!deep_edges.top  }
fn north_west_west(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b <<  (cols - 2)) & &!deep_edges.right & &!edges.top }
fn south_west_west(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >> (cols + 2)) & &!deep_edges.right & &!edges.bottom }
fn south_south_west(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >> (2 * cols + 1)) & &!edges.right & &!deep_edges.bottom  }

impl Piece for KnightPiece {
    fn generate_moves(&self, board: &Board, from: BitBoard) -> BitBoard {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let deep_edges = &board.state.edges[1];
        
        let mut moves = north_north_east(from, cols, edges, deep_edges);
        moves |= &north_east_east(from, cols, edges, deep_edges);
        moves |= &south_east_east(from, cols, edges, deep_edges);
        moves |= &south_south_east(from, cols, edges, deep_edges);
        moves |= &north_north_west(from, cols, edges, deep_edges);
        moves |= &north_west_west(from, cols, edges, deep_edges);
        moves |= &south_west_west(from, cols, edges, deep_edges);
        moves |= &south_south_west(from, cols, edges, deep_edges);

        moves
    }   

    fn can_lookup(&self) -> bool {
        true
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard {
        *self.get_attack_lookup(board, from).unwrap_or(&self.generate_moves(board, from))
    }
}