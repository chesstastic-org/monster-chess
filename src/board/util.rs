use crate::{BitSet, Piece, Edges};

pub type BitBoard = BitSet::<1>;
pub type PieceType = usize;

/*
    I doubt anyone would be practically creating boards of 340,282,366,920,938,463,463,374,607,431,768,211,456 x 340,282,366,920,938,463,463,374,607,431,768,211,456.
    However, storing these as u128s makes it much easier to interface the bitboards with (particularly, shifting bits with them.)
*/
pub type Rows = u128;
pub type Cols = u128;

pub struct BoardState {
    /*
        Blockers is a BitBoard of all pieces, because keeping this bitboard ready makes it much easier to calculate movement for slider pieces.
    */
    pub blockers: BitBoard,
    pub pieces: Vec<BitBoard>,
    pub teams: Vec<BitBoard>,
    /*
        Edges is a list of "boundary bitboards" for validating the movement of delta pieces (pieces that move in a fixed way everytime)
    */
    pub edges: Vec<Edges>,
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