use crate::{AttackDirections, BitBoard, Board, Cols, Edges, Piece, PieceSymbol, PieceType};

pub struct KnightPiece {
    pub piece_type: PieceType,
}

fn up2_right(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.up(2, cols).right(1) & &!deep_edges.bottom & &!edges.left
}

fn up_right2(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.up(1, cols).right(2) & &!edges.bottom & &!deep_edges.left
}

fn down2_right(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.down(2, cols).right(1) & &!deep_edges.top & &!edges.left
}

fn down_right2(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.down(1, cols).right(2) & &!edges.top & &!deep_edges.left
}

fn up2_left(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.up(2, cols).left(1) & &!deep_edges.bottom & &!edges.right
}

fn up_left2(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.up(1, cols).left(2) & &!edges.bottom & &!deep_edges.right
}

fn down2_left(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.down(2, cols).left(1) & &!deep_edges.top & &!edges.right
}

fn down_left2(b: BitBoard, cols: Cols, edges: &Edges, deep_edges: &Edges) -> BitBoard {
    b.down(1, cols).left(2) & &!edges.top & &!deep_edges.right
}

impl Piece for KnightPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self {
            piece_type: self.piece_type,
        })
    }

    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('n')
    }

    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let deep_edges = &board.state.edges[1];

        let mut moves = up2_right(from, cols, edges, deep_edges);
        moves |= &up_right2(from, cols, edges, deep_edges);
        moves |= &down2_right(from, cols, edges, deep_edges);
        moves |= &down_right2(from, cols, edges, deep_edges);
        moves |= &up2_left(from, cols, edges, deep_edges);
        moves |= &up_left2(from, cols, edges, deep_edges);
        moves |= &down2_left(from, cols, edges, deep_edges);
        moves |= &down_left2(from, cols, edges, deep_edges);

        vec![moves]
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    #[allow(unused_variables)]
    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let lookup = self.get_attack_lookup(board, from);
        match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_lookup_moves(board, from)[0],
        }
    }
}
