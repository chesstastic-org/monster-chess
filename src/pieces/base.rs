use crate::{BitBoard, Board, PieceType};

pub trait Piece {
    fn get_board(&self) -> Board;
    fn get_piece_type(&self) -> PieceType;

    fn get_attack_lookup(&self) -> &Vec<BitBoard>;
    fn get_moves(&self) -> BitBoard;
}