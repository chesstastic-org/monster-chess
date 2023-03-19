use crate::{board::{game::{MoveController, get_theoretical_moves_bound}, Board, actions::{Action, TheoreticalAction, Move, TheoreticalMove}}, bitboard::BitBoard};

use super::ATTACKS_MODE;

#[derive(Debug)]
pub struct ChessMoveController<const T: usize>;

impl<const T: usize> MoveController<T> for ChessMoveController<T> {
    fn transform_moves(&self, board: &mut Board<T>, mode: u16, actions: Vec<Move>) -> Vec<Move> {
        let moves = board.generate_moves(mode);
        let mut legal_moves = Vec::with_capacity(moves.len());
        for action in moves {
            if self.is_legal(board, &action) {
                legal_moves.push(action);
            }
        }
        legal_moves
    }

    fn is_legal(&self, board: &mut Board<T>, action: &Move) -> bool {
        match action {
            Move::Action(action) => {
                let to_board = BitBoard::from_lsb(action.to);
                let kings = board.state.pieces[5];
                if (to_board & kings).is_set() {
                    return false;
                }

                let current_team = board.state.moving_team;

                let undo = board.make_move(&Move::Action(*action));
                let kings = board.state.pieces[5];
                let king_board = board.state.teams[current_team as usize] & kings;
                let in_check = board.can_move(board.state.moving_team, king_board, ATTACKS_MODE);
                board.undo_move(undo);
                !in_check
            }
            Move::Pass => {
                // Null Moves are not legal in chess.
                // They wouldn't be generated anyways, but I think it would be best to show them as illegal here anyways.
                // Consumers of the library can still make null moves, it just won't be generated by the movegen.
                false
            }
        }
    }

    fn use_pseudolegal(&self) -> bool {
        return true;
    }

    fn encode_action(&self, board: &Board<T>, action: &Move) -> Vec<String> {
        vec![
            match action {
                Move::Action(action) => {
                    match action.from {
                        Some(from) => format!(
                            "{}{}{}",
                            board.encode_position(from),
                            board.encode_position(action.to),
                            board.game.pieces[action.piece_type as usize].format_info(board, action.info)
                        ),
                        None => "----".to_string()
                    }
                },
                Move::Pass => "0000".to_string()
            }   
        ]
    }

    fn get_theoretical_moves(&self, board: &Board<T>) -> Vec<TheoreticalMove> {
        get_theoretical_moves_bound(board, 4, false)
    }

    fn get_max_available_moves(&self) -> u32 {
        220
    }
}
