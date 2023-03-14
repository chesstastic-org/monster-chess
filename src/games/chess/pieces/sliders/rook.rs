use crate::{board::{
    pieces::{Piece, PieceSymbol},
    AttackDirections, Board, PieceType,
}, bitboard::BitBoard};

use super::{get_moves_ray, get_ray_attacks};

pub struct RookPiece;

impl<const T: usize> Piece<T> for RookPiece {
    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('r')
    }

    fn generate_lookup_moves(&self, board: &Board<T>, from: BitBoard<T>) -> AttackDirections<T> {
        let edges = &board.state.edges[0];
        let cols = board.state.cols;
        let mut lookups = vec![
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
        ];
        let mut combined_lookup = BitBoard::new();
        for lookup in &lookups {
            combined_lookup |= *lookup;
        }
        lookups.push(combined_lookup);
        lookups
    }

    fn can_move_mask(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        from_bit: u32,
        piece_type: usize,
        team: u32,
        mode: u32,
        to: BitBoard<T>,
    ) -> BitBoard<T> {
        let lookup = self
            .get_attack_lookup(board, piece_type)
            .expect("Could not find the queen attack lookup.");

        let from_bit = from_bit as usize;
        if (lookup[from_bit][4] & to).is_empty() {
            return BitBoard::new();
        }

        for dir in 0..4 {
            let ray = get_ray_attacks(board, from, from_bit, dir, &lookup);
            if (ray & to).is_set() {
                return ray;
            }
        }

        return BitBoard::new();
    }

    fn can_lookup(&self) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn get_moves(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        piece_type: usize,
        team: u32,
        mode: u32,
    ) -> BitBoard<T> {
        let lookup = self
            .get_attack_lookup(board, piece_type)
            .expect("Could not find the queen attack lookup.");
        let mut attacks = BitBoard::new();

        let from_bit = from.bitscan_forward() as usize;
        for dir in 0..4 {
            attacks |= get_ray_attacks(board, from, from_bit, dir, &lookup);
        }

        attacks
    }
}
