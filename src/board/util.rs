use crate::{BitSet, Piece, Edges, HistoryMove, generate_edge_list, Action, Game};

pub type BitBoard = BitSet::<1>;
pub type PieceType = usize;

/// I doubt anyone would be practically creating boards of 340,282,366,920,938,463,463,374,607,431,768,211,456 x 340,282,366,920,938,463,463,374,607,431,768,211,456.
/// However, storing these as u128s makes it much easier to interface the bitboards with (particularly, shifting bits with them.)
pub type Rows = u128;
pub type Cols = u128;

pub struct BoardState {
    /// Blockers is a BitBoard of all pieces, because keeping this bitboard ready makes it much easier to calculate movement for slider pieces.
    pub blockers: BitBoard,
    pub first_move: BitBoard,
    pub pieces: Vec<BitBoard>,
    pub teams: Vec<BitBoard>,

    pub moving_team: u32,

    /// Edges is a list of "boundary bitboards" for validating the movement of delta pieces (pieces that move in a fixed way everytime)
    pub edges: Vec<Edges>,
    pub rows: Rows,
    pub cols: Cols,

    pub history: Vec<HistoryMove>
} 

impl BoardState {
    pub fn get_piece_team_board(&self, piece: usize, team: usize) -> BitBoard {
        self.pieces[piece] & &self.teams[team]
    }
}

pub type AttackDirections = Vec<BitBoard>;


/// AttackLookup is indexed by the index of the Most Significant 1-Bit.
/// 
/// It stores an `AttackDirections` (alias for `Vec<BitBoard>`).
///     For pieces that always move the same way (like Delta Pieces), only the first slot of this AttackDirections is used, because there's no directions.
///     For slider pieces, there are different indexes for specific ray directions of it.

pub type AttackLookup = Vec<AttackDirections>;

pub struct Board {
    pub state: BoardState,
    pub game: Game,
    pub attack_lookup: Vec<AttackLookup>
}

impl Board {
    pub fn empty(game: Game, teams: u128, (rows, cols): (Rows, Cols)) -> Board {
        let pieces_state = game.pieces.iter().map(|_| BitBoard::new()).collect::<Vec<_>>().clone();

        let mut board = Board {
            attack_lookup: vec![],
            game,
            state: BoardState {
                blockers: BitBoard::new(),
                first_move: BitBoard::new(),
                pieces: pieces_state.clone(),
                teams: (0..teams).map(|_| BitBoard::new()).collect::<Vec<_>>(),
                edges: generate_edge_list(rows, cols),
                cols,
                rows,
                history: vec![],
                moving_team: 0
            }
        };

        board.generate_lookups();

        board
    }

    pub fn new(game: Game, teams: u128, (rows, cols): (Rows, Cols), fen: &str) -> Board {
        let pieces = game.pieces.iter().map(|el| el.duplicate()).collect::<Vec<_>>();

        let mut board = Board::empty(
            game, 
            teams, (rows, cols)
        );

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
                let piece_type = pieces.iter().position(|piece| piece.get_piece_symbol() == lower_char).unwrap();

                let mut first_move = true;

                if let Some(next_char) = chars.get(i + 1) {
                    if next_char == &'!' {
                        first_move = false;
                        i += 1;
                    }
                }

                let mut team: u32 = if char.is_ascii_uppercase() { 0 } else { 1 };

                if let Some(next_char) = chars.get(i + 1) {
                    if next_char == &'{' {
                        team = chars.get(i + 2).unwrap().to_digit(10).unwrap() - 1;
                        i += 3;
                    }
                }

                let piece_board = BitBoard::from_lsb(board_ind);

                board.state.teams[team as usize] |= &piece_board;
                board.state.pieces[piece_type] |= &piece_board;
                board.state.blockers |= &piece_board;
                if first_move {
                    board.state.first_move |= &piece_board;
                }
                
                board_ind += 1;
                i += 1;
            }
        }

        board
    }

    pub fn get_move_mask(&self, team: u32) -> BitBoard {
        let board_len = self.state.rows * self.state.cols;
        let mut bitboard = BitBoard::new();

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & &self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];
            
            for bit in board.iter_one_bits(board_len as u32) {
                bitboard |= &piece.get_moves(self, BitBoard::from_lsb(bit), team);
            }
        }

        bitboard
    }

    pub fn is_attacked(&self, team: u32, from: BitBoard) -> bool {
        (self.get_move_mask(team) & &from).is_set()
    }

    pub fn generate_moves(&self, team: u32) -> Vec<Action> {
        let board_len = self.state.rows * self.state.cols;
        let mut actions: Vec<Action> = Vec::with_capacity(board_len as usize);

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & &self.state.teams[team as usize];
            let piece = &self.game.pieces[ind];
            
            for bit in board.iter_one_bits(board_len as u32) {
                piece.add_actions(&mut actions, self, bit, team);
            }
        }

        actions
    }

    /*
        Don't use when writing an engine directly; use `generate_moves` and `move_restrictions.is_legal` to avoid extra legality checks during pruning.
    */
    pub fn generate_legal_moves(&mut self, team: u32) -> Vec<Action> {
        let moves = self.generate_moves(team);
        let game_restrictions = self.game.move_restrictions.duplicate();
        moves.iter().map(|el| el.clone()).filter(|el| game_restrictions.is_legal(self, el)).collect::<Vec<_>>()
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

        if team < 0 {
            (self.state.teams.len() - 1) as u32
        } else {
            team
        }
    }
}