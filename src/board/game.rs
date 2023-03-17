use std::fmt::Debug;

pub const NORMAL_MODE: u32 = 0;

use super::{actions::Action, fen::FenOptions, pieces::Piece, Board, Rows, Cols, zobrist::ZobristHashTable};

pub trait MoveController<const T: usize> : Debug + Send + Sync {
    fn transform_moves(&self, board: &mut Board<T>, mode: u32, actions: Vec<Option<Action>>) -> Vec<Option<Action>>;
    fn is_legal(&self, board: &mut Board<T>, action: &Option<Action>) -> bool;
    fn use_pseudolegal(&self) -> bool;

    fn add_moves(&self, board: &Board<T>, actions: &mut Vec<Option<Action>>) {}
    fn make_drop_move(&self, board: &mut Board<T>, action: &Action) {
        panic!("Drop moves aren't supported. Make sure to override `make_drop_move` in your game's MoveController to support them.");
    }

    fn encode_action(&self, board: &Board<T>, action: &Option<Action>) -> Vec<String>;
    fn decode_action(&self, board: &mut Board<T>, action: &str, mode: u32) -> Option<Option<Action>> {
        board.generate_moves(mode)
            .iter()
            .find(|el| self.encode_action(board, el).contains(&action.to_string()))
            .map(|el| el.clone())
    }
}

pub enum GameResults {
    Win(u32),
    Draw,
    Ongoing
}

pub trait Resolution<const T: usize> : Debug + Send + Sync {
    fn resolution(&self, board: &mut Board<T>, legal_moves: &Vec<Option<Action>>) -> GameResults;
}

pub trait ZobristController<const T: usize> : Debug + Send + Sync {
    fn get_extra_hashes(&self) -> usize { 0 }
    fn apply(&self, hash: &mut u64, zobrist: &mut ZobristHashTable<T>, board: &mut Board<T>) {}
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
    pub teams: u32,
    pub turns: u32,
    pub rows: Rows,
    pub cols: Cols,
    pub squares: u32,
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