use monster_chess::games::chess::Chess;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_time_ms() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

fn main() {
    let chess = Chess::create();
    let mut board = chess.default();

    let start = get_time_ms();
    board.perft(5, false);
    let end = get_time_ms();
    println!("{}", end - start);
}  