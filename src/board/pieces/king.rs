use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges};

pub struct KingPiece {
    pub piece_type: PieceType
}

fn left_one(from: BitBoard, edges: &Edges) -> BitBoard {
    (from << 1) & &(!(edges.right))
}

fn right_one(from: BitBoard, edges: &Edges) -> BitBoard {
    (from >> 1) & &(!(edges.left))
}

fn up_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    from >> cols & &(!(edges.bottom))
}

fn down_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    from << cols & &(!(edges.top))
}

impl Piece for KingPiece {
    fn generate_moves(&self, board: &Board, mut from: BitBoard) -> BitBoard {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut moves = left_one(from, edges) | &right_one(from, edges);
        from |= &moves;
        moves |= &up_one(from, cols, edges);
        moves |= &down_one(from, cols, edges);
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