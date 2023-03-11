use std::env;
use monster_chess::{games::chess::Chess, board::Board};

fn main() {
    env::set_var("RUST_BACKTRACE", "1000");

    let mut board = Board::new(
        Chess::create(),
        2,
        (8, 8),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );

    for i in 0..200 {
        let nodes = board.sub_perft(3);
        println!("{i}: {}", nodes);
    }
}
