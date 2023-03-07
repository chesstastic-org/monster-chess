use crate::{BitBoard, Board, Cols, Game, PieceSymbol, Rows};

impl Board {
    pub(crate) fn from_fen_state(
        game: Game,
        teams: u128,
        (rows, cols): (Rows, Cols),
        fen: &str,
    ) -> Board {
        let pieces = game
            .pieces
            .iter()
            .map(|el| el.duplicate())
            .collect::<Vec<_>>();

        let mut board = Board::empty(game, teams, (rows, cols));

        let mut board_ind = 0;
        for row in fen.split("/") {
            let chars = row.chars().collect::<Vec<_>>();
            let mut i = 0;
            while i < chars.len() {
                let char = chars[i];

                if char.is_numeric() {
                    board_ind += char.to_digit(10).expect(&format!(
                        "Could not convert {char} to an integer in FEN state."
                    ));
                    i += 1;
                    continue;
                }

                let lower_char = char.to_ascii_lowercase();
                let piece_type = pieces
                    .iter()
                    .position(|piece| match piece.get_piece_symbol() {
                        PieceSymbol::Char(char) => char == lower_char,
                        PieceSymbol::TeamSymbol(teams) => teams.contains(&char),
                    })
                    .expect(&format!("Could not find the piece type of '{char}'"));

                let mut team: u32 = if char.is_ascii_uppercase() { 0 } else { 1 };

                if let PieceSymbol::TeamSymbol(chars) =
                    board.game.pieces[piece_type].get_piece_symbol()
                {
                    team = chars
                        .iter()
                        .position(|el| el == &char)
                        .expect(&format!("Could not find the team of '{char}'"))
                        as u32;
                }

                let mut first_move = true;

                if let Some(next_char) = chars.get(i + 1) {
                    if next_char == &'!' {
                        first_move = false;
                        i += 1;
                    }
                }

                if let Some(next_char) = chars.get(i + 1) {
                    if next_char == &'{' {
                        team = chars
                            .get(i + 2)
                            .expect("Could not get the team character after braces in a FEN.")
                            .to_digit(10)
                            .expect(
                                "The team char specified after braces in the FEN was not a number.",
                            )
                            - 1;
                        i += 3;
                    }
                }

                let piece_board = BitBoard::from_lsb(board_ind);

                board.state.teams[team as usize] |= &piece_board;
                board.state.pieces[piece_type] |= &piece_board;
                board.state.all_pieces |= &piece_board;
                if first_move {
                    board.state.first_move |= &piece_board;
                }

                board_ind += 1;
                i += 1;
            }
        }

        board
    }

    pub(crate) fn to_fen_state(&self) -> String {
        let board_len = self.state.cols * self.state.rows;
        let mut fen_state = "".to_string();
        let mut empty_spaces = 0;
        for i in 0..(board_len + 1) {
            if i == board_len {
                if empty_spaces > 0 {
                    fen_state.push_str(&empty_spaces.to_string());
                }
                continue;
            }

            if i > 0 && i % self.state.cols == 0 {
                if empty_spaces > 0 {
                    fen_state.push_str(&empty_spaces.to_string());
                }
                fen_state.push_str("/");
                empty_spaces = 0;
            }

            let bitboard = BitBoard::from_lsb(i);

            if (self.state.all_pieces & &bitboard).is_empty() {
                empty_spaces += 1;
                continue;
            }

            if empty_spaces > 0 {
                fen_state.push_str(&empty_spaces.to_string());
            }
            empty_spaces = 0;

            let mut team = 0;
            for ind in 0..self.state.teams.len() {
                if (self.state.teams[ind] & &bitboard).is_set() {
                    team = ind;
                }
            }

            let mut piece_type = usize::MAX;
            for ind in 0..self.state.pieces.len() {
                if (self.state.pieces[ind] & &bitboard).is_set() {
                    piece_type = ind;
                    break;
                }
            }

            let first_move = (self.state.first_move & &bitboard).is_set();

            let mut piece_str = match self.game.pieces[piece_type].get_piece_symbol() {
                PieceSymbol::Char(char) => {
                    if self.state.teams.len() > 2 {
                        format!("{}{{{}}}", char.to_ascii_lowercase(), team)
                    } else if team == 0 {
                        char.to_ascii_uppercase().to_string()
                    } else {
                        char.to_ascii_lowercase().to_string()
                    }
                }
                PieceSymbol::TeamSymbol(teams) => teams[team].to_string(),
            };
            if self.game.fen_options.state.first_moves && !first_move {
                piece_str.push_str("!");
            }

            fen_state.push_str(&piece_str);
        }

        fen_state
    }
}
