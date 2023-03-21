use std::fmt::Debug;

use super::super::{
    actions::{Action, HistoryMove, UndoMoveError},
    game::Game,
    pieces::Piece,
    Board, Cols, Rows,
};

/// `FenTeamArgument` represents which team is going to move next in your FEN.
#[derive(Clone, Debug)]
pub enum FenTeamArgument {
    Number,
    Teams(Vec<char>),
}

impl<const T: usize> FenArgument<T> for FenTeamArgument {
    fn encode(&self, board: &Board<T>) -> String {
        match self {
            FenTeamArgument::Number => format!("{}", board.state.moving_team),
            FenTeamArgument::Teams(teams) => teams[board.state.moving_team as usize].to_string(),
        }
    }

    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError> {
        match self {
            FenTeamArgument::Number => {
                board.state.moving_team = arg.parse::<u16>().map_err(|_| {
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
                        board.state.moving_team = team as u16;
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
}

/// `FenTurns` represents how many turns have been played in your game.
/// This counter might not be a counter of all turns in the entire game, but how many since the last of a certain type of move, for instance.
/// Games can manage what counters represent by handling `update` in `MoveController`.
#[derive(Debug)]
pub struct FenTurns;

impl<const T: usize> FenArgument<T> for FenTurns {
    fn encode(&self, board: &Board<T>) -> String {
        board.state.turns.to_string()
    }

    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError> {
        board.state.turns = arg.parse::<u16>().map_err(|_| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid amount of turns, as it isn't a positive integer."
            ))
        })?;
        Ok(())
    }
}

/// `FenSubMoves` represents how many sub moves (or in the case of two players, half moves) have been played in your game.
/// This counter might not be a counter of all sub moves in the entire game, but how many since the last of a certain type of move, for instance.
/// Games can manage what counters represent by handling `update` in `MoveController`.
#[derive(Debug)]
pub struct FenSubMoves;

impl<const T: usize> FenArgument<T> for FenSubMoves {
    fn encode(&self, board: &Board<T>) -> String {
        board.state.sub_moves.to_string()
    }

    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError> {
        board.state.sub_moves = arg.parse::<u16>().map_err(|_| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid amount of sub moves, as it isn't a positive integer."
            ))
        })?;
        Ok(())
    }
}

/// `FenFullMoves` represents how many sub moves (or in the case of two players, half moves) have been played in your game.
/// This counter might not be a counter of all full moves in the entire game, but how many since the last of a certain type of move, for instance.
/// Games can manage what counters represent by handling `update` in `MoveController`.
#[derive(Debug)]
pub struct FenFullMoves;

impl<const T: usize> FenArgument<T> for FenFullMoves {
    fn encode(&self, board: &Board<T>) -> String {
        board.state.full_moves.to_string()
    }

    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError> {
        board.state.full_moves = arg.parse::<u16>().map_err(|_| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid amount of full moves, as it isn't a positive integer."
            ))
        })?;
        Ok(())
    }
}

/// This is the error that is shown if a given `FenArgument` has an error when being parsed.
#[derive(Debug, Clone)]
pub enum FenDecodeError {
    InvalidArgument(String),
}

/// `FenArgument` represents a given argument in your FEN.
/// All FENs will show the board state, but after that, games can specify a variety of arguments.
/// In chess, the FEN arguments are `FenTeamArgument`, `ChessCastlingRights`, `ChessEnPassant`, `FebSubMoves` and `FenFullMoves`.
pub trait FenArgument<const T: usize> : Debug + Send + Sync {
    /// `encode` takes in a board, and outputs what this FEN argument's encoded result would be (eg. for a team argument, it could be `"b"`)
    fn encode(&self, board: &Board<T>) -> String;

    /// `decode` takes in a board and an existing argument, and will modify the board to meet the argument (eg. changing the team to reflect the given arg team of `w`)
    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError>;
}
