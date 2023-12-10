use monster_chess::{games::{chess::Chess}, board::tests::get_time_ms};

pub fn main() {
    let game = Chess::create();
    let mut board = game.from_fen("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1");

    let start = get_time_ms();
    let nodes = board.perft(6, false);
    let end = get_time_ms();

    println!("{}: {} nodes ({} / s)", end - start, nodes, 1000 * (nodes / ((end - start) as u64)));
} 