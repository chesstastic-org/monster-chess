use std::hash::Hash;

use arrayvec::ArrayVec;

use crate::bitboard::BitBoard;

use super::{
    actions::{Action, HistoryMove, UndoMoveError, HistoryState},
    edges::{generate_edge_list, Edges},
    game::Game,
    pieces::Piece,
};

pub type PieceType = usize;

/// I doubt anyone would be practically creating boards of 4,294,967,296 x 4,294,967,296.
/// However, storing these as u32s makes it much easier to interface the bitboards with (particularly, shifting bits with them.)
pub type Rows = u32;
pub type Cols = u32;

pub fn update_turns<const T: usize>(state: &mut BoardState<T>) {
    state.turns += 1;
    state.current_turn = state.turn_lookup[state.current_turn as usize];
    if state.current_turn == 0 {
        state.sub_moves += 1;

        if state.moving_team == 0 {
            state.full_moves += 1;
        }
        state.moving_team = state.team_lookup[state.moving_team as usize];
    };
}

pub fn reverse_turns<const T: usize>(state: &mut BoardState<T>, game: &Game<T>) {
    state.turns -= 1;
    state.current_turn = state.turn_reverse_lookup[state.current_turn as usize];
    if state.current_turn == game.turns - 1 {
        state.moving_team = state.team_reverse_lookup[state.moving_team as usize];
        state.sub_moves -= 1;

        if state.moving_team == 0 {
            state.full_moves -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardState<const T: usize> {
    /// All Pieces is a BitBoard of all pieces, because keeping this bitboard ready makes it much easier to calculate movement for slider pieces.
    pub all_pieces: BitBoard<T>,
    /// Walls is all "gaps" in the board that can't be moved to, or moved through.
    pub gaps: BitBoard<T>,
    pub first_move: BitBoard<T>,
    pub pieces: Vec<BitBoard<T>>,
    pub teams: Vec<BitBoard<T>>,

    pub moving_team: u32,
    pub current_turn: u32,

    /// Full Moves is one full move, where each team completes one sub move (or all of their turns)
    pub full_moves: u32,

    /// Sub Moves is one sub move, where a single team completes all of their turns
    pub sub_moves: u32,

    /// A turn is a single movement of a piece. Chess only has one turn, but games like duck chess have two (move the piece, then move the duck)
    pub turns: u32,

    /// Edges is a list of "boundary bitboards" for validating the movement of delta pieces (pieces that move in a fixed way everytime)
    pub edges: Vec<Edges<T>>,
    pub rows: Rows,
    pub cols: Cols,
    pub squares: u32,

    pub turn_lookup: ArrayVec<u32, 16>,
    pub team_lookup: ArrayVec<u32, 16>,
    pub turn_reverse_lookup: ArrayVec<u32, 16>,
    pub team_reverse_lookup: ArrayVec<u32, 16>,
}

impl<const T: usize> BoardState<T> {
    pub fn get_piece_team_board(&self, piece: usize, team: usize) -> BitBoard<T> {
        self.pieces[piece] & self.teams[team]
    }
}

pub type AttackDirections<const T: usize> = Vec<BitBoard<T>>;

/// AttackLookup is indexed by the index of the Most Significant 1-Bit.
///
/// It stores an `AttackDirections` (alias for `Vec<BitBoard>`).
///     For pieces that always move the same way (like Delta Pieces), only the first slot of this AttackDirections is used, because there's no directions.
///     For slider pieces, there are different indexes for specific ray directions of it.

pub type AttackLookup<const T: usize> = Vec<AttackDirections<T>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<'a, const T: usize> {
    pub state: BoardState<T>,
    pub game: &'a Game<T>,
    pub attack_lookup: Vec<AttackLookup<T>>,
    pub history: ArrayVec<HistoryMove<T>, 2048>,
}



fn generate_forward_lookup(count: u32) -> ArrayVec<u32, 16> {
    let mut lookup = ArrayVec::new();
    for i in 0..count {
        let mut new_val = i + 1;
        if new_val >= count {
            new_val = 0;
        }
        lookup.push(new_val);
    }
    lookup
}

fn generate_reverse_lookup(count: u32) -> ArrayVec<u32, 16> {
    let mut lookup = ArrayVec::new();
    for i in 0..count {
        let i = i as i32;
        let mut new_val = i - 1;
        if new_val < 0 {
            new_val = (count - 1) as i32;
        }
        lookup.push(new_val as u32);
    }
    lookup
}

impl<'a, const T: usize> Board<'a, T> {
    pub fn empty(game: &'a Game<T>, (rows, cols): (Rows, Cols)) -> Board<'a, T> {
        let pieces_state = game
            .pieces
            .iter()
            .map(|_| BitBoard::new())
            .collect::<Vec<_>>()
            .clone();

        let turn_lookup = generate_forward_lookup(game.turns);
        let turn_reverse_lookup = generate_reverse_lookup(game.turns);
        let team_lookup = generate_forward_lookup(game.teams);
        let team_reverse_lookup = generate_reverse_lookup(game.teams);

        let mut board = Board {
            attack_lookup: vec![],
            game,
            history: ArrayVec::new(),
            state: BoardState {
                all_pieces: BitBoard::new(),
                first_move: BitBoard::new(),
                gaps: BitBoard::new(),
                pieces: pieces_state.clone(),
                teams: (0..game.teams).map(|_| BitBoard::new()).collect::<Vec<_>>(),
                edges: generate_edge_list(rows, cols),
                cols,
                rows,
                squares: rows * cols,
                moving_team: 0,
                current_turn: 0,
                full_moves: 0,
                sub_moves: 0,
                turns: 0,
                team_lookup,
                team_reverse_lookup,
                turn_lookup,
                turn_reverse_lookup,
            },
        };

        board.generate_lookups();

        board
    }

    pub fn get_move_mask(&self, team: u32, mode: u32) -> BitBoard<T> {
        let board_len = self.state.squares;
        let mut bitboard = BitBoard::new();

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];

            for bit in board.iter_set_bits(board_len as u32) {
                bitboard |= piece.get_moves(self, BitBoard::from_lsb(bit), ind, team, mode);
            }
        }

        bitboard
    }

    pub fn can_move(&self, team: u32, target: BitBoard<T>, mode: u32) -> bool {
        let board_len = self.state.squares;

        let team = self.state.moving_team;
        let mut mask = BitBoard::new();

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];

            for bit in board.iter_set_bits(board_len) {
                mask |= piece.can_move_mask(
                    self,
                    BitBoard::from_lsb(bit),
                    bit,
                    ind,
                    team,
                    mode,
                    target,
                );
            }
        }

        (mask & target).is_set()
    }

    pub fn generate_moves(&self, mode: u32) -> Vec<Option<Action>> {
        let board_len = self.state.squares;
        let mut actions: Vec<Option<Action>> = Vec::with_capacity(board_len as usize);

        let team = self.state.moving_team;

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];

            for bit in board.iter_set_bits(board_len as u32) {
                piece.add_actions(&mut actions, self, ind, bit, team, mode);
            }
        }

        self.game.controller.add_moves(self, &mut actions);

        actions
    }

    /*
        Don't use when writing an engine directly; use `generate_moves` and `move_restrictions.is_legal` to avoid extra legality checks during pruning.
    */
    pub fn generate_legal_moves(&mut self, mode: u32) -> Vec<Option<Action>> {
        let moves = self.generate_moves(mode);
        self.game.controller.transform_moves(self, mode, moves)
    }

    pub fn get_next_team(&self, mut team: u32) -> u32 {
        team += 1;

        if team >= self.state.teams.len() as u32 {
            0
        } else {
            team
        }
    }

    pub fn get_previous_team(&self, mut team: u32) -> u32 {
        team -= 1;

        if team == u32::MAX {
            (self.state.teams.len() - 1) as u32
        } else {
            team
        }
    }

    pub fn make_move(&mut self, action: &Option<Action>) {
        match action {
            Some(action) => {
                if action.from.is_some() {
                    self.game.pieces[action.piece_type].make_move(self, action);
                } else {
                    self.game.controller.make_drop_move(self, action);
                }
            }
            None => {
                self.history.push(HistoryMove {
                    action: None,
                    state: HistoryState::None
                });
                update_turns(&mut self.state);
                return;
            }
        }
    }

    #[inline(never)]
    pub fn undo_move(&mut self) {
        match self.history.last() {
            Some(history_move) => {
                match history_move.action {
                    Some(history_action) => {
                        self.game.pieces[history_action.piece_type].undo_move(
                            &mut self.state,
                            self.game,
                            history_move,
                        );
                    }
                    None => {
                        reverse_turns(&mut self.state, &self.game);
                    }
                };
                self.history.pop();
            }
            None => {
                // We panic instead of making it an error because this is an incredible unlikely error that almost 
                // certainly won't happen in monster-chess's code, and consumers would easily be able 
                // to come across and handle this.
                // It isn't worth the effort having to propagate the error through so many functions.

                panic!("Can't undo move when there's no history moves.");
            }
        }
    }
}
