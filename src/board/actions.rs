use crate::BitBoard;

pub struct NoHistoryMoves;

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub from: BitBoard,
    pub to: BitBoard,
    pub capture: bool
}

#[derive(Copy, Clone, Debug)]
pub struct PreviousBoard(pub BitBoard);

#[derive(Copy, Clone, Debug)]
pub struct IndexedPreviousBoard(pub usize, pub BitBoard);

#[derive(Clone, Debug)]
pub struct HistoryMove {
    pub action: Action,
    pub pieces: Vec<IndexedPreviousBoard>,
    pub teams: Vec<IndexedPreviousBoard>,
    pub blockers: PreviousBoard,
    pub first_move: PreviousBoard
}