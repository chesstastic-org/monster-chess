use crate::{BitSet, Piece};

pub type BitBoard = BitSet::<1>;
pub type PieceType = usize;

pub type Rows = u8;
pub type Cols = u8;

pub struct BoardState {
    pub blockers: BitBoard,
    pub pieces: Vec<BitBoard>,
    pub teams: Vec<BitBoard>,
    pub rows: Rows,
    pub cols: Cols
} 

impl BoardState {
    pub fn get_piece_team_board(&self, piece: usize, team: usize) -> BitBoard {
        self.pieces[piece] & &self.teams[team]
    }
}

pub struct Board {
    pub state: BoardState,
    pub pieces: Vec<Box<dyn Piece>>,
    pub attack_lookup: Vec<Vec<BitBoard>>
}