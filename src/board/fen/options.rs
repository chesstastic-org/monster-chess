use crate::FenArgument;

pub enum FenStateTeams {
    Number,
    TeamNames(Vec<char>),
}

pub struct FenState {
    pub first_moves: bool,
}

pub struct FenOptions {
    pub state: FenState,
    pub args: Vec<(String, Box<dyn FenArgument>)>,
}
