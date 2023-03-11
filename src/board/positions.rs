use super::{actions::{HistoryMove, Action, UndoMoveError}, edges::Edges, pieces::Piece, game::Game, Board};

const COLS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Board {
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
}
