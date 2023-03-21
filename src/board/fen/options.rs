use std::fmt::Debug;

use super::{
    super::{
        actions::{Action, HistoryMove, UndoMoveError},
        game::Game,
        pieces::Piece,
        Board, Cols, Rows,
    },
    FenArgument,
};

/// `FenState` represents all of the information regarding your FEN implementation's representation of the board state itself.
#[derive(Debug)]
pub struct FenState {
    /// This represents whether or not the first move of a piece should be shown in your game's FEN.
    /// If `true`, then `!` will be shown after the piece type in a FEN if that piece has moved once before.
    /// Otherwise, all pieces will be assumed to be on their first move (but you can modify this behavior with `PostProcess`)
    pub first_moves: bool, 
    /// This is what character gaps will have in your FENs.
    /// Gaps are 'holes' in the board that can't be moved to, but can be moved through depending on the piece.
    pub gaps: char
}

/// `PostProcess` allows you to apply additional changes to the board after you've parsed the FEN state and all FEN arguments.
pub trait PostProcess<const T: usize> : Debug + Send + Sync {
    /// Apply a change to the `Board` after all FEN parsing is done.
    /// For instance, `Chess` uses this to handle whether or not pawns have had their first move.
    fn apply(&self, board: &mut Board<T>);
}

/// `FenOptions` represents all of the information regarding how your FEN representation should be encoded and decoded.
#[derive(Debug)]
pub struct FenOptions<const T: usize> {
    /// `state` represents how the board state should be encoded itself.
    pub state: FenState,
    /// `args` contains all of your `FenArgument`s which handle any additional information the board state doesn't specify.
    pub args: Vec<(String, Box<dyn FenArgument<T>>)>,
    /// `post_process` applies any additional processing after your FEN parsing is done.
    pub post_process: Box<dyn PostProcess<T>>,
    /// `default_fen` is the default FEN position of your game.
    pub default_fen: String
}
