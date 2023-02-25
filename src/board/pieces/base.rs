use crate::{BitBoard, Board, PieceType};

pub trait Piece {
    fn get_board(&self) -> &Board;
    fn get_piece_type(&self) -> PieceType;

    fn can_lookup(&self) -> bool;
    fn get_attack_lookup(&self) -> Option<&Vec<BitBoard>> {
        self.get_board().attack_lookup.get(self.get_piece_type())
    }

    fn get_moves(&self, from: BitBoard) -> BitBoard;
    
    fn generate_moves(&self, from: BitBoard) -> BitBoard;
}