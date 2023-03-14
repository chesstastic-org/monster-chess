use super::{actions::Action, fen::FenOptions, pieces::Piece, Board, Rows, Cols};

pub trait MoveRestrictions<const T: usize> {
    fn is_legal(&self, board: &mut Board<T>, action: Option<&Action>) -> bool;
}

pub struct Game<const T: usize> {
    pub pieces: Vec<&'static dyn Piece<T>>,
    pub move_restrictions: Box<dyn MoveRestrictions<T>>,
    pub fen_options: FenOptions<T>,
    pub teams: u32,
    pub turns: u32,
    pub rows: Rows,
    pub cols: Cols
}
