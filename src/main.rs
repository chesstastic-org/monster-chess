mod board;
mod bitset;

use std::env;

pub use board::*;
pub use bitset::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut board = Board::new(
        vec![
            Box::new(PawnPiece {
                piece_type: 0
            }),
            Box::new(KnightPiece {
                piece_type: 1
            }),
            Box::new(BishopPiece {
                piece_type: 2
            }),
            Box::new(RookPiece {
                piece_type: 3
            }),
            Box::new(QueenPiece {
                piece_type: 4
            }),
            Box::new(KingPiece {
                piece_type: 5
            }),
        ],
        2,
        (8, 8),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );

    //println!("{}", board.state.first_move.display(8, 8));

    /*let king_pos = BitBoard::from_element(1 << 28);

    board.state.first_move |= &king_pos;
    
    let blocker = BitBoard::from_element(1 << 45);
    board.state.blockers |= &blocker;

    let moves = board.pieces[0].get_moves(&board, king_pos);

    println!("{}", king_pos.display(8, 8));
    println!("-");
    println!("{}", moves.display(8, 8));*/
}