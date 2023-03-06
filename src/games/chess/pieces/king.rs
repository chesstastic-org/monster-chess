use crate::{
    Action, AttackDirections, BitBoard, Board, Cols, Edges, HistoryMove, IndexedPreviousBoard,
    Piece, PieceType, PreviousBoard,
};

const NORMAL_KING_MOVE: usize = 0;
const CASTLING_MOVE: usize = 1;

const ROOK_PIECE_TYPE: usize = 3;

pub struct KingPiece {
    pub piece_type: PieceType,
}

fn right_one(from: BitBoard, edges: &Edges) -> BitBoard {
    from.right(1) & &!edges.right
}

fn left_one(from: BitBoard, edges: &Edges) -> BitBoard {
    from.left(1) & &!edges.left
}

fn up_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    from.up(1, cols) & &!edges.bottom
}

fn down_one(from: BitBoard, cols: Cols, edges: &Edges) -> BitBoard {
    from.down(1, cols) & &!edges.top
}

enum Direction {
    LEFT,
    RIGHT,
}

impl KingPiece {
    fn make_castling_move(&self, board: &mut Board, action: &Action, from: BitBoard, to: BitBoard) {
        let cols = board.state.cols;
        let left_center = BitBoard::from_lsb(if cols % 2 == 0 {
            (cols / 2) - 1
        } else {
            cols / 2
        });

        let castle_left_king = left_center.left(1);
        let castle_left_rook = left_center;
        let castle_right_king = left_center.right(3);
        let castle_right_rook = left_center.right(2);

        let dir = if action.from > action.to {
            Direction::RIGHT
        } else {
            Direction::LEFT
        };

        let color: usize = if (from & &board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let piece_type = self.get_piece_type();

        let history_move = HistoryMove {
            action: *action,
            teams: vec![IndexedPreviousBoard(color, board.state.teams[color])],
            pieces: vec![
                IndexedPreviousBoard(piece_type, board.state.pieces[piece_type]),
                IndexedPreviousBoard(piece_type, board.state.pieces[ROOK_PIECE_TYPE]),
            ],
            all_pieces: PreviousBoard(board.state.all_pieces),
            first_move: PreviousBoard(board.state.first_move),
        };
        board.state.history.push(history_move);

        board.state.teams[color] ^= &from;
        board.state.teams[color] ^= &to;

        board.state.pieces[piece_type] ^= &from;
        board.state.pieces[ROOK_PIECE_TYPE] ^= &to;

        board.state.all_pieces ^= &from;
        board.state.all_pieces ^= &to;

        match dir {
            Direction::LEFT => {
                board.state.pieces[piece_type] |= &castle_left_king;
                board.state.pieces[ROOK_PIECE_TYPE] |= &castle_left_rook;

                board.state.teams[color] |= &castle_left_king;
                board.state.teams[color] |= &castle_left_rook;

                board.state.all_pieces |= &castle_left_king;
                board.state.all_pieces |= &castle_left_rook;
            }
            Direction::RIGHT => {
                board.state.pieces[piece_type] |= &castle_right_king;
                board.state.pieces[ROOK_PIECE_TYPE] |= &castle_right_rook;

                board.state.teams[color] |= &castle_right_king;
                board.state.teams[color] |= &castle_right_rook;

                board.state.all_pieces |= &castle_right_king;
                board.state.all_pieces |= &castle_right_rook;
            }
        }

        board.state.first_move &= &!from;
        board.state.first_move &= &!to;
    }
}

impl Piece for KingPiece {
    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Self {
            piece_type: self.piece_type,
        })
    }

    fn get_piece_symbol(&self) -> char {
        'k'
    }

    fn generate_lookup_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut moves = right_one(from, edges) | &left_one(from, edges);
        from |= &moves;
        moves |= &up_one(from, cols, edges);
        moves |= &down_one(from, cols, edges);
        vec![moves]
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    #[allow(unused_variables)] 
    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let lookup = self.get_attack_lookup(board, from);
        match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_lookup_moves(board, from)[0],
        }
    }

    fn make_capture_move(&self, board: &mut Board, action: &Action, from: BitBoard, to: BitBoard) {
        if action.info == CASTLING_MOVE {
            self.make_castling_move(board, action, from, to);

            return;
        }

        let color: usize = if (from & &board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let captured_color: usize = if (to & &board.state.teams[0]).is_set() {
            0
        } else {
            1
        };
        let piece_type = self.get_piece_type();
        let mut captured_piece_type: usize = 0;
        for i in 0..(board.game.pieces.len()) {
            if (board.state.pieces[i] & &to).is_set() {
                captured_piece_type = i;
                break;
            }
        }

        let history_move = HistoryMove {
            action: *action,
            teams: vec![
                IndexedPreviousBoard(color, board.state.teams[color]),
                IndexedPreviousBoard(captured_color, board.state.teams[captured_color]),
            ],
            pieces: vec![
                IndexedPreviousBoard(piece_type, board.state.pieces[piece_type]),
                IndexedPreviousBoard(captured_piece_type, board.state.pieces[captured_piece_type]),
            ],
            all_pieces: PreviousBoard(board.state.all_pieces),
            first_move: PreviousBoard(board.state.first_move),
        };
        board.state.history.push(history_move);

        board.state.teams[captured_color] ^= &to;
        board.state.teams[color] ^= &from;
        board.state.teams[color] |= &to;

        board.state.pieces[captured_piece_type] ^= &to;
        board.state.pieces[piece_type] ^= &from;
        board.state.pieces[piece_type] |= &to;

        board.state.all_pieces ^= &from;

        board.state.first_move &= &!from;
        board.state.first_move &= &!to;
        // We actually don't need to swap the blockers. A blocker will still exist on `to`, just not on `from`.
    }

    fn add_actions(&self, actions: &mut Vec<Action>, board: &Board, from: u32, team: u32) {
        let rows = board.state.rows;
        let cols = board.state.cols;
        let board_len = rows * cols;

        let piece_type = self.get_piece_type();
        let from_board = BitBoard::from_lsb(from);
        let bit_actions =
            self.get_moves(board, from_board, team) & &!board.state.teams[team as usize];

        if bit_actions.is_empty() {
            return;
        }

        for bit in bit_actions.iter_one_bits(board_len) {
            actions.push(Action {
                from,
                to: bit,
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

        let bottom_row = board.state.edges[0].bottom;
        let team_board = board.state.teams[team as usize];
        let first_move = board.state.first_move;

        if (from_board & &!bottom_row & &!first_move).is_empty() {
            return;
        }

        let rooks = board.state.pieces[ROOK_PIECE_TYPE] & &team_board & &first_move & &bottom_row;

        /*
            FRC Castling brings us to the same positions that traditional chess castling would.
            To extend for bigger board sizes, we'll have to create two castling spots for left and right.
            We'll define these positions in terms of `left_center`.

            `left_center` is the centermost point on the bottom row.
            If there are two center points, `left_center` is the center point on the left.
        */

        let left_center = BitBoard::from_lsb(if board.state.cols % 2 == 0 {
            (cols / 2) - 1
        } else {
            cols / 2
        });

        let castle_left_king = left_center.left(1);
        let castle_left_rook = left_center;
        let castle_right_king = left_center.right(3);
        let castle_right_rook = left_center.right(2);

        let castle_left = castle_left_king & &castle_left_rook;
        let castle_right = castle_right_king & &castle_right_rook;

        for rook in rooks.iter_one_bits(board_len) {
            let rook_board = BitBoard::from_lsb(rook);

            /*
                For any given row, the leftmost point is lowest, and the rightmost point is highest.
                Therefore, for castling, if `from > rook_pos`, it's to the right of it, and vice-versa.
            */

            let dir = if from > rook {
                Direction::RIGHT
            } else {
                Direction::LEFT
            };

            let castling_spots = match dir {
                Direction::LEFT => castle_left,
                Direction::RIGHT => castle_right,
            };

            let in_between = match dir {
                Direction::LEFT => BitBoard::starting_at_lsb(from, rook - from + 1),
                Direction::RIGHT => BitBoard::starting_at_lsb(rook, from - rook + 1),
            };

            let all_spots = (castling_spots & &in_between) & &!(from_board & &rook_board);

            /*
                We're not checking if the squares are attacked here, because if the squares aren't empty, we won't need to.
                Calculating which squares for castling are attacked is semi-expensive, so this will avoid it if needs be.
            */
            if (all_spots & &board.state.all_pieces).is_set() {
                return;
            }

            let attack_mask = board.get_move_mask(board.get_next_team(team));
            if (all_spots & &attack_mask).is_set() {
                return;
            }

            actions.push(Action {
                from,
                to: rook,
                info: NORMAL_KING_MOVE,
                piece_type,
            });
        }
    }
}
