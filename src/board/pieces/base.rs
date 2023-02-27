use crate::{BitBoard, Board, PieceType, AttackDirections, AttackLookup};

pub struct Action {
    from: BitBoard,
    to: BitBoard,
    capture: bool
}

pub trait Piece {
    fn get_piece_type(&self) -> PieceType;

    fn can_lookup(&self) -> bool;
    fn get_attack_lookup<'a>(&self, board: &'a Board, from: BitBoard) -> Option<&'a AttackLookup> {
        board.attack_lookup.get(self.get_piece_type())
    }

    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard;
    fn generate_lookup_moves(&self, board: &Board, from: BitBoard) -> AttackDirections {
        Vec::new()
    }

    fn make_move(&self, board: &mut Board, action: &Action) {
        if action.capture {
            let color: usize = if (action.from & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let captured_color: usize = if (action.to & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let piece_type = self.get_piece_type();
            let mut captured_piece_type: usize = 0; 
            for i in 0..(board.pieces.len()) {
                if (board.state.pieces[i] & &action.to).is_set() {
                    captured_piece_type = i;
                    break;
                }
            }

            board.state.teams[color] ^= &action.from;
            board.state.teams[captured_color] ^= &action.to;
            board.state.teams[color] |= &action.to;

            board.state.pieces[piece_type] ^= &action.from;
            board.state.pieces[piece_type] |= &action.to;
            board.state.pieces[captured_piece_type] ^= &action.to;

            board.state.blockers ^= &action.from;
            // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
        } else {
            let color: usize = if (action.from & &board.state.teams[0]).is_set() {
                0
            } else {
                1
            };
            let piece_type = self.get_piece_type();

            board.state.teams[color] ^= &action.from;
            board.state.teams[color] |= &action.to;

            board.state.pieces[piece_type] ^= &action.from;
            board.state.pieces[piece_type] |= &action.to;

            board.state.blockers ^= &action.from;
            board.state.blockers |= &action.to;
        }
    }
}