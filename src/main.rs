use std::env;
use std::time::Duration;
use monster_chess::{games::chess::Chess, board::Board};

use std::time::{SystemTime, UNIX_EPOCH};

fn get_time_ms() -> u128  {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1000");

    let chess = Chess::create();
    let mut board = Board::new(
        &chess,
        (8, 8),
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    );

    println!("-");
    println!("{}", board.state.first_move.display(8, 8));

    let action = board.decode_action("d5e6", 0).unwrap();
    board.make_move(&action);
    
    println!("-");
    println!("{}", board.state.first_move.display(8, 8));

    let action = board.decode_action("d7d5", 0).unwrap();
    board.make_move(&action);

    println!("-");
    println!("{}", board.state.first_move.display(8, 8));

    return;

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
    b6d7 - 1
    c7c5 - 1
    c7c6 - 1
    d5d4 - 1
    d5e4 - 1
    e7c5 - 1
    e7d6 - 1
    e7d7 - 1
    e7d8 - 1
    e7e6 - 1
    e7f6 - 1
    e7f8 - 1
    e8c8 - 1
    e8d8 - 1
    e8f8 - 1
    e8g8 - 1
    f7e6 - 1
    g6g5 - 1
    g7f6 - 1
    g7f8 - 1
    g7h6 - 1
    h3g2 - 1
    h8f8 - 1
    h8g8 - 1
    h8h4 - 1
    h8h5 - 1
    h8h6 - 1
    h8h7 - 1
    "#.split("\n").map(|el| el.trim().to_string()).filter(|el| el.len() > 0).collect::<Vec<_>>();

    let start = get_time_ms();
    let perft = board.perft(2)
        .get_branch_results("f3f6");
    let end = get_time_ms();
    for (action, subperft) in perft.branches {
        let line = format!("{action} - {}", subperft.nodes);
        if !lines.contains(&line) {
            println!("{line}");
        }
    }
}
