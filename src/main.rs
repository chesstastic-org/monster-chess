use monster_chess::games::ataxx::Ataxx;
use monster_chess::{board::Board, games::chess::Chess};
use rand::{thread_rng, Rng};
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

    let games = vec![
        Chess::create(),
        Ataxx::create()
    ];
    
    let mut rng = thread_rng();

    for i in 0..16 {
        let game = &games[rng.gen_range(0..2)];
        let mut board = game.default();
        let start = get_time_ms();
        let nodes = board.perft(5, true);
        let end = get_time_ms();
        println!("{i}. Game {} perft(5): {} in {}ms ({}/ms npms)", game.name, nodes, end - start, (nodes as u128) / (end - start));
    }
}
