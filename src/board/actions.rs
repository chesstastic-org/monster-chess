use crate::BitBoard;

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub from: BitBoard,
    pub to: BitBoard,
    pub capture: bool
}

#[derive(Copy, Clone, Debug)]
pub struct PreviousBoard(pub BitBoard);

#[derive(Copy, Clone, Debug)]
pub struct IndexedPreviousBoard { 
    pub index: usize,
    pub previous_board: BitBoard
}

#[derive(Clone, Debug)]
pub struct UndoMove {
    pub action: Action,
    pub pieces: Vec<IndexedPreviousBoard>,
    pub teams: Vec<IndexedPreviousBoard>,
    pub blockers: PreviousBoard,
    pub first_move: PreviousBoard
}