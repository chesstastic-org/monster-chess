use crate::{BitBoard, Board, PieceType};

pub trait Piece {
    fn get_piece_type(&self) -> PieceType;

    fn can_lookup(&self) -> bool;
    fn get_attack_lookup<'a>(&self, board: &'a Board, from: BitBoard) -> Option<&'a BitBoard> {
        match board.attack_lookup.get(self.get_piece_type()) {
            None => None,
            Some(lookup) => {
                lookup.get(from.bitscan_forward() as usize)
            }
        }
    }

    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard;
    fn generate_moves(&self, board: &Board, from: BitBoard) -> BitBoard;
}