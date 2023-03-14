use crate::{board::{game::MoveRestrictions, Board, actions::Action}, bitboard::BitBoard};

use super::ATTACKS_MODE;

pub struct ChessMoveRestrictions;
impl<const T: usize> MoveRestrictions<T> for ChessMoveRestrictions {
    fn is_legal(&self, board: &mut Board<T>, action: &Action) -> bool {
        let to_board = BitBoard::from_lsb(action.to);
        let kings = board.state.pieces[5];
        if (to_board & kings).is_set() {
            return false;
        }

        let current_team = board.state.moving_team;

        board.make_move(action);
        let kings = board.state.pieces[5];
        let king_board = board.state.teams[current_team as usize] & kings;
        let in_check = board.can_move(board.state.moving_team, king_board, ATTACKS_MODE);
        board.undo_move();
        !in_check
    }
}
