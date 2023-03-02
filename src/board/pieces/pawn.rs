use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges, Cols, AttackDirections};

pub struct PawnPiece {
    pub piece_type: PieceType
}

pub fn up(bitboard: &BitBoard, shift: u32, cols: Cols, team: u32) -> BitBoard {
    match team {
        0 => bitboard.up(shift, cols),
        1 => bitboard.down(shift, cols),
        _ => bitboard.up(shift, cols)
    }
}

impl Piece for PawnPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self { piece_type: self.piece_type })
    }

    fn can_lookup(&self) -> bool {
        false
    }

    fn get_piece_symbol(&self) -> char {
        'p'
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let mut moves = BitBoard::new();
        let cols = board.state.cols;

        let single_moves = up(&from, 1, cols, team) & &!board.state.blockers;
        let first_move = (from & &board.state.first_move).is_set();

        moves |= &single_moves;

        if first_move {
            let double_moves = up(&single_moves, 1, cols, team) & &!board.state.blockers;
            moves |= &double_moves;
        }


        let up_one = from.up(1, cols);
        let mut captures = up_one.right(1);
        captures |= &up_one.left(1);
        captures &= &board.state.blockers;

        moves |= &captures;

        moves
    }
}