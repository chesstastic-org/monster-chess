use crate::{Piece, BitBoard, PieceType, Board, Rows};

pub struct KingPiece<'a> {
    pub board: &'a Board,
    pub piece_type: PieceType
}

fn left_one(from: BitBoard) -> BitBoard {
    from << 1
}

fn right_one(from: BitBoard) -> BitBoard {
    from >> 1
}

fn up_one(from: BitBoard, rows: Rows) -> BitBoard {
    from << rows
}

fn down_one(from: BitBoard, rows: Rows) -> BitBoard {
    from >> rows
}

impl<'a> Piece for KingPiece<'a> {
    fn generate_moves(&self, mut from: BitBoard) -> BitBoard {
        let rows = self.get_board().state.rows;
        let mut moves = left_one(from) | &right_one(from);
        from |= &moves;
        moves |= &up_one(from, rows);
        moves |= &down_one(from, rows);
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