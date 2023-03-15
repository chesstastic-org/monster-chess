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
        "7/7/7/7/ooooooo/ooooooo/xxxxxxx o 0 1"
    );

    println!("{}", board.state.all_pieces.display(7, 7));
    println!("-");
    
    let action = board.decode_action("a2a4", 0).unwrap();
    board.make_move(&action);

    println!("{}", board.state.all_pieces.display(7, 7));

    return;

    let lines = r#"a4 1
    b4 1
    c4 1
    d4 1
    e4 1
    f4 1
    g4 1
    a2a4 2
    a2b4 2
    a2c4 2
    b2a4 2
    b2b4 2
    b2c4 2
    b2d4 2
    c2a4 3
    c2b4 3
    c2c4 3
    c2d4 3
    c2e4 3
    d2b4 3
    d2c4 3
    d2d4 3
    d2e4 3
    d2f4 3
    e2c4 3
    e2d4 3
    e2e4 3
    e2f4 3
    e2g4 3
    f2d4 2
    f2e4 2
    f2f4 2
    f2g4 2
    g2e4 2
    g2f4 2
    g2g4 2
    a3c4 3
    a3a5 3
    a3b5 3
    a3c5 3
    b3d4 4
    b3a5 4
    b3b5 4
    b3c5 4
    b3d5 4
    c3a4 5
    c3e4 5
    c3a5 5
    c3b5 5
    c3c5 5
    c3d5 5
    c3e5 5
    d3b4 5
    d3f4 5
    d3b5 5
    d3c5 5
    d3d5 5
    d3e5 5
    d3f5 5
    e3c4 5
    e3g4 5
    e3c5 5
    e3d5 5
    e3e5 5
    e3f5 5
    e3g5 5
    f3d4 4
    f3d5 4
    f3e5 4
    f3f5 4
    f3g5 4
    g3e4 3
    g3e5 3
    g3f5 3
    g3g5 3"#;
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
