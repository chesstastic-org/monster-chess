use crate::bitboard::BitBoard;

use super::{PieceType};

#[derive(Debug)]
pub enum UndoMoveError {
    NoHistoryMoves,
}

pub type ActionInfo = usize;

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub from: u32,
    pub to: u32,
    pub team: u32,
    pub piece_type: PieceType,

    /// Moves can store extra information both for optimizing `make_move` or for specifying additional variants of a move.
    ///
    /// Eg. Pawn Promotion uses `info` to represent which piece is promoted to.
    pub info: ActionInfo
}

#[derive(Copy, Clone, Debug)]
pub struct PreviousBoard<const T: usize>(pub BitBoard<T>);

#[derive(Copy, Clone, Debug)]
pub struct IndexedPreviousBoard<const T: usize>(pub usize, pub BitBoard<T>);

#[derive(Clone, Copy, Debug)]
pub enum HistoryUpdate<const T: usize> {
    Team(IndexedPreviousBoard<T>),
    Piece(IndexedPreviousBoard<T>),
}

#[derive(Clone, Debug)]
pub enum HistoryState<const T: usize> {
    // This is when we want to a less common change where we effect many more bitboards then can be predicted. Avoid this when possible.
    Any {
        all_pieces: PreviousBoard<T>,
        first_move: PreviousBoard<T>,
        updates: Vec<HistoryUpdate<T>>,
    },
    // This is a change that only applies to one piece of one specific team, like moving a piece to a new square without capturing. Use this when possible for performancce.
    Single {
        all_pieces: PreviousBoard<T>,
        first_move: PreviousBoard<T>,
        team: IndexedPreviousBoard<T>,
        piece: IndexedPreviousBoard<T>,
    },
    None,
}

#[derive(Clone, Debug)]
pub struct HistoryMove<const T: usize> {
    pub action: Option<Action>,
    pub state: HistoryState<T>,
}
