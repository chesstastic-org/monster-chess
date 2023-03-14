mod game;
mod suite;
pub mod pieces;

pub use game::*;

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        board::{actions::HistoryMove, game::Game},
        games::chess::Chess,
    };
}
