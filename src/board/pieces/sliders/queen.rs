use crate::{Piece, BitBoard, PieceType, Board, AttackDirections, get_moves_ray, get_ray_attacks};

pub struct QueenPiece {
    pub piece_type: PieceType
}

impl Piece for QueenPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self { piece_type: self.piece_type })
    }


    fn get_piece_symbol(&self) -> char {
        'q'
    }

    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        let edges = &board.state.edges[0];
        let cols = board.state.cols;
        vec![
            get_moves_ray(from, |b| { b.left(1) }, |b| b.is_empty() || (b & &edges.left).is_set()),
            get_moves_ray(from, |b| { b.right(1) }, |b| b.is_empty() || (b & &edges.right).is_set()),
            get_moves_ray(from, |b| { b.up(1, cols) }, |b| b.is_empty() || (b & &edges.top).is_set()),
            get_moves_ray(from, |b| { b.down(1, cols) }, |b| b.is_empty() || (b & &edges.bottom).is_set()),
            get_moves_ray(from, |b| { b.left(1).up(1, cols) }, |b| b.is_empty() || (b & &(edges.left | &edges.top)).is_set()),
            get_moves_ray(from, |b| { b.left(1).down(1, cols) }, |b| b.is_empty() || (b & &(edges.left | &edges.bottom)).is_set()),
            get_moves_ray(from, |b| { b.right(1).up(1, cols) }, |b| b.is_empty() || (b & &(edges.right | &edges.top)).is_set()),
            get_moves_ray(from, |b| { b.right(1).down(1, cols) }, |b| b.is_empty() || (b & &(edges.right | &edges.bottom)).is_set())
        ]
    }   

    fn can_lookup(&self) -> bool {
        true
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let lookup = self.get_attack_lookup(board, from).unwrap();
        let mut attacks = BitBoard::new();

        let reverse_buffer = 128 - (board.state.rows + board.state.cols);

        for dir in 0..8 {
            attacks |= &get_ray_attacks(board, from, dir, &lookup, reverse_buffer);
        }

        attacks
    }
}