use super::{actions::Action, fen::FenOptions, pieces::Piece, Board, Rows, Cols};

pub trait MoveController<const T: usize> {
    fn transform_moves(&self, board: &mut Board<T>, mode: u32, actions: Vec<Option<Action>>) -> Vec<Option<Action>>;
    fn is_legal(&self, board: &mut Board<T>, action: &Option<Action>) -> bool;
    fn use_psuedolegal(&self) -> bool;

    fn encode_action(&self, board: &Board<T>, action: &Option<Action>) -> Vec<String>;

    fn decode_action(&self, board: &mut Board<T>, action: &str, mode: u32) -> Option<Option<Action>> {
        board.generate_legal_moves(mode)
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

pub trait Resolution<const T: usize> {
    fn resolution(&self, board: &mut Board<T>, legal_moves: &Vec<Option<Action>>) -> GameResults;
}

pub struct Game<const T: usize> {
    pub pieces: Vec<&'static dyn Piece<T>>,
    pub controller: Box<dyn MoveController<T>>,
    pub resolution: Box<dyn Resolution<T>>,
    pub fen_options: FenOptions<T>,
    pub name: String,
    pub teams: u32,
    pub turns: u32,
    pub rows: Rows,
    pub cols: Cols
}
