use std::fmt::Debug;

pub const NORMAL_MODE: u16 = 0;

use super::{actions::{Action, ActionInfo, TheoreticalAction, Move, TheoreticalMove, HistoryMove, TurnUpdate, CounterUpdate}, fen::FenOptions, pieces::Piece, Board, Rows, Cols, zobrist::ZobristHashTable, BoardState};

pub fn get_theoretical_moves_bound<const T: usize>(board: &Board<T>, max_info: ActionInfo, can_pass: bool) -> Vec<TheoreticalMove> {
    let mut theoretical_moves = Vec::with_capacity(((
        (board.game.squares) + 1 * board.game.squares
    ) as usize * (max_info as usize)) + (can_pass as usize));

    if can_pass {
        theoretical_moves.push(TheoreticalMove::Pass);
    }

    for to in 0..board.game.squares {
        for info in 0..max_info {
            theoretical_moves.push(TheoreticalMove::Action(TheoreticalAction {
                from: None,
                to,
                info
            }));
        }

        for from in 0..board.game.squares {
            for info in 0..max_info {
                theoretical_moves.push(TheoreticalMove::Action(TheoreticalAction {
                    from: Some(from),
                    to,
                    info
                }));
            }
        }
    }

    theoretical_moves
}

pub struct MoveLegalResponse<const T: usize> {
    pub is_legal: bool,
    pub made_move: Option<Option<HistoryMove<T>>>
}

pub trait MoveController<const T: usize> : Debug + Send + Sync {
    fn transform_moves(&self, board: &mut Board<T>, mode: u16, actions: Vec<Move>) -> Vec<Move>;
    fn is_legal(&self, board: &mut Board<T>, action: &Move, unmake_move: bool) -> MoveLegalResponse<T>;
    fn use_pseudolegal(&self) -> bool;

    fn add_moves(&self, _board: &Board<T>, _actions: &mut Vec<Move>) {}
    fn make_drop_move(&self, _board: &mut Board<T>, _action: &Action) -> Option<HistoryMove<T>> {
        panic!("Drop moves aren't supported. Make sure to override `make_drop_move` in your game's MoveController to support them.");
    }

    fn encode_action(&self, board: &Board<T>, action: &Move) -> Vec<String>;
    fn decode_action(&self, board: &mut Board<T>, action: &str, mode: u16) -> Option<Move> {
        board.generate_moves(mode)
            .iter()
            .find(|el| self.encode_action(board, el).contains(&action.to_string()))
            .map(|el| el.clone())
    }

    /// Is fetches all theoretically possible moves. These moves might not even be actually possible, they're just used for indexing.
    /// Ideally, this should be a list of all actually possible moves, but an upper bound is fine.
    fn get_theoretical_moves(&self, board: &Board<T>) -> Vec<TheoreticalMove>;

    fn find_theoretical_action(&self, board: &Board<T>, action: TheoreticalMove) -> Option<Move> {
        match action {
            TheoreticalMove::Action(action) => {
                match action.from {
                    Some(from) => {
                        let moves = board.generate_from_moves(NORMAL_MODE, from);
                        for true_action in moves {
                            if let Move::Action(true_action) = true_action {
                                if true_action.to == action.to && true_action.info == action.info {
                                    return Some(Move::Action(true_action));
                                }
                            }
                        }

                        None
                    }
                    None => {
                        let moves = board.generate_drop_moves(NORMAL_MODE);
                        for true_action in moves {
                            if let Move::Action(true_action) = true_action {
                                if true_action.to == action.to && true_action.info == action.info {
                                    return Some(Move::Action(true_action));
                                }
                            }
                        }

                        None
                    }
                }
            }
            TheoreticalMove::Pass => Some(Move::Pass)
        }
    }

    fn update(&self, _action: &Move, _state: &BoardState<T>) -> TurnUpdate {
        TurnUpdate {
            turns: CounterUpdate::Next,
            sub_moves: CounterUpdate::Next,
            full_moves: CounterUpdate::Next
        }
    }

    /// An upper-bound of all max available moves from any given position.
    fn get_max_available_moves(&self) -> u32;
}

#[derive(Debug, Copy, Clone)]
pub enum GameResults {
    Win(u16),
    Draw,
    Ongoing
}

pub trait Resolution<const T: usize> : Debug + Send + Sync {
    fn resolve(&self, board: &mut Board<T>, legal_moves: &[Move]) -> GameResults;
}

pub trait ZobristController<const T: usize> : Debug + Send + Sync {
    fn get_extra_hashes(&self) -> usize { 0 }
    fn apply(&self, _hash: &mut u64, _zobrist: &mut ZobristHashTable<T>, _board: &mut Board<T>) {}
}

#[derive(Debug)]
pub struct DefaultZobristController<const T: usize>;
impl<const T: usize> ZobristController<T> for DefaultZobristController<T> {}

#[derive(Debug)]
pub struct Game<const T: usize> {
    pub pieces: Vec<&'static dyn Piece<T>>,
    pub controller: Box<dyn MoveController<T>>,
    pub resolution: Box<dyn Resolution<T>>,
    pub fen_options: FenOptions<T>,
    pub name: String,
    pub teams: u16,
    pub turns: u16,
    pub rows: Rows,
    pub cols: Cols,
    pub squares: u16,
    pub saved_last_moves: u16,
    /// Anything not covered by first_moves, pieces, and gaps should be zobrist_info
    pub zobrist_controller: Box<dyn ZobristController<T>>,
    pub zobrist: ZobristHashTable<T>
}

impl<const T: usize> PartialEq for Game<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<const T: usize> Eq for Game<T> {}