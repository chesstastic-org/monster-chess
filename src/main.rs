use monster_chess::{board::Board, games::chess::Chess};
use std::env;
use std::time::Duration;

use std::time::{SystemTime, UNIX_EPOCH};

fn get_time_ms() -> u128 {
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
        "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
    );


    let start = get_time_ms();
    let perft = board.perft(5, true);
    let end = get_time_ms();
    println!("perft(5): {} in {}ms", perft, end - start);

    let start = get_time_ms();
    let perft = board.perft(5, false);
    let end = get_time_ms();
    println!("perft<No Legality>(5): {} in {}ms", perft, end - start);
}
