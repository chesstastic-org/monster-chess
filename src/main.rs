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
        "8/5P2/8/8/8/7K/8/n6k w - - 0 1",
    );

    let action = board.decode_action("f7f8n", 0).unwrap();
    board.make_move(&action);

    let lines = r#"
    f8d7 - 1
    f8e6 - 1
    f8g6 - 1
    f8h7 - 1
    h3g3 - 1
    h3g4 - 1
    h3h4 - 1
    "#.split("\n").map(|el| el.trim().to_string()).filter(|el| el.len() > 0).collect::<Vec<_>>();

    let start = get_time_ms();
    let perft = board.perft(2)
        .get_branch_results("a1b3");
    let end = get_time_ms();
    for (action, subperft) in perft.branches {
        let line = format!("{action} - {}", subperft.nodes);
        if true {
            println!("{line}");
        }
    }
}
