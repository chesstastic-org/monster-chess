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

#[derive(Clone, Copy, Debug)]
pub enum HistoryUpdate {
    Team(IndexedPreviousBoard),
    Piece(IndexedPreviousBoard),
}

#[derive(Clone, Debug)]
pub enum HistoryState {
    /*
        This is when we want to a less common change where we effect many more bitboards then can be predicted. Avoid this when possible.
    */
    Any {
        all_pieces: PreviousBoard,
        first_move: PreviousBoard,
        updates: Vec<HistoryUpdate>,
    },
    /*
        This is a change that only applies to one piece of one specific team, like moving a piece to a new square without capturing. Use this when possible for performancce.
    */
    Single {
        all_pieces: PreviousBoard,
        first_move: PreviousBoard,
        team: IndexedPreviousBoard,
        piece: IndexedPreviousBoard,
    },
    None,
}

#[derive(Clone, Debug)]
pub struct HistoryMove {
    pub action: Action,
    pub state: HistoryState,
}
