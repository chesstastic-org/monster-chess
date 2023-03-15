use std::cmp::Ordering;

use crate::board::{game::{Resolution, GameResults}, Board, actions::Action};

use super::ATTACKS_MODE;

pub struct ChessResolution<const T: usize>;

impl<const T: usize> Resolution<T> for ChessResolution<T> {
    fn resolution(&self, board: &mut Board<T>, legal_moves: &Vec<Option<Action>>) -> GameResults {
        if legal_moves.len() == 0 {
            let kings = board.state.pieces[5];
            let king_board = board.state.teams[board.state.moving_team as usize] & kings;

            let next_team = board.state.team_lookup[board.state.moving_team as usize];
            let in_check = board.can_move(next_team, king_board, ATTACKS_MODE);

            if in_check {
                GameResults::Win(next_team)
            } else {
                GameResults::Draw
            }
        } else {
            GameResults::Ongoing
        }
    }
}