use monster_chess::games::ataxx::Ataxx;
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

    let ataxx = Ataxx::create();
    let mut board = ataxx.from_fen(
        "x5o/7/7/7/7/7/o5x x 0 1",
    );

    let action = board.decode_action("g1g3", 0).unwrap();
    board.make_move(&action);

    println!("{}", board.to_fen());

    let perft = board.branch_perft(1);
    for (action, subperft) in perft.branches {
        println!("{action} - {}", subperft.nodes);
    }
    println!("total {}", perft.nodes);
}
