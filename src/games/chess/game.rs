use crate::{
    Action, BishopPiece, BitBoard, Board, FenFullMoves, FenOptions, FenState, FenStateTeams,
    FenSubMoves, FenTeamArgument, Game, KingPiece, KnightPiece, MoveRestrictions, PawnPiece,
    QueenPiece, RookPiece,
};

pub struct ChessMoveRestrictions;

impl MoveRestrictions for ChessMoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool {
        let to_board = BitBoard::from_lsb(action.to);
        if (to_board & &board.state.pieces[5]).is_set() {
            return false;
        }

        let from_board = BitBoard::from_lsb(action.from);

        let mut new_king_board =
            board.state.teams[board.state.moving_team as usize] & &board.state.pieces[5];
        if (from_board & &new_king_board).is_set() {
            new_king_board = to_board;
        }

        let in_check =
            board.is_attacking(board.get_next_team(board.state.moving_team), new_king_board);
        !in_check
    }

    fn duplicate(&self) -> Box<dyn MoveRestrictions> {
        Box::new(ChessMoveRestrictions)
    }
}

pub struct Chess;

impl Chess {
    pub fn create() -> Game {
        Game {
            turns: 1,
            pieces: vec![
                Box::new(PawnPiece { piece_type: 0 }),
                Box::new(KnightPiece { piece_type: 1 }),
                Box::new(BishopPiece { piece_type: 2 }),
                Box::new(RookPiece { piece_type: 3 }),
                Box::new(QueenPiece { piece_type: 4 }),
                Box::new(KingPiece { piece_type: 5 }),
            ],
            move_restrictions: Box::new(ChessMoveRestrictions),
            fen_options: FenOptions {
                state: FenState { first_moves: true },
                args: vec![
                    (
                        "team to move".to_string(),
                        Box::new(FenTeamArgument::Teams(vec!['w', 'b'])),
                    ),
                    ("half moves".to_string(), Box::new(FenSubMoves)),
                    ("full moves".to_string(), Box::new(FenFullMoves)),
                ],
            },
        }
    }
}
