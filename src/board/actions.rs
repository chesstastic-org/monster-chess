use super::{BitBoard, PieceType};

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
    pub info: ActionInfo,
}

#[derive(Copy, Clone, Debug)]
pub struct PreviousBoard(pub BitBoard);

#[derive(Copy, Clone, Debug)]
pub struct IndexedPreviousBoard(pub usize, pub BitBoard);

#[derive(Clone, Debug)]
pub struct HistoryState {
    pub pieces: Vec<IndexedPreviousBoard>,
    pub teams: Vec<IndexedPreviousBoard>,
    pub all_pieces: PreviousBoard,
    pub first_move: PreviousBoard,
}

#[derive(Clone, Debug)]
pub struct HistoryMove {
    pub action: Action,
    pub state: Option<HistoryState>,
}
