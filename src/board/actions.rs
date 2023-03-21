use crate::bitboard::BitBoard;

use super::{PieceType};

#[derive(Debug)]
pub enum UndoMoveError {
    NoHistoryMoves,
}

pub type ActionInfo = u16;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Action {
    pub from: Option<u16>,
    pub to: u16,
    pub team: u16,
    pub piece_type: PieceType,

    /// Moves can store extra information both for specifying additional variants of a move.
    /// Eg. Pawn Promotion uses `info` to represent which piece is promoted to.
    pub info: ActionInfo,
    /// Moves can also store a move type, which is used merely to tell what type of move it is (en passant, castling, etc.)
    /// The difference between this and `info` is that this is for information that can be derived from the rest of the information.
    /// For instance, you can tell if a move is an en passant based on the last move. That's not true for promotion.
    pub move_type: ActionInfo
}

/// A theoretically possible action. It doesn't even have to be actually possible. 
/// It's mainly there for Neural Networks to be able to index moves.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TheoreticalAction {
    pub from: Option<u16>,
    pub to: u16,
    pub info: ActionInfo
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PreviousBoard<const T: usize>(pub BitBoard<T>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IndexedPreviousBoard<const T: usize>(pub usize, pub BitBoard<T>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryUpdate<const T: usize> {
    Team(IndexedPreviousBoard<T>),
    Piece(IndexedPreviousBoard<T>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

pub enum CounterUpdate {
    Next,
    To(u16)
}

pub struct TurnUpdate {
    pub turns: CounterUpdate,
    pub sub_moves: CounterUpdate,
    pub full_moves: CounterUpdate
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnInfo {
    pub turns: u16,
    pub sub_moves: u16,
    pub full_moves: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryMove<const T: usize> {
    pub action: Move,
    // When undoing moves, we need to make sure we don't mess up our `history` information with the last `n` moves, because movegen will still be using it for say, en passant.
    // If we look say, 10 ply ahead, and get rid of all previous history moves, we won't remember our en passant info at say 1 ply.
    // To solve this, we store `first_history_move`.
    // If we need to get rid of `history[0]` to make space for our latest move, we'll put it here, so once we undo it, we can remember what it originally was.
    pub first_history_move: Option<Move>,
    pub turn_info: TurnInfo,
    pub state: HistoryState<T>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SimpleMove<T> {
    Pass,
    Action(T)
}

impl<T> SimpleMove<T> {
    pub fn is_pass(&self) -> bool {
        match self {
            SimpleMove::Pass => true,
            SimpleMove::Action(_) => false
        }
    }

    pub fn is_action(&self) -> bool {
        match self {
            SimpleMove::Pass => false,
            SimpleMove::Action(_) => true
        }
    }
}

pub type Move = SimpleMove<Action>;
pub type TheoreticalMove = SimpleMove<TheoreticalAction>;