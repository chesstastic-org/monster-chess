use super::{
    super::{
        actions::{Action, HistoryMove, UndoMoveError},
        game::Game,
        pieces::Piece,
        Board, Cols, Rows,
    },
    FenArgument,
};

pub enum FenStateTeams {
    Number,
    TeamNames(Vec<char>),
}

pub struct FenState {
    pub first_moves: bool, pub gaps: char
}

pub trait PostProcess<const T: usize> {
    fn apply(&self, board: &mut Board<T>);
}

pub struct FenOptions<const T: usize> {
    pub state: FenState,
    pub args: Vec<(String, Box<dyn FenArgument<T>>)>,
    pub post_process: Box<dyn PostProcess<T>>,
    pub default_fen: String
}
