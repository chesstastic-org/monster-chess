use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges, Cols, AttackDirections};

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
    fn generate_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut moves = left_one(from, edges) | &right_one(from, edges);
        from |= &moves;
        moves |= &up_one(from, cols, edges);
        moves |= &down_one(from, cols, edges);
        vec![ moves ]
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