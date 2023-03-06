use crate::{Action, BitBoard, Board, MoveRestrictions, Game, PawnPiece, KnightPiece, BishopPiece, RookPiece, QueenPiece, KingPiece, FenOptions, FenTeams};

pub struct ChessMoveRestrictions;

impl MoveRestrictions for ChessMoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool {
        let from_board = BitBoard::from_lsb(action.from);
        if (from_board & &board.state.pieces[5]).is_set() {
            return false;
        }

        let piece_trait = board.game.pieces[action.piece_type].duplicate();
        let moving_team = board.state.moving_team;
        piece_trait.make_move(board, action);
        let in_check = board.is_attacking(moving_team, from_board);
        piece_trait.undo_move(board).unwrap();
        in_check
    }

    fn duplicate(&self) -> Box<dyn MoveRestrictions> {
        Box::new(ChessMoveRestrictions)
    }
}

pub struct Chess;

impl Chess {
    pub fn create() -> Game {
        Game {
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
                first_moves: true,
                teams: FenTeams::TeamNames(vec![ 'w', 'b' ])
            }
        }
    }
}
