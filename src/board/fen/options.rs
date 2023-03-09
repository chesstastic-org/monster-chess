use crate::{FenArgument, Board};

pub enum FenStateTeams {
    Number,
    TeamNames(Vec<char>),
}

pub struct FenState {
    pub first_moves: bool,
}

pub trait PostProcess {
    fn apply(&self, board: &mut Board);

    fn duplicate(&self) -> Box<dyn PostProcess>;
}

pub struct FenOptions {
    pub state: FenState,
    pub args: Vec<(String, Box<dyn FenArgument>)>,
    pub post_process: Box<dyn PostProcess>
}
