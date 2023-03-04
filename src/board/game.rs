use crate::{Piece, Board, Action};

pub trait MoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool;
    fn duplicate(&self) -> Box<dyn MoveRestrictions>;
}

pub struct Game {
    pub pieces: Vec<Box<dyn Piece>>,
    pub move_restrictions: Box<dyn MoveRestrictions>
}