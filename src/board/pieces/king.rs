use crate::{Piece, BitBoard, PieceType, Board, Rows, Edges, Cols, AttackDirections};

pub struct KingPiece {
    pub piece_type: PieceType
}

fn right_one(from: BitBoard, edges: &Edges) -> BitBoard {
    from.right(1) & &!edges.right
}

fn left_one(from: BitBoard, edges: &Edges) -> BitBoard {
    from.left(1) & &!edges.left
}

fn up_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    from.up(1, cols) & &!edges.bottom
}

fn down_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    from.down(1, cols) & &!edges.top
}

impl Piece for KingPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self { piece_type: self.piece_type })
    }


    fn get_piece_symbol(&self) -> char {
        'k'
    }

    fn generate_lookup_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut moves = right_one(from, edges) | &left_one(from, edges);
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
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_lookup_moves(board, from)[0]
        }
    }
}