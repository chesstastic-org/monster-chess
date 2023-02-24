use crate::{BitBoard, Board, PieceType};

pub trait Piece {
    fn get_board(&self) -> Board;
    fn get_piece_type(&self) -> PieceType;

    fn get_attack_lookup(&self) -> &Option<Vec<BitBoard>>;
    fn get_moves(&self, from: BitBoard) -> BitBoard;
    
    fn generate_moves(&self, from: BitBoard) -> BitBoard;
}