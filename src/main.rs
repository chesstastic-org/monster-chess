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
    a2a3 - 181046
    a2a4 - 217832
    b1a3 - 198572
    b1c3 - 234656
    b2b3 - 215255
    b2b4 - 216145
    c2c3 - 222861
    c2c4 - 240082
    d2d3 - 328511
    d2d4 - 361790
    e2e3 - 402988
    e2e4 - 405385
    f2f3 - 178889
    f2f4 - 198473
    g1f3 - 233491
    g1h3 - 198502
    g2g3 - 217210
    g2g4 - 214048
    h2h3 - 181044
    h2h4 - 218829
    
    "#
    .split("\n")
    .map(|el| el.trim().clone().to_string())
    .filter(|el| !el.is_empty())
    .collect::<Vec<_>>();

    let perft = board
        .sub_perft(5);
    println!("TOTAL {}", perft);

    /*let new_lines = perft
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
    //println!("{}", board.state.pieces[5].display(8, 8));*/
}
