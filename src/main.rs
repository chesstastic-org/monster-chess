use monster_chess::{games::{chess::Chess}, board::tests::get_time_ms};

pub fn main() {
    let game = Chess::create();
    let board = game.from_fen("r1bq1rk1/ppp2ppp/5n2/2bp4/2NPP3/2P5/PP3PPP/RNBQK2R w KQ - 0 1");

    println!("{}", board.to_fen());
} 