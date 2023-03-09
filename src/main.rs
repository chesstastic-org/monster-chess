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
    a2a3 - 380
    a2a4 - 420
    b1a3 - 400
    b1c3 - 440
    b2b3 - 420
    b2b4 - 421
    c2c3 - 420
    c2c4 - 441
    d2d3 - 539
    d2d4 - 560
    e2e3 - 599
    e2e4 - 600
    f2f3 - 380
    f2f4 - 401
    g1f3 - 440
    g1h3 - 400
    g2g3 - 420
    g2g4 - 421
    h2h3 - 380
    h2h4 - 420"#
        .split("\n")
        .map(|el| el.trim().clone().to_string())
        .filter(|el| !el.is_empty())
        .collect::<Vec<_>>();

    let perft = board
        .perft(3);
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
