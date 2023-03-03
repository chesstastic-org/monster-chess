use crate::{BitSet, Piece, Edges, HistoryMove, generate_edge_list, Action};

pub type BitBoard = BitSet::<1>;
pub type PieceType = usize;

/*
    I doubt anyone would be practically creating boards of 340,282,366,920,938,463,463,374,607,431,768,211,456 x 340,282,366,920,938,463,463,374,607,431,768,211,456.
    However, storing these as u128s makes it much easier to interface the bitboards with (particularly, shifting bits with them.)
*/
pub type Rows = u128;
pub type Cols = u128;

pub struct BoardState {
    /*
        Blockers is a BitBoard of all pieces, because keeping this bitboard ready makes it much easier to calculate movement for slider pieces.
    */
    pub blockers: BitBoard,
    pub first_move: BitBoard,
    pub pieces: Vec<BitBoard>,
    pub teams: Vec<BitBoard>,
    /*
        Edges is a list of "boundary bitboards" for validating the movement of delta pieces (pieces that move in a fixed way everytime)
    */
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

/*
    AttackLookup is indexed by the index of the Most Significant 1-Bit.

    It stores an `AttackDirections` (alias for `Vec<BitBoard>`).
        For pieces that always move the same way (like Delta Pieces), only the first slot of this AttackDirections is used, because there's no directions.
        For slider pieces, there are different indexes for specific ray directions of it.
*/
pub type AttackLookup = Vec<AttackDirections>;

pub struct Board {
    pub state: BoardState,
    pub pieces: Vec<Box<dyn Piece>>,
    pub attack_lookup: Vec<AttackLookup>
}

/*
    This is a special generalized FEN format.

    We assume two modes of this:
        2-Teams Mode, where capitalized means white team and lowercase means block.
        3+ Teams Mode, where the team of a piece is specialized via {} around the piece
            Eg. P{2} is a Pawn of Team 2

    "!" must be placed after a piece to show that it is not its first move
        P! would be a pawn that has moved before

    Note: "!" must be before "{"

    For the normal chess game itself, a special constructor will be provided to create an 8x8 chess board.

    The rest of this should follow the same FEN format that the rest of chess does.
*/

impl Board {
    pub fn empty(pieces: Vec<Box<dyn Piece>>, teams: u128, (rows, cols): (Rows, Cols)) -> Board {
        let pieces_state = &pieces.iter().map(|_| BitBoard::new()).collect::<Vec<_>>().clone();

        let mut board = Board {
            attack_lookup: vec![],
            pieces,
            state: BoardState {
                blockers: BitBoard::new(),
                first_move: BitBoard::new(),
                pieces: pieces_state.clone(),
                teams: (0..teams).map(|_| BitBoard::new()).collect::<Vec<_>>(),
                edges: generate_edge_list(rows, cols),
                cols,
                rows,
                history: vec![]
            }
        };

        board.generate_lookups();

        board
    }

    pub fn new(pieces: Vec<Box<dyn Piece>>, teams: u128, (rows, cols): (Rows, Cols), fen: &str) -> Board {
        let mut board = Board::empty(
            pieces.iter().map(|el| el.duplicate()).collect::<Vec<_>>(), 
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

    pub fn generate_moves(&self, team: u32) -> Vec<Action> {
        let board_len = self.state.rows * self.state.cols;
        let mut actions: Vec<Action> = Vec::with_capacity(board_len as usize);

        for (ind, board) in self.state.pieces.iter().enumerate() {
            let board = *board & &self.state.teams[team as usize];
            let piece = &self.pieces[ind];
            
            for bit in board.iter_one_bits(board_len as u32) {
                piece.add_actions(&mut actions, self, bit, team);
            }
        }

        actions
    }
}