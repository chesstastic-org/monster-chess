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

    let moves =  board
        .generate_legal_moves();
    let action = moves
        .iter()
        .find(|el| board.encode_position(el.to) == "a4")
        .unwrap();

    board.make_move(
        *action
    );

    let lines = r#"
    a7a5 - 20
a7a6 - 21
b7b5 - 22
b7b6 - 21
b8a6 - 21
b8c6 - 21
c7c5 - 21
c7c6 - 21
d7d5 - 21
d7d6 - 21
e7e5 - 21
e7e6 - 21
f7f5 - 21
f7f6 - 21
g7g5 - 21
g7g6 - 21
g8f6 - 21
g8h6 - 21
h7h5 - 21
h7h6 - 21"#
        .split("\n")
        .map(|el| el.trim().clone().to_string())
        .filter(|el| !el.is_empty())
        .collect::<Vec<_>>();

    let perft = board
        .perft(2)
        /* .branches
        .iter()
        .find(|el| el.0 == ("a7".to_string(), "a5".to_string()))
        .unwrap()
        .1*/
        .clone();
    println!("TOTAL {}", perft.nodes);
    for ((from, to), results) in perft.branches {
        let line = format!("{}{} - {}", from, to, results.nodes);
        if !lines.contains(&line) {
            println!("{line}");
        }
    }

    println!("{}", board.to_fen());

    //println!("{:#?}", board.perft(1));
    //println!("{}", board.state.pieces[5].display(8, 8));
}
