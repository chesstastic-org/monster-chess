mod board;
mod bitset;

pub use board::*;
pub use bitset::*;

fn main() {
    let mut board = Board {
        state: BoardState { 
            blockers: BitBoard::new(), 
            pieces: Vec::new(), 
            teams: Vec::new(),
            first_move: BitBoard::new(),
            edges: generate_edge_list(8, 8),
            rows: 8, 
            cols: 8
        },
        pieces: vec![
            Box::new(KnightPiece {
                piece_type: 0
            }),
            Box::new(BishopPiece {
                piece_type: 1
            }),
            Box::new(RookPiece {
                piece_type: 2
            }),
            Box::new(QueenPiece {
                piece_type: 3
            }),
            Box::new(KingPiece {
                piece_type: 4
            })
        ],
        attack_lookup: Vec::new()
    };

    board.generate_lookups();

    let king_pos = BitBoard::from_element(1 << 28);
    
    //let blocker = BitBoard::from_element(1 << 29);
    //board.state.blockers |= &blocker;

    let moves = board.pieces[2].get_moves(&board, king_pos);

    println!("{}", king_pos.display(8, 8));
    println!("-");
    println!("{}", moves.display(8, 8));
}