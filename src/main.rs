mod bitset;
mod board;
mod pieces;

pub use bitset::*;
pub use board::*;
pub use pieces::*;

fn main() {
    let board = Board {
        state: BoardState { 
            blockers: BitBoard::new(), 
            pieces: Vec::new(), 
            teams: Vec::new(),
            edges: Vec::new(),
            rows: 8, 
            cols: 8
        },
        pieces: Vec::new(),
        attack_lookup: Vec::new()
    };

    let king = KingPiece {
        board: &board,
        piece_type: 5
    };

    let king_pos = BitBoard::from_element(4096);
    let moves = king.generate_moves(king_pos);
}