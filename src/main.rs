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
    a1a2 - 1
    a3a4 - 1
    b1c3 - 1
    b2b3 - 1
    b2b4 - 1
    c2c3 - 1
    c2c4 - 1
    d2d3 - 1
    d2d4 - 1
    e2e3 - 1
    e2e4 - 1
    f2f3 - 1
    f2f4 - 1
    g1f3 - 1
    g1h3 - 1
    g2g3 - 1
    g2g4 - 1
    h2h3 - 1
    h2h4 - 1"#.split("\n").map(|el| el.trim().clone().to_string()).collect::<Vec<_>>();

    let perft = board.perft(3);
    println!("TOTAL {}", perft.nodes);
    for ((from, to), nodes) in perft.branches {
        let line = format!("{}{} - {}", from, to, nodes);
        //if !lines.contains(&line) {
            println!("{line}");
        //}
    }

    println!("{}", board.to_fen());

    //println!("{:#?}", board.perft(1));
    //println!("{}", board.state.pieces[5].display(8, 8));
}
