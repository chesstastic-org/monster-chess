use std::cmp::Ordering;

use crate::board::{game::{Resolution, GameResults}, Board, actions::{Action, Move}};

#[derive(Debug)]
pub struct AtaxxResolution<const T: usize>;

impl<const T: usize> Resolution<T> for AtaxxResolution<T> {
    fn resolve(&self, board: &mut Board<T>, legal_moves: &Vec<Move>) -> GameResults {
        if legal_moves.len() > 0 {
            if board.state.sub_moves == 100 {
                return GameResults::Draw;
            }

            return GameResults::Ongoing;
        }

        let black_stones = board.state.teams[0].count_ones();
        let white_stones = board.state.teams[1].count_ones();

        match black_stones.cmp(&white_stones) {
            Ordering::Greater => {
                GameResults::Win(0)
            }
            Ordering::Equal => {
                GameResults::Draw
            }
            Ordering::Less => {
                GameResults::Win(1)
            }
        }
    }
}