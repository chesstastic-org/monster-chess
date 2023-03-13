use super::{actions::Action, fen::FenOptions, pieces::Piece, Board};

pub trait MoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool;
    fn duplicate(&self) -> Box<dyn MoveRestrictions>;
}

pub struct Game {
    pub pieces: Vec<&'static dyn Piece>,
    pub move_restrictions: Box<dyn MoveRestrictions>,
    pub fen_options: FenOptions,
    pub teams: u32,
    pub turns: u32,
}
