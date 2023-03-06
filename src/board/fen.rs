use crate::{BitBoard, PieceSymbol, Board, Game, Cols, Rows};

impl Board {
    pub fn new(game: Game, teams: u128, (rows, cols): (Rows, Cols), fen: &str) -> Board {
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
                    board_ind += char.to_digit(10).unwrap();
                    i += 1;
                    continue;
                }
    
                let lower_char = char.to_ascii_lowercase();
                let piece_type = pieces
                    .iter()
                    .position(|piece| match piece.get_piece_symbol() {
                        PieceSymbol::Char(char) => char == lower_char,
                        PieceSymbol::TeamSymbol(chars) => chars.contains(&lower_char)
                    })
                    .unwrap();
    
                let mut team: u32 = if char.is_ascii_uppercase() { 0 } else { 1 };
    
                if let PieceSymbol::TeamSymbol(chars) = board.game.pieces[piece_type].get_piece_symbol() {
                    team = chars.iter().position(|el| el == &lower_char).unwrap() as u32;
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
                        team = chars.get(i + 2).unwrap().to_digit(10).unwrap() - 1;
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
}