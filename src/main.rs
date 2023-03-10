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
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    );

    /*let actions = board.generate_legal_moves();
    let action = actions
        .iter()
        .find(|el| board.encode_position(el.to) == "d6")
        .unwrap();
    board.make_move(action);

    let actions = board.generate_legal_moves();
    let action = actions
        .iter()
        .find(|el| board.encode_position(el.to) == "e5")
        .unwrap();
    board.make_move(action);

    let actions = board.generate_legal_moves();
    let action = actions
        .iter()
        .find(|el| board.encode_position(el.to) == "f3")
        .unwrap();
    board.make_move(action);*/

    let lines = r#"
    a1b1 - 43
    a1c1 - 43
    a1d1 - 43
    a2a3 - 44
    a2a4 - 44
    b2b3 - 42
    c3a4 - 42
    c3b1 - 42
    c3b5 - 39
    c3d1 - 42
    d2c1 - 43
    d2e3 - 43
    d2f4 - 43
    d2g5 - 42
    d2h6 - 41
    d5d6 - 41
    d5e6 - 46
    e1c1 - 43
    e1d1 - 43
    e1f1 - 43
    e1g1 - 43
    e2a6 - 36
    e2b5 - 39
    e2c4 - 41
    e2d1 - 44
    e2d3 - 42
    e2f1 - 44
    e5c4 - 42
    e5c6 - 41
    e5d3 - 43
    e5d7 - 45
    e5f7 - 44
    e5g4 - 44
    e5g6 - 42
    f3d3 - 42
    f3e3 - 43
    f3f4 - 43
    f3f5 - 45
    f3f6 - 39
    f3g3 - 43
    f3g4 - 43
    f3h3 - 43
    f3h5 - 43
    g2g3 - 42
    g2g4 - 42
    g2h3 - 43
    h1f1 - 43
    h1g1 - 43
    "#
    .split("\n")
    .map(|el| el.trim().clone().to_string())
    .filter(|el| !el.is_empty())
    .collect::<Vec<_>>();

    let perft = board.perft(2);
    //.get_branch_results(("e7", "e5"))
    //.get_branch_results(("g1", "f3"));
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
    //println!("{}", board.state.pieces[5].display(8, 8));*/
}
