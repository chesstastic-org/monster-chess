use crate::Board;

#[derive(Clone)]
pub enum FenTeamArgument {
    Number,
    Teams(Vec<char>),
}

impl FenArgument for FenTeamArgument {
    fn encode(&self, board: &Board) -> String {
        match self {
            FenTeamArgument::Number => format!("{}", board.state.moving_team),
            FenTeamArgument::Teams(teams) => teams[board.state.moving_team as usize].to_string(),
        }
    }

    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        match self {
            FenTeamArgument::Number => {
                board.state.moving_team = arg.parse::<u32>().map_err(|_| {
                    FenDecodeError::InvalidArgument(format!(
                        "{} is not a valid numerical team value",
                        arg
                    ))
                })?;
            }
            FenTeamArgument::Teams(teams) => {
                let team = teams.iter().position(|el| el.to_string() == arg);
                match team {
                    Some(team) => {
                        board.state.moving_team = team as u32;
                    }
                    None => {
                        return Err(FenDecodeError::InvalidArgument(format!(
                            "{} is not a valid team argument",
                            arg
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    fn duplicate(&self) -> Box<dyn FenArgument> {
        Box::new(self.clone())
    }
}

pub struct FenTurns;

impl FenArgument for FenTurns {
    fn encode(&self, board: &Board) -> String {
        board.state.turns.to_string()
    }

    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        board.state.turns = arg.parse::<u32>().map_err(|_| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid amount of turns, as it isn't a positive integer."
            ))
        })?;
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn FenArgument> {
        Box::new(FenTurns)
    }
}

pub struct FenSubMoves;

impl FenArgument for FenSubMoves {
    fn encode(&self, board: &Board) -> String {
        board.state.sub_moves.to_string()
    }

    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        board.state.sub_moves = arg.parse::<u32>().map_err(|_| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid amount of sub moves, as it isn't a positive integer."
            ))
        })?;
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn FenArgument> {
        Box::new(FenSubMoves)
    }
}

pub struct FenFullMoves;

impl FenArgument for FenFullMoves {
    fn encode(&self, board: &Board) -> String {
        board.state.full_moves.to_string()
    }

    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        board.state.full_moves = arg.parse::<u32>().map_err(|_| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid amount of full moves, as it isn't a positive integer."
            ))
        })?;
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn FenArgument> {
        Box::new(FenFullMoves)
    }
}

#[derive(Debug, Clone)]
pub enum FenDecodeError {
    InvalidArgument(String),
}

pub trait FenArgument {
    /// `encode` takes in a board, and outputs what this FEN argument's encoded result would be (eg. for a team argument, it could be `"b"`)
    fn encode(&self, board: &Board) -> String;

    /// `decode` takes in a board and an existing argument, and will modify the board to meet the argument (eg. changing the team to reflect the given arg team of `w`)
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError>;

    fn duplicate(&self) -> Box<dyn FenArgument>;
}
