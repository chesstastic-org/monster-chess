mod bitset;
mod board;
mod games;

use std::env;

pub use bitset::*;
pub use board::*;
pub use games::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1000");

    let mut board = Board::new(
        Chess::create(),
        2,
        (8, 8),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );

    let lines = r#"
    a5b4 - 1
    a8a6 - 1
    a8a7 - 1
    b7b5 - 1
    b7b6 - 1
    b8a6 - 1
    b8c6 - 1
    c7c5 - 1
    c7c6 - 1
    d7d5 - 1
    d7d6 - 1
    e7e5 - 1
    e7e6 - 1
    f7f5 - 1
    f7f6 - 1
    g7g5 - 1
    g7g6 - 1
    g8f6 - 1
    g8h6 - 1
    h7h5 - 1
    h7h6 - 1
    "#
    .split("\n")
    .map(|el| el.trim().clone().to_string())
    .filter(|el| !el.is_empty())
    .collect::<Vec<_>>();

    let perft = board
        .perft(4);
    println!("TOTAL {}", perft.nodes);

    let new_lines = perft
        .branches
        .iter()
        .map(|((from, to), results)| format!("{}{} - {}", from, to, results.nodes))
        .collect::<Vec<_>>();

    for line in &new_lines {
        if !lines.contains(&line) {
            println!("{line}");
        }
    }

    println!("-");
    for line in lines {
        if !new_lines.contains(&line) {
            //println!("{line}");
        }
    }

    println!("{}", board.to_fen());

    //println!("{:#?}", board.perft(1));
    //println!("{}", board.state.pieces[5].display(8, 8));
}
