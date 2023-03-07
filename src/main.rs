mod bitset;
mod board;
mod games;

use std::env;

pub use bitset::*;
pub use board::*;
pub use games::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1000");

    let mut board = Board::new(Chess::create(), 2, (8, 8), "k7/8/K7/8/8/8/8/1Q6 w");

    let actions = board.generate_legal_moves(1);
    println!("{}", board.to_fen());
}
