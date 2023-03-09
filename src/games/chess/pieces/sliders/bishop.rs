use crate::{
    get_moves_ray, get_ray_attacks, AttackDirections, BitBoard, Board, Piece, PieceSymbol,
    PieceType,
};

pub struct BishopPiece {
    pub piece_type: PieceType,
}

impl Piece for BishopPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self {
            piece_type: self.piece_type,
        })
    }

    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('b')
    }

    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        let edges = &board.state.edges[0];
        let cols = board.state.cols;
        vec![
            get_moves_ray(
                from,
                |b| b.left(1).up(1, cols),
                |b| b.is_empty() || (b & &(edges.left | &edges.top)).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.left(1).down(1, cols),
                |b| b.is_empty() || (b & &(edges.left | &edges.bottom)).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.right(1).up(1, cols),
                |b| b.is_empty() || (b & &(edges.right | &edges.top)).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.right(1).down(1, cols),
                |b| b.is_empty() || (b & &(edges.right | &edges.bottom)).is_set(),
            ),
        ]
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    #[allow(unused_variables)]
    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let lookup = self
            .get_attack_lookup(board)
            .expect("Could not find the bishop attack lookup.");
        let mut attacks = BitBoard::new();

        for dir in 0..4 {
            attacks |= &get_ray_attacks(board, from, dir, &lookup);
        }

        attacks
    }
}
