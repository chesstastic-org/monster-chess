use crate::{board::{game::Game, fen::{FenOptions, FenState, FenSubMoves, FenTeamArgument, FenFullMoves}, pieces::Piece, actions::Action}, games::ataxx::AtaxxMoveController};

use super::{AtaxxPostProcess, pieces::StonePiece};

pub struct Ataxx;

const STONE: &dyn Piece<1> = &StonePiece;

pub fn is_single_move(action: &Action) -> bool {
    let dif = action.from.abs_diff(action.to);
    dif == 1 || dif == 7 || dif == 6 || dif == 8 
}

impl Ataxx {
    pub fn create() -> Game<1> {
        Game {
            teams: 2,
            turns: 1,
            rows: 7,
            cols: 7,
            pieces: vec![ STONE ],
            controller: Box::new(AtaxxMoveController),
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
