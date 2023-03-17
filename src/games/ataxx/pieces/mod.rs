use std::usize;

use crate::{board::{pieces::{Piece, PieceSymbol}, Board, AttackLookup, AttackDirections, actions::{Action, PreviousBoard, HistoryMove, HistoryState, HistoryUpdate, IndexedPreviousBoard}, edges::Edges, Cols, update_turns}, bitboard::BitBoard};

use super::is_single_move;

#[derive(Debug)] pub struct StonePiece;

fn right_one<const T: usize>(from: BitBoard<T>, edges: &Edges<T>) -> BitBoard<T> {
    (from & !edges.right).right(1)
}

fn left_one<const T: usize>(from: BitBoard<T>, edges: &Edges<T>) -> BitBoard<T> {
    (from & !edges.left).left(1)
}

fn up_one<const T: usize>(from: BitBoard<T>, cols: Cols, edges: &Edges<T>) -> BitBoard<T> {
    (from & !edges.top).up(1, cols)
}

fn down_one<const T: usize>(from: BitBoard<T>, cols: Cols, edges: &Edges<T>) -> BitBoard<T> {
    (from & !edges.bottom).down(1, cols)
}

fn get_surrounding_moves<const T: usize>(mut from: BitBoard<T>, cols: Cols, edges: &Edges<T>) -> BitBoard<T> {
    let mut moves = right_one(from, edges) | left_one(from, edges);
    from |= moves;
    moves |= up_one(from, cols, edges);
    moves |= down_one(from, cols, edges); 
    moves
}

impl<const T: usize> Piece<T> for StonePiece {
    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::TeamSymbol(vec![ 'x', 'o' ])
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn generate_lookup_moves(&self, board: &Board<T>, mut from: BitBoard<T>) -> AttackDirections<T> {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];

        let single_moves = get_surrounding_moves(from, cols, edges);
        let all_moves = get_surrounding_moves(single_moves, cols, edges);
        
        vec![ all_moves & !from, single_moves & !from ]
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
        base_moves & !(board.state.all_pieces | board.state.gaps)
    }

    fn make_move(&self, board: &mut Board<T>, action: &Action) {
        if let Some(from) = action.from {
            let from = BitBoard::<T>::from_lsb(from);
            let to = BitBoard::<T>::from_lsb(action.to);

            let piece_type = action.piece_type;
            let team = action.team as usize;
            let other_team = board.state.team_lookup[team] as usize;

            board.history.push(HistoryMove {
                action: Some(*action),
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
                            board.state.teams[team],
                        )),
                        HistoryUpdate::Team(IndexedPreviousBoard(
                            other_team,
                            board.state.teams[other_team],
                        )),
                    ],
                },
            });

            if is_single_move(action) {
                // Single Moves
                
                board.state.pieces[piece_type] |= to;
                board.state.teams[team] |= to;
                board.state.all_pieces |= to;
                board.state.first_move &= !from;
            } else {
                // Double Moves

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
                Some(lookup) => lookup[to.bitscan_reverse() as usize][1],
                None => self.generate_lookup_moves(board, to)[1],
            };

            let to_update = board.state.teams[other_team] & update_radius;

            board.state.teams[other_team] ^= to_update;
            board.state.teams[team] |= to_update;

            update_turns(&mut board.state);
        }
    }
}