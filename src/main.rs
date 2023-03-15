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
        "7/7/7/7/-------/-------/x5o x 0 0"
    );

    println!("{}", board.state.gaps.display(7, 7));
    println!("{}", board.to_fen());

    
    return;

    let lines = r#"b3 23
    c3 22
    d3 0
    b4 23
    d4 0
    b5 23
    c5 22
    d5 0
    c4a2 24
    c4b2 24
    c4c2 23
    c4d2 23
    c4e2 23
    c4a3 24
    c4e3 0
    c4a4 24
    c4a5 24
    c4e5 0
    c4a6 24
    c4b6 24
    c4c6 23
    c4d6 23
    c4e6 23"#;
    let lines = lines
        .split("\n")
        .map(|el| el.trim().to_string())
        .filter(|el| el.len() > 0)
        .collect::<Vec<_>>();

    let perft = board.branch_perft(2);
    for (action, subperft) in perft.branches {
        let line = format!("{action} {}", subperft.nodes);
        if !lines.contains(&line) {
            println!("{}", line);
        }
    }
    println!("total {}", perft.nodes);
}
