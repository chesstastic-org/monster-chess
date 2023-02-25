use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges};

pub struct KingPiece<'a> {
    pub board: &'a Board,
    pub piece_type: PieceType
}

fn left_one(from: BitBoard, edges: &Edges) -> BitBoard {
    (from << 1) & &(!(edges.right))
}

fn right_one(from: BitBoard, edges: &Edges) -> BitBoard {
    (from >> 1) & &(!(edges.left))
}

fn up_one(from: BitBoard, rows: Rows, edges: &Edges) -> BitBoard {
    from >> rows & &(!(edges.bottom))
}

fn down_one(from: BitBoard, rows: Rows, edges: &Edges) -> BitBoard {
    from << rows & &(!(edges.top))
}

impl<'a> Piece for KingPiece<'a> {
    fn generate_moves(&self, mut from: BitBoard) -> BitBoard {
        let rows = self.get_board().state.rows;
        let edges = &self.get_board().state.edges[0];
        let mut moves = left_one(from, edges) | &right_one(from, edges);
        from |= &moves;
        moves |= &up_one(from, rows, edges);
        moves |= &down_one(from, rows, edges);
        moves
    }   

    fn get_attack_lookup(&self) -> Option<&Vec<BitBoard>> {
        None
    }

    fn get_board(&self) -> &Board {
        &self.board
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, from: BitBoard) -> BitBoard {
        self.generate_moves(from)
    }
}