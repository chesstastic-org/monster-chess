use super::{actions::Action, fen::FenOptions, pieces::Piece, Board, Rows, Cols};

pub trait MoveController<const T: usize> {
    fn transform_moves(&self, board: &mut Board<T>, mode: u32, actions: Vec<Option<Action>>) -> Vec<Option<Action>>;
}

pub struct Game<const T: usize> {
    pub pieces: Vec<&'static dyn Piece<T>>,
    pub move_restrictions: Box<dyn MoveController<T>>,
    pub fen_options: FenOptions<T>,
    pub teams: u32,
    pub turns: u32,
    pub rows: Rows,
    pub cols: Cols
}
