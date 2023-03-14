use crate::{board::{game::Game, fen::{FenOptions, FenState, FenSubMoves, FenTeamArgument, FenFullMoves}}, games::ataxx::AtaxxMoveRestrictions};

use super::AtaxxPostProcess;

pub struct Ataxx;

impl Ataxx {
    pub fn create() -> Game<1> {
        Game {
            teams: 2,
            turns: 1,
            rows: 7,
            cols: 7,
            pieces: vec![],
            move_restrictions: Box::new(AtaxxMoveRestrictions),
            fen_options: FenOptions {
                state: FenState { first_moves: false },
                args: vec![
                    (
                        "team to move".to_string(),
                        Box::new(FenTeamArgument::Teams(vec!['x', 'o'])),
                    ),
                    ("half moves".to_string(), Box::new(FenSubMoves)),
                    ("full moves".to_string(), Box::new(FenFullMoves)),
                ],
                post_process: Box::new(AtaxxPostProcess)
            }
        }
    }
}
