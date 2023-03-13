use crate::{
    bitset::Direction,
    board::{
        actions::{
            Action, HistoryMove, HistoryState, HistoryUpdate, IndexedPreviousBoard, PreviousBoard,
        },
        edges::Edges,
        pieces::{Piece, PieceSymbol},
        AttackDirections, BitBoard, Board, Cols, PieceType,
    },
    games::chess::game::ATTACKS_MODE,
};

const NORMAL_KING_MOVE: usize = 0;
const CASTLING_MOVE: usize = 1;

const ROOK_PIECE_TYPE: usize = 3;

pub struct KingPiece;

fn right_one(from: BitBoard, edges: &Edges) -> BitBoard {
    (from & !edges.right).right(1) & !edges.left
}

fn left_one(from: BitBoard, edges: &Edges) -> BitBoard {
    (from & !edges.left).left(1) & !edges.right
}

fn up_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    (from & !edges.top).up(1, cols)
}

fn down_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    (from & !edges.bottom).down(1, cols)
}

impl KingPiece {
    fn make_castling_move(&self, board: &mut Board, action: &Action, from: BitBoard, to: BitBoard) {
        let cols = board.state.cols;
        let mut left_center = BitBoard::from_lsb(if cols % 2 == 0 {
            (cols / 2) - 1
        } else {
            cols / 2
        });

        if action.team == 0 {
            left_center = left_center.down(7, cols);
        }

        let castle_left_king = left_center.left(1);
        let castle_left_rook = left_center;
        let castle_right_king = left_center.right(3);
        let castle_right_rook = left_center.right(2);

        let dir = if action.from < action.to {
            Direction::RIGHT
        } else {
            Direction::LEFT
        };

        let color: usize = action.team as usize;
        let piece_type = action.piece_type;

        let history_move = HistoryMove {
            action: *action,
            state: HistoryState::Any {
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
                updates: vec![
                    HistoryUpdate::Team(IndexedPreviousBoard(color, board.state.teams[color])),
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        piece_type,
                        board.state.pieces[piece_type],
                    )),
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        ROOK_PIECE_TYPE,
                        board.state.pieces[ROOK_PIECE_TYPE],
                    )),
                ],
            },
        };
        board.history.push(history_move);

        board.state.teams[color] ^= from;
        board.state.teams[color] ^= to;

        board.state.pieces[piece_type] ^= from;
        board.state.pieces[ROOK_PIECE_TYPE] ^= to;

        board.state.all_pieces ^= from;
        board.state.all_pieces ^= to;

        match dir {
            Direction::LEFT => {
                board.state.pieces[piece_type] |= castle_left_king;
                board.state.pieces[ROOK_PIECE_TYPE] |= castle_left_rook;

                board.state.teams[color] |= castle_left_king;
                board.state.teams[color] |= castle_left_rook;

                board.state.all_pieces |= castle_left_king;
                board.state.all_pieces |= castle_left_rook;
            }
            Direction::RIGHT => {
                board.state.pieces[piece_type] |= castle_right_king;
                board.state.pieces[ROOK_PIECE_TYPE] |= castle_right_rook;

                board.state.teams[color] |= castle_right_king;
                board.state.teams[color] |= castle_right_rook;

                board.state.all_pieces |= castle_right_king;
                board.state.all_pieces |= castle_right_rook;
            }
        }

        board.state.first_move &= !from;
        board.state.first_move &= !to;
    }
}

impl Piece for KingPiece {
    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('k')
    }

    fn generate_lookup_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut moves = right_one(from, edges) | left_one(from, edges);
        from |= moves;
        moves |= up_one(from, cols, edges);
        moves |= down_one(from, cols, edges);
        vec![moves]
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn can_move_mask(
        &self,
        board: &Board,
        from: BitBoard,
        from_bit: u32,
        piece_type: usize,
        team: u32,
        mode: u32,
        to: BitBoard,
    ) -> BitBoard {
        self.get_attack_lookup(board, piece_type).unwrap()[from_bit as usize][0]
    }

    #[allow(unused_variables)]
    fn get_moves(
        &self,
        board: &Board,
        from: BitBoard,
        piece_type: usize,
        team: u32,
        mode: u32,
    ) -> BitBoard {
        let lookup = self.get_attack_lookup(board, piece_type);
        match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_lookup_moves(board, from)[0],
        }
    }

    fn make_capture_move(
        &self,
        board: &mut Board,
        action: &Action,
        piece_type: usize,
        from: BitBoard,
        to: BitBoard,
    ) {
        if action.info == CASTLING_MOVE {
            self.make_castling_move(board, action, from, to);

            return;
        }

        let color: usize = action.team as usize;
        let captured_color: usize = if (to & board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let mut captured_piece_type: usize = 0;
        for i in 0..(board.game.pieces.len()) {
            if (board.state.pieces[i] & to).is_set() {
                captured_piece_type = i;
                break;
            }
        }

        let history_move = HistoryMove {
            action: *action,
            state: HistoryState::Any {
                all_pieces: PreviousBoard(board.state.all_pieces),
                first_move: PreviousBoard(board.state.first_move),
                updates: vec![
                    HistoryUpdate::Team(IndexedPreviousBoard(color, board.state.teams[color])),
                    HistoryUpdate::Team(IndexedPreviousBoard(
                        captured_color,
                        board.state.teams[captured_color],
                    )),
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        piece_type,
                        board.state.pieces[piece_type],
                    )),
                    HistoryUpdate::Piece(IndexedPreviousBoard(
                        captured_piece_type,
                        board.state.pieces[captured_piece_type],
                    )),
                ],
            },
        };
        board.history.push(history_move);

        board.state.teams[captured_color] ^= to;
        board.state.teams[color] ^= from;
        board.state.teams[color] |= to;

        board.state.pieces[captured_piece_type] ^= to;
        board.state.pieces[piece_type] ^= from;
        board.state.pieces[piece_type] |= to;

        board.state.all_pieces ^= from;

        board.state.first_move &= !from;
        board.state.first_move &= !to;
        // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
    }

    fn add_actions(
        &self,
        actions: &mut Vec<Action>,
        board: &Board,
        piece_type: usize,
        from: u32,
        team: u32,
        mode: u32,
    ) {
        let rows = board.state.rows;
        let cols = board.state.cols;
        let board_len = board.state.squares;

        let from_board = BitBoard::from_lsb(from);
        let bit_actions = self.get_moves(board, from_board, piece_type, team, mode)
            & !board.state.teams[team as usize];

        if bit_actions.is_empty() {
            return;
        }

        for bit in bit_actions.iter_one_bits(board_len) {
            actions.push(Action {
                from,
                to: bit,
                team,
                info: NORMAL_KING_MOVE,
                piece_type,
            });
        }

        /*
            Castling (Fischer Random)

            For convenience's sake, I'm only going to support castling on the bottom row
            This may break some chess variants, but it's good enough for now.
            If necessary, I can remake this.
        */

        let bottom_row = match team {
            0 => board.state.edges[0].bottom,
            1 => board.state.edges[0].top,
            _ => board.state.edges[0].bottom,
        };
        let team_board = board.state.teams[team as usize];
        let first_move = board.state.first_move;

        if (from_board & bottom_row & first_move).is_empty() {
            return;
        }

        let rooks = board.state.pieces[ROOK_PIECE_TYPE] & team_board & first_move & bottom_row;

        /*
            FRC Castling brings us to the same positions that traditional chess castling would.
            To extend for bigger board sizes, we'll have to create two castling spots for left and right.
            We'll define these positions in terms of `left_center`.

            `left_center` is the centermost point on the bottom row.
            If there are two center points, `left_center` is the center point on the left.
        */

        let mut left_center = BitBoard::from_lsb(if board.state.cols % 2 == 0 {
            (cols / 2) - 1
        } else {
            cols / 2
        });

        if team == 0 {
            left_center = left_center.down(7, cols);
        }

        let castle_left_king = left_center.left(1);
        let castle_left_rook = left_center;
        let castle_right_king = left_center.right(3);
        let castle_right_rook = left_center.right(2);

        let castle_left = castle_left_king | castle_left_rook;
        let castle_right = castle_right_king | castle_right_rook;

        for rook in rooks.iter_one_bits(board_len) {
            let rook_board = BitBoard::from_lsb(rook);

            /*
                For any given row, the leftmost point is lowest, and the rightmost point is highest.
                Therefore, for castling, if `from > rook_pos`, it's to the right of it, and vice-versa.
            */

            let dir = if from > rook {
                Direction::LEFT
            } else {
                Direction::RIGHT
            };

            let castling_spots = match dir {
                Direction::LEFT => castle_left,
                Direction::RIGHT => castle_right,
            };

            let in_between = match dir {
                Direction::LEFT => BitBoard::starting_at_lsb(rook, from - rook + 1),
                Direction::RIGHT => BitBoard::starting_at_lsb(from, rook - from + 1),
            };

            let all_spots = (castling_spots | in_between) & !(from_board | rook_board);

            /*
                We're not checking if the squares are attacked here, because if the squares aren't empty, we won't need to.
                Calculating which squares for castling are attacked is semi-expensive, so this will avoid it if needs be.
            */
            if (all_spots & board.state.all_pieces).is_set() {
                continue;
            }

            let king_dest = match dir {
                Direction::LEFT => castle_left_king,
                Direction::RIGHT => castle_right_king,
            }
            .bitscan_forward();

            let between_king_dest = match dir {
                Direction::LEFT => BitBoard::starting_at_lsb(king_dest, from - king_dest + 1),
                Direction::RIGHT => BitBoard::starting_at_lsb(from, king_dest - from + 1),
            };

            let attack_mask = board.get_move_mask(board.get_next_team(team), ATTACKS_MODE);

            if (between_king_dest & attack_mask).is_set() {
                continue;
            }

            actions.push(Action {
                from,
                to: rook,
                team,
                info: CASTLING_MOVE,
                piece_type,
            });
        }
    }
}
