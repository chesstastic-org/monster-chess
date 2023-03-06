use crate::{Action, Board, Piece};

pub trait MoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool;
    fn duplicate(&self) -> Box<dyn MoveRestrictions>;
}

pub enum FenTeams {
    Number,
    TeamNames(Vec<char>)
}

pub struct FenOptions {
    pub first_moves: bool,
    pub teams: FenTeams
}

pub struct Game {
    pub pieces: Vec<Box<dyn Piece>>,
    pub move_restrictions: Box<dyn MoveRestrictions>,
    pub fen_options: FenOptions
}
