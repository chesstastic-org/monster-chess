mod bitset;
mod board;
mod games;

use std::env;

pub use bitset::*;
pub use board::*;
pub use games::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1000");

    let mut board = Board::new(Chess::create(), 2, (8, 8), "k7/8/K7/8/8/8/8/1Q6");

    board.state.moving_team = 0;

    /*let from = board.state.pieces[1] & &BitBoard::from_msb(1);
    let e = vec![ board.pieces[1].get_moves(&board, from, 1) & &!board.state.teams[1] ]
    .iter().map(|el| el.display(8, 8)).fold("".to_string(), |a, b| a + "\n-\n" + &b);*/

    let actions = board.generate_legal_moves(1);

    println!("{:?}", actions);

    //let action = actions[0];

    //board.game.pieces[0].duplicate().make_move(&mut board, &action);
    //println!("{}", board.state.first_move.display(8, 8));

    //println!("{}", board.state.first_move.display(8, 8));

    /*let king_pos = BitBoard::from_element(1 << 28);

    board.state.first_move |= &king_pos;

    let blocker = BitBoard::from_element(1 << 45);
    board.state.blockers |= &blocker;

    let moves = board.pieces[0].get_moves(&board, king_pos);

    println!("{}", king_pos.display(8, 8));
    println!("-");
    println!("{}", moves.display(8, 8));*/
}
