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
        "rnbqkbnr/pppp1ppp/8/1B2p3/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2",
    );

    let lines = r#"
    a7a5 - 1
    a7a6 - 1
    b7b6 - 1
    b8a6 - 1
    b8c6 - 1
    c7c5 - 1
    c7c6 - 1
    d8e7 - 1
    d8f6 - 1
    d8g5 - 1
    d8h4 - 1
    e8e7 - 1
    f7f5 - 1
    f7f6 - 1
    f8a3 - 1
    f8b4 - 1
    f8c5 - 1
    f8d6 - 1
    f8e7 - 1
    g7g5 - 1
    g7g6 - 1
    g8e7 - 1
    g8f6 - 1
    g8h6 - 1
    h7h5 - 1
    h7h6 - 1
    "#.split("\n").map(|el| el.trim().to_string()).filter(|el| el.len() > 0).collect::<Vec<_>>();

    let start = get_time_ms();
    let perft = board.perft(1);
    let end = get_time_ms();
    for ((from, to), subperft) in perft.branches {
        let line = format!("{from}{to} - {}", subperft.nodes);
        if !lines.contains(&line) {
            println!("{line}");
        }
    }
}
