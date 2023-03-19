use std::hash::{Hash, Hasher};
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use arrayvec::ArrayVec;
use std::collections::VecDeque;
use fastrand;

use crate::bitboard::BitBoard;

use super::actions::Move;
use super::{
    actions::{Action, HistoryMove, UndoMoveError, HistoryState},
    edges::{generate_edge_list, Edges},
    game::Game,
    pieces::Piece, zobrist::ZobristHashTable,
};

pub type PieceType = usize;

/// I doubt anyone would be practically creating boards of 4,294,967,296 x 4,294,967,296.
/// However, storing these as u32s makes it much easier to interface the bitboards with (particularly, shifting bits with them.)
pub type Rows = u16;
pub type Cols = u16;

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

    pub moving_team: u16,
    pub current_turn: u16,

    /// Full Moves is one full move, where each team completes one sub move (or all of their turns)
    pub full_moves: u16,

    /// Sub Moves is one sub move, where a single team completes all of their turns
    pub sub_moves: u16,

    /// A turn is a single movement of a piece. Chess only has one turn, but games like duck chess have two (move the piece, then move the duck)
    pub turns: u16,

    /// Edges is a list of "boundary bitboards" for validating the movement of delta pieces (pieces that move in a fixed way everytime)
    pub edges: Vec<Edges<T>>,
    pub rows: Rows,
    pub cols: Cols,
    pub squares: u16,

    pub turn_lookup: ArrayVec<u16, 16>,
    pub team_lookup: ArrayVec<u16, 16>,
    pub turn_reverse_lookup: ArrayVec<u16, 16>,
    pub team_reverse_lookup: ArrayVec<u16, 16>,
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

#[derive(Debug, Clone, Eq)]
pub struct Board<'a, const T: usize> {
    pub state: BoardState<T>,
    pub game: &'a Game<T>,
    pub attack_lookup: Vec<AttackLookup<T>>,
    pub history: VecDeque<Move>
}

impl<'a, const T: usize> Display for Board<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Board<{}>({})", self.game.name, self.to_fen())
    }
}

impl<'a, const T: usize> Hash for Board<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.game.zobrist.compute(self).hash(state);
    }
}

impl<'a, const T: usize> PartialEq for Board<'a, T> {
    fn eq(&self, rhs: &Board<'a, T>) -> bool {
        self.game.zobrist.compute(self) == self.game.zobrist.compute(rhs)
    }
}

fn generate_forward_lookup(count: u16) -> ArrayVec<u16, 16> {
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

fn generate_reverse_lookup(count: u16) -> ArrayVec<u16, 16> {
    let mut lookup = ArrayVec::new();
    for i in 0..count {
        let i = i as i16;
        let mut new_val = i - 1;
        if new_val < 0 {
            new_val = (count - 1) as i16;
        }
        lookup.push(new_val as u16);
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
            history: VecDeque::with_capacity(1),
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
            }
        };

        board.generate_lookups();

        board
    }

    pub fn get_move_mask(&self, team: u16, mode: u16) -> BitBoard<T> {
        let board_len = self.state.squares;
        let mut bitboard = BitBoard::new();

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];

            for bit in board.iter_set_bits(board_len) {
                bitboard |= piece.get_moves(self, BitBoard::from_lsb(bit), ind, team, mode);
            }
        }

        bitboard
    }

    pub fn can_move(&self, team: u16, target: BitBoard<T>, mode: u16) -> bool {
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

    pub fn generate_from_moves(&self, mode: u16, from: u16) -> Vec<Move> {
        let team = self.state.moving_team;
        let from_board = BitBoard::from_lsb(from);
        let mut piece_type = usize::MAX;
        for i in 0..self.game.pieces.len() {
            if (from_board & self.state.pieces[i]).is_set() {
                piece_type = i;
                break;
            }
        }

        let mut actions: Vec<Move> = Vec::with_capacity(self.state.squares as usize);

        let piece = &self.game.pieces[piece_type];
        piece.add_actions(&mut actions, self, piece_type, from, team, mode);

        vec![]
    }

    pub fn generate_drop_moves(&self, mode: u16) -> Vec<Move> {
        let team = self.state.moving_team;
        let mut actions: Vec<Move> = Vec::with_capacity(self.state.squares as usize);

        self.game.controller.add_moves(self, &mut actions);

        vec![]
    }

    pub fn generate_moves(&self, mode: u16) -> Vec<Move> {
        let board_len = self.state.squares;
        let mut actions: Vec<Move> = Vec::with_capacity(board_len as usize);

        let team = self.state.moving_team;

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];

            for bit in board.iter_set_bits(board_len) {
                piece.add_actions(&mut actions, self, ind, bit, team, mode);
            }
        }

        self.game.controller.add_moves(self, &mut actions);

        actions
    }

    /*
        Don't use when writing an engine directly; use `generate_moves` and `move_restrictions.is_legal` to avoid extra legality checks during pruning.
    */
    pub fn generate_legal_moves(&mut self, mode: u16) -> Vec<Move> {
        let moves = self.generate_moves(mode);
        self.game.controller.transform_moves(self, mode, moves)
    }

    pub fn get_next_team(&self, mut team: u16) -> u16 {
        team += 1;

        if team >= self.state.teams.len() as u16 {
            0
        } else {
            team
        }
    }

    pub fn get_previous_team(&self, mut team: u16) -> u16 {
        team -= 1;

        if team == u16::MAX {
            (self.state.teams.len() - 1) as u16
        } else {
            team
        }
    }

    pub fn retrieve_first_history_move(&mut self, action: Move) -> Option<Move> {
        if self.game.saved_last_moves == 0 {
            return None;
        }

        self.history.push_back(action);

        if self.history.len() > self.game.saved_last_moves.into() {
            self.history.pop_front()
        } else {
            None
        }
    }

    pub fn reset_first_history_move(&mut self, first_history_move: Option<Move>) {
        if self.game.saved_last_moves == 0 {
            return;
        }

        if let Some(first_history_move) = first_history_move {
            self.history.pop_back();
            self.history.push_front(first_history_move);
        }
    }

    pub fn make_move(&mut self, action: &Move) -> Option<HistoryMove<T>> {
        match action {
            Move::Action(action) => {
                if action.from.is_some() {
                    self.game.pieces[action.piece_type].make_move(self, action)
                } else {
                    self.game.controller.make_drop_move(self, action)
                }
            }
            Move::Pass => {
                let history = HistoryMove {
                    action: Move::Pass,
                    state: HistoryState::None,
                    first_history_move: self.retrieve_first_history_move(Move::Pass)
                };
                update_turns(&mut self.state);
                Some(history)
            }
        }
    }

    #[inline(never)]
    pub fn undo_move(&mut self, undo: Option<HistoryMove<T>>) {
        match undo {
            Some(history_move) => {
                self.reset_first_history_move(history_move.first_history_move);
                match history_move.action {
                    Move::Action(history_action) => {
                        self.game.pieces[history_action.piece_type].undo_move(
                            &mut self.state,
                            self.game,
                            &history_move
                        );
                    }
                    Move::Pass => {
                        reverse_turns(&mut self.state, &self.game);
                    }
                };
            }
            None => {
                // We panic instead of making it an error because this is an incredible unlikely error that almost 
                // certainly won't happen in monster-chess's code, and consumers would easily be able 
                // to come across and handle this.
                // It isn't worth the effort having to propagate the error through so many functions.

                panic!("Can't undo move when `undo` is None.");
            }
        }
    }
}