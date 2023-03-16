use crate::{board::{game::{Game, DefaultZobristController}, fen::{FenOptions, FenState, FenSubMoves, FenTeamArgument, FenFullMoves}, pieces::Piece, actions::Action, zobrist::ZobristHashTable}, games::ataxx::AtaxxMoveController};

use super::{AtaxxPostProcess, AtaxxResolution, pieces::StonePiece};

pub struct Ataxx;

pub const STONE: &dyn Piece<1> = &StonePiece;

pub const NORMAL_MODE: u32 = 1;

pub fn is_single_move(action: &Action) -> bool {
    match action.from {
        Some(from) => {
            let dif = from.abs_diff(action.to);
            dif == 1 || dif == 7 || dif == 6 || dif == 8 
        }
        None => false
    }
}

impl Ataxx {
    pub fn create() -> Game<1> {
        Game {
            teams: 2,
            turns: 1,
            rows: 7,
            cols: 7,
            squares: 49,
            zobrist_controller: Box::new(DefaultZobristController),
            zobrist: ZobristHashTable::<1>::generate(49, 2, 1, 0, || fastrand::u64(0..u64::MAX)),
            name: String::from("Ataxx"),
            pieces: vec![ STONE ],
            controller: Box::new(AtaxxMoveController),
            resolution: Box::new(AtaxxResolution),
            fen_options: FenOptions {
                state: FenState { first_moves: false, gaps: '-' },
                args: vec![
                    (
                        "team to move".to_string(),
                        Box::new(FenTeamArgument::Teams(vec!['x', 'o'])),
                    ),
                    ("half moves".to_string(), Box::new(FenSubMoves)),
                    ("full moves".to_string(), Box::new(FenFullMoves)),
                ],
                post_process: Box::new(AtaxxPostProcess),
                default_fen: String::from("x5o/7/7/7/7/7/o5x x 0 1")
            }
        }
    }
}
