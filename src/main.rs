mod board;
mod bitset;

use std::env;

pub use board::*;
pub use bitset::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut board = Board::new(
        Game {
            pieces: vec![
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
                })
            ]
        },
        2,
        (8, 8),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );

    /*let from = board.state.pieces[1] & &BitBoard::from_msb(1);
    let e = vec![ board.pieces[1].get_moves(&board, from, 1) & &!board.state.teams[1] ]
    .iter().map(|el| el.display(8, 8)).fold("".to_string(), |a, b| a + "\n-\n" + &b);*/

    println!("{}", board.get_attack_mask(1).display(8, 8));

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