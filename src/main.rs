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
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    );

    println!("team0: {}", board.state.moving_team);
    let action = board.decode_action("c4c5", 0).unwrap();
    println!("{}", board.state.first_move.display(8, 8));
    board.make_move(&action);
    println!("team1: {}", board.state.moving_team);
    println!("{}", board.state.first_move.display(8, 8));

    let action = board.decode_action("a3a4", 0).unwrap();
    board.make_move(&action);
    println!("team2: {}", board.state.moving_team);
    println!("{}", board.state.first_move.display(8, 8));

    return;

    let lines = r#"
    a1b1 - 1
    a1c1 - 1
    a2a3 - 1
    b4a3 - 1
    b4a5 - 1
    b4c3 - 1
    c5b6 - 1
    d1a4 - 1
    d1b1 - 1
    d1b3 - 1
    d1c1 - 1
    d1c2 - 1
    d1e1 - 1
    d1e2 - 1
    d2d3 - 1
    d2d4 - 1
    e4e5 - 1
    f1e1 - 1
    f1f2 - 1
    f3d4 - 1
    f3e1 - 1
    f3e5 - 1
    f3g5 - 1
    f3h4 - 1
    g1f2 - 1
    g1h1 - 1
    g2g3 - 1
    g2g4 - 1
    h2h3 - 1
    h2h4 - 1
    h6f5 - 1
    h6f7 - 1
    h6g4 - 1
    h6g8 - 1
    
    "#
    .split("\n")
    .map(|el| el.trim().to_string())
    .filter(|el| el.len() > 0)
    .collect::<Vec<_>>();

    let perft = board.perft(1);

    let start = get_time_ms();
    let end = get_time_ms();
    for (action, subperft) in perft.branches {
        let line = format!("{action} - {}", subperft.nodes);
        if !lines.contains(&line) {
            println!("{line}");
        }
    }
}
