use std::env;
use monster_chess::{games::chess::Chess, board::Board};

fn main() {
    env::set_var("RUST_BACKTRACE", "1000");

    let mut board = Board::new(
        Chess::create(),
        2,
        (8, 8),
        "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1",
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
    a6b5 - 1
    a6b7 - 1
    a6c4 - 1
    a6c8 - 1
    a6d3 - 1
    a6e2 - 1
    a8b8 - 1
    a8c8 - 1
    a8d8 - 1
    b4b3 - 1
    b4c3 - 1
    b6a4 - 1
    b6c4 - 1
    b6c8 - 1
    b6d5 - 1
    c6c5 - 1
    d7d5 - 1
    d7d6 - 1
    e8e7 - 1
    f6d5 - 1
    f6e4 - 1
    f6g4 - 1
    f6g8 - 1
    f6h5 - 1
    f6h7 - 1
    g6g5 - 1
    g7f8 - 1
    g7h6 - 1
    h3g2 - 1
    h8f8 - 1
    h8g8 - 1
    h8h4 - 1
    h8h5 - 1
    h8h6 - 1
    h8h7 - 1
    "#
    .split("\n")
    .map(|el| el.trim().clone().to_string())
    .filter(|el| !el.is_empty())
    .collect::<Vec<_>>();

    let perft = board.perft(2);
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
