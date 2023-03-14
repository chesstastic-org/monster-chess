use crate::{
    bitboard::{Direction, BitBoard},
    board::{
        actions::{Action, HistoryMove, HistoryState, IndexedPreviousBoard, PreviousBoard},
        edges::Edges,
        fen::{
            FenArgument, FenDecodeError, FenFullMoves, FenOptions, FenState, FenSubMoves,
            FenTeamArgument, PostProcess,
        },
        game::{Game, MoveRestrictions},
        pieces::{Piece, PieceSymbol},
        AttackDirections, Board, Cols, PieceType,
    },
};

use super::pieces::{down, up};


pub struct ChessCastlingRights;
impl<const T: usize> FenArgument<T> for ChessCastlingRights {
    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError> {
        if arg == "-" {
            board.state.first_move &= !board.state.pieces[3];
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

                let rook_board = board.state.pieces[3] & board.state.teams[team];
                if rook_board.is_empty() {
                    continue;
                }

                let rook = rook_board.bitscan(scan_dir);
                let rook_board = BitBoard::from_lsb(rook);

                let king_board = board.state.pieces[5] & board.state.teams[team];
                let king = king_board.bitscan_forward();

                if (rook < king && scan_dir == Direction::LEFT) || (rook > king && scan_dir == Direction::RIGHT) {
                    board.state.first_move &= !rook_board;

                }
            }
            Ok(())
        }
    }

    fn encode(&self, board: &Board<T>) -> String {
        let mut castling_rights: Vec<char> = Vec::with_capacity(4);
        for team in 0..board.state.teams.len() {
            let king = board.state.pieces[5] & board.state.teams[team] & board.state.first_move;
            if king.is_empty() {
                continue;
            }

            let rooks = board.state.pieces[3] & board.state.teams[team] & board.state.first_move;
            let mut one_bits = rooks
                .iter_one_bits(board.state.rows * board.state.cols)
                .collect::<Vec<_>>();
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
}

pub struct ChessEnPassant;

impl<const T: usize> FenArgument<T> for ChessEnPassant {
    fn decode(&self, board: &mut Board<T>, arg: &str) -> Result<(), FenDecodeError> {
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

        let to = up::<T>(&BitBoard::from_lsb(en_passant_target), 1, cols, 1);
        let from = down::<T>(&to, 2, cols, previous_team);

        board.history.push(HistoryMove {
            action: Some(Action {
                from: from.bitscan_forward(),
                to: to.bitscan_forward(),
                team: previous_team,
                piece_type: 0,
                info: 0,
            }),
            state: HistoryState::None,
        });

        Ok(())
    }

    fn encode(&self, board: &Board<T>) -> String {
        let last_move = (&board.history).last();
        if let None = last_move {
            return "-".to_string();
        }

        let last_move =
            last_move.expect("The last move for exporting an en passant FEN must be Some.");

        match last_move.action {
            Some(last_action) => {
                if last_action.piece_type != 0 {
                    return "-".to_string();
                }

                if last_action.from.abs_diff(last_action.to) != (2 * board.state.cols) {
                    return "-".to_string();
                }

                return board.encode_position(last_action.to);
            }
            None => {
                return "-".to_string();
            }
        }
    }
}