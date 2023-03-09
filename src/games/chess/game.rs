use crate::{
    Action, BishopPiece, BitBoard, Board, Direction, FenArgument, FenDecodeError, FenFullMoves,
    FenOptions, FenState, FenStateTeams, FenSubMoves, FenTeamArgument, Game, HistoryMove,
    KingPiece, KnightPiece, MoveRestrictions, PawnPiece, PostProcess, QueenPiece, RookPiece,
};

pub struct ChessCastlingRights;
impl FenArgument for ChessCastlingRights {
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        if arg == "-" {
            board.state.first_move ^= &board.state.pieces[3];
            Ok(())
        } else {
            let mut lost_castling_rights = vec!['Q', 'K', 'q', 'k'];
            let initial_lost_castling_rights = ['Q', 'K', 'q', 'k'];

            for char in arg.chars() {
                if !lost_castling_rights.contains(&char) {
                    if initial_lost_castling_rights.contains(&char) {
                        return Err(FenDecodeError::InvalidArgument(format!(
                            "The castling rights of '{char}' have already been specified."
                        )));
                    }

                    return Err(FenDecodeError::InvalidArgument(format!(
                        "'{char}' is not a valid castling rights character."
                    )));
                }

                lost_castling_rights.retain(|el| el != &char);
            }

            for char in lost_castling_rights {
                let (team, scan_dir) = match char {
                    'Q' => (0, Direction::LEFT),
                    'K' => (0, Direction::RIGHT),
                    'q' => (1, Direction::LEFT),
                    'k' => (1, Direction::RIGHT),
                    _ => {
                        return Err(FenDecodeError::InvalidArgument(format!(
                            "'{char}' is not a valid castling rights character."
                        )));
                    }
                };

                let rook = (board.state.pieces[3] & &board.state.teams[team]).bitscan(scan_dir);
                let rook_board = BitBoard::from_lsb(rook);

                board.state.first_move ^= &rook_board;
            }
            Ok(())
        }
    }

    fn encode(&self, board: &Board) -> String {
        let mut castling_rights: Vec<char> = Vec::with_capacity(4);
        for team in 0..board.state.teams.len() {
            let king = board.state.pieces[5] & &board.state.teams[team] & &board.state.first_move;
            if king.is_empty() {
                continue;
            }

            let rooks = board.state.pieces[3] & &board.state.teams[team] & &board.state.first_move;
            let mut one_bits = rooks.iter_one_bits(board.state.rows * board.state.cols);
            if one_bits.len() == 1 {
                let mut side_castling_rights = if rooks > king { 'k' } else { 'q' };

                if team == 0 {
                    side_castling_rights = side_castling_rights.to_ascii_uppercase();
                }

                castling_rights.push(side_castling_rights);
            } else if one_bits.len() > 1 {
                one_bits.reverse();
                for bit in one_bits {
                    let bit = BitBoard::from_lsb(bit);
                    let mut side_castling_rights = if bit > king { 'k' } else { 'q' };

                    if team == 0 {
                        side_castling_rights = side_castling_rights.to_ascii_uppercase();
                    }
    
                    castling_rights.push(side_castling_rights);              
                }
            }
        }

        if castling_rights.len() == 0 {
            String::from("-")
        } else {
            castling_rights
                .iter()
                .map(|el| format!("{}", el))
                .collect::<Vec<_>>()
                .join("")
        }
    }

    fn duplicate(&self) -> Box<dyn FenArgument> {
        Box::new(ChessCastlingRights)
    }
}

pub struct ChessEnPassant;

pub struct ChessPostProcess;
impl PostProcess for ChessPostProcess {
    fn apply(&self, board: &mut Board) {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut bottom = edges.bottom;
        let mut top = edges.top;

        bottom |= &bottom.up(1, cols);
        top |= &top.down(1, cols);

        let first_move = (board.state.pieces[0] & &(bottom | &top))
            | &(board.state.all_pieces ^ &board.state.pieces[0]);
        board.state.first_move &= &first_move;
    }

    fn duplicate(&self) -> Box<dyn PostProcess> {
        Box::new(ChessPostProcess)
    }
}

impl FenArgument for ChessEnPassant {
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        if arg == "-" {
            return Ok(());
        }

        let previous_team = board.get_previous_team(board.state.moving_team);
        let pos = board.decode_position(arg.to_string()).map_err(|err| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid en passant position ({})",
                err
            ))
        })?;

        let cols = board.state.cols;

        let pawn = BitBoard::from_lsb(pos);
        let from = match previous_team {
            0 => pawn.down(2, cols),
            1 => pawn.up(2, cols),
            _ => pawn.down(2, cols),
        };

        board.state.history.push(HistoryMove {
            action: Action {
                from: from.bitscan_forward(),
                to: pos,
                piece_type: 0,
                info: 0,
            },
            state: None,
        });

        Ok(())
    }

    fn encode(&self, board: &Board) -> String {
        let last_move = (&board.state.history).last();
        if let None = last_move {
            return "-".to_string();
        }

        let last_move =
            last_move.expect("The last move for exporting an en passant FEN must be Some.");
        if last_move.action.piece_type != 0 {
            return "-".to_string();
        }

        if (last_move.action.from.abs_diff(last_move.action.to) != (2 * board.state.cols)) {
            return "-".to_string();
        }

        return board.encode_position(last_move.action.to);
    }

    fn duplicate(&self) -> Box<dyn FenArgument> {
        Box::new(ChessEnPassant)
    }
}

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
                    ("castling rights".to_string(), Box::new(ChessCastlingRights)),
                    ("en passant".to_string(), Box::new(ChessEnPassant)),
                    ("half moves".to_string(), Box::new(FenSubMoves)),
                    ("full moves".to_string(), Box::new(FenFullMoves)),
                ],
                post_process: Box::new(ChessPostProcess),
            },
        }
    }
}
