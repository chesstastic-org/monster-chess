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
        2,
        (8, 8),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );

    let start = get_time_ms();
    let nodes = board.sub_perft(5);
    let end = get_time_ms();
    println!("{}: {nodes}", end - start);
}
