use std::usize;

use crate::{board::{pieces::{Piece, PieceSymbol}, Board, AttackLookup, AttackDirections, actions::{Action, PreviousBoard, HistoryMove, HistoryState, HistoryUpdate, IndexedPreviousBoard}}, bitboard::BitBoard};

pub struct StonePiece;

impl<const T: usize> Piece<T> for StonePiece {
    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::TeamSymbol(vec![ 'x', 'o' ])
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn generate_lookup_moves(&self, board: &Board<T>, mut from: BitBoard<T>) -> AttackDirections<T> {
        let cols = board.state.cols;
        let mut single_moves = from;

        single_moves |= single_moves.up(1, cols);
        single_moves |= single_moves.down(1, cols);
        single_moves |= single_moves.right(1);
        single_moves |= single_moves.left(1);      

        from |= from.up(1, cols);
        from |= from.up(1, cols);
        from |= from.down(1, cols);
        from |= from.down(1, cols);
        from |= from.right(1);
        from |= from.right(1);
        from |= from.left(1);
        from |= from.left(1);
        vec![ from, single_moves ]
    }

    fn get_moves(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        piece_type: usize,
        team: u32,
        mode: u32,
    ) -> BitBoard<T> {
        let lookup = self.get_attack_lookup(board, piece_type);
        let base_moves = match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_lookup_moves(board, from)[0],
        };
        base_moves & !board.state.pieces[0]
    }

    fn make_move(&self, board: &mut Board<T>, action: &Action) {
        let dif = action.from.abs_diff(action.to);
        let cols = board.state.cols;
        let from = BitBoard::<T>::from_lsb(action.from);
        let to = BitBoard::<T>::from_lsb(action.to);

        let piece_type = action.piece_type;
        let team = action.team as usize;
        let other_team = board.state.team_lookup[team] as usize;

        board.history.push(HistoryMove {
            action: *action,
            state: HistoryState::Any {
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
                updates: vec![
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        piece_type,
                        board.state.pieces[piece_type],
                    )),
                    HistoryUpdate::Team(IndexedPreviousBoard(
                        team,
                        board.state.teams[other_team],
                    )),
                    HistoryUpdate::Team(IndexedPreviousBoard(
                        team,
                        board.state.teams[team],
                    )),
                ],
            },
        });

        if dif == 1 || dif == 7 || dif == 6 || dif == 8 {
            // Single Moves
            
            board.state.pieces[piece_type] |= to;
            board.state.teams[team] |= to;
            board.state.all_pieces |= to;
            board.state.first_move &= !from;
        } else {
            board.state.pieces[piece_type] ^= from;
            board.state.pieces[piece_type] |= to;
            board.state.teams[team] ^= from;
            board.state.teams[team] |= to;
            board.state.all_pieces ^= from;
            board.state.all_pieces |= to;
            board.state.first_move &= !from;
        }

        let lookup = self.get_attack_lookup(board, piece_type);
        let update_radius = match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][1],
            None => self.generate_lookup_moves(board, from)[1],
        };

        let to_update = board.state.teams[other_team] & update_radius;
        board.state.teams[other_team] ^= to_update;
        board.state.teams[team] |= to_update;
    }
}