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
        "8/5P2/8/8/8/7K/8/n6k w - - 0 1",
    );

    let actions = board.generate_moves();
    let action = actions.iter().find(|el| board.encode_position(el.to) == "f8").unwrap();

    println!("{}", board.state.pieces[0].display(8, 8));
    println!("{:?}", action);
    board.make_move(action);
    println!("{}", board.state.pieces[1].display(8, 8));

    return;

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
        .perft(1);
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
        if false {// !new_lines.contains(&line) {
            println!("{line}");
        }
    }

    println!("{}", board.to_fen());

    //println!("{:#?}", board.perft(1));
    //println!("{}", board.state.pieces[5].display(8, 8));*/
}
