use crate::{board::{BitBoard, PieceType, Cols, Board, AttackDirections, edges::Edges, actions::{Action, HistoryMove, IndexedPreviousBoard, HistoryState, PreviousBoard}, pieces::{PieceSymbol, Piece}, fen::{FenDecodeError, FenArgument, FenFullMoves, FenTeamArgument, FenState, FenSubMoves, FenOptions, PostProcess}, game::{Game, MoveRestrictions}}, bitset::Direction};

use super::pieces::{KingPiece, QueenPiece, RookPiece, BishopPiece, KnightPiece, PawnPiece, down, up};

pub const NORMAL_MODE: u32 = 0;
pub const ATTACKS_MODE: u32 = 1;

pub struct ChessCastlingRights;
impl FenArgument for ChessCastlingRights {
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        if arg == "-" {
            board.state.first_move ^= board.state.pieces[3];
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

                let rook = (board.state.pieces[3] & board.state.teams[team]).bitscan(scan_dir);
                let rook_board = BitBoard::from_lsb(rook);

                board.state.first_move ^= rook_board;
            }
            Ok(())
        }
    }

    fn encode(&self, board: &Board) -> String {
        let mut castling_rights: Vec<char> = Vec::with_capacity(4);
        for team in 0..board.state.teams.len() {
            let king = board.state.pieces[5] & board.state.teams[team] & board.state.first_move;
            if king.is_empty() {
                continue;
            }

            let rooks = board.state.pieces[3] & board.state.teams[team] & board.state.first_move;
            let mut one_bits = rooks.iter_one_bits(board.state.rows * board.state.cols).collect::<Vec<_>>();
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

impl FenArgument for ChessEnPassant {
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError> {
        if arg == "-" {
            return Ok(());
        }

        let previous_team = board.get_previous_team(board.state.moving_team);
        let en_passant_target = board.decode_position(arg.to_string()).map_err(|err| {
            FenDecodeError::InvalidArgument(format!(
                "'{arg}' is not a valid en passant position ({})",
                err
            ))
        })?;

        let cols = board.state.cols;

        let to = up(&BitBoard::from_lsb(en_passant_target), 1, cols, 1);
        let from = down(&to, 2, cols, previous_team);

        board.state.history.push(HistoryMove {
            action: Action {
                from: from.bitscan_forward(),
                to: to.bitscan_forward(),
                team: previous_team,
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

pub struct ChessPostProcess;
impl PostProcess for ChessPostProcess {
    fn apply(&self, board: &mut Board) {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut bottom = edges.bottom;
        let mut top = edges.top;

        bottom |= bottom.up(1, cols);
        top |= top.down(1, cols);

        let first_move = (board.state.pieces[0] & (bottom | top))
            | (board.state.all_pieces ^ board.state.pieces[0]);
        board.state.first_move &= first_move;
    }

    fn duplicate(&self) -> Box<dyn PostProcess> {
        Box::new(ChessPostProcess)
    }
}

pub struct ChessMoveRestrictions;
impl MoveRestrictions for ChessMoveRestrictions {
    fn is_legal(&self, board: &mut Board, action: &Action) -> bool {
        let to_board = BitBoard::from_lsb(action.to);
        let kings = board.state.pieces[5];
        if (to_board & kings).is_set() {
            return false;
        }

        let current_team = board.state.moving_team;

        board.make_move(action);
        let king_board = board.state.teams[current_team as usize] & kings;
        let in_check = board.is_attacking(board.state.moving_team, king_board, ATTACKS_MODE);
        board.undo_move().unwrap();
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
                Box::new(PawnPiece),
                Box::new(KnightPiece),
                Box::new(BishopPiece),
                Box::new(RookPiece),
                Box::new(QueenPiece),
                Box::new(KingPiece)
            ],
            move_restrictions: Box::new(ChessMoveRestrictions),
            fen_options: FenOptions {
                state: FenState { first_moves: false },
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
