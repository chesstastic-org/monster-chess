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
    a2a3 - 8457
    a2a4 - 9329
    b1a3 - 8885
    b1c3 - 9755
    b2b3 - 9345
    b2b4 - 9332
    c2c3 - 9272
    c2c4 - 9744
    d2d3 - 11959
    d2d4 - 12435
    e2e3 - 13134
    e2e4 - 13160
    f2f3 - 8457
    f2f4 - 8929
    g1f3 - 9748
    g1h3 - 8881
    g2g3 - 9345
    g2g4 - 9328
    h2h3 - 8457
    h2h4 - 9329
    
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
            println!("{line}");
        }
    }

    println!("{}", board.to_fen());

    //println!("{:#?}", board.perft(1));
    //println!("{}", board.state.pieces[5].display(8, 8));
}
