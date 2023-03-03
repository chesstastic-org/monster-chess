use crate::Piece;

pub struct Game {
    pub pieces: Vec<Box<dyn Piece>>
}