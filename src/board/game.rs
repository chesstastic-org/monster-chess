use crate::{Action, Board, Piece};

pub trait MoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool;
    fn duplicate(&self) -> Box<dyn MoveRestrictions>;
}

pub struct NumberedTeam;
impl FenArgument for NumberedTeam {
    fn encode(&self, board: &Board) -> String {
        format!("{}", board.state.moving_team)
    }

    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        board.state.moving_team = arg.parse::<u32>()
            .map_err(|_| FenDecodeError::InvalidArgument(format!("{} is not a valid numerical team value", arg)))?;
        
        Ok(())
    }
}

pub enum FenDecodeError {
    InvalidArgument(String)
}

pub trait FenArgument {
    /// `encode` takes in a board, and outputs what this FEN argument's encoded result would be (eg. for a team argument, it could be `"b"`)
    fn encode(&self, board: &Board) -> String;

    /// `decode` takes in a board and an existing argument, and will modify the board to meet the argument (eg. changing the team to reflect the given arg team of `w`)
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError>;
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
