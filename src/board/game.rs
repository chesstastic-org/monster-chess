use crate::{Action, Board, Piece};

pub trait MoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool;
    fn duplicate(&self) -> Box<dyn MoveRestrictions>;
}

pub enum FenDecodeError {
    InvalidArgument(String)
}

pub trait FenArgument {
    /// `encode` takes in a board, and outputs what this FEN argument's encoded result would be (eg. for a team argument, it could be `"b"`)
    fn encode(board: &Board) -> String;

    /// `decode` takes in a board and an existing argument, and will modify the board to meet the argument (eg. changing the team to reflect the given arg team of `w`)
    fn decode(board: &mut Board, arg: &str) -> Result<(), FenDecodeError>;
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
