use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges, Cols, AttackDirections};

pub struct RookPiece {
    pub piece_type: PieceType
}

impl Piece for RookPiece {
    fn generate_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        vec![]
    }   

    fn can_lookup(&self) -> bool {
        true
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard {
        let lookup = self.get_attack_lookup(board, from);
        match lookup {
            Some(lookup) => lookup[0],
            None => self.generate_moves(board, from)[0]
        }
    }
}