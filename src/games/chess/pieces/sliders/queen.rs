use crate::board::{pieces::{PieceSymbol, Piece}, PieceType, BitBoard, Board, AttackDirections};

use super::{get_moves_ray, get_ray_attacks, can_ray_attack};

pub struct QueenPiece;

impl Piece for QueenPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self)
    }

    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('q')
    }

    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        let edges = &board.state.edges[0];
        let cols = board.state.cols;
        vec![
            get_moves_ray(
                from,
                |b| b.left(1),
                |b| b.is_empty() || (b & edges.left).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.right(1),
                |b| b.is_empty() || (b & edges.right).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.up(1, cols),
                |b| b.is_empty() || (b & edges.top).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.down(1, cols),
                |b| b.is_empty() || (b & edges.bottom).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.left(1).up(1, cols),
                |b| b.is_empty() || (b & (edges.left | edges.top)).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.left(1).down(1, cols),
                |b| b.is_empty() || (b & (edges.left | edges.bottom)).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.right(1).up(1, cols),
                |b| b.is_empty() || (b & (edges.right | edges.top)).is_set(),
            ),
            get_moves_ray(
                from,
                |b| b.right(1).down(1, cols),
                |b| b.is_empty() || (b & (edges.right | edges.bottom)).is_set(),
            ),
        ]
    }

    fn can_lookup(&self) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn can_move(&self, board: &Board, from: BitBoard, piece_type: usize, team: u32, mode: u32, to: BitBoard) -> BitBoard {
        let lookup = self
            .get_attack_lookup(board, piece_type)
            .expect("Could not find the queen attack lookup.");
        let mut attacks = BitBoard::new();

        for dir in 0..8 {
            attacks |= can_ray_attack(board, from, dir, &lookup, to);
        }

        attacks
    }

    #[allow(unused_variables)]
    fn get_moves(&self, board: &Board, from: BitBoard, piece_type: usize, team: u32, mode: u32) -> BitBoard {
        let lookup = self
            .get_attack_lookup(board, piece_type)
            .expect("Could not find the queen attack lookup.");
        let mut attacks = BitBoard::new();

        for dir in 0..8 {
            attacks |= get_ray_attacks(board, from, dir, &lookup);
        }

        attacks
    }
}
