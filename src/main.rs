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
        "rnbqkbnr/1pp1pppp/8/p2pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
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
    a5a4 - 1
    a8a6 - 1
    a8a7 - 1
    b7b5 - 1
    b7b6 - 1
    b8a6 - 1
    b8c6 - 1
    b8d7 - 1
    c7c5 - 1
    c7c6 - 1
    c7d6 - 1
    c8d7 - 1
    c8e6 - 1
    c8f5 - 1
    c8g4 - 1
    c8h3 - 1
    d8d6 - 1
    d8d7 - 1
    d8e7 - 1
    d8f6 - 1
    d8g5 - 1
    d8h4 - 1
    e5e4 - 1
    e8d7 - 1
    f7f5 - 1
    f7f6 - 1
    f8d6 - 1
    f8e7 - 1
    g7g5 - 1
    g7g6 - 1
    g8e7 - 1
    g8f6 - 1
    g8h6 - 1
    h7h5 - 1
    h7h6 - 1
    
    
    "#
    .split("\n")
    .map(|el| el.trim().clone().to_string())
    .filter(|el| !el.is_empty())
    .collect::<Vec<_>>();

    let perft = board.perft(4);
    //.get_branch_results(("e5", "d6"))
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
