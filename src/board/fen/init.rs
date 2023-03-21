use shell_words::split;

use super::super::{
    actions::{Action, HistoryMove, UndoMoveError},
    game::Game,
    pieces::Piece,
    Board, Cols, Rows,
};

impl<const T: usize> Game<T> {
    /// Initialize a `Board` from a given FEN.
    pub fn from_fen(&self, fen: &str) -> Board<T> {
        let args = split(fen).expect(&format!("{fen} cannot be split into arguments."));
        let mut board = Board::from_fen_state(self, (self.rows, self.cols), &args[0]);

        let arg_traits = board
            .game
            .fen_options
            .args
            .iter()
            .collect::<Vec<_>>();
        for (ind, (name, arg_trait)) in arg_traits.iter().enumerate() {
            let arg = &args.get(ind + 1).expect(&format!(
                "argument `{name}` wasn't specified in the given FEN string of '{fen}'."
            )); // We increment the `ind` by one because the first argument is the board state itself, so we're skipping it.

            let decode_result = arg_trait.decode(&mut board, arg);
            if let Err(err) = decode_result {
                panic!(
                    "error at argument `{name}` in the given FEN string of '{fen}': {:?}",
                    err
                );
            }
        }

        let post_process = &board.game.fen_options.post_process;
        post_process.apply(&mut board);

        board
    }

    pub fn default(&self) -> Board<T> {
        self.from_fen(&self.fen_options.default_fen)
    } 
}

impl<'a, const T: usize> Board<'a, T> {
    /// Convert this `Board` to a FEN.
    pub fn to_fen(&self) -> String {
        let mut fen = self.to_fen_state();
        if fen.contains(" ") {
            fen = format!("{:?}", fen); // surround string by quotes
        }

        for (_, arg_trait) in &self.game.fen_options.args {
            let mut fen_arg_representation = arg_trait.encode(self);
            if [" ", "\"", "'", "\\"]
                .iter()
                .any(|el| fen_arg_representation.contains(el))
            {
                fen_arg_representation = format!("{:?}", fen_arg_representation);
            }

            fen.push_str(" ");
            fen.push_str(&fen_arg_representation);
        }

        fen
    }
}
