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

    let actions = board.generate_moves();
    let action = actions.iter().find(|el| board.encode_position(el.to) == "d6").unwrap();

    board.make_move(action);

    let lines = r#"
    a5a4 - 1
    a8a6 - 1
    a8a7 - 1
    a8b8 - 1
    b7b5 - 1
    b7b6 - 1
    c6a7 - 1
    c6b4 - 1
    c6b8 - 1
    c6d4 - 1
    c6e5 - 1
    c8d7 - 1
    c8e6 - 1
    c8f5 - 1
    c8g4 - 1
    c8h3 - 1
    d8c7 - 1
    d8d2 - 1
    d8d3 - 1
    d8d4 - 1
    d8d5 - 1
    d8d6 - 1
    d8d7 - 1
    e7e5 - 1
    e7e6 - 1
    e8d7 - 1
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
        .perft(3)
        //.get_branch_results(("e5", "d6"))
        .get_branch_results(("b8", "c6"))
        .get_branch_results(("d6", "c7"));
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
