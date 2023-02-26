use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges};

pub struct KnightPiece {
    pub piece_type: PieceType
}

fn noNoEa(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard { (b << 17) & &!edges.left & &!deep_edges.top }
fn noEaEa(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b << 10) & &!deep_edges.left & &!edges.top }
fn soEaEa(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >>  6) & &!deep_edges.left & &!edges.bottom }
fn soSoEa(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >> 15) & &!edges.left & &!deep_edges.bottom }
fn noNoWe(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b << 15) & &!edges.right & &!deep_edges.top  }
fn noWeWe(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b <<  6) & &!deep_edges.right & &!edges.top }
fn soWeWe(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >> 10) & &!deep_edges.right & &!edges.bottom }
fn soSoWe(b: BitBoard, edges: &Edges, deep_edges: &Edges) -> BitBoard  { (b >> 17) & &!edges.right & &!deep_edges.bottom  }

impl Piece for KnightPiece {
    fn generate_moves(&self, board: &Board, from: BitBoard) -> BitBoard {
        let edges = &board.state.edges[0];
        let deep_edges = &board.state.edges[1];
        
        let mut moves = noNoEa(from, edges, deep_edges);
        moves |= &noEaEa(from, edges, deep_edges);
        moves |= &soEaEa(from, edges, deep_edges);
        moves |= &soSoEa(from, edges, deep_edges);
        moves |= &noNoWe(from, edges, deep_edges);
        moves |= &noWeWe(from, edges, deep_edges);
        moves |= &soWeWe(from, edges, deep_edges);
        moves |= &soSoWe(from, edges, deep_edges);

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