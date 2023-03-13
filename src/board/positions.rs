use super::{actions::{HistoryMove, Action, UndoMoveError}, edges::Edges, pieces::Piece, game::Game, Board};

const COLS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl<'a> Board<'a> {
    pub fn encode_position(&self, pos: u32) -> String {
        let base_row = pos / self.state.cols;
        let col = pos - (self.state.cols * base_row);
        let row = self.state.rows - base_row;

        return format!("{}{}", COLS[col as usize], row);
    }

    pub fn decode_position(&self, text: String) -> Result<u32, String> {
        let col = text
            .chars()
            .nth(0)
            .expect(&format!("'{text}' has no column char"));

        let col = COLS
            .iter()
            .position(|el| el == &col)
            .ok_or(format!("Cannot find board column from char '{col}'"))? as u32;
        let row = self.state.rows
            - text[1..]
                .parse::<u32>()
                .map_err(|_| format!("Cannot find board row from char '{}'", &text[1..]))?;

        Ok(col + (self.state.cols * row))
    }

    pub fn encode_action(&self, action: &Action) -> String {
        return format!("{}{}{}",
            self.encode_position(action.from),
            self.encode_position(action.to),
            self.game.pieces[action.piece_type].format_info(self, action.info)
        );
    }

    pub fn decode_action(&mut self, action: &str, mode: u32) -> Option<Action> {
        self.generate_legal_moves(mode)
            .iter()
            .find(|el| self.encode_action(el) == action)
            .map(|el| el.clone())
    }
}
